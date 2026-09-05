# Scored URL queue — linux-agent-jupiteros-fleet-15537b

Utility score = authority + novelty + stance + coverage + redundancy + freshness (0-3 each, max 18).

## Batch 1 — MQTT lifecycle & HA discovery contract (11 URLs)
| URL | Score | Why |
|---|---|---|
| https://www.home-assistant.io/integrations/mqtt/ (MQTT integration doc: birth/LWT, discovery+availability, "re-publish discovery on birth") | 17 | Canonical protocol contract; load-bearing for Sub-Q2 |
| https://www.home-assistant.io/docs/mqtt/discovery/ | 16 | Canonical discovery semantics |
| https://github.com/bytebeamio/rumqtt/issues/250 (subscriptions lost on reconnect; re-subscribe after ConnAck) | 17 | Direct root-cause evidence for the agent's silent-death bug |
| https://github.com/bytebeamio/rumqtt/issues/211 (reconnect + pending TTL; ConnAck detection) | 14 | rumqttc reconnect semantics |
| https://docs.rs/rumqttc/latest/rumqttc/ (eventloop semantics, poll-reconnect) | 15 | Crate docs (primary) |
| https://github.com/bytebeamio/rumqtt/blob/master/rumqttc/src/eventloop.rs | 13 | Source-level evidence |
| https://community.home-assistant.io/t/mqtt-discovery-availability/206758 (availability_topic ordering: unavailable until avail msg; retained state matters) | 14 | HA-side behaviour detail |
| https://community.home-assistant.io/t/my-hand-built-mqtt-devices-are-unavailable-after-hass-io-restart-but-zigbee2mqtt-ones-are/201214 (retain discovery+state = survives HA restart) | 14 | Practical retain semantics |
| https://github.com/home-assistant/core/issues/135266 (some devices unavailable after HA restart) | 12 | Failure-mode evidence |
| https://community.home-assistant.io/t/mqtt-devices-unavailable-after-mqtt-integration-restart/725798 | 11 | Symptom report |
| https://www.zigbee2mqtt.io/guide/configuration/device-availability.html (Z2G availability pattern — retained, per-device topic) | 12 | Mature-agent comparison pattern |

## Batch 2 — comparison agents: HASS.Agent + go-hass-agent (11 URLs)
| URL | Score | Why |
|---|---|---|
| https://github.com/hass-agent/HASS.Agent (Windows companion; "What it's not: a Linux/macOS client"; MQTT-based) | 15 | Comparison baseline + explicit Linux gap |
| https://github.com/hass-agent/HASS.Agent/releases (v2.1.1 fixed "MQTT autodiscovery messages were not republished after connection was lost and recovered" #230) | 16 | EXTERNAL VALIDATION of the exact bug class in ha-linux-agent |
| https://www.hass-agent.io/latest/getting-started/sensors/ | 10 | Sensor-set baseline |
| https://github.com/joshuar/go-hass-agent (Linux-native companion; sensors+MQTT controls+D-Bus+events) | 17 | Strongest existing Linux agent; direct competitor |
| https://mynixos.com/nixpkgs/package/go-hass-agent (in nixpkgs, v14.15.1) | 12 | Packaging maturity evidence |
| https://community.home-assistant.io/t/go-hass-agent-a-native-app-integration-for-desktop-laptop-devices/559250 | 10 | Community reception |
| https://pkg.go.dev/github.com/joshuar/go-hass-agent (v1-era sensor list) | 10 | Historical sensor baseline |
| https://github.com/maksimkurb/IoPC (Linux IoT HA client, referenced by HASS.Agent README) | 11 | Second Linux competitor |
| https://companion.home-assistant.io/docs/core/sensors (companion app sensor baseline) | 13 | Canonical "what a companion reports" |
| https://companion.home-assistant.io/docs/core (feature overview) | 11 | Scope framing |
| https://www.reddit.com/r/homeassistant/comments/1fgr8tm/hass_agent_alternative_for_linux/ (demand for Linux equivalent) | 9 | Ecosystem-gap evidence |

## Batch 3 — build-vs-buy: Telegraf / Glances (10 URLs)
| URL | Score | Why |
|---|---|---|
| https://github.com/influxdata/telegraf/blob/master/plugins/outputs/mqtt/README.md (no HA-discovery layout; homie only; "lifecycle management very limited"; cannot set will dynamically) | 16 | Load-bearing limitation, official source |
| https://docs.influxdata.com/telegraf/v1/output-plugins/mqtt | 13 | Official docs (duplicate of README, keep for depth) |
| https://www.home-assistant.io/integrations/glances (REST-based integration, sensor set) | 13 | Canonical Glances→HA path |
| https://github.com/home-assistant/core/issues/110551 (Glances integration loses connection, doesn't recover) | 15 | Adversarial: maturity of alternatives |
| https://github.com/home-assistant/core/issues/170384 (Glances stops reporting current data) | 12 | Adversarial |
| https://github.com/home-assistant/core/issues/32328 (Glances sensors renamed after restart) | 11 | Adversarial |
| https://community.home-assistant.io/t/releasing-telegraf2hassio-for-remote-servers-monitoring/406318 (community bridge Telegraf→HA discovery exists as third-party bandaid) | 12 | Ecosystem evidence |
| https://github.com/nicolargo/glances/issues/2504 (connect failures running as service) | 10 | Adversarial |
| https://www.derekseaman.com/2023/04/home-assistant-monitor-proxmox-with-glances.html (real deployment pattern + drivetemp modprobe) | 9 | Practitioner context |
| https://community.home-assistant.io/t/system-design-architecting-ha-data-flows-with-telegraf-mqtt-influxdb-and-grafana/312723 (architecture discussion) | 9 | Practitioner context |

## Batch 4 — build-vs-buy: Netdata / node_exporter + monitoring approaches (10 URLs)
| URL | Score | Why |
|---|---|---|
| https://learn.netdata.cloud/docs/netdata-agent/resource-utilization (100-200MB RAM, 1-5% core, root required) | 14 | Footprint evidence for kiosk fitness |
| https://community.netdata.cloud/t/insane-netdata-memory-usage/3342 (memory blowup reports + maintainer response) | 12 | Adversarial |
| https://www.netdata.cloud/resources/best-nvme-ssd-monitoring-tools/ (NVMe/SMART tooling landscape incl. Netdata/Zabbix/smartd) | 12 | Server-sensor landscape |
| https://www.home-assistant.io/integrations/prometheus (HA's Prometheus integration is export-only, not host metrics) | 13 | Closes the "just use node_exporter with HA" path |
| https://community.home-assistant.io/t/new-add-on-prometheus-node-exporter/354629 (HA doesn't ingest node_exporter natively) | 11 | Same, community voice |
| https://github.com/racksync/hass-addons-prometheus-node-exporter | 8 | Same, third-party add-on |
| https://learn.netdata.cloud/docs/collecting-metrics/collectors/databases/mosquitto (Netdata via exporters) | 9 | Architecture context |
| https://www.netdata.cloud/monitoring-101/systemdunits-monitoring/ (systemd unit state monitoring reference) | 10 | Unit-health design reference |
| https://github.com/joonty/systemd_mon (D-Bus signal-driven unit monitoring, no polling) | 11 | Design alternative for watch_units |
| https://oneuptime.com/blog/post/2026-03-04-monitor-storage-health-smart-smartctl-rhel-9/view (smartctl -H / NVMe key metrics / smartd) | 10 | SMART backend design reference |

## Batch 5 — NixOS fleet architecture + headless (11 URLs)
| URL | Score | Why |
|---|---|---|
| https://wiki.nixos.org/wiki/Systemd/User_Services/en (linger, multi-user.target for boot-time start, minimal systems lack user@.service) | 15 | Load-bearing for fleet-wide user-service deployment |
| https://serverfault.com/questions/892465/starting-systemd-services-sharing-a-session-d-bus-on-headless-system (linger + user dbus socket pattern) | 14 | The exact headless D-Bus recipe |
| https://www.baeldung.com/linux/systemd-session-dbus-headless-setup (step-by-step headless session-bus) | 11 | Same pattern, tutorial form |
| https://chimera-linux.org/docs/configuration/dbus (system vs session bus; user services own the bus at /run/user/N/bus) | 12 | Architecture grounding |
| https://discourse.nixos.org/t/what-is-the-difference-between-systemd-services-and-systemd-user-services/25222 | 9 | NixOS framing |
| https://discourse.nixos.org/t/adding-nixos-option-for-systemd-user-lingering/28762 (linger option discussion; headless syncthing case) | 12 | NixOS-native linger handling |
| https://discourse.nixos.org/t/how-to-create-user-systemd-service/57671 (systemctl --user pitfalls via sudo) | 9 | Pitfall documentation |
| https://github.com/openclaw/openclaw/issues/11805 (headless user-level systemd + XDG_RUNTIME_DIR) | 10 | Modern headless failure case |
| https://bbs.archlinux.org/viewtopic.php?id=221340 (session-bus socket plumbing history) | 8 | Deep background |
| https://github.com/hassio-addons/app-glances/blob/main/glances/DOCS.md | 7 | Add-on packaging contrast |
| https://search.nixos.org/options?query=users.users (linger option exists in NixOS) — fetch via https://search.nixos.org/options?query=users.users.%3Cname%3E.linger | 9 | Option reference |

## Batch 6 — jupiterOS feature gaps: ZFS / tailscale / NUT / nix health (10 URLs)
| URL | Score | Why |
|---|---|---|
| https://docs.oracle.com/cd/E19253-01/819-5461/gamno/index.html (zpool status states: ONLINE/DEGRADED/FAULTED...) | 12 | ZFS health semantics (canonical) |
| https://oneuptime.com/blog/post/2026-03-02-how-to-monitor-zfs-pool-health-and-status-on-ubuntu/view (zpool list -H monitoring + zed) | 10 | Practitioner ZFS monitoring |
| https://pkg.go.dev/tailscale.com/ipn/ipnstate (BackendState values, ExitNodeStatus, TailscaleIPs — the actual Go structs behind tailscale status --json) | 15 | Closes ROADMAP's "schema not pinned down" gap — authoritative source code |
| https://github.com/tailscale/tailscale/issues/9378 (exit-node list derives from status --json; .Peer[].ExitNodeOption) | 12 | Exit-node field confirmation |
| https://alexwlchan.net/notes/2025/check-if-tailscale-is-running/ (BackendState practical check) | 10 | Practitioner pattern |
| https://github.com/tailscale/tailscale/issues/17619 (--json format versioning concerns: legacy v1 implicit) | 11 | Schema-stability caveat |
| https://tailscale.com/docs/reference/tailscale-cli (CLI reference incl. --json warnings) | 11 | Official CLI docs |
| https://www.home-assistant.io/integrations/nut (NUT UPS integration in HA) | 10 | UPS sensor path |
| https://www.netdata.cloud/monitoring-101/mqtt_blackbox-monitoring/ (blackbox MQTT monitoring idea) | 7 | Adjacent pattern |
| https://forum.endeavouros.com/t/what-s-the-best-way-to-monitor-nvme-health/76296 (nvme-cli vs smartctl practitioner) | 8 | SMART tool choice |

## Batch 7 — testing strategy (10 URLs)
| URL | Score | Why |
|---|---|---|
| https://testcontainers.com/modules/mosquitto (Mosquitto testcontainer incl. Rust testcontainers-modules) | 14 | Direct integration-test path for the agent |
| https://docs.rs/testcontainers-modules/latest/testcontainers_modules/mosquitto/struct.Mosquitto.html | 13 | Rust-specific API |
| https://users.rust-lang.org/t/mocking-std-fs-for-unit-tests/22382 (std::fs can't be mocked; tempdir approach) | 12 | sysfs-mock design constraint |
| https://www.reddit.com/r/rust/comments/yfharn/write_tests_around_the_file_system/ (tempdir + real fs in tests) | 10 | Practitioner consensus |
| https://github.com/z-galaxy/zbus (zbus "Integration tests currently require a session bus on the build host") | 13 | D-Bus testing constraint, official |
| https://docs.rs/zbus/latest/zbus | 10 | Crate docs |
| https://docs.rs/tempfile (tempdir) — https://docs.rs/tempfile/3.0.4/tempfile/fn.tempdir.html | 9 | Tool reference |
| https://github.com/testcontainers/testcontainers-python/issues/568 (raspy2mqtt author's mosquitto testcontainer motivation) | 9 | Comparable project's test design |
| https://github.com/dbraun1991/mqtt-test (mosquitto testcontainer reproducibility) | 8 | Pattern example |
| https://www.hivemq.com/blog/hivemq-is-now-available-in-testcontainers/ (why broker-in-test matters) | 8 | Rationale |

## Batch 8 — MQTT protocol depth + retained messages (10 URLs)
| URL | Score | Why |
|---|---|---|
| https://emqx.medium.com/how-to-use-mqtt-in-rust-with-rumqttc-client-eec0748d56fa (rumqttc usage patterns) | 10 | Practitioner tutorial |
| https://docs.rs/rumqttc/latest/rumqttc/struct.EventLoop.html (poll reconnects; clean(); access state) | 13 | Crate docs |
| https://github.com/home-assistant/core/blob/dev/homeassistant/components/mqtt/discovery.py (HA-side discovery processing) | 12 | Protocol counterpart |
| https://esphome.io/components/mqtt/ (discovery_retain default true; birth/LWT; HA restart retain semantics) | 13 | Mature-agent comparison |
| https://community.home-assistant.io/t/setting-mqtt-birth-and-last-will-with-the-new-mqtt-integration/77862 (birth message triggers devices to re-send discovery) | 13 | The birth-trigger pattern in practice |
| https://community.home-assistant.io/t/working-with-devices-birth-lwt/290835 | 9 | Availability patterns |
| https://community.home-assistant.io/t/mqtt-discovery-availability-topics-not-receiving/758032 | 9 | Failure case |
| https://community.home-assistant.io/t/mqtt-entities-become-unknown-on-reboot/848716 (startup ordering race) | 10 | Race evidence |
| https://github.com/home-assistant/core/issues/38661 (sensors unavailable until discovery resent; availability ordering) | 11 | Failure case |
| https://www.facebook.com/groups/HomeAssistant/posts/3898642603740414/ — SKIP (login wall, low value) → replaced with https://community.home-assistant.io/t/use-automation-for-mqtt-autodiscovery/455475 | 8 | Discovery retrigger pattern |

## Batch 9 — sysfs/hardware + logind + crate docs (10 URLs)
| URL | Score | Why |
|---|---|---|
| https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-backlight (backlight sysfs ABI) — via https://docs.kernel.org/ABI/testing/sysfs-class-backlight.html | 11 | Canonical hardware ABI |
| https://docs.kernel.org/admin-guide/pm/cpufreq.html (cpufreq governor + EPP semantics) — https://docs.kernel.org/admin-guide/pm/cpu-freq.html | 12 | Canonical for governor/EPP backend |
| https://www.freedesktop.org/wiki/Software/systemd/logind/ (logind D-Bus API) | 12 | Canonical for lock/suspend/idle |
| https://www.freedesktop.org/software/systemd/man/systemd-logind.service.html — via https://www.freedesktop.org/software/systemd/man/latest/systemd-logind.service.html | 11 | Canonical man page |
| https://docs.rs/sysinfo (sysinfo crate) | 10 | Dependency docs |
| https://www.kernel.org/doc/html/latest/driver-api/thermal/sysfs-api.html (thermal zone sysfs) | 9 | hwmon/thermal reference |
| https://bbs.archlinux.org/viewtopic.php?id=248157 (NVMe smartctl output shape) | 9 | Output format reference |
| https://superuser.com/questions/1750390/how-to-check-nvme-ssd-with-smartctl (smartctl on nvme nodes) | 9 | Same |
| https://discuss.kde.org/t/how-to-get-notifications-if-a-systemd-unit-fails/5506 (unit-fail notification approaches) | 8 | Practitioner pattern |
| https://forum.endeavouros.com/t/what-s-the-best-way-to-monitor-nvme-health/76296 — dup guard, in batch 4 already → replace with https://github.com/AnalogJ/scrutiny (Scrutiny SMART monitoring webUI) | 8 | SMART tooling landscape |

## Batch 10 — fleet/kiosk context + remaining (9 URLs)
| URL | Score | Why |
|---|---|---|
| https://nixos.org/manual/nixos/stable/ (NixOS manual — module authoring patterns) | 9 | Platform docs |
| https://wiki.nixos.org/wiki/Extend_NixOS (module extension patterns) | 9 | Same |
| https://github.com/YaLTeR/niri (niri compositor — IPC context for backend-niri) | 10 | Dependency context |
| https://github.com/numtide/flake-utils (flake pattern used by this repo) | 7 | Tooling context |
| https://www.reddit.com/r/NixOS/comments/ (fleet patterns) — SKIP low value → replace with https://nixos.wiki/wiki/Flakes (flake patterns) | 7 | Platform context |
| https://learn.netdata.cloud/docs/collecting-metrics/collectors/synthetic-testing/mqtt-blackbox | 7 | Adjacent |
| https://www.hass-agent.io/2.2/getting-started/commands/ (HASS.Agent command model) | 9 | Comparison |
| https://hass-agent.io/latest/getting-started/sensors/ — dup of batch 2 → replace with https://www.hass-agent.io/latest/ (docs home: feature quadrants) | 8 | Comparison |
| https://community.home-assistant.io/t/wth-there-is-no-windows-and-linux-ha-app-similar-to-macos/810188 (WTH: no Linux HA app — ecosystem demand) | 11 | Ecosystem-gap evidence |

Total: 101 URLs across 10 batches. Every atomic item has ≥3 candidate URLs.
