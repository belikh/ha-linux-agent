//! Tailscale/headscale mesh connectivity backend.
//!
//! headscale is a self-hosted, open-source implementation of the Tailscale
//! control-plane server; from the perspective of any mesh member the client
//! side is indistinguishable from stock Tailscale — the same `tailscale`
//! CLI is installed and driven the same way regardless of which control
//! server it's pointed at. This backend therefore shells out to the
//! standard `tailscale` binary (`tailscale status --json`), the same as it
//! would on a host talking to Tailscale's own coordination server.
//!
//! `tailscale status --json` is a real, documented, stable-ish flag, but its
//! exact schema isn't pinned down anywhere authoritative outside the
//! `tailscale.com/ipn/ipnstate` Go package, so all field access here is
//! defensive (`serde_json::Value` + `.get()`/`.and_then()` chains) and any
//! single missing/renamed field just drops that sensor's value for the tick
//! rather than failing the whole poll. See the doc comment on
//! `HeadscaleBackend::poll` for the fields this relies on and which ones
//! were judged too uncertain to guess at (notably: exit-node-active).

use async_trait::async_trait;
use ha_agent_core::model::{SensorDescriptor, SensorState};
use ha_agent_core::SensorBackend;
use tokio::process::Command;
use tracing::warn;

pub struct HeadscaleBackend;

impl HeadscaleBackend {
    /// True if a `tailscale` binary is present on `$PATH`.
    ///
    /// Mirrors the niri backend's `which_niri` approach: this only checks
    /// that the binary exists and is executable, not that the daemon is
    /// actually running or logged in — `poll` handles those states via the
    /// `BackendState` field.
    pub fn detect() -> bool {
        which_tailscale()
    }

    pub fn new() -> Self {
        Self
    }
}

impl Default for HeadscaleBackend {
    fn default() -> Self {
        Self::new()
    }
}

fn which_tailscale() -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| dir.join("tailscale").is_file())
        })
        .unwrap_or(false)
}

async fn tailscale_status_json() -> Option<serde_json::Value> {
    let mut cmd = Command::new("tailscale");
    cmd.arg("status").arg("--json");
    match cmd.output().await {
        Ok(out) if out.status.success() => match serde_json::from_slice(&out.stdout) {
            Ok(v) => Some(v),
            Err(e) => {
                warn!("parsing tailscale status --json output: {e}");
                None
            }
        },
        Ok(out) => {
            warn!(
                "tailscale status --json failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            None
        }
        Err(e) => {
            warn!("running tailscale status --json: {e}");
            None
        }
    }
}

#[async_trait]
impl SensorBackend for HeadscaleBackend {
    fn id(&self) -> &str {
        "headscale"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        vec![
            SensorDescriptor::binary_sensor("mesh_connected", "Mesh Connected")
                .with_device_class("connectivity")
                .with_icon("mdi:lan-connect"),
            SensorDescriptor::sensor("mesh_backend_state", "Mesh Backend State")
                .with_icon("mdi:lan"),
            SensorDescriptor::sensor("mesh_ip", "Mesh IP")
                .with_icon("mdi:ip-network"),
        ]
    }

    /// Polls `tailscale status --json` and derives sensor values from it.
    ///
    /// Fields relied on (per general knowledge of `ipn/ipnstate.Status`,
    /// not independently verified against a live JSON dump):
    /// - top-level `BackendState` (string: `"Running"`, `"Stopped"`,
    ///   `"NeedsLogin"`, etc.) — reasonably confident this is correct.
    /// - top-level `Self` (object) — the node's own peer status.
    ///   - `Self.Online` (bool) — reasonably confident, but not certain
    ///     this key exists on the `Self` peer entry the same way it does on
    ///     other peers in `Peer`; treated as optional/best-effort.
    ///   - `Self.TailscaleIPs` (array of strings) — reasonably confident.
    ///
    /// Deliberately **not implemented**: an `mesh_exit_node_active` sensor.
    /// `ipnstate.Status` has an `ExitNodeStatus` field and peers have
    /// `ExitNode`/`ExitNodeOption` bools, but which of these (if any)
    /// reliably indicates "an exit node is currently in use *by this
    /// host*" wasn't something this implementation was confident enough
    /// about to guess at, so it's skipped entirely per the instructions
    /// rather than publishing a sensor that might silently be wrong.
    async fn poll(&self) -> Vec<SensorState> {
        let mut out = Vec::new();

        let Some(v) = tailscale_status_json().await else {
            return out;
        };

        let backend_state = v.get("BackendState").and_then(|s| s.as_str());

        if let Some(state) = backend_state {
            out.push(SensorState::new("mesh_backend_state", state));
        } else {
            warn!("tailscale status --json missing BackendState field");
        }

        let self_online = v
            .get("Self")
            .and_then(|s| s.get("Online"))
            .and_then(|o| o.as_bool());

        let connected = match backend_state {
            Some("Running") => self_online.unwrap_or(true),
            Some(_) => false,
            None => false,
        };
        out.push(SensorState::binary("mesh_connected", connected));

        if let Some(ip) = v
            .get("Self")
            .and_then(|s| s.get("TailscaleIPs"))
            .and_then(|ips| ips.as_array())
            .and_then(|ips| ips.first())
            .and_then(|ip| ip.as_str())
        {
            out.push(SensorState::new("mesh_ip", ip));
        }

        out
    }
}
