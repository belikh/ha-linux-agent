---
title: 'Go-hass-agent: a native app integration for desktop/laptop devices - Share
  your Projects! - Home Assistant Community'
id: go-hass-agent-a-native-app-integration-for-desktoplaptop-devices-share-your-proj
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- ha-linux-agent
- repo-map
- go-hass-agent
- nixos
- community-demand
- feature-gaps
created: '2026-09-02T04:02:37.758146Z'
updated: '2026-09-02T17:37:21.973067Z'
source: https://community.home-assistant.io/t/go-hass-agent-a-native-app-integration-for-desktop-laptop-devices/559250
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T04:02:33.076798Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA community thread (Apr 2023, still active Jan 2026) where go-hass-agent''s
  author introduced the project. Three practitioner data points load-bearing for the
  jupiterOS fleet case: (1) headless demand — mobrien118 wants it on a Proxmox headless
  host but the .deb fails on GUI dependencies, asking for core/GUI package split,
  still unshipped; (2) VERBATIM WORKING NIXOS PACKAGING — user Azelphur posts a complete
  buildGoModule derivation for v13.2.7 with vendorHash and X11 build inputs, proving
  community demand for exactly the jupiterOS use case (zigbee button -> WOL on, go-hass-agent
  for clean shutdown); (3) unmet feature demand — howels (Jan 2026) asks for an audio
  output device selector (toggle PipeWire sink between headphones/speakers) which
  go-hass-agent''s controls cannot express (no select/dropdown control type). Also:
  Debian install bug reported in 2023 first-days; author self-describes as hobby project
  with 1-week response cadence aspiration.'
---

Go-hass-agent: a native app integration for desktop/laptop devices - Share your Projects! - Home Assistant Community

Go-hass-agent: a native app integration for desktop/laptop devices

Share your Projects!

IsThisIt

April 12, 2023,  9:14am

1

Hi folks,

I wanted to share a project I’ve been working on, go-hass-agent. This is native app integration for desktop/laptop devices. It sits in the system tray of your desktop environment and quietly sends sensor data to a Home Assistant instance.

My motivation for writing this was originally to adjust my lights when I’m gaming on my desktop. But then I got carried away writing an entire app. I am no developer, so this was kind of just a fun hobby project for me. Maybe in a few days/months/years a robot can pick it up and help out?

Sensors

It will currently add the following sensors to a Home Assistant instance:

Device location.

Current active application and list of running applications.

Battery status (for example, laptop battery and any peripherals).

Network status (for example, network connection status, IP addresses and Wi-Fi details where relevant).

The code can be extended to add additional sensors. See Device/Sensors for details.

Use Cases

As examples of some of the things that can be done with the data published by this app:

Change your lighting depending on what active/running apps are on your laptop/desktop. For example, you could set your lights dim or activate a scene when you are gaming.

With your laptop plugged into a smart plug that is also controlled by Home Assistant, turn the smart plug on/off based on the battery charge to force a full charge/discharge cycle of the battery, extending its life over leaving it constantly charged.

Like on mobile devices, create automations based on the location of your laptop running this app.

Receive notifications from Home Assistant on your desktop/laptop.

Installation/Running

See the README for details. Note at the moment, only Linux  is supported. Though it should be extensible to other operating systems as it is written in Go which can run just about anywhere.

Issues/Feature Requests

Please create any issues/feature requests on GitHub. In particular, for issues/bugs, a log will greatly help me debug things. You can generate a log by:

Run go-hass-agent from a terminal or command-line with the --debug flag:

go-hass-agent --debug

Try to reproduce the problem.

After you have reproduced the problem, please (compress and) attach the go-hass-agent.log file found in the following location:

On Linux, in ~/.config/fyne/com.github.joshuar.go-hass-agent/go-hass-app.log

(While I have made efforts to not log sensitive information, please check the log before uploading to GitHub and remove any information you do not want to share).

Contributing

If you would like to contribute new sensors or extend the app to run on additional platforms, please see my notes on contributing that will help you get started. Also, thank you!

vincen

(Vincèn)

April 12, 2023,  9:47am

2

Thanks for the share  Wanted to try it on my Debian but it has an issue (I opened a bug report on Github

mobrien118

June 19, 2025,  7:20pm

3

I like the idea of your app. I would like to run it on a “headless” system, specifically one running (Debain based) Proxmox. When I try to install the .deb it fails because of dependencies related to the GUI. I’m wondering if you have considered splitting out the core features and the GUI into separate packages, as seems to be common in Linux software like this that could, theoretically, be useful headless with enhanced visibility in a GUI.

Azelphur

(Azelphur)

June 25, 2025, 10:01pm

4

Thanks, this works really well. I use it to bind a zigbee shortcut button to toggle my PC (WOL for on, go-hass-agent for clean shutdown) as well as notifications.

Managed to get it going on NixOS, in case it helps anyone:
{ config, lib, pkgs, inputs, ...}:

let
go-hass-agent = pkgs.buildGoModule (finalAttrs: {
pname = "go-hass-agent";
version = "13.2.7";

src = pkgs.fetchFromGitHub {
owner = "joshuar";
repo = "go-hass-agent";
tag = "v${finalAttrs.version}";
hash = "sha256-nec5gH/p65sfxNDD+5TEc7POP7B2MOA23aMqJ9aXAz8=";
};

nativeBuildInputs = with pkgs; [
pkg-config
];

subPackages = [ "." ];

buildInputs = with pkgs; [
xorg.libX11
xorg.libXrandr
xorg.libXxf86vm
xorg.libXi
xorg.libXcursor
xorg.libXinerama
xorg.libXext
xorg.libxcb
mesa
glfw
];

vendorHash = "sha256-PVicvYGaZxNVUbmNCEGUd4BZklhgaSMKpeiy898YTbM=";
meta = with pkgs.lib; {
description = "Go-based Home Assistant agent";
homepage = "https://github.com/joshuar/go-hass-agent";
license = licenses.mit;
};
});
in
{
environment.systemPackages = with pkgs; [
go-hass-agent
];
}

howels

(Howels)

January 4, 2026, 11:21am

5

Wondering if we can select audio output device with go-hass-agent, to toggle pipewire or pulseaudio output to headphones or speakers?

I can’t see a means to do a drop-down selector box with multiple text items.

Powered by Discourse, best viewed with JavaScript enabled