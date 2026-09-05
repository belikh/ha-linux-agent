---
title: LNXlink
id: lnxlink
tags:
- linux-agent-jupiteros-fleet-15537b
- lnxlink
- module-catalogue
- comparative-benchmark
- feature-gaps
- mqtt-discovery
created: '2026-09-02T04:31:16.792537Z'
updated: '2026-09-02T17:37:22.106719Z'
source: https://bkbilly.gitbook.io/lnxlink
source_domain: bkbilly.gitbook.io
fetched_at: '2026-09-02T04:31:16.791130Z'
fetch_provider: builtin
status: review
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'lnxlink official docs — the most complete module catalogue of any Linux
  HA agent: ~50 modules across GUI, system actions, system info, network, AV/input,
  tools, and custom categories. Highlights beyond ha-linux-agent''s current scope:
  notify.send_message rich desktop notifications, Open URL/File, Keep Alive (prevent
  monitor sleep), idle-time sensor, media control with metadata, brightness number-entities,
  screen on/off, fullscreen detection with window name, desktop screenshot streaming
  to an HA image entity, AUDIO SELECT (switch speaker/mic devices — the exact feature
  a go-hass-agent user asked for in Jan 2026), keyboard hotkey capture (X11 only,
  not Wayland), mouse control with a companion Touchpad Card, send keys, Steam game
  launcher dropdown, display-env detection, active window monitor, clipboard view/update,
  current unlocked graphical users (ignores SSH/locked), boot-select for next boot,
  power-profile toggle, systemd unit management (status/start/stop per unit), GPU
  usage for NVIDIA/AMD, restart-required detection (kernel updates), system-updates
  tracking, disk IO/usage/mounts, network speed/interfaces, Bluetooth device control
  with battery, WiFi strength, WOL enable/disable, BeaconDB WiFi-triangulation location,
  mic/speaker/camera/gamepad in-use binary sensors, webcam switch + camera feed, fingerprint-scanner
  (RPI UART), speech/voice input, GPIO, IR remote, bash-command custom sensors/binary_sensors/buttons/switches,
  docker container management, update-entities force-refresh, RESTful module, LNXlink
  self-update from HA, dynamic log level, and ''Inference Time'' module that measures
  sensor-collection latency for performance debugging. Docs also carry the canonical
  alternatives table: Go Hass Agent (Linux/Win/macOS, Native App API + MQTT), HASS.Agent
  (Windows, HA API + MQTT), System Bridge (Windows/Linux, HA API WebSocket), Glances
  (cross-platform, REST HTTP polling), IoTuring (cross-platform, MQTT). Linux-only
  by design: ''deep system dependencies'', no Windows/macOS planned.'
---

*Suggested by [[github-bkbillylnxlink-effortlessly-manage-your-linux-machine-using-mqtt-github]] — official lnxlink documentation containing the Linux-agent alternatives comparison table*

LNXlink

LNXlink

🌩️LNXlink
🖥️Setup
📂Configuration
🎬Media Player
🔗Modules Settings
🔱Modules Usage
🤯Automations
🛠️DevelopmentPowered by GitBook

On this page

For the complete documentation index, see llms.txt. This page is also available as Markdown.

Welcome
LNXlink is a Home Assistant companion app for Linux that bridges the gap between your PC and your smart home ecosystem. By leveraging MQTT and Autodiscovery, it allows you to monitor system stats and trigger remote commands in real-time with zero manual entity configuration.
Features

Automated Sensors: Discovers and exposes system metrics and controls automatically.

MQTT Autodiscovery: Integrates instantly with Home Assistant with update notifications.

Lightweight: Built to run with minimal system dependencies.

Extensible: Supports a modular architecture; easily import or create custom modules.
Supported Modules
🧮 Graphical Interface

Module
Description

📢 Notify

Send rich desktop notifications via notify.send_message. Usage

📂 Open URL/File

Remotely launch websites, files, or folders. Usage

🚥 Keep Alive

Prevent monitor sleep or idle states.

⌛ Idle time

Monitor user inactivity duration with a dedicated sensor.

🎶 Media

Control playback and track metadata for active media. Setup

🔆 Brightness

Adjust hardware display luminance for monitors via number entities.

💡 Screen On/Off

Toggle monitor power states

⛶ Fullscreen

Detect if a window is currently in fullscreen mode and view its name.

📸 Screenshot

Stream your desktop directly to Home Assistant via an image entity.

🎧 Audio Select

Switch between available speaker or microphone input devices.

⌨️ Keyboard Hotkeys

Capture specific keypresses for automation triggers (Not for Wayland). Settings

🖱️ Mouse control

Simulate mouse movement and clicks. Works with the LNXlink Touchpad Card.

🔑 Send Keys

Broadcast keystrokes or complex combinations. Usage

🎮 Steam

Launch Steam or non-Steam games from a dropdown list.

🪟 Display Env

Identify the current display environment (e.g., :0).

🗔 Active Window

Monitor the name and title of the currently focused window.

📋 Clipboard

View or update the system clipboard content.

👤 Current Users

Monitor active, unlocked graphical users while ignoring SSH and locked sessions.
✅ System Actions

Module
Description

🔴 Shutdown

Shut down the computer instantly from your dashboard.

⚪ Restart

Reboot the system remotely.

💤 Suspend

Trigger sleep mode to save power when not in use.

🚀 Boot Select

Choose which operating system to load on the next boot.

⚡ Power Profile

Toggle between performance, balanced, or power-saver profiles.

⚙️ SystemD

Manage Linux services; check status, start, or stop specific units. Settings
🖥 System Information

Module
Description

🧠 CPU

Monitor real-time CPU load and performance.

💾 RAM

Track memory usage and availability.

🖼️ GPU

Monitor load and usage for NVIDIA or AMD graphics cards.

🔋 Battery

Track battery levels for all connected devices.

🌡️ Temperature

Monitor thermal data from all discovered system sensors.

⚠️ Restart Required

Detect if a system reboot is needed (usually after kernel updates).

🔄 System Updates

Track pending updates in real-time.

📥 Disk IO

Measure read/write throughput for each physical disk.

📀 Disk Usage

Monitor storage capacity and percentage used per disk. Usage

🖴 Mounts

View space usage for all currently mounted volumes. Usage
📡 Network & Devices

Module
Description

📶 Network Speed

Monitor real-time upload and download speeds.

🌐 Interfaces

List active network interfaces and their assigned IP addresses.

📱 Bluetooth

Control global Bluetooth power, connect/disconnect specific devices and their battery.

🛜 WiFi

Monitor signal strength and connection metadata.

🔌 WOL

Enable or disable Wake-On-LAN support for compatible network cards.

🗺️ BeaconDB

Locate the device using WiFi triangulation or custom coordinates. Settings
🎚️ Audio/Video/Input

Module
Description

🎤 Microphone Used

Monitor if any application is currently accessing the microphone.

🔈 Speaker Used

Detect active audio output to determine if the system is "in use."

🎥 Camera Used

Track webcam activity for privacy or presence automations.

📹 Webcam

Expose a webcam switch and camera feed.

🎮 Gamepad Used

Report controller activity (active if input detected within 40s).

🔐 Fingerprint

Use an R503 fingerprint scanner over UART on Raspberry Pi. Settings
🧰 Applications & Tools

Module
Description

🌍 LNXlink Update

Update LNXlink directly from Home Assistant. Usage

🗣️ Speech

Process voice input and return responses via binary sensor attributes. Usage

🧲 GPIO

Control and monitor Raspberry Pi GPIO pins. Settings

📺 IR Remote

Control IR devices or decode incoming signals. Settings
🧩 Advanced/Other

Module
Description

🐚 Bash Commands

Create custom sensors, binary_sensors, buttons, or switches using shell scripts. Settings

🐳 Docker

Manage containers; toggle status, check for updates, or prune images. Settings

⏳ Inference Time

Debug performance by measuring sensor data collection latency.

📜 Logging Level

Change debug verbosity on-the-fly for troubleshooting.

📊 Statistics

Opt-in to send anonymous usage data to help improve LNXlink. Usage

📮 RESTful

Interact with the system using standard HTTP requests. Usage

🔁 Update Entities

Force all or selected module entities to publish a fresh update.

👁️ Watch Changes

Restart when the configuration changes
📦 Custom Modules

Module
Link

Lutris Game Launcher

Discussion #202

Active Window (Wayland)

Discussion #126

Screens On/Off (KDE)

KDE Module Source

AM2302 Temp/Humidity

Discussion #81

Satisfactory Server

Discussion #128

GPU nvidia-settings

NVIDIA Settings Source
Supported OS
LNXlink is built specifically for Linux. There are currently no plans for Windows or macOS support due to deep system dependencies. Here are some alternatives:

Application
Platform
Protocol

Go Hass Agent

Linux, Windows, macOS

Native HA Mobile App API + MQTT

HASS.Agent

Windows

HA API + MQTT

System Bridge

Windows, Linux

HA API (WebSocket)

Glances

Cross-platform (Linux, Windows, macOS, BSD)

REST API (HTTP polling)

IoTuring

Cross-platform (Windows, Linux, macOS, BSD)

MQTT

NextSetup

Last updated 22 days ago

Welcome
Features
Supported Modules
🧮 Graphical Interface
✅ System Actions
🖥 System Information
📡 Network & Devices
🎚️ Audio/Video/Input
🧰 Applications & Tools
🧩 Advanced/Other
📦 Custom Modules
Supported OS