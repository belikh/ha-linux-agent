---
title: Setting MQTT Birth and Last will with the new MQTT Integration - Configuration
  - Home Assistant Community
id: setting-mqtt-birth-and-last-will-with-the-new-mqtt-integration-configuration-hom
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- mqtt-discovery
- discovery
- birth-message
- community-thread
- practitioner-forum
- availability-semantics
created: '2026-09-02T06:41:31.068921Z'
updated: '2026-09-02T17:37:22.292696Z'
source: https://community.home-assistant.io/t/setting-mqtt-birth-and-last-will-with-the-new-mqtt-integration/77862
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T06:41:27.471697Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA community thread (2018-2020, abaskin/ronschaeffer/kiwijunglist/xannor/kkellner)
  on configuring HA''s OWN client birth/last-will: with the UI-configured MQTT integration
  there is no obvious way to set birth/will for the HA client — YAML mqtt: keys only
  work pre-UI-setup, and editing .storage/core.config_entries directly (HA stopped)
  is the documented workaround. Load-bearing findings for agent design: (1) LWT fires
  ONLY on ungraceful disconnect — ''will_message is only sent on ungraceful mqtt client
  disconnect. Since restarting hass is likely a graceful shutdown, no message is sent'',
  so a shutdown-trigger automation publishing ''offline'' is hit-or-miss; (2) the
  birth message every HA start is what triggers devices like zigbee2mqtt to republish
  their cached state — birth-message-driven state resync is the standard pattern;
  (3) tom_l: ''Birth and last will are a function of the mqtt client not the broker''.
  For ha-linux-agent: the agent (client) owns its own LWT registration at CONNECT
  time; a systemd unit that stops gracefully will NOT produce an LWT, so a systemd
  shutdown-signal handler or Explicit Disconnect handling matters — the agent should
  publish offline/retained on SIGTERM if the fleet wants clean status, and rely on
  birth message for state resync.'
---

Setting MQTT Birth and Last will with the new MQTT Integration - Configuration - Home Assistant Community

Setting MQTT Birth and Last will with the new MQTT Integration

Configuration

abaskin

(Andre Baskin)

November 9, 2018,  3:44pm

1

I have moved from configuring the MQTT broker in the configuration.yaml file to using integrations under settings. The HA connection to the broker is working fine as is discovery. I have devices that use MQTT discovery to configure themselves with HA. They use MQTT Birth and Last will to detect HA going offline and resending the discovery messages when HA comes back online. What I can’t figure out is now to configure MQTT Birth and Last will with the MQTT integration. I’ve tried adding just  the MQTT Birth and Last will configuration under mqtt: in configuration.yaml but that breaks MQTT altogether.

MQTT Last Will & Testament from HA

Howto add settings to configured integrations

Silicon_Avatar

November 15, 2018,  3:34am

2

I have the same question, hoping someone can answer this.

thomashermine1

(thomashermine)

January 15, 2019, 10:22am

3

Same question here!

In the meantime, I added a small automation to deal with the “birth” message:
- alias: HomeAssistant Restart | MQTT
trigger:
- platform: homeassistant
event: start
action:
- service: mqtt.publish
data:
topic: 'homeassistant/availability'
payload: 'online'

kiwijunglist

(Mike Stewart)

January 16, 2019,  4:36am

4

Any updates on this.

ronschaeffer

February 13, 2019,  4:52pm

5

Bump. Any official word on Birth and Last Will with the MQTT integration?

@thomashermine1, thanks for your automation.

Can anyone suggest an event to monitor to send an equivalent “offline” message?

tom_l

February 13, 2019, 10:02pm

6

Birth and last will are a function of the mqtt client not the broker.

https://www.hivemq.com/blog/mqtt-essentials-part-9-last-will-and-testament/

ronschaeffer

February 13, 2019, 10:32pm

7

Thanks. The issue here indeed concerns the HA MQTT client. In the “old way” of doing things, you could set birth and last will messages for the HA client under the mqtt: component in configuration.yaml. https://www.home-assistant.io/docs/mqtt/birth_will/

When instead setting up MQTT via Integrations in the UI, there is no obvious way to set birth and last will messages for the HA client.

kiwijunglist

(Mike Stewart)

February 14, 2019,  7:59am

8

I have this is my configuration.yaml
mqtt:
discovery: true
broker: 192.168.1.99
birth_message:
topic: 'hass/status'
payload: 'online'
will_message:
topic: 'hass/status'
payload: 'offline'

but still use the mqtt integration under the configuration menu, it seems to work.

ronschaeffer

February 14, 2019,  9:40am

9

Thanks @kiwijunglist. I used to have something similar in configuration.yaml, and it was imported into Integrations automatically. That worked.

However, I was having problems getting the Hassio zigbee2MQTT add on to publish messages. I finally resorted to deleting the text from configuration.yaml and also deleting the Integration. I then starting fresh by configuring MQTT solely in Integrations. After that, adding back in Birth and Last Will messages to configuration.yaml did not work.

kiwijunglist

(Mike Stewart)

February 14, 2019, 10:01am

10

I use docker and have ha, zigbee2mqtt, mosquito all running in separate dockers. The will is working for me when I restart h.a.

ronschaeffer

February 14, 2019, 11:52am

11

@kiwijunglist Sounds like a similar set-up but with Hassio on Ubuntu and separate dockers via add ons for everything else. Did you have MQTT configured initially in configuration.yaml, or did you initially set it up through the Integrations page?

Ring Device integration via MQTT w/ Video Streaming

kiwijunglist

(Mike Stewart)

February 14, 2019,  5:54pm

12

Can’t be certain but I think I started with integrations then added mqtt to configuration afterwards.

ronschaeffer

February 15, 2019, 12:50pm

13

I’ve been playing around and got the birth message working at least. Configuration:

Hassio on Ubuntu

Broker: Community Hass.io Add-on: MQTT Server & Web client

MQTT HA client configured initially through Integrations in the UI

I added the following to configuration.yaml after the initial configuration of MQTT:
mqtt:
broker: 192.168.xxx.xxx:1883 # broker address a0d7b954-mqtt also works for the Hassio MQTT Server and Web client Add On
username: !secret mqtt_username
password: !secret mqtt_password
birth_message:
topic: 'hass/status'
payload: 'online'
will_message:
topic: 'hass/status'
payload: 'offline'

As mentioned above, the birth message gets sent. The last will message does not.

Can someone try this with a different broker type (not the Hassio add-on) to see if the last will message works with a different broker in order to narrow down the failure conditions? Thanks

gpbenton

(Graham)

February 15, 2019,  1:58pm

14

The last will message works fine when mosquitto is installed with apt on Rassbian.

ronschaeffer

February 15, 2019,  4:07pm

15

Thanks. I’ll try reporting it as an issue with the add-on.

xannor

(Xannor)

April 25, 2020,  4:28pm

16

This is an old post, but the most relevant I could find searching for the topic. I thought I would share how I solved this issue.

if you stop HA, you can edit the .storage/core.config_entries

In that file you will have an entry of "domain": "mqtt", which above it will have a "data": { entry. it is in this area you have to insert the necessary information. (this is according to my setup so YMMV.)

You can paste the following:
"birth_message": {
"topic": "hass/status",
"payload": "online",
"qos": 0,
"retain": false
},
"will_message": {
"topic": "hass/status",
"payload": "offline",
"qos": 0,
"retain": false
}

below the "username": "homeassistant",

and above the },

then save and start home assistant and it should send the birth message. Home assistant must be stopped otherwise it will overwrite your changes. If you mess anything up, HA will refuse to start, so only do this if you have an understanding of JSON and are comfortable with editing core.config_* files.

kkellner

April 28, 2020,  9:27pm

17

@xannor,

Thank you for the json and the reference to .storage/core.config_entries.  I would not have figured that out.   The birth_message works perfectly, however the will_message does not get sent when home assistant is restarted.  This causes the hass/status to remain “online” so when it does restart, it doesn’t change state and zigbee2mqtt doesn’t know it should republish the current cached state of the world.

Any idea how to get will_message to properly publish?   It mqtt broker should auto-puslish this when the mqtt client connection is closed…  so it makes me wonder if mqtt broker is not getting the will_message set when the connection is established from hass.

kkellner

April 28, 2020,  9:33pm

18

Update:   Although will_message does not get published, I do see birth_message publishing every time when hassio is started which then triggers zigbee2mqtt to repulish its cached state.   So the end result is its working for what I needed.

Would still be nice to have the correct status in mqtt when hass is down.

kkellner

April 29, 2020, 12:20am

19

Update 2: Looks like will_message is only sent on ungraceful mqtt client disconnect.  Since restarting hass is likely a graceful shutdown, no message is sent.

Reference:

HA not sending MQTT 'will message' (offline) Configuration

Ok clear, thanks. And in case of HA crashing, would that also qualify as ‘ungracefully’ (I assume yes)?
I just want to monitor if HA is still up and running. Mosquitto is on the same machine, so I can’t test removing the power

I’ll have to research a way to post to mqtt upon hass shutdown event.

xannor

(Xannor)

April 30, 2020,  9:24am

20

posting mqtt on shutdown is an issue, and it is unfortunate that the will is literally the last will and testament and not a good bye message. The only recommendation is to use an automation with an home assistant trigger for shutdown, but that tends to be hit or miss.
alias: Shutdown
description: ''
trigger:
- event: shutdown
platform: homeassistant
condition: []
action:
- data:
payload: offline
qos: 0
retain: false
topic: hass/status
service: mqtt.publish

next page →

Powered by Discourse, best viewed with JavaScript enabled