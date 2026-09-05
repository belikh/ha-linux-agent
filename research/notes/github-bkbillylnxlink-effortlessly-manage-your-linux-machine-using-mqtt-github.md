---
title: 'GitHub - bkbilly/lnxlink: 🖥 Effortlessly manage your Linux machine using MQTT.
  · GitHub'
id: github-bkbillylnxlink-effortlessly-manage-your-linux-machine-using-mqtt-github
tags:
- linux-agent-jupiteros-fleet-15537b
- lnxlink
- comparative-benchmark
- mqtt-discovery
- linux-agent
created: '2026-09-02T04:30:34.344297Z'
updated: '2026-09-02T17:37:22.101455Z'
source: https://github.com/bkbilly/lnxlink
source_domain: github.com
fetched_at: '2026-09-02T04:30:34.342957Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'lnxlink (bkbily, 480 stars, 790 commits, MIT, Python 3.7+) is a Linux-only
  HA companion app using MQTT Autodiscovery — the closest architectural sibling to
  ha-linux-agent (MQTT-first, no native HA API). Recommended by r/homeassistant as
  ''the Hass agent for Linux''. Distribution: pipx install lnxlink, install.sh, Docker
  (bkbillybk/lnxlink), Flathub, AUR, systemd service; no sudo required except server
  environments. Expandable module architecture — auto-imports new modules, supports
  custom modules. Pairs with a separate HA integration, hass-mqtt-mediaplayer, for
  the media-player piece. GitHub README is thin; the gitbook docs carry the real module
  catalogue.'
---

*Suggested by [[reddit]] — named in the r/homeassistant HASS-Agent-alternative-for-Linux thread as the recommended Linux agent*

GitHub - bkbilly/lnxlink: 🖥 Effortlessly manage your Linux machine using MQTT. · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

bkbilly

/

lnxlink

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
57

Star
480

master

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History790 Commits

790 Commits
Folders and filesNameName
Last commit message
Last commit date

.github

.github

images

images

lnxlink

lnxlink

packaging

packaging

.dockerignore

.dockerignore

.gitignore

.gitignore

.pre-commit-config.yaml

.pre-commit-config.yaml

Dockerfile

Dockerfile

LICENSE.md

LICENSE.md

README.md

README.md

docker-compose.yaml

docker-compose.yaml

find_libraries.py

find_libraries.py

install.sh

install.sh

pyproject.toml

pyproject.toml

requirements.txt

requirements.txt

View all files

Repository files navigation

LNXlink

LNXlink is a Linux companion app that seamlessly integrates your system with external applications like Home Assistant.
It uses MQTT, a lightweight messaging protocol, to enable real-time data exchange and remote control capabilities.

With LNXlink, you can monitor your Linux machine's performance, execute commands remotely, and integrate it into your smart home ecosystem for centralized management.

Key Features

Sensor Monitoring: Automatically or manually expose sensors that monitor and control the system remotely.

Home Assistant: Utilizes MQTT Autodiscovery to create entities in Home Assistant dashboard.

Easy Installation: No sudo privileges required for installation and operation, except for server environments.

Expandable Architecture: Automatically imports new modules and allows for the addition of custom modules.

Get started

To get started with LNXlink, follow these simple steps:

Download the LNXlink application and install it on your Linux machine: pipx install lnxlink

Follow the configuration instructions to setup LNXlink: lnxlink -c lnxlink.yaml

Install and configure on Home Assistant the hass-mqtt-mediaplayer integration.

Enjoy real-time monitoring and control of your Linux machine from your Home Assistant dashboard.

For detailed installation instructions, please refer to the documentation page: bkbilly.gitbook.io/lnxlink.

Benefits

Cross-Platform Compatibility: Runs on any Linux distribution, providing flexibility and wide-ranging compatibility.

Enhanced System Insights: Gain real-time insights into your Linux machine's performance by monitoring essential system metrics.

Remote Command Execution: Execute arbitrary commands directly from your Home Assistant dashboard, granting remote control over your Linux machine.

Seamless Integration with Home Assistant: Integrate your Linux machine into your smart home ecosystem for unified control and monitoring.

Automate tasks: Set up automated tasks to perform repetitive actions and save yourself time and effort.

Support LNXlink's Development

To contribute to the development of LNXlink, you can sponsor the project through GitHub Sponsors or PayPal. Your support will help maintain the project, add new features, and fix bugs.

About
🖥 Effortlessly manage your Linux machine using MQTT.
bkbilly.gitbook.io/lnxlink
Topics
automationcommand-executioncompanion-appcontrolhome-assistanthome-automationintegrationiotlinuxmqttnotificationsremote-controlremote-monitoringsensorssmarthomesystemd-service
Resources
Readme
MIT license
Code of conduct
Code of conduct
Contributing
Contributing
Activity
Stars
480 stars
Watchers
6 watching
Forks
57 forks
Report repository

Releases

Sponsor this project

Used by

Contributors

Languages

You can’t perform that action at this time.