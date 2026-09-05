# Draft B — must_read_note_ids (n=42)
Angle: Steelman-contrarian adoption sceptic — defend adopting go-hass-agent/lnxlink or mature tooling; the custom agent is a maintenance trap; engage the minority position at full strength.

## Shared core (7)
- interim-report-mqtt-lifecycle-supervisor-spec
- interim-report-fleet-service-model-by-host-class
- interim-report-adopt-vs-build-honest-verdict
- interim-report-headless-privilege-ladder
- interim-report-rumqttc-dependency-fitness
- interim-report-testing-gate-for-hardware-daemon
- repo-map-ha-linux-agent

## The incumbents' strength (10)
- github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device (7,022 words — sensor/feature depth, capability model, security maturity)
- releases-joshuargo-hass-agent-github (monthly cadence, v14.15.1, data-race fix history)
- go-hass-agent-mynixos (in nixpkgs proper, 24 platforms)
- changelogmd (full 11,224-word changelog — the depth-of-maintenance evidence)
- lnxlink (~50-module catalogue + alternatives table)
- github-bkbillylnxlink-effortlessly-manage-your-linux-machine-using-mqtt-github
- github-hass-agenthassagent-unofficial-development-project-for-the-hassagent-plat
- home-hassagent-documentation
- using-sensors-hassagent-documentation
- using-commands-hassagent-documentation
- sensors-home-assistant-companion-docs (the canonical companion sensor vocabulary, 8,199 words)
- go-hass-agent-a-native-app-integration-for-desktop-laptop-devices-share-your-proj (headless Proxmox failure + NixOS hand-rolling evidence)

## The counter-position and its proponents (8)
- system-design-architecting-ha-data-flows-with-telegraf-mqtt-influxdb-and-grafana (koying: MQTT is an unnecessary point of failure)
- github-gamerclassn7ha_desktop_companion- (the MQTT-free native-API counter-architecture)
- github-muniterhalinuxcompanion-homeassistant-linux-companion-github (native API maturity + its MQTT migration TODO — cut both ways)
- wth-there-is-no-windows-and-linux-ha-app-similar-to-macos (community demand + mod-endorsed Linux options incl. glances/netdata/system-bridge)
- telegrafpluginsoutputsmqttreadmemd-at-master-influxdatatelegraf-github (Telegraf's breadth)
- glances-home-assistant (the incumbent monitoring path HA actually documents)
- network-ups-tools-nut-home-assistant (HA's own UPS integration — server monitoring without any agent)
- prometheus-home-assistant (the metrics export path)

## ha-linux-agent's own weaknesses (11)
- repo-map-ha-linux-agent (8 tests / failing clippy gate / 12-crate surface)
- glances-integration-loses-connection-and-doesnt-automatically-recover-issue-1105
- glances-integration-sometimes-stops-reporting-current-data-issue-170384-home-ass (steelman must honestly weigh these against the custom agent's equal unreliability)
- glances-sensors-are-renamed-after-a-restart-issue-32328-home-assistantcore-githu
- insane-netdata-memory-usage-help-netdata-community-forums
- resource-utilization-netdata-agent-learn-netdata
- agent-performance-optimization-guide-configuration
- best-nvme-ssd-health-monitoring-tools-9- (mature SMART tooling landscape — why build?)
- github-prometheus-communitysmartctl_exporter-export-smartctl-statistics-to-prome (the exporter exists already)
- github-analogjscrutiny-hard-drive-smart-monitoring-historical-trends-real-world (full SMART monitoring product exists)
- smartdconf5-smartmontools-debian-testing-debian-manpages (smartd does alerts without any agent)

## The honest fork (6)
- mqtt-home-assistant (what HA actually mandates — the steelman's burden)
- module-listnix (19+ home-automation modules exist; none for the Linux agents — cuts both ways)
- rumqttc-rust (35.34% documented, alive-but-slow — the dependency-risk side)
- automatic-reconnect-and-subscribed-topics-issue-250-bytebeamiorumqtt-github (5-year-open issue — dependency risk evidence)
- python-packaging-error-needs-specific-setuptools-version-nixos-discourse (lnxlink packaging difficulty — cuts against adopt)
- corehomeassistantcomponentsmqttconstpy-at-dev-home-assistantcore-github (notify works via MQTT — undermines the native-API-only claim)
