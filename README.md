# ha-linux-agent

A Home Assistant companion daemon for Linux. HA ships official companion
apps for Android and iOS, and Windows users have HASS.Agent — this fills the
same role on Linux: system/desktop sensors, notifications, and a handful of
remote commands (lock, suspend), published to Home Assistant over MQTT
discovery. No DE assumed by default; desktop-environment-specific extras
(currently: niri) are pluggable backends.

## How it works

The agent connects to your existing MQTT broker, publishes [HA MQTT
discovery](https://www.home-assistant.io/integrations/mqtt/#mqtt-discovery)
configs for each enabled sensor/command, then polls sensors on an interval
and publishes their values to one shared state topic per device. Home
Assistant picks the entities up automatically — no YAML required on the HA
side.

Sensors and commands come from **backends**, each implementing one or both
of:

```rust
trait SensorBackend { fn sensors(&self) -> Vec<SensorDescriptor>; async fn poll(&self) -> Vec<SensorState>; }
trait CommandBackend { fn commands(&self) -> Vec<CommandDescriptor>; async fn handle(&self, id: &str, payload: &str) -> anyhow::Result<()>; }
```

Built in:

- **`backend-generic`** (`crates/backend-generic`) — works on any Linux box:
  CPU/memory/disk/load/uptime sensors (via `sysinfo`), idle/locked state and
  a lock/suspend command (via `systemd-logind`), battery sensors (via
  UPower, skipped if no battery), and a notify command (via
  `org.freedesktop.Notifications`). Enabled by default, always available.
- **`backend-niri`** (`crates/backend-niri`) — active window title/app,
  active workspace, keyboard layout, for the [niri](https://github.com/YaLTeR/niri)
  Wayland compositor. Auto-detects (only activates inside a running niri
  session via `$NIRI_SOCKET`) — safe to leave enabled everywhere.

## Adding a desktop-environment backend

This is the extension point: support for GNOME, Sway, Hyprland, KDE, etc. is
intentionally *not* bundled — add it as a new crate and send a PR.

1. `cargo new --lib crates/backend-<name>`, add it as a workspace member.
2. Implement `SensorBackend` and/or `CommandBackend` from `ha-agent-core`.
3. Add a `pub fn detect() -> bool` that's `true` only when your DE is
   actually running (env var, socket, binary on `$PATH` — see
   `backend-niri::NiriBackend::detect` for the pattern).
4. Register it in `crates/agentd/src/main.rs` next to the niri backend.
5. Document any config knobs in `packaging/config.example.toml` and add a
   `[backends.<name>]` section to `ha-agent-core::config::BackendsConfig`.

Keep backends self-contained — `core` should never need to know a specific
DE exists.

## Installing

### NixOS (this repo's own flake)

```nix
{
  inputs.ha-linux-agent.url = "github:yourname/ha-linux-agent"; # or a path: input while developing locally

  # in your host config:
  imports = [ inputs.ha-linux-agent.nixosModules.default ];
  services.ha-linux-agent = {
    enable = true;
    settings = {
      mqtt.host = "10.1.1.20";
      mqtt.username = "ha-linux-agent";
      mqtt.password_file = "/run/secrets/mqtt_ha_linux_agent";
    };
  };
}
```

This runs the agent as a `systemd --user` service (it needs the user's D-Bus
session bus and, for `backend-niri`, the user's niri IPC socket).

### Any other distro (Debian, Arch, ...)

```bash
cargo build --release
sudo install -Dm755 target/release/ha-linux-agent /usr/local/bin/ha-linux-agent
mkdir -p ~/.config/ha-linux-agent
cp packaging/config.example.toml ~/.config/ha-linux-agent/config.toml
$EDITOR ~/.config/ha-linux-agent/config.toml   # at minimum set mqtt.host

mkdir -p ~/.config/systemd/user
cp packaging/systemd/ha-linux-agent.service ~/.config/systemd/user/
systemctl --user enable --now ha-linux-agent
```

## Configuration

See `packaging/config.example.toml` for the full schema. The agent looks for
its config at, in order: the path given as the first CLI argument, the
`HA_LINUX_AGENT_CONFIG` env var, `$XDG_CONFIG_HOME/ha-linux-agent/config.toml`,
`~/.config/ha-linux-agent/config.toml`, then `/etc/ha-linux-agent/config.toml`.

## Security

- **MQTT auth is required if you enable commands.** An anonymous broker plus
  remote lock/suspend/notify commands means anyone on the network segment
  that can reach your broker can run them. Use a dedicated, scoped MQTT user
  (`mqtt.username` / `mqtt.password_file`) and `mqtt.tls = true` where
  practical.
- There is deliberately no "run arbitrary shell command" entity in this
  project. Commands are limited to a small, fixed set of safe primitives
  (lock, suspend, notify). If you need more, that's a backend you write and
  own — keep the allowlist explicit and off by default.

## Development

```bash
nix develop            # rust toolchain, rust-analyzer, mosquitto (for local broker testing)
cargo build
cargo test
cargo clippy --all-targets -- --deny warnings
nix build .#default
```

To smoke-test against a local broker: `mosquitto -v -p 1883` in one terminal,
point `config.toml` at `127.0.0.1`, run the agent, and watch entities show up
under Home Assistant → Settings → Devices & Services → MQTT (or just
`mosquitto_sub -t 'homeassistant/#' -v` to see discovery configs land).

## License

MIT — see [LICENSE](LICENSE).
