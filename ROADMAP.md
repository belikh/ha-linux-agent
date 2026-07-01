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

**Design direction (not yet built):** don't have the agent itself understand
VTs, compositors, or game launch sequences — that's a lot of fragile,
host-specific logic to own and gets worse with every new game/session type.
Instead, treat this purely as **remote control of systemd units the host
config already defines**, since jupiter-os already models both sides of the
switch as systemd-managed sessions (`services.cage` for the kiosk,
Jovian-NixOS's gamescope session for gaming mode) — the agent just needs to
be a thin, generic remote switch for them:

- Config: a list of named "app profiles", each naming a systemd unit
  (`backends.launcher.apps = [{ id = "borderlands", name = "Borderlands 3",
  unit = "game-borderlands.service", scope = "user", group = "display" }]`).
  `scope` picks `systemctl --user` vs `systemctl` (system units, e.g. if a
  kiosk's Cage session runs as a system unit under a dedicated user, per
  jupiter-os's `dashboard-kiosk.nix`).
- Each profile is an HA `switch` (not a stateless button — it has real
  on/off state): turning it **on** runs `systemctl <scope> start <unit>`;
  turning it **off** runs `systemctl <scope> stop <unit>`; state is polled
  via `systemctl <scope> is-active <unit>`.
- `group`: profiles sharing a group are mutually exclusive — starting one
  stops any other active member of the group first (so "start game" also
  implicitly stops the kiosk browser without a separate command). The
  dashboard/kiosk session itself becomes just another profile in the same
  group, so "turn off the game" = "turn the kiosk profile back on" (or the
  agent could auto-restore the group's configured default when a profile is
  turned off with nothing else requested to replace it — needs a decision).
- All the actual hard part — killing Chromium cleanly, allocating the right
  VT/seat, starting gamescope with the right args, network/multiplayer setup
  — stays exactly where it belongs: in the systemd unit definitions
  (jupiter-os side), not in this agent. The agent's job stays small: "is
  this unit active" / "make this unit active instead of that one."
- Security: same posture as existing commands — MQTT auth required, and
  since this can start arbitrary systemd units, the config's unit list is
  the allowlist (no free-form unit names accepted over MQTT, only the
  configured `id`s).

This keeps the agent's scope honest (a remote switch, not a session
manager) while still delivering the actual feature: "ask Home Assistant to
start game Y" flips one switch, and "turn it off" flips it back — with all
the VT/session mechanics living in ordinary NixOS module code, reviewable
and testable the normal way.
