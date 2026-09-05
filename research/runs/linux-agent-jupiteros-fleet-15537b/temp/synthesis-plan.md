# Synthesis plan

## Core thesis (1-2 sentences)

ha-linux-agent's "never reliable" is not vagueness — it is two precisely-located bugs (subscribe-once at agent.rs:134-138; availability-published-once at agent.rs:139-144) amplified by three default-shaped traps, and the correct plan is a strict order: fix the MQTT lifecycle first (ConnAck-driven supervisor on the pinned rumqttc 0.24.0), deploy fleet-wide second (one system service per host, User=io, role-driven NixOS module replacing the kiosk-only wiring), and add features third — because the fleet's deciding features (the launcher mutual-exclusion session select, the declarative per-host module, the ZFS/headscale/syncthing server backends) are exactly what the incumbents cannot express, and exactly what the monitoring incumbents (Glances/Telegraf/Netdata) fail at too.

## The 3-7 strongest argumentative beats

1. **Two-bug diagnosis with external precedent** (Draft A): the "never reliable" symptom is permanently-diagnosed deafness + dead-availability; HASS.Agent shipped the identical bug class and fixed it in 2.1.1 (#230) — the ConnAck supervisor is the known cure, not a speculative design. The live presenting symptom (four kiosks reading unavailable in HA right now) opens the report.
2. **The dependency verdict by measurement** (Draft A/C + fitness interim): keep rumqttc 0.24.0 — PR #1052 is open/unreviewed/v5-scoped, Paho leaves re-subscription application-owned too, and the pinned version already ships the v5 module (the v3.1.1-only claim was factually wrong). No client switch buys reliability; the work is application-side forever.
3. **The steelman at full strength, then the packaging counter** (Draft B, resolved per conflicts file): "the launcher select is one bash module away; lnxlink has a Steam dropdown; the backend is a systemd design wearing a Rust coat" — answered by: lnxlink is not in nixpkgs and its dependency stack resisted packaging (audited), go-hass-agent has zero select/state-sync in 11k words of changelog plus documented split-brain, and neither incumbent has a NixOS module; the typed declarative entity surface + state round-trip is the actual differentiation, with the mechanism-vs-surface decomposition conceded.
4. **The 2026 boundary redraw** (Draft C): PR #517768 (May 2026) makes switch-to-configuration restart user units for every live user manager — user units went from stale-on-switch (the old reason to retire them) to disruptively-restarted-on-switch (the new, worse reason). Cite stc-ng source + the GDM-kickout regression thread; retire the user-unit shape on current evidence.
5. **The zero-capability privilege matrix** (privilege interim, live-proven): smartd -M exec hooks (no CAP_SYS_ADMIN), ZFS procfs + zedlet bridge (no root; ZED auto-enabled), system-bus D-Bus match rules (live-proven unprivileged on callisto), udev GROUP=video + KOBJ_CHANGE sync (kernel-source-proven) — and delete the broken sysfs-chmod workaround whose group-root writes io (wheel-only) can never use: brightness writes fail EACCES silently today.
6. **notify.mqtt confirmed end-to-end** (step-8 wave): "notify" is in mqtt/const.py SUPPORTED_COMPONENTS; PR #115653 (HA 2024.5, Quality Scale gold) added the platform; the upstream test publishes the working payload — one-way only. Plus the dead-code finding: no notification daemon ships anywhere in the fleet; the kiosk last-mile is the customer-display VFD.
7. **The testing ladder as definition-of-done** (testing interim + gap-04): clippy fix → cargoTest wiring (the 8 existing tests currently run nowhere) → sysfs injection → Agent::run seam → mosquitto-subprocess harness with 5 defect-named regression tests (Nix sandbox loopback upstream-blessed; testcontainers dead-on-arrival on this containerless fleet) → private dbus harness → runNixOSTest.

## Section structure

The 8 required H2 headings from prompt-decomposition.json, in order, plus `## Opinionated Synthesis` (mandated by decomposition) and a Source Tensions treatment woven through (per the tensions file, each tension engaged as a visible beat — the dedicated tensions material lives inside sections 2, 4, 5 as argument).

## Per-section commitments

### Section 1: Current State: A Verified Reliability Audit
- Evidence: build/test/clippy exit codes (0/0/101 — the clippy gate FAILS on backend-hardware:117 map_or); the defect list with file:line; 8-tests-none-run; README drift (backend-hardware undocumented); dead tls knob (config.rs:88); hardware backend's fake select options; the io-not-in-video chmod defect; the live unavailable-kiosks symptom; the 4/6 coverage table with per-host opt-outs .
- Beat: the audit as ground truth — measured exit codes and verbatim source, not vibes. Opens the report.

### Section 2: The MQTT Lifecycle Problem: Why It Is Never Reliable
- Evidence: agent.rs subscribe-once + availability-once; rumqttc #250 (5 years open) + #211; clean_session=true default; HA birth-message contract; HASS.Agent #230 precedent; callisto mosquitto persistence false + the recalibrated max_queued finding ($SYS check, hygiene sizing); the supervisor spec itself (ConnAck handler, jitter, SIGTERM offline-then-disconnect, per-backend isolation fixing the hung-syncthing stall, cap-0).
- Tension engaged: retain-everything vs republish → resolved as the reliability contract (retain AND republish).

### Section 3: What a Proper Linux Agent Looks Like: The Comparison Set
- Evidence: HASS.Agent (Windows-only, the feature bar), go-hass-agent (v14.15.1, sensor depth, unsynced controls, split-brain, refuses root, no NixOS module), lnxlink (~50 modules, Steam dropdown, not packaged), companion-app sensor vocabulary, the WTH thread's open niche.
- Beat: the comparison set as a feature bar table with honest concessions (~25% sensor coverage).

### Section 4: The Build-vs-Buy Dialectic: Custom Rust Agent vs Telegraf, Glances, and Netdata
- Evidence: Telegraf (no HA discovery, homie-only, no dynamic LWT), Glances (6-year silent-failure record, 4 issues), Netdata (footprint + blowups + the stripped-production recipe), node_exporter (HA Prometheus export-only); then the agent-vs-agent fork with the steelman (conflict 1) at full strength and the packaging counter; the maintenance arithmetic vs the inexpressibility argument.
- Tensions engaged: MQTT-hop vs DB (koying — scoped out for this fleet); Netdata footprint dispute.
- Beat: the honest concession table then the verdict — improve, with flip conditions named (go-hass-agent ships select+state-sync or a NixOS module; fleet wants lnxlink's desktop long-tail).

### Section 5: NixOS Fleet Architecture: From Kiosk-Only to Every Host in jupiterOS
- Evidence: the host-class/role design (system service User=io everywhere; AmbientCapabilities only-if-SMART — presented then rejected in favour of the zero-capability ladder); PR #517768 stc-ng change; linger option (PR #260248) and its disable-undeclared-users footgun; DBUS_SESSION_BUS_ADDRESS reachability (mkSessionLauncher precedent); niri socket formula (same-UID globbing); the module moving from kiosk profile to common with per-host roles; sops-nix credential wiring;

### Section 6: JupiterOS-Specific Feature Gaps
- Evidence: the zero-capability matrix per subsystem (smartd hooks, zed bridge, D-Bus match rules + the systemd lazy-signals Subscribe() subtlety, udev video group); unit health watch_units; nix generation/boot health; tailscale ipnstate schema (authoritative, unstable-by-policy → version-tolerant); NUT; the ROADMAP's Steam/Heroic/OBS leftovers.
- Beat: every gap lands as an unprivileged event-driven channel — the privilege design IS the feature design.

### Section 7: Testing a Hardware-Touching Daemon
- Evidence: the 7-step ladder; mosquitto-subprocess harness; Nix sandbox loopback confirmation (local-derivation-goal.cc brings up lo; "two concurrent builds can listen on the same port" since Nix 1.1); the 5 defect-named regression tests; the containerless-fleet ground truth overriding the testcontainers consensus; zbus session-bus CI constraint; runNixOSTest cribbed from nixpkgs' mosquitto test.
- Tension engaged: testcontainers consensus vs fleet ground truth (the meta-principle: web consensus optimises the generic case).

### Section 8: Ordered Improvement Roadmap
- Evidence: phased, dependency-ordered, each phase with verification: Phase 0 (days): clippy fix, cargoTest wiring, notify.mqtt entity, tls knob removal-or-wiring, README/backend-hardware docs. Phase 1 (reliability): the supervisor spec + the 5 regression tests + broker hygiene (persistence true, $SYS check). Phase 2 (fleet): the system-service module + role-driven per-host enabling + europa/callisto rollout + Flip conditions for adoption named throughout.
- Beat: the order is the thesis — reliability first because deaf agents make features meaningless; fleet second because 4/6 coverage contradicts the ambition; features third because the bar is now known.

## Where drafts disagreed

- **Improve vs adopt:** A/C say improve; B says adopt. **Commit to improve** (per conflicts verdict: the packaging counter + typed declarative surface + the incumbents' own gaps), with B's steelman presented at full strength in §4 and its flip conditions carried into §8. Never hedge the verdict.
- **Test count:** 8 (not 3) — zero running in any gate.
- **Launcher select expressibility:** genuine fork — engage honestly with the mechanism/surface decomposition (concede the mechanism is fleet-owned systemd; defend the typed surface as the differentiation).

## Length target

- response_format: argumentative
- Pass 1 target: ~7500 words
- Pass 2 final target: ~6800 words (mid-range of 5000-10000; gate is ±20% of the profile word target)
