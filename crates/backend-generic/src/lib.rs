mod dbus;

use async_trait::async_trait;
use dbus::{Login1ManagerProxy, Login1SessionProxy, NotificationsProxy, UPowerDeviceProxy, UPowerProxy};
use ha_agent_core::config::GenericBackendConfig;
use ha_agent_core::model::{CommandDescriptor, SensorDescriptor, SensorState};
use ha_agent_core::{CommandBackend, SensorBackend};
use sysinfo::{Disks, System};
use tokio::sync::Mutex;
use tracing::warn;
use zbus::zvariant::OwnedObjectPath;
use zbus::Connection;

/// DE-agnostic sensors/commands: works on any Linux box with systemd-logind
/// and (optionally) UPower / a notification daemon. This is the backend
/// every install gets by default — see `ha-agent-backend-niri` for an
/// example of a DE-specific backend layered on top.
pub struct GenericBackend {
    config: GenericBackendConfig,
    sys: Mutex<System>,
    system_bus: Option<Connection>,
    session_bus: Option<Connection>,
    login1_session: Option<OwnedObjectPath>,
    battery_device: Option<OwnedObjectPath>,
}

impl GenericBackend {
    /// Always available — pure D-Bus/procfs, no DE assumptions.
    pub fn detect() -> bool {
        true
    }

    pub async fn new(config: GenericBackendConfig) -> Self {
        let system_bus = match Connection::system().await {
            Ok(c) => Some(c),
            Err(e) => {
                warn!("generic backend: no system bus, login1/UPower sensors disabled: {e}");
                None
            }
        };
        let session_bus = match Connection::session().await {
            Ok(c) => Some(c),
            Err(e) => {
                warn!("generic backend: no session bus, notifications disabled: {e}");
                None
            }
        };

        let login1_session = match &system_bus {
            Some(bus) => match Login1ManagerProxy::new(bus).await {
                Ok(mgr) => match mgr.get_session_by_pid(std::process::id()).await {
                    Ok(path) => Some(path),
                    Err(e) => {
                        warn!("generic backend: could not resolve login1 session: {e}");
                        None
                    }
                },
                Err(e) => {
                    warn!("generic backend: login1 manager proxy: {e}");
                    None
                }
            },
            None => None,
        };

        let battery_device = match &system_bus {
            Some(bus) => match UPowerProxy::new(bus).await {
                Ok(up) => match up.enumerate_devices().await {
                    // Heuristic: pick the first device whose path looks like a
                    // battery rather than a line power / display aggregate.
                    Ok(devices) => devices
                        .into_iter()
                        .find(|p| p.as_str().contains("battery")),
                    Err(e) => {
                        warn!("generic backend: UPower EnumerateDevices: {e}");
                        None
                    }
                },
                Err(e) => {
                    warn!("generic backend: UPower proxy: {e}");
                    None
                }
            },
            None => None,
        };

        // `System::new()` + targeted refreshes, never `new_all()` /
        // `refresh_all()`: sysinfo 0.32's `refresh_all` refreshes processes
        // with `remove_dead_processes = false`, so the process table (cmd,
        // environ, exe, an open `stat` fd each) gains an entry for every
        // process that ever existed and never shrinks. This backend reads
        // only global CPU and memory, so it never asks for processes.
        let mut sys = System::new();
        // Seed the CPU baseline; usage is a delta between two refreshes.
        sys.refresh_cpu_usage();
        Self {
            config,
            sys: Mutex::new(sys),
            system_bus,
            session_bus,
            login1_session,
            battery_device,
        }
    }

    async fn login1_session_proxy(&self) -> Option<Login1SessionProxy<'_>> {
        let bus = self.system_bus.as_ref()?;
        let path = self.login1_session.as_ref()?;
        Login1SessionProxy::builder(bus)
            .path(path.clone())
            .ok()?
            .build()
            .await
            .ok()
    }

    async fn battery_proxy(&self) -> Option<UPowerDeviceProxy<'_>> {
        let bus = self.system_bus.as_ref()?;
        let path = self.battery_device.as_ref()?;
        UPowerDeviceProxy::builder(bus)
            .path(path.clone())
            .ok()?
            .build()
            .await
            .ok()
    }

    /// Number of processes the sysinfo `System` is tracking. Must stay 0:
    /// a non-zero count means a process refresh crept back in and the
    /// table grows with every process the host ever spawns.
    #[cfg(test)]
    async fn tracked_process_count(&self) -> usize {
        self.sys.lock().await.processes().len()
    }

    fn disk_mounts(&self) -> Vec<String> {
        if self.config.disks.is_empty() {
            vec!["/".to_string()]
        } else {
            self.config.disks.clone()
        }
    }
}

#[async_trait]
impl SensorBackend for GenericBackend {
    fn id(&self) -> &str {
        "generic"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        let mut sensors = vec![
            SensorDescriptor::sensor("cpu_usage", "CPU Usage")
                .with_unit("%")
                .with_icon("mdi:cpu-64-bit"),
            SensorDescriptor::sensor("memory_usage", "Memory Usage")
                .with_unit("%")
                .with_icon("mdi:memory"),
            SensorDescriptor::sensor("load_1m", "Load Average (1m)")
                .with_unit("")
                .with_state_class("measurement")
                .with_icon("mdi:gauge"),
            SensorDescriptor::sensor("uptime_seconds", "Uptime")
                .with_unit("s")
                .with_icon("mdi:clock-outline"),
        ];
        for mount in self.disk_mounts() {
            let id = disk_sensor_id(&mount);
            sensors.push(
                SensorDescriptor::sensor(id, format!("Disk Usage ({mount})"))
                    .with_unit("%")
                    .with_icon("mdi:harddisk"),
            );
        }
        if self.login1_session.is_some() {
            sensors.push(
                SensorDescriptor::binary_sensor("idle", "Idle")
                    .with_device_class("running")
                    .with_icon("mdi:sleep"),
            );
            sensors.push(
                SensorDescriptor::binary_sensor("locked", "Screen Locked")
                    .with_device_class("lock")
                    .with_icon("mdi:lock"),
            );
        }
        if self.battery_device.is_some() {
            sensors.push(
                SensorDescriptor::sensor("battery_percent", "Battery")
                    .with_unit("%")
                    .with_device_class("battery")
                    .with_icon("mdi:battery"),
            );
            sensors.push(
                SensorDescriptor::binary_sensor("battery_charging", "Battery Charging")
                    .with_device_class("battery_charging")
                    .with_icon("mdi:battery-charging"),
            );
        }
        sensors
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut out = Vec::new();

        {
            let mut sys = self.sys.lock().await;
            sys.refresh_cpu_usage();
            sys.refresh_memory();
            out.push(SensorState::new("cpu_usage", round1(sys.global_cpu_usage())));
            let mem_pct = if sys.total_memory() > 0 {
                100.0 * sys.used_memory() as f64 / sys.total_memory() as f64
            } else {
                0.0
            };
            out.push(SensorState::new("memory_usage", round1(mem_pct as f32)));
        }

        let load = System::load_average();
        out.push(SensorState::new("load_1m", round1(load.one as f32)));
        out.push(SensorState::new("uptime_seconds", System::uptime()));

        let disks = Disks::new_with_refreshed_list();
        for mount in self.disk_mounts() {
            if let Some(disk) = disks
                .iter()
                .find(|d| d.mount_point().to_string_lossy() == mount)
            {
                let total = disk.total_space();
                let avail = disk.available_space();
                if total > 0 {
                    let used_pct = 100.0 * (total - avail) as f64 / total as f64;
                    out.push(SensorState::new(disk_sensor_id(&mount), round1(used_pct as f32)));
                }
            }
        }

        if let Some(session) = self.login1_session_proxy().await {
            if let Ok(idle) = session.idle_hint().await {
                out.push(SensorState::binary("idle", idle));
            }
            if let Ok(locked) = session.locked_hint().await {
                out.push(SensorState::binary("locked", locked));
            }
        }

        if let Some(battery) = self.battery_proxy().await {
            if let Ok(pct) = battery.percentage().await {
                out.push(SensorState::new("battery_percent", round1(pct as f32)));
            }
            if let Ok(state) = battery.state().await {
                // UPower device state: 1 = charging, others = not charging.
                out.push(SensorState::binary("battery_charging", state == 1));
            }
        }

        out
    }
}

#[async_trait]
impl CommandBackend for GenericBackend {
    fn id(&self) -> &str {
        "generic"
    }

    fn commands(&self) -> Vec<CommandDescriptor> {
        let mut cmds = Vec::new();
        if self.login1_session.is_some() {
            cmds.push(CommandDescriptor::button("lock", "Lock Screen").with_icon("mdi:lock"));
        }
        if self.system_bus.is_some() {
            cmds.push(CommandDescriptor::button("suspend", "Suspend").with_icon("mdi:power-sleep"));
        }
        if self.session_bus.is_some() && self.config.notifications {
            // A first-class HA notify entity (2024.5+): automations call
            // `notify.send_message` against it and the message text arrives
            // on the command topic. The handler also accepts a JSON object
            // with `title`/`message` keys for callers that want both.
            cmds.push(CommandDescriptor::notify("notify", "Send Notification").with_icon("mdi:bell"));
        }
        cmds
    }

    async fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {
        match command_id {
            "lock" => {
                let session = self
                    .login1_session_proxy()
                    .await
                    .ok_or_else(|| anyhow::anyhow!("no login1 session available"))?;
                session.lock().await?;
                Ok(())
            }
            "suspend" => {
                let bus = self
                    .system_bus
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("no system bus available"))?;
                let mgr = Login1ManagerProxy::new(bus).await?;
                mgr.suspend(true).await?;
                Ok(())
            }
            "notify" => {
                let bus = self
                    .session_bus
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("no session bus available"))?;
                let proxy = NotificationsProxy::new(bus).await?;
                // notify-entity payloads are the message text; raw MQTT
                // publishes may send a JSON object with title/message.
                let (title, body) = parse_notify_payload(payload);
                proxy
                    .notify(
                        "Home Assistant",
                        0,
                        "",
                        &title,
                        &body,
                        &[],
                        Default::default(),
                        5000,
                    )
                    .await?;
                Ok(())
            }
            other => Err(anyhow::anyhow!("unknown generic command: {other}")),
        }
    }
}

fn disk_sensor_id(mount: &str) -> String {
    if mount == "/" {
        "disk_usage_root".to_string()
    } else {
        format!("disk_usage_{}", mount.trim_matches('/').replace('/', "_"))
    }
}

/// Notify payload shapes: raw text (`message`), a JSON object with optional
/// `title` plus `message`, or empty (a friendly default — the notify
/// entity always carries text, but raw MQTT publishes may be empty).
fn parse_notify_payload(payload: &str) -> (String, String) {
    if payload.is_empty() {
        return ("Home Assistant".to_string(), "Hello from Home Assistant".to_string());
    }
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(obj) = v.as_object() {
            let message = obj
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Hello from Home Assistant");
            let title = obj
                .get("title")
                .and_then(|t| t.as_str())
                .unwrap_or("Home Assistant");
            return (title.to_string(), message.to_string());
        }
    }
    ("Home Assistant".to_string(), payload.to_string())
}

fn round1(v: f32) -> f64 {
    ((v as f64) * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::{parse_notify_payload, GenericBackend};
    use ha_agent_core::config::GenericBackendConfig;
    use ha_agent_core::SensorBackend;

    /// Regression: poll() used `refresh_all()`, which never evicts dead
    /// processes, so RSS grew ~130 MB/day on a host with process churn.
    /// Spawning and reaping children between polls must leave the
    /// tracked-process table empty.
    #[tokio::test]
    async fn poll_does_not_accumulate_process_table() {
        let backend = GenericBackend::new(GenericBackendConfig::default()).await;
        for _ in 0..50 {
            let mut child = std::process::Command::new("true").spawn().expect("spawn true");
            child.wait().expect("wait");
            let states = backend.poll().await;
            assert!(states.iter().any(|s| s.id == "cpu_usage"));
            assert!(states.iter().any(|s| s.id == "memory_usage"));
        }
        assert_eq!(backend.tracked_process_count().await, 0);
    }

    #[test]
    fn notify_payload_accepts_raw_text() {
        let (title, body) = parse_notify_payload("washing machine done");
        assert_eq!(title, "Home Assistant");
        assert_eq!(body, "washing machine done");
    }

    #[test]
    fn notify_payload_accepts_json_title_and_message() {
        let (title, body) =
            parse_notify_payload(r#"{"title": "Alert", "message": "pool level low"}"#);
        assert_eq!(title, "Alert");
        assert_eq!(body, "pool level low");
    }

    #[test]
    fn notify_payload_defaults_when_empty() {
        let (title, body) = parse_notify_payload("");
        assert_eq!(title, "Home Assistant");
        assert!(!body.is_empty());
    }
}

