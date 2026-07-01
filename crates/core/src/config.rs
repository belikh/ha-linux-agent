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
    #[serde(default)]
    pub kde: KdeBackendConfig,
    #[serde(default)]
    pub zfs: ZfsBackendConfig,
    #[serde(default)]
    pub syncthing: SyncthingBackendConfig,
    #[serde(default)]
    pub headscale: HeadscaleBackendConfig,
    #[serde(default)]
    pub gamescope: GamescopeBackendConfig,
    #[serde(default)]
    pub lutris: LutrisBackendConfig,
    #[serde(default)]
    pub launcher: LauncherBackendConfig,
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

#[derive(Debug, Clone, Deserialize)]
pub struct KdeBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for KdeBackendConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZfsBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
    /// Pools to report on. Empty = auto-discover every imported pool.
    #[serde(default)]
    pub pools: Vec<String>,
}

impl Default for ZfsBackendConfig {
    fn default() -> Self {
        Self {
            enable: true,
            pools: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SyncthingBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default = "default_syncthing_address")]
    pub address: String,
    #[serde(default)]
    pub api_key: Option<String>,
    /// Path to a file containing the API key — preferred over inline
    /// `api_key`, same rationale as `MqttConfig::password_file`.
    #[serde(default)]
    pub api_key_file: Option<String>,
}

fn default_syncthing_address() -> String {
    "http://127.0.0.1:8384".to_string()
}

impl Default for SyncthingBackendConfig {
    fn default() -> Self {
        Self {
            enable: true,
            address: default_syncthing_address(),
            api_key: None,
            api_key_file: None,
        }
    }
}

impl SyncthingBackendConfig {
    pub fn resolve_api_key(&self) -> anyhow::Result<Option<String>> {
        if let Some(p) = &self.api_key_file {
            let key = std::fs::read_to_string(p)
                .map_err(|e| anyhow::anyhow!("reading syncthing api_key_file {p}: {e}"))?;
            return Ok(Some(key.trim_end().to_string()));
        }
        Ok(self.api_key.clone())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeadscaleBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for HeadscaleBackendConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GamescopeBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for GamescopeBackendConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LutrisBackendConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
}

impl Default for LutrisBackendConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

/// Mirrors `ha_agent_backend_launcher::UnitScope`. Duplicated here (rather
/// than imported) because `core` must not depend on backend crates — they
/// depend on `core`, not the other way around. `agentd` maps this to the
/// backend's real type when constructing the launcher backend.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LauncherScope {
    User,
    System,
}

/// Mirrors `ha_agent_backend_launcher::LauncherProfile` — see `LauncherScope`
/// doc comment for why this is duplicated rather than imported.
#[derive(Debug, Clone, Deserialize)]
pub struct LauncherProfileConfig {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub scope: LauncherScope,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LauncherBackendConfig {
    /// No separate `enable` toggle — an empty `apps` list already means
    /// "nothing to do", which is exactly what the backend's own `detect()`
    /// checks for.
    #[serde(default)]
    pub apps: Vec<LauncherProfileConfig>,
}
