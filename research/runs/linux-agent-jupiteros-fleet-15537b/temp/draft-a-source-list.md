# Draft A — must_read_note_ids (n=44)
Angle: Strongest-thesis reliability-first advocate — the defects are precisely diagnosed, the supervisor spec is implementable, fix-first then deploy then feature.

## Shared core (7)
- interim-report-mqtt-lifecycle-supervisor-spec: the ConnAck supervisor spec + file:line defect audit
- interim-report-fleet-service-model-by-host-class: system-service-per-host design
- interim-report-adopt-vs-build-honest-verdict: improve-don't-adopt + concession table
- interim-report-headless-privilege-ladder: zero-capability event-driven matrix
- interim-report-rumqttc-dependency-fitness: keep 0.24.0, v5 module exists
- interim-report-testing-gate-for-hardware-daemon: 7-step test ladder
- repo-map-ha-linux-agent: the structural ground truth

## MQTT protocol & root cause (14)
- automatic-reconnect-and-subscribed-topics-issue-250-bytebeamiorumqtt-github
- rumqttc-reconnection-to-mqtt-broker-and-pending-messages-ttl-issue-211-bytebeami
- rumqttrumqttcsrceventlooprs-at-main-bytebeamiorumqtt-github
- librs
- eventloop-in-rumqttc-rust
- rumqttc-rust
- no-keepalive-0-still-allowed-with-max_keepalive-set-issue-2117-eclipse-mosquitto
- mqtt-home-assistant
- comments (HA #135266 maintainer thread — max_queued silent drop)
- some-mqtt-devices-unavailable-after-ha-restart-issue-135266-home-assistantcore-g
- mqtt-discovery-availability-home-assistant-os-home-assistant-community
- my-hand-built-mqtt-devices-are-unavailable-after-hassio-restart-but-zigbee2mqtt
- mqtt-sensors-unavailable-in-01140bx-othe
- releases-hass-agenthassagent-github (HASS.Agent shipped the same rediscovery bug, fixed 2.1.1)

## Step-8 gap-wave confirmations (7)
- corehomeassistantcomponentsmqttconstpy-at-dev-home-assistantcore-github (notify in SUPPORTED_COMPONENTS)
- add-mqtt-notify-platform-by-jbouwh-pull-request-115653-home-assistantcore-github
- test_notifypy (working notify discovery payload)
- mosquitto8-mosquitto-debian-unstable-debian-manpages ($SYS dropped counters)
- databasec (per-client queue drop mechanism in source)
- switch-to-configuration-ng-harden-user-unit-migration-second-pass-by-r-vdp-pull (PR #517768 — switch now restarts user units, disruptively)
- mainrs (stc-ng mechanism source)

## Fleet + comparison support (16)
- systemduser-archwiki
- what-is-the-difference-between-systemdservices-and-systemduserservices-help-nixo
- nixosusers-groups-add-user-option-to-enable-lingering-by-toxicfrog-pull-request
- enabling-persistent-user-instance-systemd-issue-3702-nixosnixpkgs-github
- clarification-of-volume-permissions-issu (CAP_SYS_ADMIN NVMe evidence)
- smart-archwiki (smartd -M exec hooks)
- github-joontysystemd_mon-monitor-for-sys (D-Bus subscription, no polling)
- see-also-3 (OnFailure drop-in pattern)
- how-to-monitor-zfs-pool-health-and-status-on-ubu (zed + thresholds)
- sysfs-class-backlight
- ipnstate-package-tailscalecomipnipnstate-go-packages
- github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device (the incumbent's own admissions)
- go-hass-agent-mynixos
- lnxlink
- device-availability-zigbee2mqtt (production jitter/backoff reference)
- mqtt-client-component-esphome-smart-home-made-simple (birth/LWT/shutdown_message reference implementation)

(Each list member is a vault note id; read via note show.)
