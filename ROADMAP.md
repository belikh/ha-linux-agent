# Roadmap

Backends and features under consideration, roughly in priority order. None
of this is committed to an API yet — filing an issue/PR against one of these
before starting large work is welcome, to avoid duplicate effort.

## Backends

### `backend-zfs` (high priority)

Pool/dataset health, since jupiter-os (and presumably other users) is
ZFS-backed end to end. `backend-generic`'s disk sensors only report
filesystem `%used`, not pool health — the thing you'd actually want an HA
alert on.

- Sensors: pool capacity `%used` (`zpool list -H -p -o cap`), pool health
  state (`ONLINE`/`DEGRADED`/`FAULTED`/... via `zpool status -x` or
  `-j`/JSON if the installed zfs version supports it), last scrub
  timestamp + result, per-vdev error counts.
- One set of sensors per configured pool (config: `backends.zfs.pools =
  ["rpool", "tank"]`, default: autodetect via `zpool list -H -o name`).
- No commands in v1 (starting a scrub remotely is plausible later, but
  `zpool scrub` needs root — would need a narrowly-scoped polkit rule or a
  root-run system-level companion unit, not the user-level agent).

### `backend-syncthing` (high priority)

Per-folder sync status via Syncthing's local REST API
(`GET /rest/db/status`, `GET /rest/system/status` for the API key). Directly
useful on any host with `jupiter.services.syncthing.enable` — "is my data
actually synced" is exactly the kind of thing that deserves an HA
automation/alert, and today there's no visibility into it at all.

- Sensors: per-folder state (`idle`/`scanning`/`syncing`/`error`), out-of-sync
  item count, last sync time.
- Needs an API key (Syncthing generates one; config: `backends.syncthing.api_key`
  or `api_key_file`, `backends.syncthing.address` default
  `http://127.0.0.1:8384`).
- Detect: reachable at the configured address.

### `backend-gamescope` (nice to have)

Session state for hosts running the Jovian-NixOS/gamescope "gaming mode"
profile (`jupiter.gaming.bazzite.enable` in jupiter-os).

- Sensors: gamescope session active (bool), currently-focused app if
  discoverable (gamescope sets `STEAM_GAME` / exposes some state via its
  Wayland protocol extensions — needs investigation, not guessed at here).
- Detect: `gamescope` process running / `$GAMESCOPE_WAYLAND_DISPLAY` set.

### `backend-headscale` / mesh connectivity (nice to have)

Every jupiter-os host sits on a headscale mesh. A small backend reporting
Tailscale/headscale client state (`tailscale status --json`: connected,
exit-node in use, last handshake age) would catch "this host silently fell
off the mesh" before it becomes a support call.

### Generic systemd unit health (not DE-specific — could live in `backend-generic`)

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

### Layer 1 — session switch (systemd units, confirmed real)

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

### Layer 2 — per-game control inside the gaming session

This is the more interesting find: several apps in `bazzite.nix`'s
`appCatalog` already have their own remote-launch mechanisms, so "control
individual games" doesn't need to be invented — it needs to shell out to
tools that already do it:

- **Lutris — verified, and the strongest option.** `lutris --list-games
  --json` dumps every installed game with a stable numeric ID; `lutris
  lutris:rungameid/<id>` launches that exact game. This means a
  `backend-lutris` doesn't need hand-maintained per-game config at all — it
  can **auto-discover** installed games at startup/poll time and publish one
  HA switch per game automatically, the same way HA integrations
  auto-discover devices. (Verified directly: `lutris --help` documents both
  flags; `lutris:rungameid/N` is the numeric-ID form for when a game name
  collides.) Stopping a specific game cleanly (vs. just killing gamescope)
  needs more investigation — Lutris tracks a running-game PID internally but
  I didn't find a documented "stop" CLI verb, only launch/install.
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

1. Ship Layer 1 (session switch) first — it's fully self-contained, uses
   only `systemctl`, and already delivers "flip a switch, kiosk becomes
   gaming mode."
2. Add `backend-lutris` for auto-discovered per-game switches inside the
   gaming session — highest-confidence per-app win, since both discovery
   and launch are confirmed CLI features.
3. Steam per-game launch as a config-driven list (appid → name), since
   unlike Lutris there's no confirmed "list installed games" CLI to
   auto-discover from.
4. Heroic/emulators/OBS as later, separately-scoped additions once each is
   actually investigated rather than assumed.

This keeps the agent's scope honest — a remote switch/launcher, not a
session manager reinventing game-library management — while still
delivering the real feature end to end: "ask Home Assistant to start game Y"
flips one switch, and "turn it off" flips it back.
