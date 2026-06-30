use async_trait::async_trait;
use ha_agent_core::model::{SensorDescriptor, SensorState};
use ha_agent_core::SensorBackend;
use tokio::process::Command;
use tracing::warn;

/// niri (Wayland scrollable-tiling compositor) sensors: active window,
/// active workspace, keyboard layout.
///
/// Implementation note: this shells out to `niri msg --json <subcommand>`
/// rather than speaking niri's raw IPC socket protocol directly, trading a
/// little overhead for relying on niri's documented, versioned CLI/JSON
/// output instead of hand-rolling the socket framing. Switching to a direct
/// socket client (e.g. the `niri-ipc` crate) is a welcome follow-up PR — see
/// the workspace README's "adding a DE backend" section. Field names below
/// were implemented against niri's documented JSON output and should be
/// re-verified against `niri msg --json focused-window` etc. on a live niri
/// session before depending on this in production.
pub struct NiriBackend;

impl NiriBackend {
    /// Available only inside a running niri session.
    pub fn detect() -> bool {
        std::env::var_os("NIRI_SOCKET").is_some() && which_niri()
    }

    pub fn new() -> Self {
        Self
    }
}

impl Default for NiriBackend {
    fn default() -> Self {
        Self::new()
    }
}

fn which_niri() -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| dir.join("niri").is_file())
        })
        .unwrap_or(false)
}

async fn niri_json(args: &[&str]) -> Option<serde_json::Value> {
    let mut cmd = Command::new("niri");
    cmd.arg("msg").arg("--json").args(args);
    match cmd.output().await {
        Ok(out) if out.status.success() => serde_json::from_slice(&out.stdout).ok(),
        Ok(out) => {
            warn!(
                "niri msg --json {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr)
            );
            None
        }
        Err(e) => {
            warn!("running niri msg --json {}: {e}", args.join(" "));
            None
        }
    }
}

#[async_trait]
impl SensorBackend for NiriBackend {
    fn id(&self) -> &str {
        "niri"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        vec![
            SensorDescriptor::sensor("niri_window_title", "Active Window Title").with_icon("mdi:window-maximize"),
            SensorDescriptor::sensor("niri_window_app_id", "Active Window App").with_icon("mdi:application"),
            SensorDescriptor::sensor("niri_workspace", "Active Workspace").with_icon("mdi:view-grid"),
            SensorDescriptor::sensor("niri_keyboard_layout", "Keyboard Layout").with_icon("mdi:keyboard"),
        ]
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut out = Vec::new();

        if let Some(v) = niri_json(&["focused-window"]).await {
            if let Some(title) = v.get("title").and_then(|t| t.as_str()) {
                out.push(SensorState::new("niri_window_title", title));
            }
            if let Some(app_id) = v.get("app_id").and_then(|a| a.as_str()) {
                out.push(SensorState::new("niri_window_app_id", app_id));
            }
        }

        if let Some(v) = niri_json(&["workspaces"]).await {
            if let Some(workspaces) = v.as_array() {
                let active = workspaces.iter().find(|w| {
                    w.get("is_focused")
                        .or_else(|| w.get("is_active"))
                        .and_then(|b| b.as_bool())
                        .unwrap_or(false)
                });
                if let Some(active) = active {
                    let label = active
                        .get("name")
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| active.get("idx").and_then(|i| i.as_u64()).map(|i| i.to_string()));
                    if let Some(label) = label {
                        out.push(SensorState::new("niri_workspace", label));
                    }
                }
            }
        }

        if let Some(v) = niri_json(&["keyboard-layouts"]).await {
            let names = v.get("names").and_then(|n| n.as_array());
            let idx = v.get("current_idx").and_then(|i| i.as_u64());
            if let (Some(names), Some(idx)) = (names, idx) {
                if let Some(layout) = names.get(idx as usize).and_then(|n| n.as_str()) {
                    out.push(SensorState::new("niri_keyboard_layout", layout));
                }
            }
        }

        out
    }
}
