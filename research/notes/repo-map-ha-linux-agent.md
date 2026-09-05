---
title: 'Repo map: ha-linux-agent'
id: repo-map-ha-linux-agent
tags:
- ha-linux-agent
- repo-map
- repo-source
created: '2026-09-02T03:34:37.366334Z'
updated: '2026-09-05T10:51:21.561364Z'
source: file:///home/io/projects/ha-linux-agent
fetched_at: '2026-09-02T03:34:37.366098Z'
fetch_provider: repo-map:tree-sitter
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'Extraction lane: tree-sitter · Files: 43 · Symbols: 278 · Cross-file reference
  edges: 109'
repo_map_lane: tree-sitter
---

# Repository map: /home/io/projects/ha-linux-agent

**Extraction lane:** tree-sitter · **Files:** 43 · **Symbols:** 278 · **Cross-file reference edges:** 109

**Most load-bearing files** (PageRank over the reference graph):
1. `crates/backend-hardware/src/lib.rs` (rust, centrality 1.00)
2. `crates/core/src/config.rs` (rust, centrality 0.96)
3. `crates/backend-gamescope/src/lib.rs` (rust, centrality 0.93)
4. `crates/core/src/agent.rs` (rust, centrality 0.10)
5. `crates/core/src/discovery.rs` (rust, centrality 0.04)
6. `crates/agentd/src/main.rs` (rust, centrality 0.02)
7. `crates/backend-generic/src/dbus.rs` (rust, centrality 0.02)
8. `.hyperresearch/templates/note.md` (markdown, centrality 0.01)
9. `.opencode/commands/hyperresearch.md` (markdown, centrality 0.01)
10. `.opencode/skills/hyperresearch-1-5-chapter-partition/SKILL.md` (markdown, centrality 0.01)
11. `.opencode/skills/hyperresearch-1-decompose/SKILL.md` (markdown, centrality 0.01)
12. `.opencode/skills/hyperresearch-10-triple-draft/SKILL.md` (markdown, centrality 0.01)
13. `.opencode/skills/hyperresearch-11-synthesize/SKILL.md` (markdown, centrality 0.01)
14. `.opencode/skills/hyperresearch-12-critics/SKILL.md` (markdown, centrality 0.01)
15. `.opencode/skills/hyperresearch-13-gap-fetch/SKILL.md` (markdown, centrality 0.01)
16. `.opencode/skills/hyperresearch-14-5-cite-check/SKILL.md` (markdown, centrality 0.01)
17. `.opencode/skills/hyperresearch-14-patcher/SKILL.md` (markdown, centrality 0.01)
18. `.opencode/skills/hyperresearch-15-polish/SKILL.md` (markdown, centrality 0.01)
19. `.opencode/skills/hyperresearch-16-readability-audit/SKILL.md` (markdown, centrality 0.01)
20. `.opencode/skills/hyperresearch-2-width-sweep/SKILL.md` (markdown, centrality 0.01)
21. `.opencode/skills/hyperresearch-3-contradiction-graph/SKILL.md` (markdown, centrality 0.01)
22. `.opencode/skills/hyperresearch-4-loci-analysis/SKILL.md` (markdown, centrality 0.01)
23. `.opencode/skills/hyperresearch-5-depth-investigation/SKILL.md` (markdown, centrality 0.01)
24. `.opencode/skills/hyperresearch-6-cross-locus-reconcile/SKILL.md` (markdown, centrality 0.01)
25. `.opencode/skills/hyperresearch-7-source-tensions/SKILL.md` (markdown, centrality 0.01)
26. `.opencode/skills/hyperresearch-8-corpus-critic/SKILL.md` (markdown, centrality 0.01)
27. `.opencode/skills/hyperresearch-9-evidence-digest/SKILL.md` (markdown, centrality 0.01)
28. `.opencode/skills/hyperresearch/SKILL.md` (markdown, centrality 0.01)
29. `AGENTS.md` (markdown, centrality 0.01)
30. `README.md` (markdown, centrality 0.01)
31. `ROADMAP.md` (markdown, centrality 0.01)
32. `crates/backend-generic/src/lib.rs` (rust, centrality 0.01)
33. `crates/backend-headscale/src/lib.rs` (rust, centrality 0.01)
34. `crates/backend-kde/src/dbus.rs` (rust, centrality 0.01)
35. `crates/backend-kde/src/lib.rs` (rust, centrality 0.01)
36. `crates/backend-launcher/src/lib.rs` (rust, centrality 0.01)
37. `crates/backend-lutris/src/lib.rs` (rust, centrality 0.01)
38. `crates/backend-niri/src/lib.rs` (rust, centrality 0.01)
39. `crates/backend-syncthing/src/lib.rs` (rust, centrality 0.01)
40. `crates/backend-zfs/src/lib.rs` (rust, centrality 0.01)

## Ranked file detail

### `crates/backend-hardware/src/lib.rs`

(rust · centrality 1.00 · 15 symbols)

- **struct** `HardwareBackend` (line 9) — `pub struct HardwareBackend {`
- **impl** `HardwareBackend` (line 17) — `impl HardwareBackend {`
- **function** `detect` (line 18) — `pub fn detect() -> bool {`
- **function** `new` (line 23) — `pub fn new(config: HardwareBackendConfig) -> Self {`
- **function** `read_sys_file` (line 103) — `fn read_sys_file(&self, path: &Path) -> anyhow::Result<String> {`
- **function** `write_sys_file` (line 108) — `fn write_sys_file(&self, path: &Path, val: &str) -> anyhow::Result<()> {`
- **function** `write_cpu_files` (line 113) — `fn write_cpu_files(&self, filename: &str, val: &str) -> anyhow::Result<()> {`
- **impl** `SensorBackend` (line 132) — `impl SensorBackend for HardwareBackend {`
- **function** `id` (line 133) — `fn id(&self) -> &str {`
- **function** `sensors` (line 137) — `fn sensors(&self) -> Vec<SensorDescriptor> {`
- **function** `oll(` (line 183) — `sync fn poll(&self) -> Vec<SensorState> {`
- **impl** `ommandBackend ` (line 244) — `mpl CommandBackend for HardwareBackend {`
- **function** `d(` (line 245) — `n id(&self) -> &str {`
- **function** `ommands(` (line 249) — `n commands(&self) -> Vec<CommandDescriptor> {`
- **function** `andle(` (line 294) — `sync fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {`

### `crates/core/src/config.rs`

(rust · centrality 0.96 · 50 symbols)

- **function** `default_device_id` (line 4) — `fn default_device_id() -> String {`
- **function** `default_device_name` (line 8) — `fn default_device_name() -> String {`
- **function** `hostname` (line 12) — `fn hostname() -> String {`
- **function** `default_mqtt_port` (line 20) — `fn default_mqtt_port() -> u16 {`
- **function** `default_discovery_prefix` (line 24) — `fn default_discovery_prefix() -> String {`
- **function** `default_poll_interval` (line 28) — `fn default_poll_interval() -> u64 {`
- **function** `default_true` (line 32) — `fn default_true() -> bool {`
- **struct** `Config` (line 37) — `pub struct Config {`
- **impl** `Config` (line 45) — `impl Config {`
- **function** `load` (line 46) — `pub fn load(path: &Path) -> anyhow::Result<Self> {`
- **struct** `DeviceConfig` (line 56) — `pub struct DeviceConfig {`
- **impl** `Default` (line 63) — `impl Default for DeviceConfig {`
- **function** `default` (line 64) — `fn default() -> Self {`
- **struct** `MqttConfig` (line 73) — `pub struct MqttConfig {`
- **impl** `ttConfig {` (line 95) — `pl MqttConfig {`
- **function** `solve_password(&` (line 96) — `b fn resolve_password(&self) -> anyhow::Result<Option<String>> {`
- **struct** `ckendsConfig {` (line 107) — `b struct BackendsConfig {`
- **struct** `rdwareBackendConfig {` (line 131) — `b struct HardwareBackendConfig {`
- **impl** `ult for` (line 151) — `Default for HardwareBackendConfig {`
- **function** `ult() -` (line 152) — `efault() -> Self {`
- **struct** `ricBackendConfig {
 ` (line 164) — `struct GenericBackendConfig {`
- **impl** `ult for` (line 174) — `Default for GenericBackendConfig {`
- **function** `ult() -` (line 175) — `efault() -> Self {`
- **struct** `BackendConfig {
 ` (line 185) — `struct NiriBackendConfig {`
- **impl** `ult for` (line 190) — `Default for NiriBackendConfig {`
- … 25 more symbols

### `crates/backend-gamescope/src/lib.rs`

(rust · centrality 0.93 · 12 symbols)

- **struct** `scopeBackend;

i` (line 22) — `struct GamescopeBackend;`
- **impl** `scopeBackend {
 ` (line 24) — `GamescopeBackend {`
- **function** `() -> ` (line 28) — `detect() -> bool {`
- **function** `-> ` (line 32) — `new() -> Self {`
- **impl** `t for G` (line 37) — `efault for GamescopeBackend {`
- **function** `t() -> ` (line 38) — `ault() -> Self {`
- **function** `gamescope() -> ` (line 43) — `ch_gamescope() -> bool {`
- **function** `ope_process_running() -> ` (line 55) — `fn gamescope_process_running() -> bool {`
- **impl** `Backend for G` (line 88) — `ensorBackend for GamescopeBackend {`
- **function** `lf` (line 89) — `&self) -> &str {`
- **function** `s(&self` (line 93) — `sors(&self) -> Vec<SensorDescriptor> {`
- **function** `self` (line 102) — `fn poll(&self) -> Vec<SensorState> {`

### `crates/core/src/agent.rs`

(rust · centrality 0.10 · 4 symbols)

- **struct** `Agent` (line 12) — `pub struct Agent {`
- **impl** `Agent` (line 18) — `impl Agent {`
- **function** `new` (line 19) — `pub fn new(`
- **function** `run` (line 31) — `pub async fn run(self) -> anyhow::Result<()> {`

### `crates/core/src/discovery.rs`

(rust · centrality 0.04 · 11 symbols)

- **function** `state_topic` (line 6) — `pub fn state_topic(device_id: &str) -> String {`
- **function** `availability_topic` (line 12) — `pub fn availability_topic(device_id: &str) -> String {`
- **function** `command_topic` (line 17) — `pub fn command_topic(device_id: &str, command_id: &str) -> String {`
- **function** `discovery_config_topic` (line 22) — `fn discovery_config_topic(`
- **function** `unique_id` (line 31) — `fn unique_id(device_id: &str, entity_id: &str) -> String {`
- **function** `sensor_discovery` (line 35) — `pub fn sensor_discovery(`
- **function** `mmand_discovery(
` (line 73) — `b fn command_discovery(`
- **function** `ad_1m_is_numeric_with_state_class()` (line 167) — `load_1m_is_numeric_with_state_class() {`
- **function** `_device() -` (line 192) — `est_device() -> DeviceInfo {`
- **function** `t_uses_default_schema_and_reshapes_shared_state() {` (line 208) — `ight_uses_default_schema_and_reshapes_shared_state() {`
- **function** `t_brightness_descriptor_is_not_discoverable() {` (line 249) — `ight_brightness_descriptor_is_not_discoverable() {`

### `crates/agentd/src/main.rs`

(rust · centrality 0.02 · 2 symbols)

- **function** `config_path` (line 17) — `fn config_path() -> PathBuf {`
- **function** `main` (line 40) — `async fn main() -> anyhow::Result<()> {`

### `crates/backend-generic/src/dbus.rs`

(rust · centrality 0.02 · 5 symbols)

- **trait** `Login1Manager` (line 8) — `pub trait Login1Manager {`
- **trait** `Login1Session` (line 15) — `pub trait Login1Session {`
- **trait** `UPower` (line 28) — `pub trait UPower {`
- **trait** `UPowerDevice` (line 33) — `pub trait UPowerDevice {`
- **trait** `Notifications` (line 46) — `pub trait Notifications {`

### `.hyperresearch/templates/note.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/commands/hyperresearch.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-1-5-chapter-partition/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-1-decompose/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-10-triple-draft/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-11-synthesize/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-12-critics/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-13-gap-fetch/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-14-5-cite-check/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-14-patcher/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-15-polish/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-16-readability-audit/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-2-width-sweep/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-3-contradiction-graph/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-4-loci-analysis/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-5-depth-investigation/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-6-cross-locus-reconcile/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-7-source-tensions/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-8-corpus-critic/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch-9-evidence-digest/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `.opencode/skills/hyperresearch/SKILL.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `AGENTS.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `README.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `ROADMAP.md`

(markdown · centrality 0.01 · 0 symbols)

(no definitions extracted)

### `crates/backend-generic/src/lib.rs`

(rust · centrality 0.01 · 17 symbols)

- **struct** `nericBackend {` (line 18) — `b struct GenericBackend {`
- **impl** `nericBackend {` (line 27) — `pl GenericBackend {`
- **function** `ct() -` (line 29) — `fn detect() -> bool {`
- **function** `con` (line 33) — `async fn new(config: GenericBackendConfig) -> Self {`
- **function** `n1_session_proxy(&se` (line 97) — `c fn login1_session_proxy(&self) -> Option<Login1SessionProxy<'_>> {`
- **function** `ery_proxy(&se` (line 108) — `c fn battery_proxy(&self) -> Option<UPowerDeviceProxy<'_>> {`
- **function** `_mounts(&se` (line 119) — `isk_mounts(&self) -> Vec<String> {`
- **impl** `orBackend for` (line 129) — `SensorBackend for GenericBackend {`
- **function** `se` (line 130) — `d(&self) -> &str {`
- **function** `ors(&se` (line 134) — `ensors(&self) -> Vec<SensorDescriptor> {`
- **function** `(&se` (line 186) — `c fn poll(&self) -> Vec<SensorState> {`
- **impl** `andBackend for` (line 244) — `CommandBackend for GenericBackend {`
- **function** `se` (line 245) — `d(&self) -> &str {`
- **function** `ands(&se` (line 249) — `ommands(&self) -> Vec<CommandDescriptor> {`
- **function** `le(&se` (line 266) — `c fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {`
- **function** `_sensor_id(mou` (line 315) — `isk_sensor_id(mount: &str) -> String {`
- **function** `d1(v: ` (line 323) — `ound1(v: f32) -> f64 {`

### `crates/backend-headscale/src/lib.rs`

(rust · centrality 0.01 · 12 symbols)

- **struct** `adscaleBackend;
` (line 26) — `b struct HeadscaleBackend;`
- **impl** `adscaleBackend {` (line 28) — `pl HeadscaleBackend {`
- **function** `ct() -` (line 35) — `fn detect() -> bool {`
- **function** `) -` (line 39) — `fn new() -> Self {`
- **impl** `ult for` (line 44) — `Default for HeadscaleBackend {`
- **function** `ult() -` (line 45) — `efault() -> Self {`
- **function** `h_tailscale() -` (line 50) — `hich_tailscale() -> bool {`
- **function** `scale_status_json() -` (line 58) — `c fn tailscale_status_json() -> Option<serde_json::Value> {`
- **impl** `orBackend for` (line 84) — `SensorBackend for HeadscaleBackend {`
- **function** `se` (line 85) — `d(&self) -> &str {`
- **function** `ors(&se` (line 89) — `ensors(&self) -> Vec<SensorDescriptor> {`
- **function** `-> V` (line 120) — `l(&self) -> Vec<SensorState> {`

### `crates/backend-kde/src/dbus.rs`

(rust · centrality 0.01 · 1 symbols)

- **trait** `tivities {` (line 11) — `b trait Activities {`

### `crates/backend-kde/src/lib.rs`

(rust · centrality 0.01 · 8 symbols)

- **struct** `ackend {
 ` (line 20) — `struct KdeBackend {`
- **impl** `ackend {
 ` (line 24) — `KdeBackend {`
- **function** `ct() -` (line 27) — `async fn detect() -> bool {`
- **function** `) -` (line 40) — `async fn new() -> anyhow::Result<Self> {`
- **impl** `orBackend for` (line 47) — `SensorBackend for KdeBackend {`
- **function** `se` (line 48) — `d(&self) -> &str {`
- **function** `ors(&se` (line 52) — `ensors(&self) -> Vec<SensorDescriptor> {`
- **function** `(&se` (line 56) — `c fn poll(&self) -> Vec<SensorState> {`

### `crates/backend-launcher/src/lib.rs`

(rust · centrality 0.01 · 46 symbols)

- **enum** `ope {
   ` (line 39) — `um UnitScope {`
- **struct** `Profile {
    /` (line 47) — `ct LauncherProfile {`
- **impl** `ofile {
    ///` (line 75) — `herProfile {`
- **function** `&self) ->` (line 80) — `id(&self) -> String {`
- **function** `&self) ->` (line 87) — `id(&self) -> String {`
- **function** `ve_key(&self) ->` (line 95) — `ctive_key(&self) -> String {`
- **function** `htness_key(&self) ->` (line 99) — `rightness_key(&self) -> String {`
- **function** `ness_command_id(&self) -> S` (line 108) — `ghtness_command_id(&self) -> String {`
- **function** `th(&self) -> O` (line 117) — `_path(&self) -> Option<PathBuf> {`
- **function** `ht(path: &Path` (line 130) — `light(path: &Path) -> Option<(u32, u32)> {`
- **function** `ght_raw(path: &Path` (line 151) — `klight_raw(path: &Path, raw: u32) -> anyhow::Result<()> {`
- **function** `-> S` (line 156) — `f) -> String {`
- **function** `self) -> O` (line 162) — `g(&self) -> Option<&'static str> {`
- **function** `elf) -> C` (line 169) — `(&self) -> Command {`
- **function** `f) -> boo` (line 181) — `tive(&self) -> bool {`
- **function** `> any` (line 196) — `(&self) -> anyhow::Result<()> {`
- **function** ` any` (line 200) — `&self) -> anyhow::Result<()> {`
- **function** `verb(&self, verb: ` (line 204) — `ystemctl_verb(&self, verb: &str) -> anyhow::Result<()> {`
- **function** `rt(&self) {
    ` (line 230) — `best_effort(&self) {`
- **struct** `d {
    profile` (line 243) — `ncherBackend {`
- **impl** `d {
    /// No ` (line 247) — `ackend {`
- **function** `c<L` (line 249) — `les: Vec<LauncherProfile>) -> Self {`
- **function** `[Launc` (line 258) — `iles: &[LauncherProfile]) -> bool {`
- **function** `> Vec<Strin` (line 266) — `) -> Vec<String> {`
- **function** `>(&'a self, group` (line 278) — `<'a>(&'a self, group: &str) -> Vec<&'a LauncherProfile> {`
- … 21 more symbols

### `crates/backend-lutris/src/lib.rs`

(rust · centrality 0.01 · 13 symbols)

- **struct** `
    id: S` (line 36) — `sGame {`
- **struct** `d {
    games` (line 43) — `utrisBackend {`
- **impl** `d {
    /// C` (line 47) — `ackend {`
- **function** `ol {
 ` (line 49) — `) -> bool {`
- **function** `:Re` (line 58) — `() -> anyhow::Result<Self> {`
- **function** `ng(v: &serde_json:` (line 107) — `tring(v: &serde_json::Value) -> Option<String> {`
- **function** ` &str) -> S` (line 121) — `aw: &str) -> String {`
- **function** `d: &str) -> St` (line 125) — `r(id: &str) -> String {`
- **function** `> bool {
   ` (line 129) — `) -> bool {`
- **impl** `or LutrisBacke` (line 136) — `end for LutrisBackend {`
- **function** `r ` (line 137) — `&str {`
- **function** `-> Vec<C` (line 141) — `f) -> Vec<CommandDescriptor> {`
- **function** `mmand_` (line 151) — `&self, command_id: &str, _payload: &str) -> anyhow::Result<()> {`

### `crates/backend-niri/src/lib.rs`

(rust · centrality 0.01 · 12 symbols)

- **struct** `riBackend;
` (line 19) — `b struct NiriBackend;`
- **impl** `riBackend {` (line 21) — `pl NiriBackend {`
- **function** `tect()` (line 23) — `b fn detect() -> bool {`
- **function** `w()` (line 27) — `b fn new() -> Self {`
- **impl** `fault f` (line 32) — `pl Default for NiriBackend {`
- **function** `fault()` (line 33) — `default() -> Self {`
- **function** `ich_niri()` (line 38) — `which_niri() -> bool {`
- **function** `ri_json(a` (line 46) — `ync fn niri_json(args: &[&str]) -> Option<serde_json::Value> {`
- **impl** `nsorBackend f` (line 67) — `pl SensorBackend for NiriBackend {`
- **function** `(&` (line 68) — `id(&self) -> &str {`
- **function** `nsors(&` (line 72) — `sensors(&self) -> Vec<SensorDescriptor> {`
- **function** `ll(&` (line 81) — `ync fn poll(&self) -> Vec<SensorState> {`

### `crates/backend-syncthing/src/lib.rs`

(rust · centrality 0.01 · 19 symbols)

- **struct** `er {
 ` (line 29) — `ct Folder {`
- **struct** `thingBackend {
 ` (line 43) — `struct SyncthingBackend {`
- **function** `tize(id:` (line 50) — `anitize(id: &str) -> String {`
- **impl** `thingBackend {
 ` (line 62) — `SyncthingBackend {`
- **function** `(addre` (line 74) — `ync fn detect(address: &str, api_key: &str) -> bool {`
- **function** `ess` (line 97) — `c fn new(address: String, api_key: String) -> anyhow::Result<Self> {`
- **function** `tate_id(folder:` (line 108) — `r_state_id(folder: &Folder) -> String {`
- **function** `ut_of_sync_id(folder:` (line 112) — `r_out_of_sync_id(folder: &Folder) -> String {`
- **struct** `onse {
    fol` (line 125) — `figResponse {`
- **struct** `er {
    id:` (line 130) — `figFolder {`
- **struct** `onse {
    #[ser` (line 146) — `tusResponse {`
- **struct** `esponse {
    conne` (line 161) — `ctionsResponse {`
- **struct** `try {
    #[ser` (line 166) — `ctionEntry {`
- **function** `serde::d` (line 171) — `_json<T: serde::de::DeserializeOwned>(`
- **function** `s(
    client` (line 187) — `ch_folders(`
- **impl** `d for Syncthi` (line 212) — `ackend for SyncthingBackend {`
- **function** ` &` (line 213) — `-> &str {`
- **function** `f) -> V` (line 217) — `self) -> Vec<SensorDescriptor> {`
- **function** `-> V` (line 245) — `l(&self) -> Vec<SensorState> {`

### `crates/backend-zfs/src/lib.rs`

(rust · centrality 0.01 · 16 symbols)

- **struct** `ackend {
 ` (line 22) — `struct ZfsBackend {`
- **impl** `kend {
   ` (line 31) — `fsBackend {`
- **function** `() -> ` (line 33) — `detect() -> bool {`
- **function** `ols` (line 40) — `new(pools: Vec<String>) -> Self {`
- **impl** `t for Z` (line 50) — `efault for ZfsBackend {`
- **function** `t() -> ` (line 51) — `ault() -> Self {`
- **function** `zpool() -> ` (line 56) — `ch_zpool() -> bool {`
- **function** `ze_pool_id(pool:` (line 64) — `itize_pool_id(pool: &str) -> String {`
- **function** `er_pools_blocking() -> ` (line 71) — `cover_pools_blocking() -> Vec<String> {`
- **function** `lines(args:` (line 95) — `fn zpool_lines(args: &[&str]) -> Option<Vec<String>> {`
- **function** `acity_percent(pool: &` (line 122) — `pool_capacity_percent(pool: &str) -> Option<f64> {`
- **function** `h(pool: &st` (line 129) — `ool_health(pool: &str) -> Option<String> {`
- **impl** `end for ZfsBa` (line 135) — `rBackend for ZfsBackend {`
- **function** `->` (line 136) — `f) -> &str {`
- **function** `elf) ->` (line 140) — `(&self) -> Vec<SensorDescriptor> {`
- **function** `) ->` (line 163) — `oll(&self) -> Vec<SensorState> {`

## Remaining files

- `crates/core/src/lib.rs` (rust): (no definitions extracted)
- `crates/core/src/model.rs` (rust): mponent {, mponent {, scovery_key(&, nsorDescriptor {, nsorDescriptor {, nsor(i, nary_sensor(i, th_unit(m, +17 more
- `crates/core/src/traits.rs` (rust): nsorBackend: , dBackend: Send, Backend for s, lf, s(&self, self, dBackend for s, lf, +2 more

---
*Method: definitions extracted via the tree-sitter lane; files ranked by PageRank over textual cross-file references (Aider's repo-map heuristic). Limitations: reference edges are textual identifier matches — same-named symbols in different scopes can mis-wire; unreferenced entry points rank low. Treat centrality as a reading order, not a correctness claim.*
