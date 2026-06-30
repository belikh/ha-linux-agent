use serde::Deserialize;
use std::path::Path;

fn default_device_id() -> String {
    hostname()
}

fn default_device_name() -> String {
    hostname()
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "linux-host".to_string())
}

fn default_mqtt_port() -> u16 {
    1883
}

fn default_discovery_prefix() -> String {
    "homeassistant".to_string()
}

fn default_poll_interval() -> u64 {
    30
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub device: DeviceConfig,
    pub mqtt: MqttConfig,
    #[serde(default)]
    pub backends: BackendsConfig,
}

impl Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("reading config {}: {e}", path.display()))?;
        let cfg: Config = toml::from_str(&raw)
            .map_err(|e| anyhow::anyhow!("parsing config {}: {e}", path.display()))?;
        Ok(cfg)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceConfig {
    #[serde(default = "default_device_id")]
    pub id: String,
    #[serde(default = "default_device_name")]
    pub name: String,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            id: default_device_id(),
            name: default_device_name(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MqttConfig {
    pub host: String,
    #[serde(default = "default_mqtt_port")]
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    /// Path to a file containing the password — preferred over inline
    /// `password` so the config file itself can be world-readable (mirrors
    /// how jupiter-os and most secret managers hand out credentials as
    /// files, e.g. sops-nix).
    #[serde(default)]
    pub password_file: Option<String>,
    #[serde(default)]
    pub tls: bool,
    #[serde(default = "default_discovery_prefix")]
    pub discovery_prefix: String,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
}

impl MqttConfig {
    pub fn resolve_password(&self) -> anyhow::Result<Option<String>> {
        if let Some(p) = &self.password_file {
            let pw = std::fs::read_to_string(p)
                .map_err(|e| anyhow::anyhow!("reading mqtt password_file {p}: {e}"))?;
            return Ok(Some(pw.trim_end().to_string()));
        }
        Ok(self.password.clone())
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BackendsConfig {
    #[serde(default)]
    pub generic: GenericBackendConfig,
    #[serde(default)]
    pub niri: NiriBackendConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenericBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
    /// Mount points to report disk usage for. Empty = report "/" only.
    #[serde(default)]
    pub disks: Vec<String>,
    #[serde(default = "default_true")]
    pub notifications: bool,
}

impl Default for GenericBackendConfig {
    fn default() -> Self {
        Self {
            enable: true,
            disks: Vec::new(),
            notifications: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NiriBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for NiriBackendConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}
