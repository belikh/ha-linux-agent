---
title: 'GitHub - muniter/halinuxcompanion: HomeAssistant Linux Companion · GitHub'
id: github-muniterhalinuxcompanion-homeassistant-linux-companion-github
tags:
- linux-agent-jupiteros-fleet-15537b
- halinuxcompanion
- native-app-integration
- mqtt-migration-rationale
- comparative-benchmark
created: '2026-09-02T04:33:42.303050Z'
updated: '2026-09-02T17:37:22.136318Z'
source: https://github.com/muniter/halinuxcompanion
source_domain: github.com
fetched_at: '2026-09-02T04:33:42.301548Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'halinuxcompanion (muniter, 90 stars, 106 commits, MIT, Python 3.10+/aiohttp/dbus_next)
  is a third Linux-native HA companion using the Native App Integration REST API —
  registers as a mobile_app device, receives actionable notifications via a LOCAL
  aiohttp HTTP server (configurable port 8400) that HA POSTs to, with D-Bus for desktop
  notification display + action listening, plus sleep/shutdown signal listening to
  update a Status sensor right before power events. Config-declarative: sensors (cpu,
  memory, uptime, status, battery_level/state, camera_state) and notification-attached
  commands (suspend/poweroff/reboot/hibernate via systemctl, xdg-open URLs, flatpak
  launches). LOAD-BEARING for ha-linux-agent: its To-do section explicitly documents
  why it wants to MOVE SENSORS TO MQTT — the native API cannot reflect a host going
  offline if the app dies before sending the state, whereas MQTT will topics update
  sensors on lost connectivity. This is the strongest third-party confirmation of
  ha-linux-agent''s MQTT architecture choice. Runs as systemd --user service (no sudo),
  Docker also supported.'
---

*Suggested by [[sensors-home-assistant-companion-docs]] — a third Linux-native HA companion surfaced while mapping the Linux agent ecosystem; uses the HA native API rather than MQTT*

GitHub - muniter/halinuxcompanion: HomeAssistant Linux Companion · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

muniter

/

halinuxcompanion

Public

Notifications
You must be signed in to change notification settings

Fork
13

Star
90

master

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History106 Commits

106 Commits
Folders and filesNameName
Last commit message
Last commit date

.github

.github

halinuxcompanion

halinuxcompanion

tests

tests

.gitignore

.gitignore

Dockerfile

Dockerfile

LICENSE

LICENSE

README.md

README.md

config.example.json

config.example.json

docker-compose.yaml

docker-compose.yaml

requirements.txt

requirements.txt

setup.cfg

setup.cfg

sonar-project.properties

sonar-project.properties

tox.ini

tox.ini

View all files

Repository files navigation

Home Assistant Linux Companion

Application to run on Linux desktop computer to provide sensor data to Home Assistant, and get notifications as if it was a mobile device.

How To

Requirements

Python 3.10+ and the related dev dependencies (usually python3-dev or python3-devel on your package manager)

Instructions

Get a long-lived access token from your Home Assistant user

Clone this repository in a subfolder from your home directory (unless you don't want to run the service from systemd)

Create a Python virtual environment and install all the requirements:

cd halinuxcompanion  # this is the root of the cloned project
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

Copy config.example.json to config.json.

Modify config.json to match your setup and desired options.

Run the application, either from:

the virtual environment directly: python -m halinuxcompanion --config config.json. In this case, you'll need to run it again when you restart.

or setting up a systemd service (don't use sudo for any of the commands below; if you need it, something is probably wrong with your setup):

Copy the sample unit file from halinuxcompanion/resources/halinuxcompanion.service to ~/.config/systemd/user/

Modify it to match your setup - mainly, the installation paths at WorkingDirectory and ExecStart

(Re)Load it with systemctl --user daemon-reload

Start it with systemctl --user start halinuxcompanion

You can check if it went well with systemctl --user status halinuxcompanion. If it errored, you can check logs with journalctl --user -u halinuxcompanion

If all went well, you can enable it permanently with systemctl --user enable halinuxcompanion

Now in your Home Assistant you will see a new device in the "mobile_app" integration, and there will be a new service to notify your Linux desktop. Notification actions work and the expected events will be fired in Home Assistant.

Example configuration file

{
"ha_url": "http://homeassistant.local:8123/",
"ha_token": "mysuperlongtoken",
"device_id": "computername",
"device_name": "whatever you want can be left empty",
"manufacturer": "whatever you want can be left empty",
"model": "Computer",
"computer_ip": "192.168.1.15",
"computer_port": 8400,
"refresh_interval": 15,
"loglevel": "INFO",
"sensors": {
"cpu": {
"enabled": true,
"name": "CPU"
},
"memory": {
"enabled": true,
"name": "Memory Load"
},
"uptime": {
"enabled": true,
"name": "Uptime"
},
"status": {
"enabled": true,
"name": "Status"
},
"battery_level": {
"enabled": true,
"name": "Battery Level"
},
"battery_state": {
"enabled": true,
"name": "Battery State"
},
"camera_state": {
"enabled": true,
"name": "Camera State"
}
},
"services": {
"notifications": {
"enabled": true,
"url_program": "xdg-open",
"commands": {
"command_suspend": {
"name": "Suspend",
"command": ["systemctl", "suspend"]
},
"command_poweroff": {
"name": "Power off",
"command": ["systemctl", "poweroff"]
},
"command_reboot": {
"name": "Reboot",
"command": ["systemctl", "reboot"]
},
"command_hibernate": {
"name": "Hibernate",
"command": ["systemctl", "hibernate"]
},
"command_open_ha": {
"name": "Open Home Assistant",
"command": ["xdg-open", "http://homeassistant.local:8123/"]
},
"command_open_spotify": {
"name": "Open Spotify Flatpak",
"command": ["flatpak", "run", "com.spotify.Client"]
}
}
}
}
}

Technical

Home Assistant Native App Integration

Home Assistant REST API

Asynchronous (because why not 😄)

HTTP Server (aiohttp): Listen to POST notification service call from Home Assistant

Client (aiohttp): POST to Home Assistant api, sensors, events, etc

Dbus interface (dbus_next): Sending notifications and listening to notification actions from the desktop, also listens to sleep, shutdown to update the status sensor

To-do

Implement encryption

Move sensors to MQTT

The reasoning for the change is the limitations of the API, naturally is expected that desktop and laptops would go offline and I would like for the sensors to reflect this new state. But if for some reason the application is unable to send this new state to Home Assistant the values of the sensors would be stuck. But if the app uses MQTT it can set will topics for the sensors to be updated when the client can't communicate with the server.

One day make it work with remote and local instance, for laptops roaming networks

Status sensors that listens to sleep, wakeup, shutdown, power_on

Add more sensors

Finish notifications functionality

Add notification commands

Notifications Clearing

Notification Icon

Features

Sensors:

CPU

Memory

Uptime

Status: Computer status, reflects if the computer went to sleep, wakes up, shutdown, turned on. The sensor is updated right before any of these events happen by listening to dbus signals.

Battery Level

Batter State

Notifications:

Actionable Notifications (Triggers event in Home Assistant)

Local action handler using URI: only relative style /lovelace/myviwew and http(s) uri supported so far.

Notification cleared/dismissed (Triggers event in Home Assistant)

Timeout

Commands

Replacing

Clearing

Icon TODO

Default commands (example config):

Suspend

Power off

Reboot

Hibernate

About
HomeAssistant Linux Companion
Resources
Readme
MIT license
Activity
Stars
90 stars
Watchers
5 watching
Forks
13 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.