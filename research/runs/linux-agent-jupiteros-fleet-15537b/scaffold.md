# Scaffold — linux-agent-jupiteros-fleet-15537b

## User Prompt (VERBATIM — gospel)

> use the repo part of @hyperresearch on this repo, then broaden the research topic to improving it - currently it's in a sloppy state - never reliable,  probably lacking functions and features for a proper "Linux Agent" or at least a "jupiterOS agent (nixos)" -  I want it to run on EVERY host in jupiterOS, not just the kiosks.

## Run config

- vault_tag: `linux-agent-jupiteros-fleet-15537b`
- query_file_path: `research/runs/linux-agent-jupiteros-fleet-15537b/query.md`
- modality: **synthesize** — a defended improvement thesis with evidence chains
  (what to fix, in what order, and why, grounded in the repo audit + fleet
  reality + external comparison agents)
- profile/gear: full (55–80 sources, 16 steps)
- tier: to be classified by step 1 (expected: full — argumentative,
  multi-facet research query touching reliability engineering, NixOS fleet
  deployment, and feature-gap analysis against established Linux agents)
- wrapper requirements: none (no research/prompt.txt, no wrapper_contract.json)
- save path (final report): `research/notes/final_report_linux-agent-jupiteros-fleet-15537b.md`

## Context established BEFORE the pipeline (ground-truth audit)

The "repo part of hyperresearch" (repo map + repo wiki) was already executed:

- `hpr repo map .` → note `repo-map-ha-linux-agent` (2978 words, structural
  map incl. centrality; the working tree contains .opencode/ + research/
  hyperresearch scaffolding which slightly inflates its file list)
- `hpr repo wiki belikh/ha-linux-agent` → 1 note, 13 words (DeepWiki has
  essentially nothing indexed for this repo — repo map is the meaningful
  artifact of the two)

Build verification performed with observed exit codes (nix shell toolchain,
Rust 1.97.1):

- `cargo build --workspace` → exit 0
- `cargo test --workspace` → exit 0, but only **3 tests** in the entire
  12-crate workspace (all in `core::discovery`; every backend crate has zero
  tests) — 3758 lines of Rust, 3 regression tests
- `cargo clippy --workspace --all-targets -- --deny warnings` → **exit 101**
  (fails the repo's own documented gate): `crates/backend-hardware/src/lib.rs:117:43`
  `unnecessary_map_or` (`map_or(false, …)` → `is_some_and(…)`)

Code-level reliability defects found (file:line):

1. `crates/core/src/agent.rs:92-145` — discovery/setup task races the eventloop
   connection: publishes/subscribes immediately on a client whose TCP
   connection may not be established yet (rumqttc queues, but subscriptions
   made before connack can be lost); **no re-publish of discovery on
   reconnect** — after a broker restart, entities can vanish or go stale in HA
   until agent restart.
2. `crates/core/src/agent.rs:177-196` — on MQTT error: logs, sleeps 5s,
   loops — but the setup/poll tasks hold a client handle from the *old*
   connection... actually rumqttc eventloop reconnects internally, but
   discovery is never re-published after the first attempt; LWT marks the
   device offline then nothing re-asserts entity availability cleanly on
   flaky links.
3. `crates/core/src/agent.rs:149-172` — the poll loop publishes the merged
   JSON **retained** (`retain=true`): every 30s a retained message. Harmless
   for HA but retains stale state on the broker for new subscribers;
   design-smell.
4. **No graceful shutdown**: no ctrl_c/SIGTERM handling anywhere (grep across
   crates: zero matches in runtime code). systemd sends SIGTERM on stop;
   agent just dies → LWT offline published by broker; user services restart
   on-failure. "Unclean" but works — however no in-flight command acks.
5. `crates/backend-syncthing/src/lib.rs:75,98` — `reqwest` client built with
   NO timeout (`.build()` default = no timeout). A hung Syncthing REST daemon
   stalls the shared poll loop forever (all backends poll sequentially in one
   tick — one stuck backend starves every other sensor on the host).
6. `crates/core/src/config.rs:88` — `tls: bool` config knob is dead: parsed
   but never passed to rumqttc (grep: only occurrence). README advertises
   "mqtt.tls = true where practical" — the knob does nothing. Misleading doc.
7. `crates/backend-hardware/src/lib.rs:74-92` — sysfs EPP/governor detection
   fallback hardcodes invented lists ("powersave/performance", five EPP
   values) when sysfs read fails: on hosts without cpufreq, the backend
   publishes a `select` entity with fake options — an entity that can never
   be set successfully. Should publish nothing instead.
8. `crates/backend-hardware/src/lib.rs:117` — clippy gate failure (above).
9. README drift: `backend-hardware` (backlight/governor/EPP sensors +
   commands, default-enabled, sysfs writes) is entirely absent from the
   README's backend list and entity reference — despite being the backend
   that writes to sysfs (security-relevant: the README's Security section
   doesn't know it exists).
10. Zero tests in 10 of 11 backend crates; the launcher backend (682 lines,
    the most complex, group mutual-exclusion logic) has no tests at all.
11. Packaging drift: `packaging/systemd/ha-linux-agent.service` installs to
    `/usr/local/bin` but flake's nixosModules uses the store path. The manual
    install path in README uses `sudo install` — fine, but the unit ships
    `WantedBy=default.target` + `After=graphical-session.target` without
    `PartOf`/`BindsTo` — agent dies with session, never restarts until login.
    The jupiter-os ha-agent.nix does it better (linger + ConditionUser).

Fleet reality (jupiter-os, /home/io/projects/jupiter-os — 7 hosts):

- 4 TCxWave kiosks (adrastea, amalthea, metis, thebe) — ha-agent enabled via
  `jupiter.tcxWaveKiosk` profile → `modules/services/ha-agent.nix`; sysfs
  perms service chmods 0664 backlight+cpufreq nodes
- callisto (10.1.1.3) — mosquitto broker host + iscsi-target + build machine;
  has manual `services.ha-linux-agent` wiring in host config (ZFS + launcher
  + headscale sensors)
- europa (build farm / NFS game libraries) — no agent
- pallene (ZFS host, disk-configuration references) — no agent
- ganymede is mentioned in ROADMAP.md ("always-on/headless boxes like
  ganymede (mosquitto, the HA VM, n8n)") but no host dir exists — either
  renamed/retired; the fleet today is the 7 dirs in hosts/
- The user wants: agent on EVERY host — including headless servers (europa,
  pallene, and any ganymede-successor) where there is NO user D-Bus session
  and no graphical session. Current architecture (systemd --user service +
  session bus dependency) does not fit headless hosts: niri/KDE/notification
  backends would fail; also `ConditionUser=io` + linger pattern needs
  fleet-wide treatment (every host has an io user? or run as system service
  on headless boxes?)

Key architectural questions the research must answer (breadthened topic):

A. Reliability engineering: MQTT reconnect + discovery re-publish patterns
   (HA MQTT discovery best practice: birth message, re-publish on reconnect,
   expanded state descriptions), rumqttc v0.24 eventloop patterns, per-backend
   timeout isolation, spawn per-backend poll tasks vs sequential merge.
B. What a "proper Linux agent" for HA looks like in 2026: HASS.Agent (Windows
   .NET), HA companion mobile apps' sensor set as baseline; which sensors
   matter on servers vs desktops vs kiosks (SMART, NVMe, ZFS, systemd units,
   docker/podman containers, D-Bus system bus sensors, network stats).
C. NixOS fleet-native agent design: system-level service vs user-level,
   `network-online.target` vs `graphical-session.target`, per-host backend
   enabling via NixOS modules (jupiter.services.haAgent already exists —
   needs to move out of the kiosk-only profile into common.nix), headless
   operation (no session bus), secrets handling (sops-nix password_file
   pattern already in place), persisting state across boots.
D. Comparison against established agents: Telegraf (the metrics daemon
   king), Telegraf MQTT output + HA ingestion; node_exporter (Prometheus
   ecosystem); Glances (the classic HA integration via MQTT); Netdata
   (single-host monitoring with MQTT export); Telegraf vs custom Rust agent
   trade-offs. Is a custom agent even the right shape, or should jupiterOS
   adopt Telegraf + HA MQTT discovery glue? (dialectic locus candidate)
E. Testing strategy for a hardware-touching daemon: mock sysfs, D-Bus test
   harness, integration tests against a real mosquitto (nix develop already
   ships mosquitto), `cargo-nextest`, flake checks + CI.
F. Feature gaps for "jupiterOS agent": systemd unit health (ROADMAP open
   item), SMART/NVMe health, ZFS status (unverified in live env!),
   headscale/tailscale mesh state, container/podman monitoring, UPS via
   NUT, reboot/required-reboot detection (nixos-rebuild switch applied?),
   nix store gc status, generation/boot health, version reporting (nix flake
   rev → HA sensor), remote commands: reboot, poweroff (logind), and the
   ROADMAP's Steam/Heroic/OBS layers.

## Modality classification rationale

The query asks for improvement direction: "broaden the research topic to
improving it". This is a synthesize modality — the report must defend an
ordered improvement plan (reliability first, fleet-wide deployment second,
feature gaps third) against external comparison agents (Telegraf/Glances/
Netdata "why not just use X" is the natural dialectic) and against the
repo's own audit evidence. Collect-modality (enumerate every possible
feature) would produce a shopping list without priorities; the user's "I
want it to run on EVERY host" is a deployment-architecture claim that needs
argued trade-offs (user-service vs system-service on headless hosts), not a
list.

## Tier rationale (filled after step 1)

Classified `full` + `argumentative`: the query demands a defended improvement
thesis ("broaden the research topic to improving it", "I want it to run on
EVERY host") grounded in a verified code audit, an external comparison set
(5+ agents), and a contested build-vs-buy dialectic — multi-faceted,
research-grade, with the register `advocate` (argue for a specific course of
action) since the user asks for an improvement direction, not a survey.

## Wrapper requirements

None.
