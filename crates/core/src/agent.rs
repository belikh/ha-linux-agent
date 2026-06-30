use crate::config::Config;
use crate::discovery::{availability_topic, command_discovery, command_topic, sensor_discovery, state_topic};
use crate::model::DeviceInfo;
use crate::traits::{CommandBackend, SensorBackend};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn};

pub struct Agent {
    config: Config,
    sensor_backends: Vec<Box<dyn SensorBackend>>,
    command_backends: Vec<Box<dyn CommandBackend>>,
}

impl Agent {
    pub fn new(
        config: Config,
        sensor_backends: Vec<Box<dyn SensorBackend>>,
        command_backends: Vec<Box<dyn CommandBackend>>,
    ) -> Self {
        Self {
            config,
            sensor_backends,
            command_backends,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let device_id = self.config.device.id.clone();
        let prefix = self.config.mqtt.discovery_prefix.clone();
        let poll_interval = Duration::from_secs(self.config.mqtt.poll_interval_secs.max(1));

        let device = DeviceInfo {
            identifiers: vec![device_id.clone()],
            name: self.config.device.name.clone(),
            model: "ha-linux-agent".to_string(),
            manufacturer: "ha-linux-agent".to_string(),
            sw_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let mut opts = MqttOptions::new(format!("ha-linux-agent-{device_id}"), self.config.mqtt.host.clone(), self.config.mqtt.port);
        opts.set_keep_alive(Duration::from_secs(30));
        if let Some(user) = &self.config.mqtt.username {
            let pass = self.config.mqtt.resolve_password()?.unwrap_or_default();
            opts.set_credentials(user, pass);
        }
        let avail_topic = availability_topic(&device_id);
        opts.set_last_will(rumqttc::LastWill::new(&avail_topic, "offline", QoS::AtLeastOnce, true));

        let (client, mut eventloop) = AsyncClient::new(opts, 64);

        // command_topic -> (backend index, command id)
        let mut command_routes: HashMap<String, (usize, String)> = HashMap::new();
        for (idx, backend) in self.command_backends.iter().enumerate() {
            for cmd in backend.commands() {
                let topic = command_topic(&device_id, &cmd.id);
                command_routes.insert(topic.clone(), (idx, cmd.id.clone()));
            }
        }

        let client_for_poll = client.clone();
        let client_for_setup = client.clone();
        let avail_topic_setup = avail_topic.clone();
        let command_routes_keys: Vec<String> = command_routes.keys().cloned().collect();

        let sensor_descriptors: Vec<_> = self
            .sensor_backends
            .iter()
            .flat_map(|b| b.sensors())
            .collect();
        let command_descriptors: Vec<_> = self
            .command_backends
            .iter()
            .flat_map(|b| b.commands())
            .collect();

        let device_for_setup = device.clone();
        let prefix_for_setup = prefix.clone();
        let device_id_for_setup = device_id.clone();
        let _setup_task = tokio::spawn(async move {
            for d in &sensor_descriptors {
                let (topic, payload) = sensor_discovery(&prefix_for_setup, &device_for_setup, &device_id_for_setup, d);
                if let Err(e) = client_for_setup
                    .publish(topic, QoS::AtLeastOnce, true, serde_json::to_vec(&payload).unwrap())
                    .await
                {
                    warn!("publishing discovery for sensor {}: {e}", d.id);
                }
            }
            for d in &command_descriptors {
                let (topic, payload) = command_discovery(&prefix_for_setup, &device_for_setup, &device_id_for_setup, d);
                if let Err(e) = client_for_setup
                    .publish(topic, QoS::AtLeastOnce, true, serde_json::to_vec(&payload).unwrap())
                    .await
                {
                    warn!("publishing discovery for command {}: {e}", d.id);
                }
            }
            for topic in &command_routes_keys {
                if let Err(e) = client_for_setup.subscribe(topic, QoS::AtLeastOnce).await {
                    warn!("subscribing to {topic}: {e}");
                }
            }
            if let Err(e) = client_for_setup
                .publish(&avail_topic_setup, QoS::AtLeastOnce, true, "online")
                .await
            {
                warn!("publishing availability: {e}");
            }
        });

        let state_topic = state_topic(&device_id);
        let sensor_backends = self.sensor_backends;
        let _poll_task = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(poll_interval);
            loop {
                ticker.tick().await;
                let mut merged = serde_json::Map::new();
                for backend in &sensor_backends {
                    for state in backend.poll().await {
                        merged.insert(state.id, state.value);
                    }
                }
                let payload = serde_json::Value::Object(merged);
                if let Err(e) = client_for_poll
                    .publish(&state_topic, QoS::AtLeastOnce, true, serde_json::to_vec(&payload).unwrap())
                    .await
                {
                    warn!("publishing state: {e}");
                }
            }
        });

        let command_backends = self.command_backends;
        info!(device = %device_id, "ha-linux-agent connecting to {}:{}", self.config.mqtt.host, self.config.mqtt.port);

        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let topic = p.topic.clone();
                    if let Some((backend_idx, command_id)) = command_routes.get(&topic) {
                        let payload = String::from_utf8_lossy(&p.payload).to_string();
                        let backend = &command_backends[*backend_idx];
                        info!(command = %command_id, "dispatching command");
                        if let Err(e) = backend.handle(command_id, &payload).await {
                            warn!("command {command_id} failed: {e}");
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    warn!("mqtt connection error: {e}");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}
