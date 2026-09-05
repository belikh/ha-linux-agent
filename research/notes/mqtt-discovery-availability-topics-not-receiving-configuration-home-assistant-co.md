---
title: MQTT Discovery availability topics not receiving - Configuration - Home Assistant
  Community
id: mqtt-discovery-availability-topics-not-receiving-configuration-home-assistant-co
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- mqtt-discovery
- availability
- discovery
- community-thread
- practitioner-forum
- broker-config
- known-issue
created: '2026-09-02T06:41:31.078305Z'
updated: '2026-09-05T10:51:22.020504Z'
source: https://community.home-assistant.io/t/mqtt-discovery-availability-topics-not-receiving/758032
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T06:41:28.666572Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA community thread (Aug 2024, aho/Alex): a custom ESP32 device publishing
  a correct MQTT discovery payload (with ~ base ''b/office/dev1'', avty_t, stat_t,
  cmd_t and a dev block) created the entity and HA subscribed to the availability/state
  topics (log shows ''Subscribing with mid: 24 to topics with qos: [(b/office/dev1/available,
  0), (b/office/dev1/state, 0)]''), but messages posted to those topics were NEVER
  received by HA. Root cause per the OP: the external third-party broker — EMQX (emqxsl.com
  cloud) — despite HA debug logs showing healthy register/unregister write cycles.
  ''I''ve updated my HASS instance to use Mosquitto Broker and the issue is resolved.''
  TLDR from the OP: ''If you have issues receiving MQTT messages in Home Assistant
  using external 3rd party MQTT broker - check first if the issue persists when using
  Mosquitto.'' For ha-linux-agent fleet design: broker choice matters at the margin;
  the official/supported path is Mosquitto, and delivery failures on EMQX cloud with
  no HA-side error are an audited failure mode — if jupiterOS standardises on a non-Mosquitto
  broker, subscription-delivery quirks like this must be tested. Provenance: single
  practitioner report, n=1, but the debug-log evidence is verbatim.'
---

MQTT Discovery availability topics not receiving - Configuration - Home Assistant Community

MQTT Discovery availability topics not receiving

Configuration

mqtt

aho

(Alex)

August 9, 2024, 12:37pm

1

Hi!

I am implemeting a device compatible with HASS MQTT discovery feature.

During my experiments I have faced unexpected behaviour: the availability and status topics subscriptions for autodiscovered devices do not receive any messages.

I have added a new device using the following message to topic:

homeassistant/switch/dev1/config
{
"~":"b/office/dev1",
"uniq_id":"dev1",
"cmd_t":"~/set",
"stat_t":"~/state",
"avty_t":"~/available",
"dev":{
"ids":"b_esp32",
"name":"B",
"mf":"B",
"mdl":"153-e",
"sw":"1.0",
"sn":"ea3350945afc",
"hw":"B5"
}
}

This creates a new device with an entity and correct subscriptions.

image625×718 26.2 KB

Then I post the messages:

b/office/dev1/available

online

and

b/office/dev1/state

OFF

But neither of the listed Subsribed topics receives the incoming messages.

The home-assistant.log says:
2024-08-09 14:21:48.705 DEBUG (MainThread) [homeassistant.components.mqtt.client] Received message on homeassistant/switch/dev1/config (qos=0): b'{\r\n   "~":"b/office/dev1",\r\n   "uniq_id":"dev1",\r\n   "cmd_t":"~/set",\r\n   "stat_t":"~/state",\r\n   "avty_t":"~/available",\r\n   "dev":{\r\n      "ids":"b_esp32",\r\n      "name":"B",\r\n      "mf":"B",\r\n      "mdl":"153-e",\r\n      "sw":"1.0",\r\n      "sn":"ea3350945afc",\r\n      "hw":"B5"\r\n   }\r\n}'
2024-08-09 14:21:48.706 DEBUG (MainThread) [homeassistant.components.mqtt.discovery] Process discovery payload {'device': {'name': 'B', 'model': '153-e', 'hw_version': 'B5', 'serial_number': 'ea3350945afc', 'identifiers': 'b_esp32', 'manufacturer': 'B', 'sw_version': '1.0'}, 'unique_id': 'dev1', 'command_topic': 'b/office/dev1/set', 'availability_topic': 'b/office/dev1/available', 'state_topic': 'b/office/dev1/state', 'platform': 'mqtt'}
2024-08-09 14:21:48.706 INFO (MainThread) [homeassistant.components.mqtt.discovery] Found new component: switch dev1
2024-08-09 14:21:48.707 DEBUG (MainThread) [homeassistant.components.mqtt.discovery] Pending discovery for ('switch', 'dev1'): deque([])
2024-08-09 14:21:48.810 DEBUG (MainThread) [homeassistant.components.mqtt.client] t144cc11.ala.eu-central-1.emqxsl.com: register write 15
2024-08-09 14:21:48.810 DEBUG (MainThread) [homeassistant.components.mqtt.client] Subscribing with mid: 24 to topics with qos: [('b/office/dev1/available', 0), ('b/office/dev1/state', 0)]
2024-08-09 14:21:48.811 DEBUG (MainThread) [homeassistant.components.mqtt.client] t144cc11.ala.eu-central-1.emqxsl.com: unregister write 15
2024-08-09 14:22:39.882 DEBUG (MainThread) [homeassistant.components.mqtt.client] t144cc11.ala.eu-central-1.emqxsl.com: register write 15
2024-08-09 14:22:39.887 DEBUG (MainThread) [homeassistant.components.mqtt.client] t144cc11.ala.eu-central-1.emqxsl.com: unregister write 15

register write 15 and unregister write 15 are the only records I get in home-assistant.log when I send messages to the availability or state topics.

Any ideas how to find out why HASS doesn’t receive the messages though everything looks like it should?

aho

(Alex)

August 12, 2024, 11:09am

2

Turns out the issue is related to using the external third party MQTT Broker. EMQX in my case.

I’ve payed more attention to the MQTT documentation and the warning message saying that the only recommended option is Mosquitto broker.

I’ve updated my HASS instance to use Mosquitto Broker and the issue is resolved.

TLDR for future references:

If you have issues receiving MQTT messages in Home Assistant using external 3rd party MQTT broker – check first if the issue persists when using Mosquitto.

Powered by Discourse, best viewed with JavaScript enabled