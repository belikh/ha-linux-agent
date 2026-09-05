# Search Plan — linux-agent-jupiteros-fleet-15537b

| Atomic item | Search query | Type | Lens | Target |
|---|---|---|---|---|
| Sub-Q2 MQTT lifecycle | Home Assistant MQTT discovery availability birth message last will | web | breadth | HA docs (canonical) |
| Sub-Q2 MQTT lifecycle | MQTT broker restart discovery re-publish reconnect best practice | web | breadth | best practice |
| Sub-Q2 MQTT lifecycle | rumqttc eventloop reconnect pattern tokio | web | breadth | crate usage patterns |
| Sub-Q2 MQTT lifecycle | rumqttc reconnection issues connection drops | web | adversarial | GitHub issues, failure cases |
| Sub-Q2 MQTT lifecycle | MQTT client architecture reliable telemetry paper | academic | depth | canonical MQTT research |
| Sub-Q2 MQTT lifecycle | MQTT QoS reliability evaluation study | academic | depth | MQTT literature |
| Sub-Q3 agent comparison | HASS.Agent Windows sensors features | web | breadth | comparison agent |
| Sub-Q3 agent comparison | HASS.Agent limitations problems | web | adversarial | comparison agent |
| Sub-Q3 agent comparison | Home Assistant companion app Android sensor list | web | breadth | sensor-set baseline |
| Sub-Q5 build-vs-buy | Telegraf MQTT output plugin Home Assistant discovery | web | breadth | Telegraf→HA path |
| Sub-Q5 build-vs-buy | Telegraf limitations Home Assistant integration | web | adversarial | trade-offs |
| Sub-Q5 build-vs-buy | Glances Home Assistant integration MQTT sensor | web | breadth | Glances→HA path |
| Sub-Q5 build-vs-buy | Glances HA integration problems broken | web | adversarial | failure cases |
| Sub-Q5 build-vs-buy | Netdata MQTT export monitoring | web | breadth | Netdata path |
| Sub-Q5 build-vs-buy | Netdata criticism resource usage heavy | web | adversarial | trade-offs |
| Sub-Q5 build-vs-buy | node_exporter Home Assistant Prometheus MQTT | web | breadth | Prometheus-ecosystem fit |
| Sub-Q5 build-vs-buy | MQTT vs Prometheus telemetry tradeoffs monitoring | web | adversarial | architecture dialectic |
| Sub-Q4 NixOS fleet | NixOS systemd user service linger headless no session | web | breadth | headless user services |
| Sub-Q4 NixOS fleet | systemd user services pitfalls problems linger | web | adversarial | failure cases |
| Sub-Q4 NixOS fleet | NixOS flake nixosModules shared config fleet | web | breadth | module patterns |
| Sub-Q4 NixOS fleet | sops-nix secrets files systemd services | web | depth | secrets pattern |
| Sub-Q4 NixOS fleet | systemd --user service without graphical session dbus | web | breadth | D-Bus session problem |
| Sub-Q4 NixOS fleet | dbus-broker system bus session bus daemon headless | web | breadth | D-Bus architecture |
| Sub-Q6 jupiterOS features | SMART monitoring smartctl NVMe health sensor | web | breadth | SMART backend |
| Sub-Q6 jupiterOS features | ZFS zpool status monitoring automation alerting | web | breadth | ZFS backend |
| Sub-Q6 jupiterOS features | systemd unit state monitoring is-failed alerting | web | breadth | unit-health backend |
| Sub-Q6 jupiterOS features | tailscale status JSON schema fields | web | depth | mesh backend verification |
| Sub-Q6 jupiterOS features | NUT network UPS monitoring Home Assistant | web | breadth | UPS sensor |
| Sub-Q6 jupiterOS features | NixOS generation boot entries rollback health check | web | breadth | nix health sensor |
| Sub-Q7 testing | Rust testing mock filesystem sysfs tempdir | web | breadth | mock sysfs |
| Sub-Q7 testing | Rust D-Bus testing zbus test harness | web | breadth | D-Bus harness |
| Sub-Q7 testing | Rust MQTT integration test mosquitto testcontainer | web | breadth | broker test harness |
| Sub-Q7 testing | cargo nextest workspace CI best practice | web | breadth | test tooling |
| Sub-Q1 audit (MQTT root cause) | MQTT retained message problems stale state | web | adversarial | retained-state smell |
| Sub-Q1 audit | tokio select spawn background task error handling pattern | web | breadth | task supervision patterns |
| Entity: HA MQTT docs | site:home-assistant.io mqtt discovery availability | web | depth | canonical docs |
| Entity: rumqttc | rumqttc crate documentation AsyncClient | web | depth | crate docs |
| Entity: zbus | zbus crate documentation session bus system bus | web | depth | crate docs |
| Entity: sysinfo | sysinfo crate documentation refresh sensors | web | depth | crate docs |
| Entity: logind | systemd-logind D-Bus API documentation | web | depth | freedesktop docs |
| Sub-Q5 (uptime-kuma etc.) | uptime kuma vs home assistant device monitoring | web | breadth | adjacent tooling |
| Sub-Q2 (HA side) | Home Assistant MQTT device unavailable after broker restart | web | adversarial | HA-side symptom reports |
| Sub-Q6 reboot cmd | systemd-logind Reboot PowerOff D-Bus privileged | web | depth | remote reboot commands |
| Sub-Q4 kiosk fleet | kiosk fleet management Linux NixOS | web | breadth | fleet pattern context |

Repo lane (Lens E) — already executed pre-pipeline:
- `hpr repo map .` → note repo-map-ha-linux-agent (structural map)
- `hpr repo wiki belikh/ha-linux-agent` → 13 words (unindexed in DeepWiki)
- jupiter-os local checkout read directly (hosts/, modules/services/ha-agent.nix, tcxwave-kiosk.nix)

Academic sweep (Lens B, targeted): arXiv + Semantic Scholar on MQTT reliability/QoS and agent-based monitoring — small, this topic's real literature is docs and code, not papers.
