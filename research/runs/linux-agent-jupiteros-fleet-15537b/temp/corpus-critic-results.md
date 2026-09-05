# Corpus-critic results — step 8

All 7 gaps attempted; 3 critical all resolved, 4 high all resolved. 28 new notes, ~51 new claims.

## gap-01 notification-daemon-dead-code (critical, overturning — resolved by direct audit, no fetch needed)
The critic's own grep of jupiter-os stands as the finding: no notification daemon (mako/dunst/swaync/fnott) ships anywhere in the fleet — org.freedesktop.Notifications delivery is dead code on every host TODAY. **Effect on positions:** Tension 5's residual risk (notification-from-system-service) is moot for the current fleet — there is nothing to notify on the kiosks except the customer-display VFD, which has its own MQTT overlay path (customer-display.nix, verified live). The roadmap's notify item keeps notify.mqtt (now confirmed implementable, see gap-02) as the transport, with the last mile being the VFD overlay on kiosks rather than session-bus notifications. Service-model position: unchanged in shape, Environment block now explicitly conditional on a future desktop host class.

## gap-02 notify.mqtt discovery end-to-end (critical, verification — CONFIRMED)
The zero-'notify' discovery.py finding was an import artifact: discovery.py imports SUPPORTED_COMPONENTS from mqtt/const.py, where "notify" sits in the tuple. PR #115653 (merged 2024.5, Quality Scale gold, has-tests) added the platform; upstream test_notify.py publishes the literal working discovery payload `{"name": "test", "command_topic": "test_topic"}` and tests discovery setup/update/removal. Runtime: notify.send_message publishes the message to command_topic; entity state = send timestamp; one-way only (no title/actions schema). **Effect:** adopt-vs-build position's notification resolution is now fully implementable as specced — commit harder, and the roadmap's notify.mqtt item carries the working payload shape.

## gap-03 user-unit restart on switch (critical, overturning — OVERTURNED, position survives with corrected mechanism)
The Oct-2025 snapshot ("switch only reactivates user services for logged-in users") is FALSE on current NixOS: PR #517768 (merged 2026-05-26, backported to 26.05) makes switch-to-configuration-ng run a full per-user switch with restart_unit calls for ANY user with an active user manager (linger or not; logind.list_users + /run/user/{uid} stat). First fired 2026-05-26; the fleet pins nixos-unstable so it is live NOW. **BUT the new behaviour makes user units WORSE for an appliance fleet, not better:** stc-ng restarts active user targets including graphical-session.target (RefuseManualStart=yes → one-way session teardown), and a mid-switch session death leaves half-applied generations. Per-unit opt-out: restartIfChanged = false. **Effect on Tension/service-model position:** "retire the user-unit shape" survives, but the load-bearing reason CHANGES from "user units silently stale on switch" (now false) to "user units now restart DISRUPTIVELY on switch — every agent config change would tear down the kiosk session unless carefully opted out". The draft must cite PR #517768/stc-ng, NOT the Feb-2023/Oct-2025 threads. The prior corpus note (restart-oneshot-systemd-service-on-every-rebuild) is dated, not wrong.

## gap-04 Nix sandbox loopback (high, overturning attempt — FAILED, harness stands)
Nix's own build source brings up `lo` in the private netns (SIOCSIFFLAGS, IFF_UP|IFF_LOOPBACK) for every non-FOD build; the Nix manual since 1.1 explicitly blesses "two concurrent builds can listen on the same port (e.g. as part of a test)"; a merged nixpkgs PR serves HTTP on 127.0.0.1:8000 inside checkPhase. Constraint: the broker must be a nativeCheckInput and spawn inside the test process tree (host→sandbox loopback is blocked; in-sandbox pairs work — exactly the harness design). **Effect:** testing-gate step 5 stands as written.

## gap-05 incumbent drift (high, overturning attempt — FAILED, position confirmed)
go-hass-agent: full 11,224-word CHANGELOG v4.3.1→v14.15.1 contains ZERO occurrences of select entities, state sync, or NixOS — still "States are not kept in sync" in the README. Neither incumbent has a nixpkgs service module (module-list.nix: 0 matches in 19+ home-automation modules); lnxlink is not even packaged in nixpkgs (by-name 404; failed DIY packaging thread Oct 2025). **Effect:** adopt-vs-build's "architecturally inexpressible" and "declarative differentiator" pillars confirmed current — commit with version-stamped evidence (v14.15.1, changelog full-text).

## gap-06 niri socket reachability (high, resolved from primary source)
Socket path: `$XDG_RUNTIME_DIR/niri.{wayland_socket_name}.{pid}.sock` (e.g. /run/user/1000/niri.wayland-1.2474.sock); no SO_PEERCRED/chmod/umask — access gated purely by filesystem permissions. A same-UID process outside the session CAN connect: glob `/run/user/<uid>/niri.wayland-*.sock` to discover the path, speak the JSON protocol directly (or export NIRI_SOCKET). Security framing from niri's wiki: IPC access = full session control by design. **Effect:** the system-service-everywhere design survives the future himalia desktop host — backend-niri needs same-UID (User=io) + socket globbing, not a session unit.

## gap-07 mosquitto queue sizing (high, verification — CONFIRMED + RECALIBRATED)
Mechanism now 5-source-confirmed (man page $SYS counters incl. `publish/messages/dropped`; database.c source: per-client queue, one-time NOTICE log, QoS 0 also capped; two HA-independent witness issues). BUT the sizing rule is recalibrated: the binding quantity is HA's per-client `#`-wildcard queue vs broker-wide retained replay, not agent entity inventory. At the fleet's measured ~25 agent entities (1267-state HA instance), default 1000 never binds. **Effect:** MQTT-lifecycle D9 lands as cheap hygiene (max_queued_messages modestly raised + persistence true) + a $SYS confirmation check (watch publish/messages/dropped > 0; store-vs-retained divergence), NOT urgent 8192 sizing. Draft must avoid `$SYS/broker/mqtt/*` variants (never published per issue #3726).

## Live finding inherited (from the critic)
The four kiosks' HA entities currently read UNAVAILABLE on callisto's HA — the exact "never reliable" symptom under research, live right now. The draft's audit section should cite this as the presenting symptom (not hypothetical).

## Updated confidence levels
- Service model: HIGH on the shape (system service User=io), with the lifecycle reason CORRECTED to switch-disruptive-restarts (was switch-stale). Environment block now conditional/deferred until a desktop host exists.
- MQTT lifecycle: D9 recalibrated to hygiene+$SYS; all other defects unchanged (HIGH).
- Adopt-vs-build: HIGH (both flip conditions tested and failed to overturn).
- Privilege ladder: unchanged (HIGH) — gap wave didn't touch it.
- rumqttc fitness: unchanged (HIGH, measurement-backed).
- Testing gate: step 5 confirmed (HIGH); mosquitto as nativeCheckInput detail added.
