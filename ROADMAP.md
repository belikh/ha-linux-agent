# Roadmap

Backends and features under consideration, roughly in priority order. None
of this is committed to an API yet — filing an issue/PR against one of these
before starting large work is welcome, to avoid duplicate effort.

## Backends

### `backend-zfs` — implemented

Pool health, since jupiter-os (and presumably other users) is ZFS-backed end
to end. `backend-generic`'s disk sensors only report filesystem `%used`, not
pool health — the thing you'd actually want an HA alert on.

Shipped: pool capacity `%` (`zpool list -H -p -o cap`) and a `problem`
binary_sensor derived from `zpool list -H -o health` (ON when not `ONLINE`),
one pair per configured or auto-discovered pool (`backends.zfs.pools`,
default: every pool from `zpool list -H -o name`). No commands (scrubbing
needs root, still out of scope).

**Not yet shipped / open:** last-scrub timestamp+result, per-vdev error
counts — `zpool status` parsing is more involved than `zpool list` and was
left for a follow-up. **Unverified in a live environment** — implemented
against documented OpenZFS behavior only, no ZFS available in the dev
sandbox to test against; check before trusting in production.

### `backend-syncthing` — implemented

Per-folder sync status via Syncthing's local REST API. Directly useful on
any host with `jupiter.services.syncthing.enable` — "is my data actually
synced" deserves an HA automation/alert, and previously there was no
visibility into it at all.

Shipped: overall connected-device count, and per folder (auto-discovered
from `/rest/config`) a state sensor (`idle`/`scanning`/`syncing`/`error`)
and an out-of-sync binary_sensor. Needs `backends.syncthing.api_key` (or
`api_key_file`) even with `enable = true` — opt-in by design since it needs
a real credential.

**Not yet shipped:** last-sync timestamp. **Several REST field names are
flagged `unverified:`** in the source (notably `needTotalItems` on
`/rest/db/status`) — check against a live Syncthing instance before trusting
this in production.

### `backend-gamescope` — implemented (scoped down)

Session presence for hosts running the Jovian-NixOS/gamescope "gaming mode"
profile (`jupiter.gaming.bazzite.enable` in jupiter-os).

Shipped: a single `gamescope_running` binary_sensor (`/proc` scan for a
`gamescope` process). Currently-focused-app tracking was investigated and
explicitly dropped: gamescope has no stable, documented way to expose that
outside internal Wayland protocol extensions, unlike niri's `msg --json`
CLI — not worth depending on undocumented internals for v1.

### `backend-headscale` / mesh connectivity — implemented (scoped down)

Every jupiter-os host sits on a headscale mesh. Shipped: `mesh_connected`
(binary_sensor), `mesh_backend_state` (raw state string), `mesh_ip` — via
`tailscale status --json`.

**Not shipped:** exit-node-in-use and last-handshake-age sensors. The
`tailscale status --json` schema isn't pinned down anywhere authoritative
outside Tailscale's own Go source, and the exit-node field in particular
wasn't confident enough to guess at — flagged in the source rather than
publishing a sensor that might silently be wrong. A follow-up PR that
verifies the schema against a live `tailscale status --json` dump is
welcome.

### Generic systemd unit health (not DE-specific — could live in `backend-generic`) — not yet implemented

A configurable list of systemd units (`backends.generic.watch_units = [...]`)
each exposed as a binary_sensor (`is-active`). Mainly useful on
always-on/headless boxes like ganymede (mosquitto, the HA VM, n8n) where
there's no desktop session to attach a DE backend to at all.

## Feature: remote app/session control ("switch to game night")

The ask: from Home Assistant, flip a switch for "Game Y" and have a host
(or several, e.g. all four TCx Wave dashboards) tear down whatever it's
currently showing and bring up that game instead — then flipping it off
brings the dashboard/kiosk session back.

This turns out to split into two independent layers — **which session is
running** (kiosk vs. gaming mode) and **which game is running inside the
gaming session** — investigated below against jupiter-os's actual gaming
stack (`modules/gaming/bazzite.nix`) and the real upstream projects it wires
in (Jovian-NixOS, Lutris), not guessed at.

### Layer 1 — session switch (systemd units, confirmed real) — implemented as `backend-launcher`

Don't have the agent understand VTs or compositors itself — treat it as
**remote control of systemd units the host config already defines**.
jupiter-os already models both sides as systemd-managed sessions:
`services.cage` for the kiosk, and Jovian-NixOS's gaming-mode session, which
I confirmed by reading the upstream source
(`Jovian-Experiments/Jovian-NixOS`, `modules/steam/autostart.nix`) is a real
`systemd.user.services.gamescope-session` unit (`wantedBy =
["graphical-session.target"]`) — not something synthesized for this design,
it already exists today whenever `jupiter.gaming.bazzite.gamingMode.enable`
is on.

- Config: named "app profiles", each naming a unit (`backends.launcher.apps
  = [{ id = "gaming-mode", unit = "gamescope-session.service", scope =
  "user", group = "display" }, { id = "kiosk", unit = "cage.service", scope
  = "system", group = "display" }]`). `scope` picks `systemctl --user` vs
  `systemctl` (Cage's kiosk session runs as a system unit under a dedicated
  `kiosk` user per `dashboard-kiosk.nix`, so it needs the system-scope path).
- Each profile is an HA `switch` (real on/off state, not a stateless
  button): on → `systemctl <scope> start <unit>`, off → `... stop <unit>`,
  state polled via `... is-active <unit>`.
- `group`: profiles sharing a group are mutually exclusive — starting one
  stops other active members first, so "start gaming mode" implicitly stops
  the kiosk without a separate command, and vice versa.
- Security: MQTT auth required; the configured `id` list is the allowlist,
  no free-form unit names accepted over MQTT.

Shipped essentially as designed: `LauncherProfile { id, name, unit, scope,
group, icon }` configured via `[[backends.launcher.apps]]`, one `switch` +
paired `binary_sensor` per profile, group-based mutual exclusion (best-effort
sequential stop of group-mates before starting the target). One real bug
found and fixed during integration: entity IDs containing characters outside
Jinja identifier syntax (e.g. a profile id with a hyphen) broke the
`value_template`'s dot-notation JSON lookup — fixed in `core::discovery` by
switching to bracket notation (`value_json['<id>']`) for every entity, not
just launcher's, so this class of bug can't recur for any backend.

### Layer 2 — per-game control inside the gaming session

Several apps in `bazzite.nix`'s `appCatalog` already have their own
remote-launch mechanisms, so "control individual games" doesn't need to be
invented — it needs to shell out to tools that already do it:

- **Lutris — implemented as `backend-lutris`.** `lutris --list-games --json`
  dumps every installed game with a stable numeric ID; `lutris
  lutris:rungameid/<id>` launches that exact game. Shipped as auto-discovered
  launch buttons — no hand-maintained per-game config, one HA button per
  installed game, published the same way HA integrations auto-discover
  devices. (Verified directly: `lutris --help` documents both flags.) **Per
  the original plan, there is no stop command** — Lutris tracks a running
  game's PID internally but no documented "stop" CLI verb was found, so this
  is launch-only, not a stateful switch. **The exact per-game JSON field
  names are hedged, not confirmed** (`id`/`slug`, `name`/`title` fallbacks) —
  check against a real Lutris install before trusting it in production.
- **Steam — well-established, not independently re-verified here.**
  `steam -applaunch <appid>` / the `steam://rungameid/<appid>` URI launch a
  specific installed game; `steam -shutdown` cleanly quits Steam (and with
  it, gamescope games launched through it). This is long-standing, widely
  documented Valve CLI behavior, distinct from Lutris's own confirmation
  above.
- **Heroic (Epic/GOG/Amazon) — needs investigation.** Primarily a GUI
  Electron app; whether it has an equivalent CLI/deep-link launch verb
  wasn't confirmed and shouldn't be assumed before building against it.
- **Emulators (PCSX2, shadPS4) — lower priority.** Typically one ROM/game
  per invocation with a file path argument rather than an installed-game
  catalog, so they're a worse fit for the "list of switches" model above.
- **Bonus, unrelated to games:** OBS Studio is already in the catalog
  (`appCatalog.capture`) for game-capture, and modern OBS ships
  `obs-websocket` (protocol v5) built in — a genuinely stable, officially
  documented remote-control API. Start/stop recording or streaming as HA
  buttons would be a small, low-risk addition riding on infrastructure
  that's already installed on any host with the `capture` app enabled.

### Suggested build order

1. ~~Ship Layer 1 (session switch) first~~ — done (`backend-launcher`).
2. ~~Add `backend-lutris` for auto-discovered per-game switches~~ — done.
3. **Next:** Steam per-game launch as a config-driven list (appid → name),
   since unlike Lutris there's no confirmed "list installed games" CLI to
   auto-discover from.
4. Heroic/emulators/OBS as later, separately-scoped additions once each is
   actually investigated rather than assumed. Generic systemd unit health
   (for headless boxes like ganymede) is also still open.

This keeps the agent's scope honest — a remote switch/launcher, not a
session manager reinventing game-library management — while still
delivering the real feature end to end: "ask Home Assistant to start game Y"
flips one switch, and "turn it off" flips it back.
