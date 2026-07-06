use ha_agent_backend_gamescope::GamescopeBackend;
use ha_agent_backend_generic::GenericBackend;
use ha_agent_backend_hardware::HardwareBackend;
use ha_agent_backend_headscale::HeadscaleBackend;
use ha_agent_backend_kde::KdeBackend;
use ha_agent_backend_launcher::{LauncherBackend, LauncherProfile, UnitScope};
use ha_agent_backend_lutris::LutrisBackend;
use ha_agent_backend_niri::NiriBackend;
use ha_agent_backend_syncthing::SyncthingBackend;
use ha_agent_backend_zfs::ZfsBackend;
use ha_agent_core::config::LauncherScope;
use ha_agent_core::traits::{CommandBackend, SensorBackend};
use ha_agent_core::{Agent, Config};
use std::path::PathBuf;
use std::sync::Arc;

fn config_path() -> PathBuf {
    if let Some(arg) = std::env::args().nth(1) {
        return PathBuf::from(arg);
    }
    if let Ok(env_path) = std::env::var("HA_LINUX_AGENT_CONFIG") {
        return PathBuf::from(env_path);
    }
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        let p = PathBuf::from(xdg).join("ha-linux-agent/config.toml");
        if p.exists() {
            return p;
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        let p = PathBuf::from(home).join(".config/ha-linux-agent/config.toml");
        if p.exists() {
            return p;
        }
    }
    PathBuf::from("/etc/ha-linux-agent/config.toml")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let path = config_path();
    let config = Config::load(&path)?;
    tracing::info!(path = %path.display(), device = %config.device.id, "loaded config");

    let mut sensor_backends: Vec<Box<dyn SensorBackend>> = Vec::new();
    let mut command_backends: Vec<Box<dyn CommandBackend>> = Vec::new();

    if config.backends.generic.enable && GenericBackend::detect() {
        let backend = Arc::new(GenericBackend::new(config.backends.generic.clone()).await);
        sensor_backends.push(Box::new(backend.clone()));
        command_backends.push(Box::new(backend));
    }

    if config.backends.hardware.enable && HardwareBackend::detect() {
        let backend = Arc::new(HardwareBackend::new(config.backends.hardware.clone()));
        sensor_backends.push(Box::new(backend.clone()));
        command_backends.push(Box::new(backend));
    }

    if config.backends.niri.enable && NiriBackend::detect() {
        sensor_backends.push(Box::new(NiriBackend::new()));
    }

    if config.backends.kde.enable && KdeBackend::detect().await {
        match KdeBackend::new().await {
            Ok(backend) => sensor_backends.push(Box::new(backend)),
            Err(e) => tracing::warn!("kde backend init failed: {e}"),
        }
    }

    if config.backends.zfs.enable && ZfsBackend::detect() {
        sensor_backends.push(Box::new(ZfsBackend::new(config.backends.zfs.pools.clone())));
    }

    if config.backends.headscale.enable && HeadscaleBackend::detect() {
        sensor_backends.push(Box::new(HeadscaleBackend::new()));
    }

    if config.backends.gamescope.enable && GamescopeBackend::detect() {
        sensor_backends.push(Box::new(GamescopeBackend::new()));
    }

    if config.backends.syncthing.enable {
        match config.backends.syncthing.resolve_api_key() {
            Ok(Some(api_key)) => {
                let address = config.backends.syncthing.address.clone();
                if SyncthingBackend::detect(&address, &api_key).await {
                    match SyncthingBackend::new(address, api_key).await {
                        Ok(backend) => sensor_backends.push(Box::new(backend)),
                        Err(e) => tracing::warn!("syncthing backend init failed: {e}"),
                    }
                }
            }
            Ok(None) => {} // no api_key configured — silently skip, this backend is opt-in
            Err(e) => tracing::warn!("resolving syncthing api key: {e}"),
        }
    }

    if config.backends.lutris.enable && LutrisBackend::detect() {
        match LutrisBackend::new().await {
            Ok(backend) => command_backends.push(Box::new(backend)),
            Err(e) => tracing::warn!("lutris backend init failed: {e}"),
        }
    }

    if !config.backends.launcher.apps.is_empty() {
        let profiles: Vec<LauncherProfile> = config
            .backends
            .launcher
            .apps
            .iter()
            .map(|p| LauncherProfile {
                id: p.id.clone(),
                name: p.name.clone(),
                unit: p.unit.clone(),
                scope: match p.scope {
                    LauncherScope::User => UnitScope::User,
                    LauncherScope::System => UnitScope::System,
                },
                group: p.group.clone(),
                icon: p.icon.clone(),
            })
            .collect();
        if LauncherBackend::detect(&profiles) {
            let backend = Arc::new(LauncherBackend::new(profiles));
            sensor_backends.push(Box::new(backend.clone()));
            command_backends.push(Box::new(backend));
        }
    }

    if sensor_backends.is_empty() && command_backends.is_empty() {
        tracing::warn!("no backends enabled");
    }

    let agent = Agent::new(config, sensor_backends, command_backends);
    agent.run().await
}
