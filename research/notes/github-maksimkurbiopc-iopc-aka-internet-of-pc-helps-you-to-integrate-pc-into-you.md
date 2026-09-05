---
title: 'GitHub - maksimkurb/IoPC: IoPC (aka Internet of PC) helps you to integrate
  PC into your smart home system. · GitHub'
id: github-maksimkurbiopc-iopc-aka-internet-of-pc-helps-you-to-integrate-pc-into-you
tags:
- linux-agent-jupiteros-fleet-15537b
- iopc
- comparative-benchmark
- mqtt-service-bus
- linux-agent
created: '2026-09-02T04:02:37.769031Z'
updated: '2026-09-05T10:51:21.674946Z'
source: https://github.com/maksimkurb/IoPC
source_domain: github.com
fetched_at: '2026-09-02T04:02:34.641988Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'IoPC (maksimkurb, 40 stars, 16 commits, GPL-3.0, Kotlin/Gradle + an iopc-native
  component) is a small MQTT-ONLY smart-home PC integration service recommended by
  HASS.Agent''s own README as its Linux alternative. Architecture: flat JSON service-bus
  over MQTT — inbound {PREFIX}/{CLIENT_ID}/command/#, outbound {PREFIX}/{CLIENT_ID}/#;
  web config UI on 127.0.0.1:60555 or IOPC_MQTT_* env vars. Services: keyboard.press,
  media.control (virtual media keys), presentation.control, volume.set/get, command.execute
  (entrypoint IDs defined in CommandModule config, with env injection), power.shutdown/reboot/sleep/hibernate.
  Sensors: availability state, volume/level, sysinfo ram free/total, cpu usage, per-drive
  info, activity/lastUserInput, activity/state (active/idle-30s/away-5min). KEY LIMITATION
  for the jupiterOS fleet case: ''sensors are not updated automatically, so you should
  run a special service to update sensor value'' — polling-only, no schedules, no
  HA auto-discovery semantics documented. Low project maturity (0 forks, 16 commits)
  but clean minimal service-bus design worth comparing against ha-linux-agent''s MQTT
  topic layout.'
---

GitHub - maksimkurb/IoPC: IoPC (aka Internet of PC) helps you to integrate PC into your smart home system. · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

maksimkurb

/

IoPC

Public

Notifications
You must be signed in to change notification settings

Fork
0

Star
40

master

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History16 Commits

16 Commits
Folders and filesNameName
Last commit message
Last commit date

docs

docs

gradle/wrapper

gradle/wrapper

iopc-native

iopc-native

iopc-server

iopc-server

iopc_client

iopc_client

.gitignore

.gitignore

LICENSE.txt

LICENSE.txt

README.md

README.md

build.gradle

build.gradle

gradlew

gradlew

gradlew.bat

gradlew.bat

local.properties

local.properties

settings.gradle

settings.gradle

View all files

Repository files navigation

IoPC

(aka Internet of PC)

What is it?

IoPC is a service program that lets you integrate your PC into smart home system.
Now it supports MQTT protocol, so you can integrate IoPC with HomeAssistant.

Configuration

IoPC has web interface where you can configure app, and it's modules which is available on http://127.0.0.1:60555/ by default.

You can quickly open it via tray icon:

Also you can configure IoPC with env vars:

IOPC_MQTT_SERVER_URI=tcp://127.0.0.1:1883
IOPC_MQTT_USERNAME=mylogin
IOPC_MQTT_PASSWORD=mypassword

# Default MQTT prefix for IoPC
IOPC_MQTT_PREFIX=iopc
# Client ID is used to generate MQTT topic for individual computer
IOPC_MQTT_CLIENT_ID=myComputer

Services

By default, inbound MQTT prefix for IoPC instance is {MQTT_PREFIX}/{MQTT_CLIENT_ID}/command/#.
You can publish any messages to subtopics, and they will be parsed by MqttModule.

For example, we want to set PC volume from HomeAssistant. We should send following message to topic iopc/myComputer/command/homeassistant

{
"service": "volume.set",
"payload": {
"volume": 54
}
}

Available services

Service
Payload
Description

keyboard.press
{ "keyCode": 65 }
Presses the button on keyboard with specified keyCode

media.control
{ "action": "PLAY_PAUSE" }
Controls multimedia by simulating virtual keys (you can find them on some keyboards). Available actions: PLAY_PAUSE, VOLUME_UP, VOLUME_DOWN, MUTE, PREVIOUS, NEXT, STOP

presentation.control
{ "action": "NEXT" }
Controls PowerPoint/LibreOffice/etc presentation via pressing arrow keys or F5/ESC to start or end presentation. Available actions: NEXT, PREVIOUS, START, STOP

volume.set
{ volume: 54 }
Sets audio volume

volume.get
{}
Publishes actual audio volume to MQTT

command.execute
{ "entrypointId": "help", "environment": { "KEY": "VALUE", "KEY2": "VALUE2" } }
Runs custom command which must be defined in CommandModule configuration, e.g. for EntrypointID run-explorer, entrypoint can be explorer.exe

power.shutdown
{}
Shutdown computer

power.reboot
{}
Reboot computer

power.sleep
{}
Suspend computer

power.hibernate
{}
Hibernate computer

Sensors

By default, outbound MQTT prefix for IoPC instance is {MQTT_PREFIX}/{MQTT_CLIENT_ID}/#.
Under this prefix you can find values from PC indicators.
For example, PC volume level will be published to topic iopc/myComputer/volume/level as integer (e.g. 54)

For now, sensors are not updated automatically, so you should run a special service to update sensor value.

Available sensors

Sensor topic
Value example
Description

state
online
Availability topic of IoPC. Can be online or offline

volume/level
54
Audio volume. You must call volume.get service to refresh value of this sensor

sysinfo/ram/free
8575254528
Free RAM in bytes

sysinfo/ram/total
9940004528
Total RAM in bytes

sysinfo/cpu/usage
14
CPU usage in %

sysinfo/drive/{id}
{ "path": "C:\\", "totalSpace": 255414235136, "freeSpace": 96031997952, "usableSpace": 96031997952 }
Drive info. {id} - number of drive, starts from 0.

activity/lastUserInput
2021-05-25T18:52:00.0123287
Last user input (e.g. mouse movement or keyboard press)

activity/state
active
User state. Can be active, idle (if not active for 30sec), away (if not active for 5min), unknown

About
IoPC (aka Internet of PC) helps you to integrate PC into your smart home system.
Topics
home-automationhomeassistantiopciotmqttsmarthome
Resources
Readme
GPL-3.0 license
Activity
Stars
40 stars
Watchers
3 watching
Forks
0 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.