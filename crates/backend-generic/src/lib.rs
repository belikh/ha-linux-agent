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

        Self {
            config,
            sys: Mutex::new(System::new_all()),
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
            sys.refresh_all();
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
            // Payload (if any) becomes the notification body; with no
            // payload a default greeting is sent. Trigger custom messages
            // via HA's "MQTT: Publish a packet" service.
            cmds.push(CommandDescriptor::button("notify", "Send Notification").with_icon("mdi:bell"));
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
                let body = if payload.is_empty() {
                    "Hello from Home Assistant"
                } else {
                    payload
                };
                proxy
                    .notify(
                        "Home Assistant",
                        0,
                        "",
                        "Home Assistant",
                        body,
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

fn round1(v: f32) -> f64 {
    ((v as f64) * 10.0).round() / 10.0
}
