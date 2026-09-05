# Coverage report — step 2 width sweep

**Corpus**: 128 substantive notes (135 tagged, 7 deprecated), 116 claims files, ~500 extracted claims. Target 55–80 exceeded (128) — draft curation in step 10 will rank down to the envelope.

## Atomic item coverage (post-wave-1)

| Atomic item | Sources | Status |
|---|---|---|
| Sub-Q1: verified defect audit | repo-map-ha-linux-agent + direct code reads (agent.rs, discovery.rs, config.rs, backend-*.rs) + build/test/clippy exit codes (pre-pipeline ground truth) + rumqttc lib.rs/eventloop.rs (clean_session default) | **Well-covered** (code + 4 external) |
| Sub-Q2: MQTT lifecycle | mqtt-home-assistant (+2, -3), rumqttc #250/#211, eventloop.rs + lib.rs source, docs.rs EventLoop, 7 community failure threads, HA #135266 + #38661, ESPHome, Z2M, HiveMQ LWT, HA discovery.py + abbreviations.py source, mosquitto #2117 | **Well-covered** (15+) |
| Sub-Q3: proper Linux agent sensor/command set | HASS.Agent repo/releases/docs (4), go-hass-agent repo (7,022 words) + nixpkgs + releases, lnxlink repo + docs (~50 modules), companion-app sensors (8,199 words), ha_desktop_companion, halinuxcompanion, IoPC, WTH thread | **Well-covered** (12+) |
| Sub-Q4: NixOS fleet architecture | NixOS wiki user services, Arch wiki systemd/user, nixpkgs #3702 + PR #260248 (linger option), discourse linger + user-service threads, openclaw headless ladder, baeldung, chimera dbus, systemd.unit/DocBook OnFailure pattern, NixOS manual (Mosquitto ACL deny-all, module authoring, NixOS tests), Extend_NixOS, flakes output schema, flake-utils | **Well-covered** (14+) |
| Sub-Q5: build-vs-buy dialectic | Telegraf MQTT README + docs + homie spec, Telegraf2Hassio, Glances integration doc + 4 failure issues, Netdata footprint + memory thread + optimisation guide, Prometheus HA export-only + node-exporter add-ons, smartctl_exporter, koying counter-position, ha_desktop_companion (no-MQTT counter-architecture) | **Well-covered** (14+) |
| Sub-Q6: jupiterOS feature gaps | ipnstate Go structs, tailscale CLI ref + #9378 + #17619, alexwlchan BackendState, Oracle ZFS states + OneUptime ZFS, ArchWiki SMART + smartd -M exec + smartd.conf man, NVMe CAP_SYS_ADMIN (scrutiny #26), HA NUT doc + ceilings, EndeavourOS NVMe, netdata systemd-units spec, systemd_mon | **Well-covered** (13+) |
| Sub-Q7: testing strategy | testcontainers mosquitto (3), zbus (2, session-bus CI constraint), std::fs mocking forum, tempfile 3.27 (SIGINT leak), dbraun mqtt-test, hivemq blog, NixOS manual (runNixOSTest) | **Well-covered** (9+) |
| Sub-Q8: ordered roadmap | Derived from Q1–Q7 evidence; no standalone sources needed — synthesis product | **Covered by construction** |
| Entity: rumqttc | lib.rs, eventloop.rs, docs.rs ×2, #211, #250 | Well-covered |
| Entity: zbus/sysinfo/tempfile | Batch 7 + 9 | Well-covered |
| Entity: TCxWave kiosks | jupiter-os configs (direct) + tcxwave modules (direct) + niri IPC spec | Well-covered |
| Entity: headless hosts | linger PR #260248, openclaw, zbus session-bus, Arch wiki | Well-covered |

## Gaps & notes

1. **Browser-lane escalations (2 queued, deferred lane)**: freedesktop logind wiki pages (418 bot wall — content recovered via systemd DocBook source); native-app-integration API docs (login wall). Neither blocks coverage: logind D-Bus semantics are covered by systemd source; the native-app API is covered via go-hass-agent's description of it. Reddit 403s recovered via search excerpts (LOW reliability, flagged).
2. **Utility scores could not be persisted** (fetch-batch lacks per-URL --utility-score) — score mapping retained in scored-urls.md; ranking covered by claims density + sources score instead.
3. **Long sources flagged for step-3/5 attention**: go-hass-agent README (7,022 w), companion sensors (8,199 w), nixos-manual (88.5k w — fleet chapters already claim-extracted), systemd.unit DocBook (13.5k w), systemd.service DocBook (11.2k w), NVMe tools comparison (6.7k w).
4. **Dialectic representation**: koying's "MQTT is an unnecessary point of failure" counter-position (batch 3) + ha_desktop_companion native-API architecture (batch 10) vs MQTT-native consensus (halinuxcompanion migrating TO MQTT) — genuine two-sided corpus.

No uncovered atomic items. Wave 2 unnecessary — coverage target met in wave 1.
