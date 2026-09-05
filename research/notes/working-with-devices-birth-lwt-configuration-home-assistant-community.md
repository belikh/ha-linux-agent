---
title: Working with Device's Birth, LWT? - Configuration - Home Assistant Community
id: working-with-devices-birth-lwt-configuration-home-assistant-community
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- mqtt-discovery
- availability
- discovery
- community-thread
- practitioner-forum
- availability-semantics
- known-issue
created: '2026-09-02T06:41:31.073907Z'
updated: '2026-09-05T10:51:22.015079Z'
source: https://community.home-assistant.io/t/working-with-devices-birth-lwt/290835
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T06:41:28.080590Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA community thread (2021-2023, pearson/finity/alluser/francisp) on handling
  DEVICE-side birth/LWT: HA does NOT automatically mark an MQTT entity unavailable
  when the device''s LWT fires unless availability is wired into the discovery config
  — finity: ''You should get a unavailable state on the devices when they go offline...
  handled by the LWT message automatically in the broker'' but alluser reports ''home
  assistant remembers last state and it does not change it'' for a device (Shelly1
  without discovery-script-provided availability config) turned off for 30+ minutes.
  Root cause: the Shelly discovery script adds ''an availability as part of the discovery
  message''; without avty_t in the discovery payload HA has no availability topic
  to subscribe to, so the entity keeps its last state indefinitely. Workaround used:
  create a shadow MQTT sensor on the device''s state topic with expire_after: 60 —
  and the thread notes expire_after exists for MQTT sensors but NOT for MQTT switches.
  Direct implication for ha-linux-agent: every discovered entity MUST include an availability
  topic (avty_t) in its discovery payload, and for state topics that publish sparsely
  (uptime, temperatures), expire_after is the HA-side staleness fallback that complements
  LWT. Provenance: practitioner forum, self-reported diagnoses.'
---

Working with Device's Birth, LWT? - Configuration - Home Assistant Community

Working with Device's Birth, LWT?

Configuration

mqtt

pearson

(Chris Pearson)

March 16, 2021,  6:37pm

1

My apologies if this duplicates other questions. I’ve seen people asking questions regarding notification when devices go offline, but searches regarding a device’s birth and LWT messages are difficult as the results refer to Home Assistant’s own birth and LWT messages.

My question: can the MQTT Integration automatically handle a device’s birth and LWT messages? If not (and it doesn’t appear to me that it can), do people have suggestions as to how to handle them?

I’m looking for ways to be notified when a device has gone offline. The automatic send of a LWT message by the MQTT broker seems like a good method, if there’s a way to easily handle those messages in Home Assistant. Given that I’m using MQTT Discovery for my devices, I’m really hoping for something very automated, but beggars can’t be choosers. Or something like that.

Thanks!

finity

March 17, 2021,  2:32am

2

You should get a “unavailable” state on the devices when they go offline.

For MQTT devices that is handled by the LWT message automatically in the broker, which sends out the LWT message to all clients subscribed to that topic. And since HA is usually subscribed to the topic for the devices it should also receive those messages.

So in your automation you need to check for the state being “unavailable”.

alluser

(all user)

March 20, 2023,  9:16am

3

But home assistant remembers last state and it does not change it.

I turn off one mqtt device for more than half of hour and home assistant did not change state to unavailable

francisp

(Francis)

March 20, 2023, 11:05am

4

Does that device send a proper LWT message in that half hour ?

alluser

(all user)

March 20, 2023, 11:52am

5

Its is a shelly1 switch and i did not find any way for device to send that message.

francisp

(Francis)

March 21, 2023,  3:09pm

6

Do you use the Shelly discovery script ?

Shellies Discovery Script Scripts

This script adds MQTT discovery support for Shelly devices and without configuration it adds entities to Home Assistant.
[Buy Me A Coffee]
This is screenshots with Home Assistant Integrations page:
[shellies-integration]

Gen2 devices information
Pro/Plus devies are supported by Shellies Discovery Gen2 script.

Supported devices:

Shelly 1 (with external sensors)
Shelly 1L
Shelly 1PM (with external sensors)
Shelly 2 (relays and roller mode)
Shelly 2.5 (relays and roller mode)
Shelly 3EM
Sh…

alluser

(all user)

March 22, 2023, 12:56pm

7

francisp:

Do you use the Shelly discovery script ?

No, because devices are on another network using a VPS / VM and the devices are in my home but the vm is in another country xD

Also after i discover them this script will not run anymore ( i think ). I need something to check is device connected or not.

I fount that there is a expire_after MQTT Sensor - Home Assistant option for MQTT Sensors. But i did not found one for MQTT Switch. Is there similar option for MQTT Switch ?

The only think that is come to mind right now is to get state_topic: "shellies/shelly1-id/relay/0" and create a mqtt sensor with it and add a expire_after

francisp

(Francis)

March 22, 2023,  1:48pm

8

I asked, because the discovery script does create an availability as part of the discovery message.

Can’t check myself, as all my Shellies run Tasmota.

alluser

(all user)

March 22, 2023,  2:51pm

9

I think this is most simple/stupid work around.

It give you constant return of information in about every 30s, and when house power goes down ( for example ) HA will change state.

Stupid thing is that it needs another card to show the actual status.

If HA Devs add  expire_after: 60 in a MQTT Switch will be great!

So yea this is my workaround:
mqtt:
sensor:
### Shelly Switches availability
# Physiical device switch
- unique_id: shelly1-switch-availability
name: shelly1-availability
state_topic: "shellies/shelly1-ID/relay/0"
expire_after: 60

Still im open for suggestions

Powered by Discourse, best viewed with JavaScript enabled