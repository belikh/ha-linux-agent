# Synthesis outline

## Executive summary
The "sloppy, never reliable" state is two precisely-diagnosed MQTT lifecycle bugs plus default-shaped traps, all fixable on the pinned dependency; the fix-order is reliability → fleet-wide system-service deployment (all 7 hosts, role-driven) → features, because the fleet's deciding features are what both the agent incumbents and the monitoring incumbents cannot express.

## 1. Current State: A Verified Reliability Audit
Build/test/clippy exit codes, the file:line defect list, 8-tests-none-run, the silent chmod/EACCES defect, the live unavailable-kiosks symptom, the 4/7 coverage table with per-host opt-outs and the pallene secrets decision.

## 2. The MQTT Lifecycle Problem: Why It Is Never Reliable
Subscribe-once + availability-once (agent.rs:134-144) against rumqttc #250/#211 and HA's birth contract; HASS.Agent #230 precedent; the ConnAck-driven supervisor spec with per-backend isolation; broker-side hygiene (persistence, $SYS check) recalibrated to real scale.

## 3. What a Proper Linux Agent Looks Like: The Comparison Set
HASS.Agent as the feature bar; go-hass-agent v14.15.1 (depth + admissions: unsynced controls, split-brain, v5 requirement, no NixOS module); lnxlink (~50 modules, unpackaged); companion-app vocabulary; the open Linux niche.

## 4. The Build-vs-Buy Dialectic: Custom Rust Agent vs Telegraf, Glances, and Netdata
Monitoring incumbents eliminated on audited evidence; the agent-vs-agent steelman at full strength ("systemd design wearing a Rust coat", lnxlink Steam dropdown) answered by packaging + typed surface + state round-trip; the honest concession table; the verdict: improve, with named flip conditions.

## 5. NixOS Fleet Architecture: From Kiosk-Only to Every Host in jupiterOS
One system service per host (User=io), user-unit shape retired on the PR #517768 evidence; zero-capability ladder over AmbientCapabilities; the role-driven module moving out of the kiosk profile; DBUS/niri socket reachability; linger footgun; sops-nix wiring; the 7-host rollout with per-host roles.

## 6. JupiterOS-Specific Feature Gaps
The per-subsystem event-driven unprivileged matrix (smartd hooks, zed bridge, D-Bus match rules, udev video group) as the design principle; watch_units; nix generation health; version-tolerant tailscale schema; NUT; ROADMAP leftovers.

## 7. Testing a Hardware-Touching Daemon
The 7-step ladder: clippy → cargoTest (8 tests running) → sysfs injection → Agent::run seam → mosquitto-subprocess harness with 5 defect-named regression tests (Nix sandbox loopback confirmed) → dbus harness → runNixOSTest; the containerless-fleet override of the testcontainers consensus.

## 8. Ordered Improvement Roadmap
Phase 0 (days, no behaviour change), Phase 1 (reliability + tests), Phase 2 (fleet deployment, all 7 hosts), Phase 3 (features via the event matrix); flip conditions for adoption; verification per phase.

## Opinionated Synthesis
The committed reading: the agent's problems are small and precisely known, its differentiation is real and uncovered, and the only defensible order is reliability → fleet → features; the strongest objection (adopt go-hass-agent) fails on packaging + expressibility but stays live as named flip conditions.
