---
title: lnxlink · PyPI
id: lnxlink-pypi
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-adopt-vs-build-honest-verdict
- adopt-vs-build
created: '2026-09-02T12:05:59.134069Z'
updated: '2026-09-02T17:37:22.507003Z'
source: https://pypi.org/project/lnxlink/2023.6.0/
source_domain: pypi.org
fetched_at: '2026-09-02T12:05:59.132423Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'lnxlink 2023.6.0 PyPI README (bkbilly, MIT, Python >=3.7, monthly releases
  through 2026.8.0): a Linux MQTT companion app using HA MQTT autodiscovery, with
  system control (shutdown, restart, suspend, send keys, notify, media, screen on/off,
  open URL/file, bash, brightness, boot select) and monitoring modules (CPU, RAM,
  network, idle, battery, disk usage, updates); headless install is explicit — ''used
  for linux environments that don''t use a Graphical Interface like servers'', installed
  as root with a system service, and some GUI-dependent modules must be removed from
  config; notifications are plain JSON via mqtt.publish to {prefix}/{clientId}/commands/notify
  with title/message/iconUrl — no action-button round-trip is documented.'
---

*Suggested by [[github-bkbillylnxlink-effortlessly-manage-your-linux-machine-using-mqtt-github]]*

lnxlink · PyPI

Skip to main content
Switch to mobile version

Warning

Some features may not work without JavaScript. Please try enabling it if you encounter problems.

Search PyPI

Search

lnxlink 2023.6.0

Internet Of Things (IOT) integration with Linux using MQTT

pip install lnxlink==2023.6.0

Copy PIP instructions

LNXlink

This is a Linux companion app for integrating your system with an external application like Home Assistant using MQTT.
It's very usefull for remote controling a linux PC, receiving notifications and monitoring it's stats.

Table of contents

Features

Installation

Headless Installation

Examples

FAQ

Features

System control: Shutdown, Restart, Suspend, Send Keys, Notify, Media, Screen On/Off, open URL/File, bash, Keep Alive, Brightness, Boot select.

System monitor: CPU, Ram, Network, Media, Microphone, Idle, Battery, Disk usage, Required restart, Nvidia GPU, Camera, Memory, Update required, System updates, Webcam, Screenshot.

Home Assistant: Uses MQTT Autodiscovery to create entities and shows if update is required.

No sudo required: No need to be root user to install and use, unless used on server setup.

Easily expanded: Any new module is automatically imported and custom modules can be added.

Installation

Install or update:
# For debian based distros:
sudo apt install patchelf meson libdbus-glib-1-dev libglib2.0-dev libasound2-dev python3-pip xdotool xprintidle xdg-utils
# For Red Hat based distros:
sudo dnf install python39-pip.noarch gcc cmake dbus-devel glib2-devel python39-devel alsa-lib-devel
pip3 install -U lnxlink
# When asked, it's recommended to install as a user service.
lnxlink -c config.yaml

You can manually update the configuration file config.yaml and restart the service with the use of systemctl:
systemctl --user restart lnxlink.service

Headless Installation

The headless installation is used for linux environments that don't use a Graphical Interface like servers.
sudo apt install patchelf meson libdbus-glib-1-dev libglib2.0-dev libasound2-dev python3-pip
sudo pip3 install -U lnxlink
# When asked, it's recommended to answer false on install as a user service.
sudo lnxlink -c config.yaml

Some modules depend on graphical interface, so if you choose to use this option for installation, you will have to find which ones stop lnxlink from starting and remove them from the config file.
sudo systemctl restart lnxlink.service

Examples

Send a notification with an image as a preview:
service: mqtt.publish
data:
topic: {prefix}/{clientId}/commands/notify
payload: >-
{ "title": "Notification Title",
"message": "Testing notification",
"iconUrl": "http://hass.local:8123/local/myimage.jpg" }

Send a command:
service: mqtt.publish
data:
topic: {prefix}/{clientId}/commands/bash
payload: "ctrl+shift+t"

Send a series of keys:
service: mqtt.publish
data:
topic: {prefix}/{clientId}/commands/send_keys
payload: "ctrl+f H e l l o space W o r l d"

Open a URL or a File
service: mqtt.publish
data:
topic: lnxlink/desktop-linux/commands/xdg_open
payload: "https://www.google.com"  # or "myimg.jpeg" for file

Combine with Wake on Lan to control your PC with one switch:
switch:
- platform: template
switches:
my_pc:
friendly_name: "My PC"
unique_id: my_pc
value_template: "{{ not is_state('button.shutdown', 'unavailable') }}"
turn_on:
service: switch.turn_on
data:
entity_id: switch.pc_wol
turn_off:
service: button.press
data:
entity_id: button.shutdown

Create a media player using mqtt-mediaplayer using the information collected from the media sensor:

Supports playing remote or local media using cvlc which should be installed.

Text To Speach

service: tts.google_say
data:
entity_id: media_player.desktop_linux
message: Hello world!

Play Media

service: media_player.play_media
data:
media_content_id: /home/user/imag.jpg
media_content_type: media  # Not used, but required by home assistant
target:
entity_id: media_player.desktop_linux

Camera Play Stream

service: camera.play_stream
data:
media_player: media_player.desktop_linux
target:
entity_id: camera.demo_camera

Create a custom module

You can create custom modules and import them to your configuration with their full path. Check out examples here and this is an example of how to add the mytest module to your configuration.
modules:
- /home/user/mytest.py

FAQ

Windows compatibility

Only Linux is supported and there is no plan on supporting Windows. A recomended companion app for windows is HASS.Agent.

Config file location

Your config file is located at the directory you were when you first run lnxlink. This can be anything you write instead of the config.yaml that I suggested. You can find where it is from the systemd service:
cat ~/.config/systemd/user/lnxlink.service  | grep -i ExecStart

Reinitiate systemd service

If you want to create the service from scratch, you will have to disable the running service and start lnxlink again:
systemctl --user disable lnxlink.service
lnxlink -c config.yaml

One of my integration is not working

By default all modules are automatically loaded. This happens when the modules section is empty like this:
modules:

You should select the ones you want to load. All supported modules can be found here and the configuration should look like this:
modules:
- notify
- camera_used
- idle
- keep_alive
- shutdown
- brightness

LNXlink doesn't become unavailable after shutdown

Just before LNXlink stops, it sends to MQTT an OFF command, but sometimes it doesn't stop gracefouly.
To fix this, you will have to create an automation on Home Assistant which checks for when was the last time one of the sensors got a value and if it exceeds it sends the OFF command to the MQTT server.

This is an example of the automation which checks events for the idle sensor:
alias: lnxlink powered down
description: ""
mode: single
trigger:
- platform: template
value_template: >-
{{ (now() | as_timestamp -
states.sensor.desktop_linux_idle.last_changed | as_timestamp) >
10 }}
condition: []
action:
- service: mqtt.publish
data:
qos: 0
retain: true
topic: lnxlink/desktop-linux/lwt
payload: "OFF"

Use Boot Select addon

This control needs to run as root, but it's not recomended to run lnxlink as a super user. To fix this, you need to allow the command grub-reboot to run without asking for password:
# Edit the sudoers file:
sudo visudo
# Add this line at the end (replace USER with your username):
USER ALL=(ALL) NOPASSWD: /usr/sbin/grub-reboot

How to help the development

In case you have found the solution to a bug or you want to create a new feature, follow these instructions to get you started:
# Install system dependencies
sudo apt install git patchelf meson libdbus-glib-1-dev libglib2.0-dev libasound2-dev python3-pip
# Fork my repository and then download it
git clone git@github.com:<yourusername>/lnxlink.git
# Install lnxlink as editable package
cd lnxlink
pip3 install -e .
# Run it manually
lnxlink -c config.yaml

Project links

Homepage

Source Code

Key dates

PyPI data

Data sourced directly from PyPI's database.

Released:
May 31, 2023

Newer release available (2026.8.0)

1 maintainer

PyPI data

Data sourced directly from PyPI's database.

bkbilly

Credits

Author:
bkbilly

License

MIT License

Requires

Python >=3.7.0

Tags

lnxlink

Classifiers

License

OSI Approved :: MIT License

Operating System

Unix

Programming Language

Python :: 3

Report project as malware

Download files

Download the file for your platform. If you're not sure which to choose, learn more about installing packages.

Source Distribution

lnxlink-2023.6.0.tar.gz
(21.7 kB
view details)

Uploaded
May 31, 2023
Source

Built Distribution

Filter files by name, interpreter, ABI, and platform.

If you're not sure about the file name format, learn more about wheel file names.

The dropdown lists show the available interpreters, ABIs, and platforms.

Enable javascript to be able to filter the list of wheel files.

Copy a direct link to the current filters

Copy

File name

Interpreter

Interpreter
py3

ABI

ABI
none

Platform

Platform
any

lnxlink-2023.6.0-py3-none-any.whl
(27.9 kB
view details)

Uploaded
May 31, 2023
Python 3

File details

Details for the file lnxlink-2023.6.0.tar.gz.

File metadata

Download URL: lnxlink-2023.6.0.tar.gz

Upload date:
May 31, 2023

Size: 21.7 kB

Tags: Source

Uploaded using Trusted Publishing? No

Uploaded via:         twine/4.0.2 CPython/3.9.16

File hashes

Hashes for lnxlink-2023.6.0.tar.gz

Algorithm
Hash digest

SHA256

f51eae07793fba6ccbd309ced69ee180eb2074669316dc2985ca4cd7d394d8a7

Copy

MD5

a51d710f94a72c5d514736a457ecd87c

Copy

BLAKE2b-256

b7f8a1412995f8abb99bd833117e3c1caaf3c944e52f3febf64c0769999018d0

Copy

See more details on using hashes here.

File details

Details for the file lnxlink-2023.6.0-py3-none-any.whl.

File metadata

Download URL: lnxlink-2023.6.0-py3-none-any.whl

Upload date:
May 31, 2023

Size: 27.9 kB

Tags: Python 3

Uploaded using Trusted Publishing? No

Uploaded via:         twine/4.0.2 CPython/3.9.16

File hashes

Hashes for lnxlink-2023.6.0-py3-none-any.whl

Algorithm
Hash digest

SHA256

5327a4f8ab2bc859e1a3f9f6b31f7c292c143828ef7c54f19118e5cb157f47d7

Copy

MD5

9c06e977595655608ed21eadec388b97

Copy

BLAKE2b-256

6d9df836250de95b191c4aee7f89e3e78707a11d1f423ec7c2568d59c0314f11

Copy

See more details on using hashes here.

Release history

Release notifications |
RSS feed

2026.8.0

Aug 8, 2026

2 files

2026.7.0

Jul 13, 2026

2 files

2026.6.0

Jun 2, 2026

2 files

2026.2.0

Jan 31, 2026

2 files

2025.12.0

Dec 21, 2025

2 files

2025.10.0

Oct 11, 2025

2 files

2025.7.0

Jul 9, 2025

2 files

2025.6.0

May 31, 2025

2 files

2025.5.0

May 1, 2025

2 files

2025.2.0

Mar 1, 2025

2 files

2025.1.0

Jan 4, 2025

2 files

2024.11.0

Nov 4, 2024

2 files

2024.10.1

Oct 4, 2024

2 files

2024.10.0

Oct 3, 2024

2 files

2024.9.0

Sep 1, 2024

2 files

2024.8.1

Aug 5, 2024

2 files

2024.7.0

Jun 30, 2024

2 files

2024.6.1

Jun 7, 2024

2 files

2024.6.0

May 31, 2024

2 files

2024.5.0

May 1, 2024

2 files

2024.4.0

Mar 29, 2024

2 files

2024.3.0

Feb 29, 2024

2 files

2024.2.2

Feb 12, 2024

2 files

2024.2.1

Feb 9, 2024

2 files

2024.2.0

Feb 1, 2024

2 files

2024.1.0

Jan 1, 2024

2 files

2023.12.2

Dec 12, 2023

2 files

2023.12.1

Dec 8, 2023

2 files

2023.12.0

Nov 25, 2023

2 files

2023.11.0

Nov 1, 2023

2 files

2023.10.0

Oct 6, 2023

2 files

2023.9.1

Sep 12, 2023

2 files

2023.9.0

Sep 1, 2023

2 files

2023.8.0

Jul 31, 2023

2 files

2023.7.1

Jul 6, 2023

2 files

2023.6.1

Jun 12, 2023

2 files

This release

2023.6.0
This release

May 31, 2023

2 files

2023.5.0

Apr 28, 2023

2 files

2023.4.1

Apr 20, 2023

2 files

2023.4.0

Mar 31, 2023

2 files

2023.3.1

Mar 18, 2023

2 files

2023.3.0

Mar 3, 2023

2 files

2023.2.0

Feb 3, 2023

2 files

2023.1.4

Jan 23, 2023

2 files

2023.1.3

Jan 21, 2023

2 files

2023.1.1

Jan 7, 2023

2 files

2023.1.0

Jan 3, 2023

2 files

2022.12.0

Dec 29, 2022

2 files

2022.11.4

Nov 19, 2022

2 files

2022.11.3

Nov 11, 2022

2 files

2022.11.2

Nov 2, 2022

2 files

2022.11.1

Nov 1, 2022

2 files

2022.11.0

Nov 1, 2022

2 files

2022.10.4

Oct 31, 2022

2 files

2022.10.3

Oct 30, 2022

2 files

2022.10.2

Oct 30, 2022

2 files

2022.10.1

Oct 30, 2022

2 files

Yanked

2022.10.0

Oct 30, 2022

2 files

Yanked reason: WIP

0.1

Oct 30, 2022

2 files

About PyPI

Contributing to PyPI

Using PyPI

Switch to desktop version

Anthropic, PBC
Visionary sponsor

Bloomberg
Visionary sponsor

Hudson River Trading
Visionary sponsor

Meta
Visionary sponsor

NVIDIA
Visionary sponsor

Microsoft
Sustainability sponsor

Depot
Continuous Integration

AWS
Cloud computing and Security Sponsor

Datadog
Monitoring

Fastly
CDN

Google
Download Analytics

Sentry
Error logging

StatusPage
Status page

"PyPI", "Python Package Index", and the blocks logos are registered trademarks of the Python Software Foundation.

© 2026 Python Software Foundation

Site map

Deployed from f7d9211