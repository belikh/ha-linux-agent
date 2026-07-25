//! Remote control of systemd units as HA entities, with mutual-exclusion
//! groups — implements ROADMAP.md's "Layer 1 — session switch".
//!
//! The core idea: don't teach the agent anything about VTs, compositors, or
//! session managers. Every "app profile" (kiosk mode, gaming mode, ...) is
//! just a named systemd unit that jupiter-os's host configs already define
//! and manage (`services.cage`, Jovian-NixOS's `gamescope-session.service`,
//! etc). This backend's whole job is `systemctl start|stop|is-active` on
//! those units.
//!
//! **Grouped vs. ungrouped profiles are exposed differently.** A profile
//! with no `group` gets the original shape: one `switch` entity plus a
//! paired `binary_sensor` reporting live state. Profiles that share a
//! `group` (they occupy the same physical resource, e.g. the display: kiosk
//! vs. gaming mode) are instead collapsed into ONE `select` entity per
//! group, whose options are the member profiles' names and whose state is
//! whichever one is currently active — not N independent switches that can
//! (and did) end up in a mutually-contradictory state. Picking an option
//! best-effort-stops every other member of the group, then starts the
//! chosen unit.
//!
//! Group-mate stops are deliberately best-effort (logged, not propagated):
//! the user's actual intent is "switch to profile X", and a stray failure to
//! stop some *other* unit shouldn't block that. The target profile's own
//! start, by contrast, is exactly what was asked for, so its failure is
//! propagated as a real error.
use async_trait::async_trait;
use ha_agent_core::model::{CommandDescriptor, SensorDescriptor, SensorState};
use ha_agent_core::{CommandBackend, SensorBackend};
use std::fs;
use std::path::{Path, PathBuf};
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
    /// When set, this profile is exposed as a dimmable `light` instead of a
    /// plain `switch` -- on/off still goes through this profile's own unit
    /// (start/stop), but brightness is read/written directly against a
    /// `/sys/class/backlight/<device>` node. Empty string auto-detects the
    /// first backlight device, matching `backend-hardware`'s convention.
    /// Deliberately NOT combined with `group`: a dimmable screen and a
    /// mutually-exclusive session switch are different enough concerns that
    /// mixing them wasn't worth the complexity for what jupiter-os needs.
    pub backlight: Option<String>,
}

impl LauncherProfile {
    /// Entity id for this profile's switch/light (and the id `handle()`
    /// matches against, since command ids are the `CommandDescriptor::id`s
    /// published in `commands()`). Same naming scheme regardless of which
    /// component this profile ends up exposed as.
    fn switch_id(&self) -> String {
        format!("launcher_{}", self.id)
    }

    /// Entity id for this profile's paired active-state binary_sensor
    /// (switch profiles only -- lights read this same key straight off the
    /// shared state topic instead, no separate HA entity for it).
    fn sensor_id(&self) -> String {
        format!("launcher_{}_active", self.id)
    }

    /// State-topic key the light's `active` field reshapes from. Same
    /// literal key as `sensor_id()` on purpose (there's no HA entity for it
    /// when this profile is a light, but it's still just "is the unit
    /// active", published under a stable name).
    fn light_active_key(&self) -> String {
        format!("{}_active", self.switch_id())
    }

    fn light_brightness_key(&self) -> String {
        format!("{}_brightness", self.switch_id())
    }

    /// Command id for this profile's brightness half (the hidden,
    /// non-discoverable descriptor paired with the light's on/off entity —
    /// see `CommandDescriptor::light_brightness`). Same literal string as
    /// `light_brightness_key()` on purpose: one names a state-json key, the
    /// other a command-topic path component, different namespaces.
    fn light_brightness_command_id(&self) -> String {
        format!("{}_brightness", self.switch_id())
    }

    /// Resolves this profile's backlight device path, if `backlight` is set.
    /// Empty string means auto-detect (first entry in
    /// `/sys/class/backlight`), matching `backend-hardware`'s convention so
    /// a host that already has that backend configured doesn't need a
    /// second, differently-spelled way to say "the one backlight device".
    fn backlight_path(&self) -> Option<PathBuf> {
        let device = self.backlight.as_ref()?;
        if device.is_empty() {
            fs::read_dir("/sys/class/backlight")
                .ok()?
                .find_map(|e| e.ok().map(|e| e.path()))
        } else {
            let p = PathBuf::from("/sys/class/backlight").join(device);
            p.exists().then_some(p)
        }
    }

    /// Current (raw, unscaled) and max brightness, straight from sysfs.
    fn read_backlight(path: &Path) -> Option<(u32, u32)> {
        let cur = fs::read_to_string(path.join("brightness"))
            .ok()?
            .trim()
            .parse()
            .ok()?;
        let max = fs::read_to_string(path.join("max_brightness"))
            .ok()?
            .trim()
            .parse()
            .ok()?;
        Some((cur, max))
    }

    /// Writes a raw (unscaled) brightness value. Deliberately brightness
    /// only, never `bl_power` -- confirmed live on amalthea 2026-07-25 that
    /// `bl_power` cuts power the touch digitizer shares a rail with, while
    /// brightness alone gives the correct dark appearance without that side
    /// effect (see jupiter-os's tcxwave-touch-wake.nix history for the full
    /// story). Whatever calls this must stay off bl_power for the same
    /// reason.
    fn write_backlight_raw(path: &Path, raw: u32) -> anyhow::Result<()> {
        fs::write(path.join("brightness"), raw.to_string())?;
        Ok(())
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
            warn!(
                "launcher: best-effort stop of group-mate '{}' failed: {e}",
                self.id
            );
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

    /// Distinct group names, in first-seen order (stable entity ordering
    /// across restarts, unlike a `BTreeSet`'s alphabetical order — matters
    /// less for correctness than for a config's profiles list to visibly
    /// map onto discovery order when debugging).
    fn group_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for p in &self.profiles {
            if let Some(g) = &p.group {
                if !names.contains(g) {
                    names.push(g.clone());
                }
            }
        }
        names
    }

    fn profiles_in_group<'a>(&'a self, group: &str) -> Vec<&'a LauncherProfile> {
        self.profiles
            .iter()
            .filter(|p| p.group.as_deref() == Some(group))
            .collect()
    }

    fn ungrouped_profiles(&self) -> Vec<&LauncherProfile> {
        self.profiles.iter().filter(|p| p.group.is_none()).collect()
    }

    /// Ungrouped profiles with no `backlight` set — the original plain-
    /// switch shape.
    fn plain_switch_profiles(&self) -> Vec<&LauncherProfile> {
        self.ungrouped_profiles()
            .into_iter()
            .filter(|p| p.backlight.is_none())
            .collect()
    }

    /// Ungrouped profiles with `backlight` set — exposed as a light.
    fn light_profiles(&self) -> Vec<&LauncherProfile> {
        self.ungrouped_profiles()
            .into_iter()
            .filter(|p| p.backlight.is_some())
            .collect()
    }

    /// Entity id for a group's select (the command id `handle()` matches
    /// against, and the sensor-state key its state comes from).
    fn group_select_id(group: &str) -> String {
        format!("launcher_group_{group}")
    }
}

#[async_trait]
impl SensorBackend for LauncherBackend {
    fn id(&self) -> &str {
        "launcher"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        // Grouped profiles have no sensor of their own — their group's
        // select entity carries state via the same shared state-topic
        // mechanism (see poll()), same as it would for a switch/select.
        // Light profiles are the same: their active/brightness keys feed
        // the light's own value_template directly, no separate HA entity.
        self.plain_switch_profiles()
            .iter()
            .map(|p| {
                SensorDescriptor::binary_sensor(p.sensor_id(), format!("{} Active", p.name))
                    .with_icon(p.icon())
            })
            .collect()
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut states = Vec::with_capacity(self.profiles.len());
        for profile in self.plain_switch_profiles() {
            let active = profile.is_active().await;
            states.push(SensorState::binary(profile.sensor_id(), active));
        }
        for profile in self.light_profiles() {
            let active = profile.is_active().await;
            states.push(SensorState::binary(profile.light_active_key(), active));
            // Brightness reads independently of `active`: even if the panel
            // is nominally off, publishing the last real sysfs value (not a
            // synthesized 0) keeps the slider honest about where it'll land
            // next time the light turns on, matching how the ExecStart
            // script actually restores it (max_brightness, not "whatever it
            // last was set to over MQTT").
            if let Some(path) = profile.backlight_path() {
                if let Some((cur, max)) = LauncherProfile::read_backlight(&path) {
                    let scaled = if max > 0 {
                        (cur as u64 * 255 / max as u64) as u32
                    } else {
                        0
                    };
                    states.push(SensorState::new(profile.light_brightness_key(), scaled));
                }
            }
        }
        for group in self.group_names() {
            let members = self.profiles_in_group(&group);
            let mut current = String::new();
            for p in &members {
                if p.is_active().await {
                    current = p.name.clone();
                    break;
                }
            }
            // Empty string when no member is active: doesn't match any of
            // the select's configured options, so HA shows it as unknown
            // rather than silently picking one — an honest state for e.g.
            // the boot window before either session unit has started.
            states.push(SensorState::new(Self::group_select_id(&group), current));
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
        let mut out: Vec<CommandDescriptor> = self
            .plain_switch_profiles()
            .iter()
            .map(|p| CommandDescriptor::switch(p.switch_id(), p.name.clone()).with_icon(p.icon()))
            .collect();
        for p in self.light_profiles() {
            out.push(CommandDescriptor::light(p.switch_id(), p.name.clone()).with_icon(p.icon()));
            out.push(CommandDescriptor::light_brightness(
                p.light_brightness_command_id(),
            ));
        }
        for group in self.group_names() {
            let members = self.profiles_in_group(&group);
            let options: Vec<String> = members.iter().map(|p| p.name.clone()).collect();
            let icon = members
                .first()
                .map(|p| p.icon())
                .unwrap_or_else(|| "mdi:swap-horizontal".to_string());
            out.push(
                CommandDescriptor::select(Self::group_select_id(&group), group.clone(), options)
                    .with_icon(icon),
            );
        }
        out
    }

    async fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {
        // Light commands: default MQTT light schema, so on/off and
        // brightness arrive on two separate topics/descriptors, each with a
        // plain (non-JSON) payload -- see core::discovery's Light branch for
        // why (the JSON schema doesn't template state_topic at all).
        if let Some(profile) = self
            .light_profiles()
            .into_iter()
            .find(|p| p.switch_id() == command_id)
        {
            return handle_light_power_command(profile, payload).await;
        }
        if let Some(profile) = self
            .light_profiles()
            .into_iter()
            .find(|p| p.light_brightness_command_id() == command_id)
        {
            return handle_light_brightness_command(profile, payload).await;
        }

        // Group-select commands: payload is the chosen option (one of the
        // member profiles' `name`s), not on/off.
        for group in self.group_names() {
            if Self::group_select_id(&group) != command_id {
                continue;
            }
            let members = self.profiles_in_group(&group);
            let target = members
                .iter()
                .find(|p| p.name == payload)
                .ok_or_else(|| anyhow::anyhow!("unknown option '{payload}' for group '{group}'"))?;
            // Mutual exclusion first: best-effort stop every other group
            // member before starting the target. Sequential, not
            // concurrent, so group-mates are down before the target comes
            // up — e.g. a display-owning kiosk unit releases its resource
            // before gamescope grabs it.
            for mate in members.iter().filter(|p| p.id != target.id) {
                mate.stop_best_effort().await;
            }
            return target.start().await;
        }

        // Ungrouped switch commands. Commands are published under the
        // switch id (`launcher_<id>`), so match against that rather than
        // the bare profile id.
        let profile = self
            .profiles
            .iter()
            .find(|p| p.group.is_none() && p.backlight.is_none() && p.switch_id() == command_id)
            .ok_or_else(|| anyhow::anyhow!("unknown launcher profile: {command_id}"))?;

        match payload.to_ascii_lowercase().as_str() {
            "on" => profile.start().await,
            "off" => profile.stop().await,
            other => Err(anyhow::anyhow!("unrecognized launcher payload: {other}")),
        }
    }
}

/// Applies a plain `ON`/`OFF` payload from the light's `command_topic`.
/// Goes through the profile's own systemd unit (start/stop) so this stays in
/// sync with anything else that also starts/stops it (e.g. jupiter-os's
/// touch-wake daemon calling the same `tcxwave-screen-power.service`) --
/// deliberately NOT a second, independent way to turn the unit on/off.
async fn handle_light_power_command(
    profile: &LauncherProfile,
    payload: &str,
) -> anyhow::Result<()> {
    match payload.trim().to_ascii_uppercase().as_str() {
        "ON" => profile.start().await,
        "OFF" => profile.stop().await,
        other => Err(anyhow::anyhow!("unrecognized light payload: {other}")),
    }
}

/// Applies a bare integer (0-255) payload from the light's
/// `brightness_command_topic`, scaled down to this device's real
/// `max_brightness` and written straight to sysfs.
async fn handle_light_brightness_command(
    profile: &LauncherProfile,
    payload: &str,
) -> anyhow::Result<()> {
    let b255: u64 = payload.trim().parse().map_err(|e| {
        anyhow::anyhow!(
            "light brightness command for '{}': invalid integer payload {payload:?}: {e}",
            profile.id
        )
    })?;

    let path = profile
        .backlight_path()
        .ok_or_else(|| anyhow::anyhow!("light '{}': no backlight device found", profile.id))?;
    let (_, max) = LauncherProfile::read_backlight(&path)
        .ok_or_else(|| anyhow::anyhow!("light '{}': failed reading backlight state", profile.id))?;
    let raw = ((b255.min(255) * max as u64) / 255) as u32;
    LauncherProfile::write_backlight_raw(&path, raw)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ha_agent_core::model::Component;

    fn profile(id: &str, name: &str, group: Option<&str>) -> LauncherProfile {
        LauncherProfile {
            id: id.to_string(),
            name: name.to_string(),
            unit: format!("{id}.service"),
            scope: UnitScope::System,
            group: group.map(str::to_string),
            icon: None,
            backlight: None,
        }
    }

    fn light_profile(id: &str, name: &str) -> LauncherProfile {
        LauncherProfile {
            backlight: Some(String::new()),
            ..profile(id, name, None)
        }
    }

    // Regression guard: dashboard/gaming used to surface as two independent
    // switches that could both read "on" at once -- exactly what io asked
    // to have replaced. A grouped pair must collapse to ONE select entity
    // carrying both names as options, with no per-profile switch left over.
    #[test]
    fn grouped_profiles_become_one_select_not_two_switches() {
        let backend = LauncherBackend::new(vec![
            profile("dashboard", "amalthea dashboard", Some("session")),
            profile("gaming", "amalthea gaming", Some("session")),
        ]);

        let commands = backend.commands();
        assert_eq!(
            commands.len(),
            1,
            "grouped profiles must yield exactly one entity"
        );

        let select = &commands[0];
        assert_eq!(select.component, Component::Select);
        assert_eq!(select.id, "launcher_group_session");
        assert_eq!(
            select.options.as_deref(),
            Some(
                [
                    "amalthea dashboard".to_string(),
                    "amalthea gaming".to_string()
                ]
                .as_slice()
            )
        );
    }

    #[test]
    fn ungrouped_profile_keeps_the_original_switch_shape() {
        let backend = LauncherBackend::new(vec![profile("screen-power", "amalthea screen", None)]);

        let commands = backend.commands();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].component, Component::Switch);
        assert_eq!(commands[0].id, "launcher_screen-power");

        let sensors = backend.sensors();
        assert_eq!(sensors.len(), 1);
        assert_eq!(sensors[0].id, "launcher_screen-power_active");
    }

    #[test]
    fn mixed_grouped_and_ungrouped_profiles_dont_cross_contaminate() {
        let backend = LauncherBackend::new(vec![
            profile("dashboard", "amalthea dashboard", Some("session")),
            profile("gaming", "amalthea gaming", Some("session")),
            profile("screen-power", "amalthea screen", None),
        ]);

        let commands = backend.commands();
        assert_eq!(
            commands.len(),
            2,
            "one select for the group, one switch for the ungrouped profile"
        );
        assert!(commands
            .iter()
            .any(|c| c.component == Component::Select && c.id == "launcher_group_session"));
        assert!(commands
            .iter()
            .any(|c| c.component == Component::Switch && c.id == "launcher_screen-power"));

        // The ungrouped profile must not appear as a select option, and the
        // grouped profiles must not each get their own switch.
        let select = commands
            .iter()
            .find(|c| c.component == Component::Select)
            .unwrap();
        assert_eq!(select.options.as_deref().unwrap().len(), 2);
    }

    // Regression guard: the screen must become ONE dimmable light entity
    // (plus its hidden, non-discoverable brightness command descriptor),
    // not the switch-plus-number-slider pair it used to be, and must not
    // also leak a binary_sensor the way a plain switch profile would.
    #[test]
    fn backlight_profile_becomes_a_light_not_a_switch() {
        let backend = LauncherBackend::new(vec![light_profile("screen-power", "amalthea screen")]);

        let commands = backend.commands();
        assert_eq!(
            commands.len(),
            2,
            "one discoverable light entity plus its hidden brightness command"
        );
        let discoverable: Vec<_> = commands.iter().filter(|c| c.discoverable).collect();
        assert_eq!(
            discoverable.len(),
            1,
            "only one HA entity must be published"
        );
        assert_eq!(discoverable[0].component, Component::Light);
        assert_eq!(discoverable[0].id, "launcher_screen-power");

        let hidden = commands.iter().find(|c| !c.discoverable).unwrap();
        assert_eq!(hidden.id, "launcher_screen-power_brightness");

        assert!(
            backend.sensors().is_empty(),
            "a light profile publishes no separate binary_sensor entity"
        );
    }

    #[test]
    fn switches_groups_and_lights_all_coexist_without_cross_contamination() {
        let backend = LauncherBackend::new(vec![
            profile("dashboard", "amalthea dashboard", Some("session")),
            profile("gaming", "amalthea gaming", Some("session")),
            light_profile("screen", "amalthea screen"),
            profile("other-switch", "amalthea other", None),
        ]);

        let commands = backend.commands();
        let discoverable: Vec<_> = commands.iter().filter(|c| c.discoverable).collect();
        assert_eq!(
            discoverable.len(),
            3,
            "one select, one light, one plain switch -- as HA entities"
        );
        assert!(discoverable
            .iter()
            .any(|c| c.component == Component::Select));
        assert!(discoverable
            .iter()
            .any(|c| c.component == Component::Light && c.id == "launcher_screen"));
        assert!(discoverable
            .iter()
            .any(|c| c.component == Component::Switch && c.id == "launcher_other-switch"));

        assert_eq!(
            commands.iter().filter(|c| !c.discoverable).count(),
            1,
            "the light's hidden brightness command descriptor"
        );

        // Only the plain switch gets a binary_sensor -- not the light, not
        // the grouped profiles.
        let sensors = backend.sensors();
        assert_eq!(sensors.len(), 1);
        assert_eq!(sensors[0].id, "launcher_other-switch_active");
    }
}
