//! Remote control of systemd units as HA switches, with mutual-exclusion
//! groups — implements ROADMAP.md's "Layer 1 — session switch".
//!
//! The core idea: don't teach the agent anything about VTs, compositors, or
//! session managers. Every "app profile" (kiosk mode, gaming mode, ...) is
//! just a named systemd unit that jupiter-os's host configs already define
//! and manage (`services.cage`, Jovian-NixOS's `gamescope-session.service`,
//! etc). This backend's whole job is `systemctl start|stop|is-active` on
//! those units, exposed to HA as one `switch` entity per profile plus a
//! paired `binary_sensor` reporting live state.
//!
//! **Mutual exclusion.** Profiles that occupy the same physical resource
//! (e.g. the display: kiosk vs. gaming mode) share a `group`. Turning a
//! profile *on* first best-effort-stops every other profile in its group,
//! then starts the target unit — so "start gaming mode" implicitly kicks the
//! kiosk off the screen without a separate command, and vice versa. Turning
//! a profile *off* is untouched by grouping — it only ever stops its own
//! unit.
//!
//! Group-mate stops are deliberately best-effort (logged, not propagated):
//! the user's actual intent is "start profile X", and a stray failure to
//! stop some *other* unit shouldn't block that. The target profile's own
//! start/stop, by contrast, is exactly what was asked for, so its failure is
//! propagated as a real error.
use async_trait::async_trait;
use ha_agent_core::model::{Component, CommandDescriptor, SensorDescriptor, SensorState};
use ha_agent_core::{CommandBackend, SensorBackend};
use tokio::process::Command;
use tracing::warn;

/// Whether a profile's unit is a user unit (`systemctl --user ...`) or a
/// system unit (plain `systemctl ...`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnitScope {
    User,
    System,
}

/// One configured "app profile" — a named systemd unit this backend can
/// start/stop/poll on HA's behalf.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct LauncherProfile {
    /// Stable id, used to derive both the MQTT command topic (via the
    /// switch entity id) and the paired binary_sensor id. This is also the
    /// allowlist: only ids present in config are ever passed to
    /// `systemctl`, so no free-form unit name can arrive over MQTT.
    pub id: String,
    /// HA-facing display name.
    pub name: String,
    /// systemd unit name, e.g. "gamescope-session.service".
    pub unit: String,
    /// User or System scope — picks `systemctl --user` vs plain `systemctl`.
    pub scope: UnitScope,
    /// Profiles sharing a group are mutually exclusive: starting one first
    /// best-effort-stops every other member of the same group.
    pub group: Option<String>,
    /// Optional mdi icon override; defaults to "mdi:application" when unset.
    pub icon: Option<String>,
}

impl LauncherProfile {
    /// Entity id for this profile's switch (and the id `handle()` matches
    /// against, since command ids are the `CommandDescriptor::id`s
    /// published in `commands()`).
    fn switch_id(&self) -> String {
        format!("launcher_{}", self.id)
    }

    /// Entity id for this profile's paired active-state binary_sensor.
    fn sensor_id(&self) -> String {
        format!("launcher_{}_active", self.id)
    }

    fn icon(&self) -> String {
        self.icon
            .clone()
            .unwrap_or_else(|| "mdi:application".to_string())
    }

    fn scope_flag(&self) -> Option<&'static str> {
        match self.scope {
            UnitScope::User => Some("--user"),
            UnitScope::System => None,
        }
    }

    fn systemctl(&self) -> Command {
        let mut cmd = Command::new("systemctl");
        if let Some(flag) = self.scope_flag() {
            cmd.arg(flag);
        }
        cmd
    }

    /// Query `systemctl is-active`. Never propagates a subprocess failure —
    /// spawn errors, non-zero exits, and any stdout other than exactly
    /// "active" (inactive/failed/activating/unknown/...) are all treated as
    /// OFF, with a warning logged for the actual-error case.
    async fn is_active(&self) -> bool {
        let mut cmd = self.systemctl();
        cmd.arg("is-active").arg(&self.unit);
        match cmd.output().await {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "active",
            Err(e) => {
                warn!(
                    "launcher: querying is-active for '{}' ({}): {e}",
                    self.id, self.unit
                );
                false
            }
        }
    }

    async fn start(&self) -> anyhow::Result<()> {
        self.run_systemctl_verb("start").await
    }

    async fn stop(&self) -> anyhow::Result<()> {
        self.run_systemctl_verb("stop").await
    }

    async fn run_systemctl_verb(&self, verb: &str) -> anyhow::Result<()> {
        let mut cmd = self.systemctl();
        cmd.arg(verb).arg(&self.unit);
        let output = cmd.output().await.map_err(|e| {
            anyhow::anyhow!(
                "launcher: spawning 'systemctl {verb} {}' for profile '{}': {e}",
                self.unit,
                self.id
            )
        })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "launcher: 'systemctl {verb} {}' for profile '{}' failed ({}): {}",
                self.unit,
                self.id,
                output.status,
                stderr.trim()
            ));
        }
        Ok(())
    }

    /// Best-effort stop used for group-mates: never propagated as an error,
    /// only logged. Distinct from `stop()`, which is used for the actually
    /// -requested profile and does propagate failures.
    async fn stop_best_effort(&self) {
        if let Err(e) = self.stop().await {
            warn!("launcher: best-effort stop of group-mate '{}' failed: {e}", self.id);
        }
    }
}

/// Backend implementing both `SensorBackend` (per-profile active
/// binary_sensor) and `CommandBackend` (per-profile switch) over a fixed,
/// config-supplied list of `LauncherProfile`s.
pub struct LauncherBackend {
    profiles: Vec<LauncherProfile>,
}

impl LauncherBackend {
    /// No async work needed at construction — just store the profile list.
    pub fn new(profiles: Vec<LauncherProfile>) -> Self {
        Self { profiles }
    }

    /// This backend has no external host capability to probe (unlike e.g.
    /// gamescope-presence detection) — it's meaningful exactly when the
    /// config actually lists at least one profile. Takes the profile slice
    /// directly (rather than being a bare `fn() -> bool`) since "is there
    /// anything configured" is the only signal available here.
    pub fn detect(profiles: &[LauncherProfile]) -> bool {
        !profiles.is_empty()
    }

    /// Every other profile sharing `profile`'s group (empty if ungrouped).
    fn group_mates<'a>(&'a self, profile: &LauncherProfile) -> Vec<&'a LauncherProfile> {
        match &profile.group {
            Some(group) => self
                .profiles
                .iter()
                .filter(|p| p.id != profile.id && p.group.as_deref() == Some(group.as_str()))
                .collect(),
            None => Vec::new(),
        }
    }
}

#[async_trait]
impl SensorBackend for LauncherBackend {
    fn id(&self) -> &str {
        "launcher"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        self.profiles
            .iter()
            .map(|p| {
                SensorDescriptor::binary_sensor(p.sensor_id(), format!("{} Active", p.name))
                    .with_icon(p.icon())
            })
            .collect()
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut states = Vec::with_capacity(self.profiles.len());
        for profile in &self.profiles {
            let active = profile.is_active().await;
            states.push(SensorState::binary(profile.sensor_id(), active));
        }
        states
    }
}

#[async_trait]
impl CommandBackend for LauncherBackend {
    fn id(&self) -> &str {
        "launcher"
    }

    fn commands(&self) -> Vec<CommandDescriptor> {
        self.profiles
            .iter()
            .map(|p| CommandDescriptor {
                id: p.switch_id(),
                name: p.name.clone(),
                component: Component::Switch,
                icon: Some(p.icon()),
            })
            .collect()
    }

    async fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {
        // Commands are published under the switch id (`launcher_<id>`), so
        // match against that rather than the bare profile id.
        let profile = self
            .profiles
            .iter()
            .find(|p| p.switch_id() == command_id)
            .ok_or_else(|| anyhow::anyhow!("unknown launcher profile: {command_id}"))?;

        match payload.to_ascii_lowercase().as_str() {
            "on" => {
                // Mutual exclusion first: best-effort stop every other
                // group member before starting the target. Run
                // sequentially (not concurrently) so group-mates are down
                // before the target comes up — e.g. a display-owning kiosk
                // unit releases its resource before gamescope grabs it.
                for mate in self.group_mates(profile) {
                    mate.stop_best_effort().await;
                }
                profile.start().await
            }
            "off" => profile.stop().await,
            other => Err(anyhow::anyhow!("unrecognized launcher payload: {other}")),
        }
    }
}
