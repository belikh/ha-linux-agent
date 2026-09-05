---
title: MQTT entities become unknown on reboot - Configuration - Home Assistant Community
id: mqtt-entities-become-unknown-on-reboot-configuration-home-assistant-community
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- official-docs
- mqtt-discovery
- ha-linux-agent
- community-thread
- practitioner-forum
- reliability-failure-modes
- known-issue
created: '2026-09-02T06:41:31.083070Z'
updated: '2026-09-02T17:37:22.297733Z'
source: https://community.home-assistant.io/t/mqtt-entities-become-unknown-on-reboot/848716
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T06:41:29.279246Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA community thread (Feb 2025, DrVoidberg/Sir_Goodenough): CumulusMX weather
  station data over MQTT — after every reboot all discovered sensors turn Unknown,
  requiring MQTT integration deletion + reinstallation (breaking dashboards each time).
  OP''s diagnosis (post 5): ''The problem seems to be a delay between CumulusMX and
  Home Assistant booting up. CumulusMX is up a few seconds after system reboot, while
  HA takes a few minutes... It looks like that causes CumulusMX to fail to connect
  to the broker and then simply not trying again it seems. Manually restarting CumulusMX
  after HA is up solves the problem.'' The workaround is a delayed service restart
  5 min after boot. Also documents the retained-message rename hazard: flipping retain
  on CumulusMQ''s config re-broke entity names/dashboards because HA rediscovered
  with original names. For ha-linux-agent this is the canonical boot-race failure
  mode: a publisher that fails its FIRST broker connection during the HA/broker boot
  window and then never retries sits dead indefinitely — the agent''s rumqttc poll
  loop (which self-reconnects) plus systemd After=/Requires= broker ordering and a
  restart-with-backoff is exactly the fix; also warns that NixOS unit ordering must
  ensure mosquitto is up before the agent, or the agent must tolerate initial-connection
  failure. Provenance: practitioner forum, self-diagnosed, workaround unverified in-thread.'
---

MQTT entities become unknown on reboot - Configuration - Home Assistant Community

MQTT entities become unknown on reboot

Configuration

mqtt

DrVoidberg

February 16, 2025,  9:01pm

1

Hi all,

I’m a bit lost on this one… dug through similar topics, but no luck.

I read in my weather station data from CumulusMX into Home Assistant via MQTT. Works fine in principle, but when I reboot the system, all sensors turn to ‘Unknown’.

grafik1352×736 66.5 KB

The data still arrives as far as I can tell (from the timestamp) from the MQTT explorer:

grafik1576×733 51.1 KB

The first time setting it up I followed the (supposedly wrong?) docs and created a ‘homeassistant’ login for MQTT. When it stopped working no steps I found in other threads worked, so I deleted the MQTT integration and the integration and re-installed. This time following the advice and NOT creating a login, but rather a mqtt user in HA. The entities were re-discovered and reconnected to the previous data, but I had to remake the dashboard, which is quite annoying given the number of sensors.

Few hours ago I shut down the system temporarily and the sensors are Unknown again.

Does anyone have any clue how that might be solved? Re-installing everything after every reboot is obviously not an option. Especially since I plan to have the system reboot every night.

Home Assistant OS is running in a VM on Proxmox on a Raspberry Pi 5, which also runs CumulusMX directly in the RaspiOS.

I have the suspicion that may somehow be the cause of the problem, but I don’t even know what I’m doing half of the time with Linux.

Sir_Goodenough

((SG) WhatAreWeFixing.Today)

February 16, 2025,  9:03pm

2

Hello DrVoidberg,

You may want to turn on retain on some of those and others you may want to find a way to update them after a home assistant restart.

DrVoidberg

February 16, 2025,  9:28pm

3

Thanks for the (unbelievably) fast reply.

I gratefully took the (originally french, lol) MQTT configuration for CumulusMX from somebody who had it all set to retain. From what I’ve read about retaining it also kind of made sense to always have the latest state available.

I changed the MQTT configuration in CumulusMX to not retain, deleted the retained topics and rebooted. The sensors are back, but returned to their original names, breaking the dashboard again. Still, that’s a step forward, thanks. I’ll keep the sensor names and the topic open for now to see if everything stays intact after reboots.

DrVoidberg

February 17, 2025,  8:33pm

4

I’ve let everything run until now and did a reboot.

Now all the sensors are ‘Unavailable’ and not coming back. Any more ideas?

DrVoidberg

March 1, 2025,  1:40pm

5

Just in case anyone else with the problem stumbles over this:

The problem seems to be a delay between CumulusMX and Home Assistant booting up.

CumulusMX is up a few seconds after system reboot, while HA takes a few minutes (running in a VM and all that).

It looks like that causes CumulusMX to fail to connect to the broker and then simply not trying again it seems. Manually restarting CumulusMX after HA is up solves the problem.

I will try with automatically restarting the service 5 minutes after boot. Not an elegant solution, but at least a solution if it works.

Powered by Discourse, best viewed with JavaScript enabled