---
title: MQTT devices unavailable after MQTT integration restart - Configuration - Home
  Assistant Community
id: mqtt-devices-unavailable-after-mqtt-integration-restart-configuration-home-assis
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- mqtt
- availability
- birth-message
- broker-persistence
- community-thread
created: '2026-09-02T04:02:29.666374Z'
updated: '2026-09-02T17:37:21.888227Z'
source: https://community.home-assistant.io/t/mqtt-devices-unavailable-after-mqtt-integration-restart/725798
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T04:02:29.294324Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA community thread (May 2024 - Oct 2025, Frank Figiel + francisp/nabeelr/maxym):
  after HA restart or MQTT integration reload all MQTT devices went unavailable. Resolution
  parts: (1) birth_message/will_message are NO LONGER configurable in configuration.yaml
  — moved to UI: Settings > Devices & Services > MQTT > Configure > Reconfigure MQTT
  (second page). (2) After fixing birth topic (hass/status online), HUE bulbs/contact
  sensors returned but curtain motors and Valetudo robot stayed unavailable — i.e.
  only publishers that re-publish on trigger recovered; (3) nabeelr (Sept 2025): devices
  stayed ''unknown'' until state changed and needed endpoint reboots; final root cause
  found Oct 2025: mosquitto''s persistence DB wasn''t being written despite config,
  so the broker lost retained state on restart — once the DB wrote, state survived
  mosquitto reboots. Two extra recovery levers: Shelly-style announce messages to
  trigger republish, and retained state where devices support it. For ha-linux-agent:
  the failure chain is HA restart → integration reload → entities unavailable unless
  (a) birth-triggered re-publish, (b) retained state on a broker with working persistence.
  Also documents that birth/will config location is UI-only since the YAML deprecation.'
---

MQTT devices unavailable after MQTT integration restart - Configuration - Home Assistant Community

MQTT devices unavailable after MQTT integration restart

Configuration

Frank2604

(Frank Figiel)

May 5, 2024,  5:13pm

1

As described in the title. If I restart / reload the MQTT integration or restart the whole Home Assistant, all devices / entities that are integrated via MQTT are not available in HA.

I come from a HA Docker container version back to HAOS. If I have observed it correctly, the problem only occurs since switching to HAOS.

I use Mosquitto as MQTT broker in a docker container.

The solutions in this older post marked as solved are no longer valid or do not work.

https://community.home-assistant.io/t/zigbee2mqtt-device-unavailable-after-ha-restart/166413
mqtt:
discovery: true
broker: 192.168.x.x
birth_message:
topic: 'hass/status'
payload: 'online'
will_message:
topic: 'hass/status'
payload: 'offline'

bitrh_meaasge is no longer allowed in configuration.yaml

Sending
action:
- delay:
hours: 0
minutes: 0
seconds: 30
milliseconds: 0
- service: mqtt.publish
data:
topic: hass/status
payload: online

do not solve the issue.

Can someone point me to the actual solution of this problem?

francisp

(Francis)

May 5, 2024,  5:32pm

2

Settings → Devices & Services → MQTT →  Configure → Reconfigure MQTT → second page

afbeelding712×207 6.54 KB

afbeelding613×902 44.7 KB

Frank2604

(Frank Figiel)

May 6, 2024,  5:52pm

3

@francisp Thanks for showing me this options in the integration.

It seems that this solves a part of the problem.

All entities of the HUE bulbs and contact sensors (except all update.domain entities) are now back after HA reboot.

But the curtain motors and the vaccum robot with valetudo stays unavailable.

Any additional idea?

Frank2604

(Frank Figiel)

May 11, 2024,  1:49pm

4

I have tested around a bit these days and installed the MQTT explorer add-on to understand a bit more of MQTT.

I also moved from my Docker MQTT-Broker to the HA add-on Broker.

After the last HA restarts all the Z2M and frigate devices/entities are coming back. Only the Valetudo robot not or with some delay. I will make some more research to this.

nabeelr

September 17, 2025,  3:41pm

5

Any solution to this? After rebooting HA, MQTT devices are unavailable, and the only way to bring them back seems to be rebooting the devices reporting to my mqtt broker, which is mosquitto in my case. Then when they do come back, everything is listed as “unknown” until their state changes, which means I have to re-trigger everything again for things to go back to knowing what state they’re in.

It really sucks.

francisp

(Francis)

September 17, 2025,  5:38pm

6

nabeelr:

everything is listed as “unknown” until their state changes,

That is normal behavior

The devices needing to be reboot to reconnect, not

I assume you are using the add-on?

maxym

September 17, 2025,  6:35pm

7

Some devices reporting to mqtt have ability to republish their state on demand. for example Shelly devices can be triggered by announce message.

you can publish such request on ha start or on demand or other events

Other option is to setup these devices to publish retained messages. Not all have this option. For example Shelly gen1 have, while newer ones don’t

nabeelr

October 28, 2025,  1:25am

8

It turns out, the DB for mosquitto wasn’t being written, despite being in the config. I don’t know what change I made but it started writing the DB and things will preserve state between mosquitto server reboots now.

I also can’t seem to get it in the state where I need to reboot the endpoint device anymore, so perhaps it was that config issue, whatever it was, that was causing the mosquitto db not to be created that caused it all?

Powered by Discourse, best viewed with JavaScript enabled