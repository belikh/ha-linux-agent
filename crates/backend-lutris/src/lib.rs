//! Lutris auto-discovered per-game launch buttons.
//!
//! Discovery and launch both shell out to Lutris's own CLI, confirmed real
//! via `lutris --help` (see ROADMAP.md, "Layer 2 — per-game control"):
//!
//! - `lutris --list-games --json` — documented flag that dumps every
//!   installed game as a JSON array. **Confirmed:** this flag exists and
//!   emits JSON. **Not independently confirmed:** the exact field names in
//!   each entry. This implementation assumes the common Lutris JSON shape
//!   where each entry has a numeric `id` and a string `name`, but hedges
//!   defensively — it also tries `slug` if `id` is absent, and `title` if
//!   `name` is absent — and skips any entry where neither alternative
//!   resolves, rather than failing discovery outright. **This should be
//!   checked against a real Lutris install before trusting it in
//!   production.**
//! - `lutris lutris:rungameid/<id>` — documented URI-scheme launch syntax
//!   confirmed via `lutris --help`; launches the game with the given
//!   numeric ID. Used with the `id` captured during discovery, not
//!   re-derived from user input.
//!
//! There is no command to stop a running game: Lutris tracks a running
//! game's PID internally, but no documented "stop" CLI verb was found
//! (`lutris --help` only documents install/launch-oriented flags). So this
//! backend is launch-only buttons (`CommandBackend`), not stateful
//! switches — implementing `SensorBackend` (e.g. an "is running" sensor)
//! would require a mechanism that doesn't currently exist.

use async_trait::async_trait;
use ha_agent_core::model::CommandDescriptor;
use ha_agent_core::CommandBackend;
use tokio::process::Command;
use tracing::warn;

/// One discovered Lutris game: its numeric (or, in the fallback path,
/// possibly non-numeric) id and display name.
struct LutrisGame {
    id: String,
    name: String,
}

/// Lutris per-game launch buttons, auto-discovered at startup from
/// `lutris --list-games --json`.
pub struct LutrisBackend {
    games: Vec<LutrisGame>,
}

impl LutrisBackend {
    /// Cheap PATH scan for the `lutris` binary — no subprocess call.
    pub fn detect() -> bool {
        which_lutris()
    }

    /// Runs `lutris --list-games --json` once to discover the installed
    /// game catalog. Fails if the command can't be run or its output can't
    /// be parsed as JSON — callers are expected to log a warning and skip
    /// registering this backend in that case, rather than registering it
    /// with an empty catalog.
    pub async fn new() -> anyhow::Result<Self> {
        let mut cmd = Command::new("lutris");
        cmd.arg("--list-games").arg("--json");

        let out = cmd
            .output()
            .await
            .map_err(|e| anyhow::anyhow!("running lutris --list-games --json: {e}"))?;

        if !out.status.success() {
            return Err(anyhow::anyhow!(
                "lutris --list-games --json failed: {}",
                String::from_utf8_lossy(&out.stderr)
            ));
        }

        let value: serde_json::Value = serde_json::from_slice(&out.stdout)
            .map_err(|e| anyhow::anyhow!("parsing lutris --list-games --json output: {e}"))?;

        let entries = value
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("lutris --list-games --json did not return a JSON array"))?;

        let mut games = Vec::new();
        for entry in entries {
            let id = entry
                .get("id")
                .or_else(|| entry.get("slug"))
                .and_then(value_as_id_string);
            let name = entry
                .get("name")
                .or_else(|| entry.get("title"))
                .and_then(|v| v.as_str())
                .map(str::to_string);

            match (id, name) {
                (Some(id), Some(name)) => games.push(LutrisGame { id, name }),
                _ => {
                    warn!("skipping lutris game entry missing id/name field: {entry}");
                }
            }
        }

        Ok(Self { games })
    }
}

/// Accepts either a JSON number or string for the id field (defensive,
/// since we're not 100% sure of Lutris's exact JSON typing here).
fn value_as_id_string(v: &serde_json::Value) -> Option<String> {
    if let Some(n) = v.as_u64() {
        return Some(n.to_string());
    }
    if let Some(n) = v.as_i64() {
        return Some(n.to_string());
    }
    v.as_str().map(str::to_string)
}

/// Strips/escapes any non-alphanumeric characters from a raw id before
/// embedding it in an MQTT-safe entity id. Cheap insurance in case a
/// fallback field (e.g. `slug`) ever contains something other than a plain
/// numeric id.
fn sanitize_id(raw: &str) -> String {
    raw.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
}

fn command_id_for(id: &str) -> String {
    format!("lutris_launch_{}", sanitize_id(id))
}

fn which_lutris() -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join("lutris").is_file()))
        .unwrap_or(false)
}

#[async_trait]
impl CommandBackend for LutrisBackend {
    fn id(&self) -> &str {
        "lutris"
    }

    fn commands(&self) -> Vec<CommandDescriptor> {
        self.games
            .iter()
            .map(|game| {
                CommandDescriptor::button(command_id_for(&game.id), format!("Launch {}", game.name))
                    .with_icon("mdi:controller-classic")
            })
            .collect()
    }

    async fn handle(&self, command_id: &str, _payload: &str) -> anyhow::Result<()> {
        let raw_id = command_id
            .strip_prefix("lutris_launch_")
            .ok_or_else(|| anyhow::anyhow!("unknown lutris command: {command_id}"))?;

        let game = self
            .games
            .iter()
            .find(|g| sanitize_id(&g.id) == raw_id)
            .ok_or_else(|| anyhow::anyhow!("unknown lutris game: {command_id}"))?;

        let mut cmd = Command::new("lutris");
        cmd.arg(format!("lutris:rungameid/{}", game.id));

        cmd.spawn()
            .map_err(|e| anyhow::anyhow!("spawning lutris to launch game {}: {e}", game.id))?;

        Ok(())
    }
}
