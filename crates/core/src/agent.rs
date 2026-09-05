use crate::config::Config;
use crate::discovery::{
    availability_topic, command_discovery, command_topic, owned_topics, sensor_discovery,
    state_topic,
};
use crate::model::DeviceInfo;
use crate::sd_notify;
use crate::traits::{CommandBackend, SensorBackend};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, Transport};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{watch, Notify};
use tokio::task::JoinSet;
use tracing::{info, warn};

/// Per-backend poll and per-command execution bound. A backend that hangs
/// (a wedged daemon, a stalled D-Bus call) is skipped for this tick — it
/// can never freeze the shared state publish again.
const BACKEND_TIMEOUT: Duration = Duration::from_secs(5);
/// Bound on any single publish against the cap-0 request channel. With
/// capacity 0 (rumqttc's guidance when the app manages its own pending
/// state) a publish rendezvouses with the eventloop; if the eventloop is
/// stalled, the tick is dropped rather than queued as stale telemetry.
const PUBLISH_TIMEOUT: Duration = Duration::from_secs(5);
/// Reconnect backoff bounds: exponential 1 s → 60 s with ±20 % jitter so a
/// broker restart cannot synchronise seven hosts into a reconnect stampede.
const BACKOFF_MIN: Duration = Duration::from_secs(1);
const BACKOFF_MAX: Duration = Duration::from_secs(60);

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

    /// Run until SIGTERM/SIGINT: retained `offline` then a clean
    /// `disconnect()` on exit (the broker discards the last will on a
    /// graceful disconnect, so the offline must be published explicitly).
    pub async fn run(self) -> anyhow::Result<()> {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        tokio::spawn(async move {
            shutdown_signal().await;
            let _ = shutdown_tx.send(true);
        });
        self.run_with_shutdown(shutdown_rx).await
    }

    /// The supervisor loop. Public so integration tests drive the shutdown
    /// channel directly instead of raising process signals.
    ///
    /// The whole lifecycle hangs on one structural rule: **the connection
    /// is driven from ConnAck, not from startup**. rumqttc reconnects
    /// forever but never re-establishes subscriptions (bytebeamio/rumqtt
    /// #250, open since 2021) and never re-asserts availability, so every
    /// connect — first or reconnect, any `session_present` — re-issues the
    /// command-topic subscribes, re-publishes retained `online`, and asks
    /// the poll task for an immediate fresh state publish.
    pub async fn run_with_shutdown(
        self,
        mut shutdown: watch::Receiver<bool>,
    ) -> anyhow::Result<()> {
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

        let mut opts = MqttOptions::new(
            format!("ha-linux-agent-{device_id}"),
            self.config.mqtt.host.clone(),
            self.config.mqtt.port,
        );
        opts.set_keep_alive(Duration::from_secs(30));
        // Explicit, stable-clean sessions: a persistent session's only
        // unique deliverable is broker-side queueing of QoS 1 commands for
        // an offline agent — a week-old launcher toggle replaying as fresh
        // on a kiosk's Monday boot, with no TTL in MQTT v3.1.1 to age it
        // by. ConnAck-driven re-subscribe makes session state unnecessary.
        opts.set_clean_session(true);
        if let Some(user) = &self.config.mqtt.username {
            let pass = self.config.mqtt.resolve_password()?.unwrap_or_default();
            opts.set_credentials(user, pass);
        }
        if self.config.mqtt.tls {
            let ca = self
                .config
                .mqtt
                .ca_file
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("mqtt.tls = true requires mqtt.ca_file (PEM path) — rumqttc builds its root store from these bytes"))?;
            let pem = std::fs::read(ca)
                .map_err(|e| anyhow::anyhow!("reading mqtt.ca_file {ca}: {e}"))?;
            opts.set_transport(Transport::tls(pem, None, None));
        }

        let avail_topic = availability_topic(&device_id);
        opts.set_last_will(rumqttc::LastWill::new(
            &avail_topic,
            "offline",
            QoS::AtLeastOnce,
            true,
        ));

        // Capacity 10, NOT 0. A zero-capacity rendezvous channel deadlocks
        // with rumqttc 0.24's drain discipline: the eventloop only drains
        // pending requests via try_recv at poll() ENTRY, and while poll()
        // is parked awaiting the next INCOMING packet, a sender parked on
        // the rendezvous never completes — and with no subscriptions yet
        // established (the ConnAck subscribe itself is the parked sender)
        // no incoming packet ever arrives, so the first subscribe hangs
        // forever and every publish queued behind it with it. Observed as
        // connected-but-silent (broker sees the client, keepalives flow,
        // no subscribe/publish ever lands) — reproduced deterministically
        // in the runNixOSTest module VM and on a live host, racing on
        // spawn-vs-park timing. A small bounded capacity keeps every send
        // immediate while still bounding stale telemetry to a handful of
        // messages; the retained state topic self-heals by construction (a
        // fresh publish overwrites whatever went stale), and the
        // per-publish PUBLISH_TIMEOUT bounds a wedged eventloop regardless.
        let (client, mut eventloop) = AsyncClient::new(opts, 10);

        // -- Static surface, computed once so decommission and discovery
        //    can never drift apart. --
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
        let discovery_messages: Vec<(String, Vec<u8>)> = sensor_descriptors
            .iter()
            .map(|d| sensor_discovery(&prefix, &device, &device_id, d))
            .chain(
                command_descriptors
                    .iter()
                    .filter(|d| d.discoverable)
                    .map(|d| command_discovery(&prefix, &device, &device_id, d)),
            )
            .map(|(topic, payload)| (topic, serde_json::to_vec(&payload).unwrap()))
            .collect();

        // command_topic -> (backend index, command id)
        let mut command_routes: HashMap<String, (usize, String)> = HashMap::new();
        let command_backends: Vec<Arc<dyn CommandBackend>> = self
            .command_backends
            .into_iter()
            .map(Arc::from)
            .collect();
        for (idx, backend) in command_backends.iter().enumerate() {
            for cmd in backend.commands() {
                let topic = command_topic(&device_id, &cmd.id);
                command_routes.insert(topic, (idx, cmd.id.clone()));
            }
        }

        // HA publishes `online` here at startup (default birth topic);
        // replying with our discovery is HA's documented re-announcement
        // contract for discovery devices.
        let birth_topic = format!("{prefix}/status");
        let mut subscribe_topics: Vec<String> = command_routes.keys().cloned().collect();
        subscribe_topics.push(birth_topic.clone());

        let sensor_backends: Vec<Arc<dyn SensorBackend>> =
            self.sensor_backends.into_iter().map(Arc::from).collect();

        // -- Poll task: interval ticks plus on-demand refreshes (ConnAck),
        //    every backend isolated under BACKEND_TIMEOUT, merged into one
        //    retained state publish. --
        let state_topic = state_topic(&device_id);
        let poll_client = client.clone();
        let mut poll_shutdown = shutdown.clone();
        let poll_now = Arc::new(Notify::new());
        let poll_now_connack = poll_now.clone();
        let watchdog = sd_notify::watchdog_interval();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(poll_interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                tokio::select! {
                    _ = ticker.tick() => {}
                    _ = poll_now.notified() => {}
                    _ = poll_shutdown.changed() => return,
                }
                let mut jobs: JoinSet<Vec<crate::model::SensorState>> = JoinSet::new();
                for backend in &sensor_backends {
                    let backend = backend.clone();
                    jobs.spawn(async move {
                        match tokio::time::timeout(BACKEND_TIMEOUT, backend.poll()).await {
                            Ok(states) => states,
                            Err(_) => {
                                warn!(backend = backend.id(), "poll timed out, skipping this tick");
                                Vec::new()
                            }
                        }
                    });
                }
                let mut merged = serde_json::Map::new();
                while let Some(res) = jobs.join_next().await {
                    for state in res.unwrap_or_default() {
                        merged.insert(state.id, state.value);
                    }
                }
                let payload = serde_json::Value::Object(merged);
                match tokio::time::timeout(
                    PUBLISH_TIMEOUT,
                    poll_client.publish(&state_topic, QoS::AtLeastOnce, true, serde_json::to_vec(&payload).unwrap()),
                )
                .await
                {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => warn!("publishing state: {e}"),
                    Err(_) => warn!("state publish timed out (eventloop stalled?), dropping tick"),
                }
                // The watchdog rides the poll loop deliberately: a wedged
                // loop stops pinging and systemd restarts the agent. Pings
                // every turn are far below the needed cadence in practice
                // (WatchdogSec 15 min vs a 30 s default tick); a poll
                // interval slower than the watchdog window means the loop
                // genuinely cannot keep the service alive — restart is the
                // correct outcome.
                if watchdog.is_some() {
                    let _ = sd_notify::watchdog_ping().await;
                }
            }
        });

        let sd = sd_notify::notify_ready().await;
        if sd.is_err() {
            // Normal outside systemd (manual runs, tests): not even worth a
            // warn line in the common case.
            tracing::debug!("sd_notify READY not sent (no NOTIFY_SOCKET)");
        }

        info!(device = %device_id, "ha-linux-agent connecting to {}:{}", self.config.mqtt.host, self.config.mqtt.port);

        let announced = Arc::new(AtomicBool::new(false));
        let mut backoff = BACKOFF_MIN;
        loop {
            let ev = tokio::select! {
                ev = eventloop.poll() => ev,
                _ = shutdown.changed() => {
                    // Deliberate stop: publish retained offline explicitly —
                    // the broker drops the stored will on a graceful
                    // DISCONNECT, so without this availability would stick
                    // `online` forever (ESPHome's shutdown_message pattern).
                    // The publish rides a spawned task while this loop keeps
                    // polling: with the cap-0 rendezvous channel the send
                    // only completes once the eventloop drains it.
                    let c2 = client.clone();
                    let avail2 = avail_topic.clone();
                    tokio::spawn(async move {
                        let _ = tokio::time::timeout(
                            PUBLISH_TIMEOUT,
                            c2.publish(&avail2, QoS::AtLeastOnce, true, "offline"),
                        )
                        .await;
                        let _ = tokio::time::timeout(PUBLISH_TIMEOUT, c2.disconnect()).await;
                    });
                    // Drain until the disconnect lands (poll errors once the
                    // connection closes), bounded.
                    while let Ok(Ok(_)) =
                        tokio::time::timeout(PUBLISH_TIMEOUT, eventloop.poll()).await
                    {}
                    info!("shutdown: published offline, disconnected");
                    return Ok(());
                }
            };
            match ev {
                Ok(Event::Incoming(Packet::ConnAck(_))) => {
                    backoff = BACKOFF_MIN;
                    let client2 = client.clone();
                    let avail = avail_topic.clone();
                    let subs = subscribe_topics.clone();
                    let discovery = discovery_messages.clone();
                    let poll_now = poll_now_connack.clone();
                    let announced = announced.clone();
                    let state_dir = PathBuf::from(self.config.state_dir.clone());
                    let prefix2 = prefix.clone();
                    let device_id2 = device_id.clone();
                    let sensors = sensor_descriptors.clone();
                    let commands = command_descriptors.clone();
                    // Never block the eventloop while iterating (rumqttc's
                    // cardinal rule) — the connect work is a spawned task
                    // whose requests buffer rendezvous-style into the loop.
                    tokio::spawn(async move {
                        for topic in &subs {
                            if let Err(e) = client2.subscribe(topic, QoS::AtLeastOnce).await {
                                warn!("subscribing to {topic}: {e}");
                            }
                        }
                        if let Err(e) = tokio::time::timeout(
                            PUBLISH_TIMEOUT,
                            client2.publish(&avail, QoS::AtLeastOnce, true, "online"),
                        )
                        .await
                        {
                            warn!("re-asserting availability: {e:?}");
                        }
                        // Fresh retained state right away — a reconnect
                        // after an outage should not wait a full interval to
                        // overwrite whatever went stale while we were down.
                        poll_now.notify_one();
                        if !announced.swap(true, Ordering::SeqCst) {
                            publish_discovery(&client2, &discovery).await;
                            record_discovery(&state_dir, &prefix2, &device_id2, &sensors, &commands, &client2).await;
                        }
                    });
                }
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let topic = p.topic.clone();
                    let payload = String::from_utf8_lossy(&p.payload).to_string();
                    if topic == birth_topic && payload == "online" {
                        // Home Assistant (re)started: entities sit unavailable
                        // until discovery arrives again — republish after a
                        // per-host jitter (HA's own IO-spike recommendation;
                        // Zigbee2MQTT's 0–30 s reference pattern).
                        let client2 = client.clone();
                        let discovery = discovery_messages.clone();
                        let jitter = jitter_duration(self.config.mqtt.discovery_jitter_secs);
                        tokio::spawn(async move {
                            if jitter > Duration::ZERO {
                                tokio::time::sleep(jitter).await;
                            }
                            publish_discovery(&client2, &discovery).await;
                        });
                    } else if let Some((backend_idx, command_id)) = command_routes.get(&topic) {
                        let backend = command_backends[*backend_idx].clone();
                        let id = command_id.clone();
                        info!(command = %id, "dispatching command");
                        tokio::spawn(async move {
                            match tokio::time::timeout(BACKEND_TIMEOUT, backend.handle(&id, &payload)).await {
                                Ok(Ok(())) => {}
                                Ok(Err(e)) => warn!("command {id} failed: {e}"),
                                Err(_) => warn!("command {id} timed out"),
                            }
                        });
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    warn!("mqtt connection error: {e}");
                    tokio::time::sleep(jitter_backoff(backoff)).await;
                    backoff = (backoff * 2).min(BACKOFF_MAX);
                }
            }
        }
    }

    /// `--decommission`: publish zero-length retained payloads to every
    /// owned topic — state and availability first, discovery configs last —
    /// which is HA's official entity-removal semantic. Exits after
    /// clearing; safe to run while the agent service is stopped.
    pub async fn run_decommission(self) -> anyhow::Result<()> {
        let device_id = self.config.device.id.clone();
        let prefix = self.config.mqtt.discovery_prefix.clone();
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

        let mut opts = MqttOptions::new(
            format!("ha-linux-agent-decommission-{device_id}"),
            self.config.mqtt.host.clone(),
            self.config.mqtt.port,
        );
        opts.set_clean_session(true);
        if let Some(user) = &self.config.mqtt.username {
            let pass = self.config.mqtt.resolve_password()?.unwrap_or_default();
            opts.set_credentials(user, pass);
        }
        if self.config.mqtt.tls {
            let ca = self
                .config
                .mqtt
                .ca_file
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("mqtt.tls = true requires mqtt.ca_file"))?;
            let pem = std::fs::read(ca)
                .map_err(|e| anyhow::anyhow!("reading mqtt.ca_file {ca}: {e}"))?;
            opts.set_transport(Transport::tls(pem, None, None));
        }
        // One-shot path: a bounded channel is fine here (the cap-0
        // discipline exists to bound stale telemetry in the long-running
        // loop; decommission clears and exits).
        let (client, mut eventloop) = AsyncClient::new(opts, 64);

        let topics = owned_topics(
            &prefix,
            &device_id,
            &sensor_descriptors,
            &command_descriptors,
        );
        info!(device = %device_id, count = topics.len(), "decommission: clearing owned topics");

        let expected = topics.len();
        let mut publishing = false;
        let mut acks = 0usize;
        let mut disconnecting = false;
        loop {
            if disconnecting {
                // Drain until the disconnect lands (poll errors once the
                // connection closes), bounded so a silent broker cannot
                // hang a decommission.
                match tokio::time::timeout(Duration::from_secs(2), eventloop.poll()).await {
                    Ok(Ok(_)) => continue,
                    Ok(Err(_)) | Err(_) => break,
                }
            }
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::ConnAck(_))) => {
                    // Spawned: the publishes must not block the poll loop
                    // that drains them.
                    let c2 = client.clone();
                    let topics2 = topics.clone();
                    tokio::spawn(async move {
                        for topic in &topics2 {
                            if let Err(e) = c2.publish(topic, QoS::AtLeastOnce, true, "").await {
                                warn!("decommission: clearing {topic}: {e}");
                            }
                        }
                    });
                    publishing = true;
                }
                Ok(Event::Incoming(Packet::PubAck(_))) => {
                    acks += 1;
                    if publishing && acks >= expected {
                        client.disconnect().await?;
                        disconnecting = true;
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    if publishing && acks >= expected {
                        break;
                    }
                    warn!("decommission: mqtt error: {e}");
                    tokio::time::sleep(jitter_backoff(BACKOFF_MIN)).await;
                }
            }
        }
        // Every cleared topic was PUBACKed before the disconnect — the
        // broker holds the zero-length retained payloads now. Manifest
        // gone: a later start re-announces from scratch.
        let manifest = PathBuf::from(self.config.state_dir.clone()).join("last-discovery.json");
        let _ = std::fs::remove_file(manifest);
        info!("decommission complete");
        Ok(())
    }
}

async fn publish_discovery(client: &AsyncClient, messages: &[(String, Vec<u8>)]) {
    for (topic, payload) in messages {
        match tokio::time::timeout(
            PUBLISH_TIMEOUT,
            client.publish(topic, QoS::AtLeastOnce, true, payload.clone()),
        )
        .await
        {
            Ok(Ok(())) => {}
            Ok(Err(e)) => warn!("publishing discovery to {topic}: {e}"),
            Err(_) => warn!("discovery publish to {topic} timed out"),
        }
    }
}

/// Opportunistic decommission: if the descriptor set changed since the last
/// run (a backend disabled, a pool renamed), the topics that vanished get
/// zero-length retained payloads so HA drops the stale entities instead of
/// resurrecting them from retained configs on its next restart.
async fn record_discovery(
    state_dir: &std::path::Path,
    prefix: &str,
    device_id: &str,
    sensors: &[crate::model::SensorDescriptor],
    commands: &[crate::model::CommandDescriptor],
    client: &AsyncClient,
) {
    let current = owned_topics(prefix, device_id, sensors, commands);
    let manifest = state_dir.join("last-discovery.json");
    let previous: Vec<String> = std::fs::read(&manifest)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default();
    for topic in previous.iter().filter(|t| !current.contains(t)) {
        match tokio::time::timeout(
            PUBLISH_TIMEOUT,
            client.publish(topic, QoS::AtLeastOnce, true, ""),
        )
        .await
        {
            Ok(Ok(())) => info!("cleared stale discovery topic {topic}"),
            Ok(Err(e)) => warn!("clearing stale topic {topic}: {e}"),
            Err(_) => warn!("clearing stale topic {topic} timed out"),
        }
    }
    if let Err(e) = std::fs::create_dir_all(state_dir) {
        warn!("creating state dir {}: {e}", state_dir.display());
        return;
    }
    if let Err(e) = serde_json::to_vec(&current)
        .map_err(anyhow::Error::new)
        .and_then(|bytes| std::fs::write(&manifest, bytes).map_err(anyhow::Error::new))
    {
        warn!("writing {}: {e}", manifest.display());
    }
}

/// 0–`max_secs`, sub-second resolution — good enough for IO-spike
/// avoidance (this is not cryptography).
fn jitter_duration(max_secs: u64) -> Duration {
    if max_secs == 0 {
        return Duration::ZERO;
    }
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let millis = nanos as u64 % (max_secs * 1000);
    Duration::from_millis(millis)
}

/// ±20 % around the nominal backoff.
fn jitter_backoff(nominal: Duration) -> Duration {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0) as f64;
    let frac = 0.8 + 0.4 * (nanos / 1_000_000_000.0);
    Duration::from_secs_f64(nominal.as_secs_f64() * frac)
}

/// Resolve SIGTERM or SIGINT, whichever arrives first.
async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut term = signal(SignalKind::terminate()).expect("installing SIGTERM handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term.recv() => {}
    }
}
