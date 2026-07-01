//! gamescope (SteamOS-style "gaming mode" compositor) presence sensor.
//!
//! This backend publishes a single binary sensor reporting whether a
//! gamescope process is currently running on the host. It's intended for
//! jupiter-os hosts with the bazzite gaming profile enabled (via
//! Jovian-NixOS), but `detect()` is cheap and side-effect-free so it's safe
//! to leave enabled on non-gaming hosts too — it just won't fire there.
//!
//! Scope decision: this backend deliberately does **not** attempt to
//! identify the currently-focused game/app inside the gamescope session.
//! Gamescope has no stable, well-documented way to expose that outside its
//! internal Wayland protocol extensions (unlike niri's documented `msg
//! --json` CLI, for example), so tracking "which game is focused" would mean
//! depending on undocumented/unstable internals. Scope is intentionally
//! limited to "is a gamescope session running at all" — see this project's
//! roadmap for that call.
use async_trait::async_trait;
use ha_agent_core::model::{SensorDescriptor, SensorState};
use ha_agent_core::SensorBackend;
use tracing::warn;

pub struct GamescopeBackend;

impl GamescopeBackend {
    /// Static host-capability check: is gamescope installed and/or is this
    /// process running inside a gamescope session? Cheap — PATH scan plus an
    /// env var check, no subprocess calls.
    pub fn detect() -> bool {
        which_gamescope() || std::env::var_os("GAMESCOPE_WAYLAND_DISPLAY").is_some()
    }

    pub fn new() -> Self {
        Self
    }
}

impl Default for GamescopeBackend {
    fn default() -> Self {
        Self::new()
    }
}

fn which_gamescope() -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| dir.join("gamescope").is_file())
        })
        .unwrap_or(false)
}

/// Scan `/proc` for a process whose `comm` is exactly `gamescope`. Tolerates
/// per-entry I/O errors (permission denied, process exited mid-scan) by
/// skipping that entry; only a failure to open `/proc` itself is logged and
/// treated as "not running".
async fn gamescope_process_running() -> bool {
    let mut entries = match tokio::fs::read_dir("/proc").await {
        Ok(entries) => entries,
        Err(e) => {
            warn!("reading /proc: {e}");
            return false;
        }
    };

    loop {
        let entry = match entries.next_entry().await {
            Ok(Some(entry)) => entry,
            Ok(None) => break,
            Err(_) => continue,
        };

        let pid: u32 = match entry.file_name().to_str().and_then(|s| s.parse().ok()) {
            Some(pid) => pid,
            None => continue,
        };

        let comm_path = format!("/proc/{pid}/comm");
        if let Ok(comm) = tokio::fs::read_to_string(&comm_path).await {
            if comm.trim() == "gamescope" {
                return true;
            }
        }
    }

    false
}

#[async_trait]
impl SensorBackend for GamescopeBackend {
    fn id(&self) -> &str {
        "gamescope"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        vec![SensorDescriptor::binary_sensor(
            "gamescope_running",
            "Gamescope Running",
        )
        .with_device_class("running")
        .with_icon("mdi:gamepad-variant")]
    }

    async fn poll(&self) -> Vec<SensorState> {
        vec![SensorState::binary(
            "gamescope_running",
            gamescope_process_running().await,
        )]
    }
}
