---
title: MQTT | Zigbee2MQTT
id: mqtt-zigbee2mqtt
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-mqtt-lifecycle-supervisor-spec
created: '2026-09-02T09:58:21.527525Z'
updated: '2026-09-05T10:51:22.201944Z'
source: https://www.zigbee2mqtt.io/guide/configuration/mqtt.html
source_domain: www.zigbee2mqtt.io
fetched_at: '2026-09-02T09:58:21.526262Z'
fetch_provider: builtin
status: evergreen
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'Zigbee2MQTT MQTT configuration guide (alternate URL — the requested /guide/mqtt/
  page 404s; this page covers the same server-connection config). Publisher-side connection
  settings: mqtt.server URL (mqtt:// or mqtts://), base_topic default zigbee2mqtt,
  optional client_id, keepalive default 60s, version 4 default with version 5 opt-in
  needed for MQTT5 ''retention'' device config, force_disable_retain escape hatch
  for brokers without retention (breaks HA integration), maximum_packet_size 1 MiB
  matching broker config. Publisher-behaviour settings highly relevant to the agent
  design: cache_state (true, required for HA integration — full-attribute payloads),
  cache_state_persistent (persist cached state to disk), cache_state_send_on_startup
  (republish cached state on startup) — the state-cache-on-disk + resend-on-startup
  pattern that complements retained availability. Also last_seen attribute options
  (disable/ISO_8601/epoch) and output payload formats (json/attribute/attribute_and_json).
  NB: the specific availability-topic section lives on the separate Device Availability
  page, not this page.'
---

MQTT | Zigbee2MQTT

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

MQTT
Server connection
Zigbee2MQTT requires a MQTT-Server connection to operate.
# Required: MQTT settings
mqtt:
# Required: MQTT server URL (use mqtts:// for SSL/TLS connection)
# Example: 'localhost', when using the Mosquito HA addon use 'core-mosquitto'
server: 'mqtt://localhost:1883'
# Optional: MQTT base topic for Zigbee2MQTT MQTT messages (default: zigbee2mqtt)
base_topic: zigbee2mqtt
# Optional: absolute path to SSL/TLS certificate of CA used to sign server and client certificates (default: nothing)
ca: '/etc/ssl/mqtt-ca.crt'
# Optional: absolute paths to SSL/TLS key and certificate for client-authentication (default: nothing)
key: '/etc/ssl/mqtt-client.key'
cert: '/etc/ssl/mqtt-client.crt'
# Optional: MQTT server authentication user (default: nothing)
user: my_user
# Optional: MQTT server authentication password (default: nothing)
password: my_password
# Optional: MQTT client ID (default: nothing)
client_id: 'MY_CLIENT_ID'
# Optional: disable self-signed SSL certificates (default: true)
reject_unauthorized: true
# Optional: override the TLS SNI / hostname used for certificate verification when it differs
# from the host in 'server', e.g. connecting to an internal hostname while validating a public
# certificate SAN. Leave unset to use the hostname from 'server'. (default: nothing)
server_name: 'mqtt.example.com'
# Optional: Include device information to mqtt messages (default: false)
include_device_information: true
# Optional: MQTT keepalive in seconds (default: 60)
keepalive: 60
# Optional: MQTT protocol version (default: 4), set this to 5 if you
# use the 'retention' device specific configuration
version: 4
# Optional: Disable retain for all send messages. ONLY enable if your MQTT broker doesn't
# support retained message (e.g. AWS IoT core, Azure IoT Hub, Google Cloud IoT core, IBM Watson IoT Platform).
# Enabling will break the Home Assistant integration. (default: false)
force_disable_retain: false
# Specifies the maximum allowed packet length (in bytes) that the server can send to Zigbee2MQTT. NOTE: The same value exists in your MQTT broker but for the length the client can send to it instead. (default: 1048576)
maximum_packet_size: 1048576

Specifying MQTT server/user/password and network_key in a different file
To specify the MQTT server/user/password in a different file, e.g secret.yaml, use the following configuration.
configuration.yaml
# IMPORTANT: Don't forget the quotes!
mqtt:
server: '!secret.yaml server'
user: '!secret.yaml user'
password: '!secret.yaml password'

secret.yaml
server: 'mqtt://localhost:1883'
user: mqtt_user
password: mqtt_password

MQTT behaviour
advanced:
# Optional: state caching, MQTT message payload will contain all attributes, not only changed ones.
# Has to be true when integrating via Home Assistant (default: true)
cache_state: true
# Optional: persist cached state, only used when cache_state: true (default: true)
cache_state_persistent: true
# Optional: send cached state on startup, only used when cache_state_persistent: true (default: true)
cache_state_send_on_startup: true
# Optional: Add a last_seen attribute to MQTT messages, contains date/time of last Zigbee message
# possible values are: disable (default), ISO_8601, ISO_8601_local, epoch (default: disable)
last_seen: 'disable'
# Optional: Add an elapsed attribute to MQTT messages, contains milliseconds since the previous msg (default: false)
elapsed: false
# Optional: MQTT output type: json, attribute or attribute_and_json (default: shown below)
# Examples when 'state' of a device is published
# json: topic: 'zigbee2mqtt/my_bulb' payload '{"state": "ON"}'
# attribute: topic 'zigbee2mqtt/my_bulb/state' payload 'ON"
# attribute_and_json: both json and attribute (see above)
output: 'json'