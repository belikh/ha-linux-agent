---
title: 'GitHub - GamerClassN7/HA_Desktop_Companion: App which is using native HA Api
  to comunicate and report data to HA · GitHub'
id: github-gamerclassn7ha_desktop_companion-app-which-is-using-native-ha-api-to-comu
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-native-api
- companion-app
created: '2026-09-02T06:52:06.618805Z'
updated: '2026-09-02T17:37:22.764156Z'
source: https://github.com/GamerClassN7/HA_Desktop_Companion
source_domain: github.com
fetched_at: '2026-09-02T06:52:06.617436Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'HA_Desktop_Companion (GamerClassN7, 85 stars, C#/.NET, Windows): a HA desktop
  companion built specifically because the author ''don''t like existing implementations
  using MQTT'' — inspired by ESPHome''s native protocol to HA. Configure with URL
  + API token (long-lived). Implemented sensors: battery_level, battery_state, is_charging,
  wifi_ssid, cpu_temp, current_active_window, uptime, camera_in_use, cpu_temperature,
  free_ram, and a wmic platform (any WMI query via wmic_path/wmic_selector/value_map,
  e.g. Win32_Battery). Notifications with inline images/audio/send-key emulation (requires
  keys: in configuration.yaml). README points Linux users to muniter/halinuxcompanion.
  Future TODO: encryption, improved debug mode. Architecture proof that a sensor+command
  agent can run MQTT-free against the native HA API — the key architectural alternative
  for ha-linux-agent.'
---

*Suggested by [[wth-there-is-no-windows-and-linux-ha-app-similar-to-macos-month-of-what-the-heck]] — cited in HA community thread as a desktop companion using the native HA API instead of MQTT - direct feature/architecture benchmark for ha-linux-agent*

GitHub - GamerClassN7/HA_Desktop_Companion: App which is using native HA Api to comunicate and report data to HA · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

GamerClassN7

/

HA_Desktop_Companion

Public

Notifications
You must be signed in to change notification settings

Fork
7

Star
85

master

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History209 Commits

209 Commits
Folders and filesNameName
Last commit message
Last commit date

.github

.github

HA

HA

HADC_REBORN

HADC_REBORN

Test

Test

.gitignore

.gitignore

BUILD.md

BUILD.md

HA.sln

HA.sln

README.md

README.md

configuration.yaml

configuration.yaml

ha_logo.ico

ha_logo.ico

ha_logo.png

ha_logo.png

View all files

Repository files navigation

Most of the time new version introduce new bugs so please if you are using working version keep using it until some stable release come out :)

HA Desktop Companion - Reborn

Why did I make this app ?

Cause I don't like existing implementations using MQTT and I took inspiration from awesome ESPhome and its native communication protocol to HA and implemented it my own way :)

Feel free to contribute any time :)

Looking for linux version ?

muniter/halinuxcompanion

Comunity Contact

HomeAssistant Comunity Forum

Discord

Looking for linux version ?

muniter/halinuxcompanion

Installation

Download latest release HERE

Extract the zip file to some folder on your system,

Run HA.exe

Fill in "URL" & "API Token"

Click "Save"

Sensors implemented currently:

battery_level

battery_state

is_charging

wifi_ssid

cpu_temp

current_active_window

uptime

camera_in_use

cpu_temperature (only native api supported)

free_ram

wmic (You can integrate any wmix query syou want :))

- platform: wmic
wmic_path: Win32_Battery
wmic_selector: BatteryStatus
wmic_namespace: \\root\CIMV2
value_map: "Discharging|On AC|Fully Charged|Low|Critical|Charging|Charging and High|Charging and Low|Undefined|Partially Charged"
name: Battery State
unique_id: battery_state
icon: "mdi:battery-minus"
entity_category: "diagnostic"
device_class: battery

App which is using native HA Api to comunicate and report data to HA

Screenshots

Future plans TODO:

Improved debug mode

Encryption

Notifications

Example Basic Notification:

Example Inline Image Notification:

{
"image":"https://upload.wikimedia.org/wikipedia/commons/9/9f/Old_wikipedia_logo.png"
}

Example Audio Notification:

Example Emulate Send Key Notification:

Require keys: in your configuration.yaml

Keys Codes can be found Here in Colum: Value

Automation Ideas:

Pause TTS when camera is in use (usefull when working from home) credits: Hellis81

alias: Washing machine done
description: ""
trigger:
- platform: numeric_state
entity_id: sensor.washing_machine_program_progress
above: "99"
- platform: state
entity_id: sensor.washing_machine_operation_state
from: Run
to: Finished
- platform: state
entity_id: sensor.washing_machine_operation_state
from: Run
to: Ready
condition: []
action:
- if:
- condition: state
entity_id: binary_sensor.axlt2801_camera_in_use
state: "on"
then:
- wait_for_trigger:
- platform: state
entity_id:
- binary_sensor.axlt2801_camera_in_use
to: "off"
continue_on_timeout: false
else: []
- service: tts.cloud_say
data:
entity_id: media_player.hela_huset
message: "{{ states('sensor.washing_machine_tts') }}"
language: sv-SE
- repeat:
while:
- condition: or
conditions:
- condition: state
entity_id: binary_sensor.washing_machine_door
state: "off"
- condition: state
entity_id: sensor.washing_machine_program_progress
state: "100"
sequence:
- delay:
hours: 0
minutes: 5
seconds: 0
milliseconds: 0
- choose:
- conditions:
- condition: and
conditions:
- condition: state
entity_id: binary_sensor.washing_machine_door
state: "off"
- condition: state
entity_id: sensor.washing_machine_program_progress
state: "100"
sequence:
- service: tts.cloud_say
data:
entity_id: media_player.hela_huset
message: >-
"{{ states('sensor.washing_machine_tts') }} och luckan är
fortfarande stängd."
language: sv-SE
- service: homeassistant.update_entity
target:
entity_id: sensor.washing_machine_json
data: {}
default: []
mode: single

Contributors

Star History

]

About
App which is using native HA Api to comunicate and report data to HA
Resources
Readme
Activity
Stars
85 stars
Watchers
7 watching
Forks
7 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.