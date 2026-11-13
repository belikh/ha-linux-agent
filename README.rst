.. raw:: html

   <p align="right">
     <picture>
       <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/belikh/design/latest/packages/graphics/logo/jupiter-logo-dark.svg">
       <img alt="jupiter.au" src="https://raw.githubusercontent.com/belikh/design/latest/packages/graphics/logo/jupiter-logo-light.svg" width="260">
     </picture>
   </p>

   <h1>ha-linux-agent</h1>

.. meta::
   :description: Home Assistant companion daemon for Linux that publishes system and desktop sensors and runs a few remote commands over MQTT discovery.

A Home Assistant companion daemon for Linux. Home Assistant has official companion apps for Android and iOS, and Windows users have HASS.Agent. This agent does the same for Linux: it publishes system and desktop sensors, receives notifications and runs a few remote commands (lock, suspend), all over MQTT discovery. It assumes no desktop environment. Desktop-specific extras such as niri and KDE are pluggable backends.

`Install`_ ·
`Configuration`_ ·
`Security`_ ·
`How it works`_ ·
`Backends`_ ·
`Entity reference`_ ·
`Adding a desktop-environment backend <Add a backend_>`_

|

.. raw:: html

   <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/rule/end-dark.svg"><img alt="" src="https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/rule/end-light.svg" width="50%"></picture></p>

|

Install
-------

NixOS (this repo's own flake)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: nix

   {
     inputs.ha-linux-agent.url = "github:belikh/ha-linux-agent"; # or a path: input while developing locally

     # in your host config:
     imports = [ inputs.ha-linux-agent.nixosModules.default ];
     services.ha-linux-agent = {
       enable = true;
       role = "kiosk"; # kiosk | server | minimal — see below
       settings = {
         mqtt.host = "10.1.1.20";
         mqtt.username = "ha-linux-agent";
         mqtt.password_file = "/run/secrets/mqtt_ha_linux_agent";
       };
     };
   }

The module ships **one system service** (``systemd.services.ha-linux-agent``,
running as ``services.ha-linux-agent.user``, default ``io``) — not a user unit.
A system service restarts cleanly on every ``nixos-rebuild switch`` and
orders against ``network-online.target``, which a user unit cannot do. The
``role`` switch shapes the unit:

- ``kiosk`` — adds the session-bus ``Environment`` block
  (``XDG_RUNTIME_DIR``/``DBUS_SESSION_BUS_ADDRESS`` at ``/run/user/<uid>/bus``)
  for hosts whose user lingers with a live per-user bus, and a udev rule
  granting the ``video`` group write access to backlight ``brightness`` nodes.
- ``server`` — headless: no session bus to reach, no backlight to write; the
  session-dependent features simply warn-and-disable.
- ``minimal`` — baseline unit only.

The unit sets ``Restart=on-failure``, ``StateDirectory=ha-linux-agent`` (the
last-discovery manifest used to clear stale entities), and
``WatchdogSec=15min`` (the agent pings systemd's watchdog from its poll loop,
so a wedged loop costs one restart rather than silent death).

Any other distro (Debian, Arch, ...)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: sh

   cargo build --release
   sudo install -Dm755 target/release/ha-linux-agent /usr/local/bin/ha-linux-agent
   mkdir -p ~/.config/ha-linux-agent
   cp packaging/config.example.toml ~/.config/ha-linux-agent/config.toml
   $EDITOR ~/.config/ha-linux-agent/config.toml   # at minimum set mqtt.host

   mkdir -p ~/.config/systemd/user
   cp packaging/systemd/ha-linux-agent.service ~/.config/systemd/user/
   systemctl --user enable --now ha-linux-agent

|

Configuration
-------------

See ``packaging/config.example.toml`` for the full schema. The agent looks for
its config at, in order: the path given as the first CLI argument, the
``HA_LINUX_AGENT_CONFIG`` env var, ``$XDG_CONFIG_HOME/ha-linux-agent/config.toml``,
``~/.config/ha-linux-agent/config.toml``, then ``/etc/ha-linux-agent/config.toml``.

|

Security
--------

- **MQTT auth is required if you enable commands.** An anonymous broker plus
  remote lock/suspend/notify commands means anyone on the network segment
  that can reach your broker can run them. Use a dedicated, scoped MQTT user
  (``mqtt.username`` / ``mqtt.password_file``). With ``mqtt.tls = true`` the
  broker's CA must be provided via ``mqtt.ca_file`` — the agent refuses to
  start TLS without a trust store rather than silently connecting
  unverified.
- There is deliberately no "run arbitrary shell command" entity in this
  project. Commands are limited to a small, fixed set of safe primitives
  (lock, suspend, notify). If you need more, that's a backend you write and
  own — keep the allowlist explicit and off by default.

|

How it works
------------

The agent connects to your existing MQTT broker, publishes `HA MQTT
discovery <https://www.home-assistant.io/integrations/mqtt/#mqtt-discovery>`__
configs for each enabled sensor/command, then polls sensors on an interval
and publishes their values to one shared state topic per device. Home
Assistant picks the entities up automatically — no YAML required on the HA
side.

**The connection lifecycle is driven from ConnAck** (the broker's connect
acknowledgement), not from startup: on every connect — first or reconnect —
the agent re-issues its command-topic subscriptions, re-asserts retained
``online`` availability, and publishes a fresh state payload immediately.
rumqttc reconnects automatically but never re-subscribes on its own, so a
lifecycle keyed to startup goes permanently deaf after the first broker
restart. The agent also subscribes to ``homeassistant/status`` and re-publishes
discovery when Home Assistant announces its own restart (after a
configurable per-host jitter, HA's own IO-spike recommendation). Commands
run on isolated tasks with a 5-second bound; each backend poll is isolated
the same way, so one hung backend (a wedged Syncthing daemon, say) skips its
tick instead of freezing every sensor on the host. On SIGTERM the agent
publishes retained ``offline`` and disconnects cleanly; on MQTT errors it
backs off exponentially (1→60 s, ±20 % jitter) so a broker restart cannot
synchronise a fleet into a reconnect stampede.

``ha-linux-agent --decommission`` publishes zero-length retained payloads to
every topic the agent owns (state and availability first, discovery configs
last) — Home Assistant's official entity-removal semantic, for uninstall or
device retirement. The agent also persists its announced entity list to
``<state_dir>/last-discovery.json`` and clears removed topics opportunistically
on the next start, so disabled backends never leave stale entities behind.

|

Backends
--------

Sensors and commands come from **backends**, each implementing one or both
of:

.. code-block:: rust

   trait SensorBackend { fn sensors(&self) -> Vec<SensorDescriptor>; async fn poll(&self) -> Vec<SensorState>; }
   trait CommandBackend { fn commands(&self) -> Vec<CommandDescriptor>; async fn handle(&self, id: &str, payload: &str) -> anyhow::Result<()>; }

Built in:

- **backend-generic** (``crates/backend-generic``) — works on any Linux box:
  CPU/memory/disk/load/uptime sensors (via ``sysinfo``), idle/locked state and
  a lock/suspend command (via ``systemd-logind``), battery sensors (via
  UPower, skipped if no battery), and a notify command (via
  ``org.freedesktop.Notifications``). Enabled by default, always available.
- **backend-niri** (``crates/backend-niri``) — active window title/app,
  active workspace, keyboard layout, for the `niri <https://github.com/YaLTeR/niri>`__
  Wayland compositor. Auto-detects (only activates inside a running niri
  session via ``$NIRI_SOCKET``) — safe to leave enabled everywhere.
- **backend-kde** (``crates/backend-kde``) — current Activity, for KDE
  Plasma (via kactivitymanagerd's ``org.kde.ActivityManager`` D-Bus API).
  Auto-detects (``$XDG_CURRENT_DESKTOP`` contains ``KDE`` and the D-Bus service
  is reachable) — safe to leave enabled everywhere.
- **backend-zfs** (``crates/backend-zfs``) — ZFS pool capacity and health,
  auto-discovering every imported pool by default. Auto-detects (``zpool`` on
  ``$PATH``).
- **backend-syncthing** (``crates/backend-syncthing``) — per-folder sync
  state via Syncthing's local REST API. Opt-in: needs an ``api_key`` even when
  ``enable = true`` (see ``packaging/config.example.toml``).
- **backend-headscale** (``crates/backend-headscale``) — Tailscale/headscale
  mesh connectivity (connected state, backend state, mesh IP) via the
  standard ``tailscale`` client CLI. Auto-detects (``tailscale`` on ``$PATH``).
- **backend-gamescope** (``crates/backend-gamescope``) — a single presence
  sensor for the gamescope gaming-session compositor. Auto-detects
  (``gamescope`` on ``$PATH`` or ``$GAMESCOPE_WAYLAND_DISPLAY`` set). Deliberately
  doesn't track the focused game — see its entity table below.
- **backend-lutris** (``crates/backend-lutris``) — one launch button per
  installed `Lutris <https://lutris.net/>`__ game, auto-discovered at startup.
  Auto-detects (``lutris`` on ``$PATH``).
- **backend-launcher** (``crates/backend-launcher``) — remote control of
  systemd units as HA switches, with mutual-exclusion groups (e.g. "starting
  gaming mode stops the dashboard kiosk automatically"). Config-driven, no
  auto-detection — see its entity table below and ``ROADMAP.md``'s "Layer 1 —
  session switch" for the design.
- **backend-hardware** (``crates/backend-hardware``) — CPU temperature,
  backlight brightness (sensor + settable number), CPU governor and energy
  performance preference (sensors + selects) via sysfs. Enabled by default;
  each sensor appears only when the backing sysfs node exists. Writes need
  permission on the sysfs nodes — see the udev note in the NixOS module
  (backlight brightness only; ``bl_power`` is deliberately never touched — on
  the TCxWave kiosks that node cuts power to a rail the touch digitiser
  shares).

|

Entity reference
----------------

Every entity below is published as an `HA MQTT discovery <https://www.home-assistant.io/integrations/mqtt/#mqtt-discovery>`__
config the first time the agent connects, so nothing needs to be configured
manually in Home Assistant — this table exists for reference (and for
writing automations/dashboards against a specific entity).

**MQTT topics** (``<device_id>`` defaults to the hostname, see ``[device]`` in
config):

.. list-table::
   :header-rows: 1

   * - Purpose
     - Topic
   * - |i-send-l| |i-send-d| Discovery config (one per entity)
     - ``<discovery_prefix>/<component>/<device_id>_<entity_id>/config`` (default prefix ``homeassistant``)
   * - |i-send-l| |i-send-d| Shared sensor state
     - ``ha-linux-agent/<device_id>/state`` — one retained JSON payload, e.g. ``{"cpu_usage": 12.3, "idle": "OFF", ...}``; each sensor's discovery config points at this topic with a ``value_template`` that pulls out its own key
   * - |i-send-l| |i-send-d| Availability (LWT)
     - ``ha-linux-agent/<device_id>/availability`` — ``online`` while connected, ``offline`` if the agent dies without disconnecting cleanly
   * - |i-send-l| |i-send-d| Command (one per button/switch)
     - ``ha-linux-agent/<device_id>/cmd/<entity_id>`` — HA publishes here to invoke the entity

.. raw:: html

   <details><summary><b>backend-generic</b></summary>

Active: always enabled, entity availability varies by host.

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Unit / device class
     - Published when
   * - |i-pulse-l| |i-pulse-d| ``cpu_usage``
     - sensor
     - CPU Usage
     - %
     - always
   * - |i-pulse-l| |i-pulse-d| ``memory_usage``
     - sensor
     - Memory Usage
     - %
     - always
   * - |i-pulse-l| |i-pulse-d| ``load_1m``
     - sensor
     - Load Average (1m)
     - —
     - always
   * - |i-pulse-l| |i-pulse-d| ``uptime_seconds``
     - sensor
     - Uptime
     - s
     - always
   * - |i-pulse-l| |i-pulse-d| ``disk_usage_<mount>``
     - sensor
     - Disk Usage (``<mount>``)
     - %
     - one per mount in ``backends.generic.disks`` (default: ``/`` → ``disk_usage_root``)
   * - |i-check-l| |i-check-d| ``idle``
     - binary_sensor
     - Idle
     - device_class ``running``
     - a systemd-logind session was resolved (``IdleHint``)
   * - |i-check-l| |i-check-d| ``locked``
     - binary_sensor
     - Screen Locked
     - device_class ``lock``
     - a systemd-logind session was resolved (``LockedHint``)
   * - |i-pulse-l| |i-pulse-d| ``battery_percent``
     - sensor
     - Battery
     - %, device_class ``battery``
     - UPower reports a device whose object path contains ``battery``
   * - |i-check-l| |i-check-d| ``battery_charging``
     - binary_sensor
     - Battery Charging
     - device_class ``battery_charging``
     - same battery-device condition as above

Numeric sensor values are rounded to one decimal place.

Commands (all ``button``, momentary — HA shows a "press" UI, no on/off state):

.. list-table::
   :header-rows: 1

   * - Command ID
     - Name
     - Published when
     - Behavior
   * - |i-play-l| |i-play-d| ``lock``
     - Lock Screen
     - logind session resolved
     - calls ``org.freedesktop.login1.Session.Lock``
   * - |i-play-l| |i-play-d| ``suspend``
     - Suspend
     - system D-Bus reachable
     - calls ``org.freedesktop.login1.Manager.Suspend(interactive=true)``

Notification (an HA ``notify`` entity — ``notify.<device>``; automations call
``notify.send_message`` against it and the message text arrives on the
command topic; a JSON object with ``title``/``message`` keys is also accepted):

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Published when
     - Behavior
   * - |i-bell-l| |i-bell-d| ``notify``
     - notify
     - Send Notification
     - session D-Bus reachable and ``backends.generic.notifications = true``
     - sends a desktop notification via ``org.freedesktop.Notifications.Notify``

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-niri</b></summary>

Active: only when a niri session is detected (``$NIRI_SOCKET`` set + ``niri`` on ``$PATH``).

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Notes
   * - |i-pulse-l| |i-pulse-d| ``niri_window_title``
     - sensor
     - Active Window Title
     - from ``niri msg --json focused-window``
   * - |i-pulse-l| |i-pulse-d| ``niri_window_app_id``
     - sensor
     - Active Window App
     - app ID (e.g. ``firefox``), same source
   * - |i-pulse-l| |i-pulse-d| ``niri_workspace``
     - sensor
     - Active Workspace
     - workspace name if set, else its index
   * - |i-pulse-l| |i-pulse-d| ``niri_keyboard_layout``
     - sensor
     - Keyboard Layout
     - current layout from ``niri msg --json keyboard-layouts``

No commands — read-only sensors.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-kde</b></summary>

Active: only when a Plasma session is detected (``$XDG_CURRENT_DESKTOP`` contains ``KDE``, ``org.kde.ActivityManager`` reachable on the session bus).

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Notes
   * - |i-pulse-l| |i-pulse-d| ``kde_activity``
     - sensor
     - Active Activity
     - current `KDE Activity <https://userbase.kde.org/Plasma/Activities>`__ name, via kactivitymanagerd

No commands — read-only sensor. Active-window title/app tracking is
intentionally not included: KWin has no stable, scripting-free D-Bus method
for it (it requires loading a KWin script at runtime), which is a bigger
commitment than this backend's v1 takes on — a natural follow-up PR.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-zfs</b></summary>

Active: only when the ``zpool`` binary is on ``$PATH``.

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Unit / device class
     - Notes
   * - |i-pulse-l| |i-pulse-d| ``zfs_<pool>_capacity_percent``
     - sensor
     - ZFS Pool ``<pool>`` Capacity
     - %
     - one per pool in ``backends.zfs.pools`` (default: every pool from ``zpool list -H -o name``)
   * - |i-check-l| |i-check-d| ``zfs_<pool>_problem``
     - binary_sensor
     - ZFS Pool ``<pool>`` Problem
     - device_class ``problem``
     - ON when ``zpool list -H -o health`` reports anything other than ``ONLINE``

``<pool>`` in entity IDs is sanitized (lowercase, non-alphanumeric → ``_``). No
commands — starting a scrub needs root, out of scope for this backend.
**Unverified in a live environment:** the exact ``zpool list -H [-p] -o
<fields>`` output shape was implemented from documented OpenZFS behavior, not
tested against a real pool (this project's dev sandbox has no ZFS) — check
it against your actual ``zpool`` version before relying on it.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-hardware</b></summary>

Active: always enabled; each sensor appears only when its sysfs node exists.

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Unit / Published when
   * - |i-pulse-l| |i-pulse-d| ``cpu_temperature``
     - sensor
     - CPU Temperature
     - °C — a ``coretemp`` hwmon node (or ``thermal_zone0``) exists
   * - |i-sliders-l| |i-sliders-d| ``backlight_brightness``
     - number
     - Set Display Brightness + paired sensor
     - % — a ``/sys/class/backlight`` device exists and ``backends.hardware.backlight = true``
   * - |i-filter-l| |i-filter-d| ``cpu_governor``
     - select (+ paired sensor)
     - Set CPU Governor
     - options read from ``scaling_available_governors``
   * - |i-filter-l| |i-filter-d| ``cpu_energy_performance_preference``
     - select (+ paired sensor)
     - Set CPU Energy Preference
     - options read from ``energy_performance_available_preferences``

Writes (brightness, governor, EPP) go straight to the sysfs nodes and need
permission on them — the NixOS module ships a udev rule granting the video
group write access to ``brightness``. Governor/EPP writes apply to every
``cpuN`` present. Options are enumerated from sysfs; if a node read fails,
the select falls back to a small hardcoded list rather than vanishing.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-syncthing</b></summary>

Active: enabled and reachable with a valid API key.

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Unit / device class
     - Notes
   * - |i-pulse-l| |i-pulse-d| ``syncthing_connections``
     - sensor
     - Syncthing Connected Devices
     - —
     - count of remote devices currently connected, from ``/rest/system/connections``
   * - |i-pulse-l| |i-pulse-d| ``syncthing_folder_<slug>_state``
     - sensor
     - Syncthing ``<label>`` Folder State
     - —
     - one per folder from ``/rest/config``; value is Syncthing's own state string (``idle``/``scanning``/``syncing``/``error``)
   * - |i-check-l| |i-check-d| ``syncthing_folder_<slug>_out_of_sync``
     - binary_sensor
     - Syncthing ``<label>`` Out Of Sync
     - device_class ``problem``
     - ON when state isn't ``idle`` or the folder has items needing sync

``<slug>`` is the folder's Syncthing ID, sanitized. No commands — read-only.
**Unverified against a live daemon:** several REST field names (notably
``needTotalItems`` on ``/rest/db/status``) are flagged ``unverified:`` in the
source — check them against a real Syncthing instance before trusting this
in production.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-headscale</b></summary>

Active: only when the ``tailscale`` binary is on ``$PATH``.

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Unit / device class
     - Notes
   * - |i-check-l| |i-check-d| ``mesh_connected``
     - binary_sensor
     - Mesh Connected
     - device_class ``connectivity``
     - ON when ``tailscale status --json``'s ``BackendState`` is ``Running`` (and ``Self.Online``, if present, is also true)
   * - |i-pulse-l| |i-pulse-d| ``mesh_backend_state``
     - sensor
     - Mesh Backend State
     - —
     - raw ``BackendState`` string (``Running``/``Stopped``/``NeedsLogin``/...)
   * - |i-pulse-l| |i-pulse-d| ``mesh_ip``
     - sensor
     - Mesh IP
     - —
     - first entry of ``Self.TailscaleIPs``, omitted if absent

No commands — read-only. An exit-node-in-use sensor was deliberately left
out: the relevant ``tailscale status --json`` field wasn't confident enough to
guess at rather than risk publishing a silently-wrong sensor.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-gamescope</b></summary>

Active: only when gamescope is installed or the host is currently inside a gamescope session.

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Unit / device class
     - Notes
   * - |i-check-l| |i-check-d| ``gamescope_running``
     - binary_sensor
     - Gamescope Running
     - device_class ``running``
     - ON when a process named ``gamescope`` is found running (``/proc`` scan)

No commands, and deliberately no focused-game tracking — see
``backend-gamescope``'s module doc comment and ``ROADMAP.md`` for why.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-lutris</b></summary>

Active: only when the ``lutris`` binary is on ``$PATH``.

No sensors — commands only, one ``button`` per installed game, auto-discovered
at startup from ``lutris --list-games --json``:

.. list-table::
   :header-rows: 1

   * - Command ID
     - Name
     - Behavior
   * - |i-play-l| |i-play-d| ``lutris_launch_<id>``
     - Launch ``<game name>``
     - runs ``lutris lutris:rungameid/<id>`` (spawned, not awaited)

``<id>`` is Lutris's own numeric game ID, sanitized. There is no stop/kill
command — Lutris has no documented CLI verb for it. **Partially
unverified:** the ``--list-games --json`` flag and the ``lutris:rungameid/<id>``
launch syntax are confirmed real (``lutris --help``), but the exact per-game
JSON field names are hedged (``id``/``slug``, ``name``/``title``) rather than
confirmed against a real Lutris install — check before relying on this.

.. raw:: html

   </details>

.. raw:: html

   <details><summary><b>backend-launcher</b></summary>

Active: config-driven, active whenever ``backends.launcher.apps`` is non-empty.

Remote control of systemd units as HA switches, with mutual-exclusion
groups. See ``packaging/config.example.toml`` for the ``[[backends.launcher.apps]]``
schema (``id``, ``name``, ``unit``, ``scope``, ``group``, ``icon``) and ``ROADMAP.md``'s
"Layer 1 — session switch" for the full design rationale.

.. list-table::
   :header-rows: 1

   * - Entity ID
     - HA component
     - Name
     - Unit / device class
     - Notes
   * - |i-check-l| |i-check-d| ``launcher_<id>_active``
     - binary_sensor
     - ``<name>`` Active
     - —
     - one per configured profile; polled via ``systemctl [--user] is-active <unit>``
   * - |i-switch-l| |i-switch-d| ``launcher_<id>``
     - switch
     - ``<name>``
     - —
     - one per configured profile; ``ON`` payload runs ``systemctl [--user] start <unit>`` (after best-effort-stopping every other profile sharing its ``group``), ``OFF`` runs ``stop``

``<id>`` is exactly the profile's configured ``id`` (not sanitized — avoid
spaces, since it's also used as an MQTT topic segment). Only the configured
``unit`` names are ever passed to ``systemctl`` — no free-form unit name can
arrive over MQTT.

.. raw:: html

   </details>

|

Add a backend
-------------

Niri and KDE are built in. Support for GNOME, Sway, Hyprland and other desktops can be added as a new crate; send a PR.

1. ``cargo new --lib crates/backend-<name>``, add it as a workspace member.
2. Implement ``SensorBackend`` and/or ``CommandBackend`` from ``ha-agent-core``.
3. Add a ``pub fn detect() -> bool`` that's ``true`` only when your DE is
   actually running (env var, socket, binary on ``$PATH`` — see
   ``backend-niri::NiriBackend::detect`` for the pattern).
4. Register it in ``crates/agentd/src/main.rs`` next to the niri backend.
5. Document any config knobs in ``packaging/config.example.toml`` and add a
   ``[backends.<name>]`` section to ``ha-agent-core::config::BackendsConfig``.

Keep backends self-contained — ``core`` should never need to know a specific
DE exists.

|

Develop
-------

.. code-block:: sh

   nix develop            # rust toolchain, rust-analyzer, mosquitto (for local broker testing)
   cargo build
   cargo test
   cargo clippy --all-targets -- --deny warnings
   nix build .#default

To smoke-test against a local broker: ``mosquitto -v -p 1883`` in one terminal,
point ``config.toml`` at ``127.0.0.1``, run the agent, and watch entities show up
under Home Assistant → Settings → Devices & Services → MQTT (or just
``mosquitto_sub -t 'homeassistant/#' -v`` to see discovery configs land).

|

Roadmap
-------

``backend-zfs``, ``backend-syncthing``, ``backend-headscale``, ``backend-gamescope``,
``backend-lutris``, and ``backend-launcher`` (Layer 1 session-switch) from the
original roadmap are now implemented — see the Entity reference above. See
`ROADMAP.md <ROADMAP.md>`__ for what's still open: generic systemd unit health,
Layer 2 per-game control beyond Lutris (Steam, Heroic, emulators), and the
OBS ``obs-websocket`` bonus.

|

Licence
-------

MIT. See `LICENSE <LICENSE>`__.

|

.. raw:: html

   <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/rule/end-dark.svg"><img alt="" src="https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/rule/end-light.svg" width="50%"></picture></p>

|

.. raw:: html

   <p align="right"><sub>jupiter.au</sub></p>

.. |i-bell-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/bell-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-bell-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/bell-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
.. |i-check-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/check-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-check-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/check-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
.. |i-filter-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/filter-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-filter-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/filter-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
.. |i-play-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/play-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-play-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/play-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
.. |i-pulse-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/pulse-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-pulse-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/pulse-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
.. |i-send-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/send-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-send-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/send-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
.. |i-sliders-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/sliders-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-sliders-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/sliders-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
.. |i-switch-l| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/switch-light.svg#gh-light-mode-only
   :width: 20
   :alt:
.. |i-switch-d| image:: https://raw.githubusercontent.com/belikh/design/latest/packages/github/graphics/icon/switch-dark.svg#gh-dark-mode-only
   :width: 20
   :alt:
