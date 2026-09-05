---
title: Device Availability | Zigbee2MQTT
id: device-availability-zigbee2mqtt
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- zigbee2mqtt
- availability
- retained-messages
- official-docs
created: '2026-09-02T04:02:29.670617Z'
updated: '2026-09-05T10:51:21.596326Z'
source: https://www.zigbee2mqtt.io/guide/configuration/device-availability.html
source_domain: www.zigbee2mqtt.io
fetched_at: '2026-09-02T04:02:29.614669Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Zigbee2MQTT official docs — Device Availability feature: publishes availability
  to zigbee2mqtt/[FRIENDLY_NAME]/availability with payload {"state":"online"/"offline"}
  as a RETAINED message. Asymmetric timeouts: active (mains-powered) devices must
  check in every 10 minutes (default) or get pinged then marked offline; passive (battery)
  devices 25 hours (1500 minutes). Timeout state is persisted across Z2M restarts
  — stopping Z2M for >10 min marks all active devices offline until they check in
  again. Advanced options: per-device timeout override, max_jitter (30s default) to
  spread availability pings, ping backoff pattern x1.5/x3/x6/x12, pause_on_backoff_gt,
  per-device enable/disable. On reconnect/announce, Z2M re-reads state attributes
  (state, brightness, color_temp, colour). Groups marked available when at least one
  member is. Design template for ha-linux-agent fleet: active availability pings with
  jitter + exponential backoff + check-in windows differentiated by device class,
  and retained availability topics so HA restarts see the last known availability
  immediately.'
---

Device Availability | Zigbee2MQTT

SearchCtrlK

Getting started
Supported Hardware
Adapters
Devices
Installation
Linux
Docker
Home Assistant addon
openHABian
Windows
FreeBSD jail
Kubernetes
Securing the installation
Watchdog
Zigbee2MQTT fails to start/crashes runtime
Configuration
Adapter settings
MQTT
Zigbee network
Frontend
Devices and Groups
Logging
Device blocklist / passlist
OTA device firmware update
Device Availability
Home Assistant integration
More configuration options
Configuration update
All settings
Usage
Allowing devices to join
Integrations
Touchlink
Scenes
Binding
Groups
OTA updates
MQTT Topics and Messages
Exposes
Health
Troubleshooting
FAQ

Device Availability
The availability feature checks whether your devices are online. The availability state of a device is published to zigbee2mqtt/[FRIENDLY_NAME]/availability with payload {"state":"online"} or {"state":"offline"} (this message is a retained MQTT message).
# Optional: Availability feature
availability:
# Enable the feature (default: false)
enabled: true

The availability feature works differently for active and passive devices, since passive devices cannot be pinged.
Active devices (non battery-powered): by default they have to check-in** every 10 minutes. If they don't, they will be pinged, if that fails the device will be marked as offline.
Passive devices (battery-powered): by default they have to check-in** every 25 hours. If they don't they will be marked as offline.
Note that this timeout is persisted between Zigbee2MQTT restarts. So if you for example stop Zigbee2MQTT for longer than 10 minutes, all your active devices will be marked as offline initially until they check-in** again.
** A check-in is any kind of Zigbee message from the device that reaches Zigbee2MQTT (even internal updates that are not displayed/reported).
Advanced configuration
# Note: all options are optional
availability:
enabled: true
active:
# Time after which an active device will be marked as offline in minutes (default: 10 minutes)
timeout: 10
# Maximum jitter (in msec) allowed on timeout to avoid availability pings trying to trigger around the same time (default: 30000, min: 1000)
max_jitter: 30000
# Enable timeout backoff on failed availability pings (default: true)
# Pattern used: x1.5, x3, x6, x12... (with default timeout of 10min: 10, 15, 30, 60, 120...)
backoff: true
# Pause availability pings when backoff reaches over this limit until a new Zigbee message is received from the device. (default: 0, min: 0)
# A value of zero disables pausing, else see `backoff` pattern above.
pause_on_backoff_gt: 0
passive:
# Time after which a passive device will be marked as offline in minutes (default: 1500 minutes aka 25 hours)
timeout: 1500

devices:
'0x12345678':
friendly_name: 'my_bulb'
# Set availability: false to disable the availability feature for a specific device
availability: false
'0x87654321':
friendly_name: 'my_switch'
# Change availability timeout to 3 minutes for this device only
availability:
timeout: 3
# active devices also can specify `max_jitter`, `backoff`, `pause_on_backoff_gt` (see above)

If you want to enable the availability feature for only certain devices, don't add availability: enabled: true in your configuration.yaml but specify it for that device only, e.g.
devices:
'0x87654321':
friendly_name: 'my_switch'
# Enable availability for just 'my_switch'
availability: true

State retrieval
When the availability feature is enabled and a device reconnects or announces itself on the network, Zigbee2MQTT will retrieve the state of the device. This is e.g. handy when a bulb turns itself on after being reconnected to mains power. The following attributes will be read: state, brightness, color_temp and color.
Performance considerations
The pinging can be heavy on the coordinator, especially if you are using a CC2530 or CC2531 adapter.
Higher timeout for active devices results in less pinging so less stress on the coordinator.
Groups
When enabling device availability, availability will also be enabled for groups. A group is marked as available when at least one device in it is available.