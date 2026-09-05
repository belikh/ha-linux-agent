---
title: 'GitHub - niri-wm/niri: A scrollable-tiling Wayland compositor. · GitHub'
id: github-niri-wmniri-a-scrollable-tiling-wayland-compositor-github
tags:
- linux-agent-jupiteros-fleet-15537b
- testing
- rust
- official-docs
- api-docs
- birth-message
- wayland
- kiosk
created: '2026-09-02T06:41:40.104593Z'
updated: '2026-09-02T17:37:22.748166Z'
source: https://github.com/niri-wm/niri
source_domain: github.com
fetched_at: '2026-09-02T06:41:37.099990Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'niri (niri-wm/niri, 27.4k stars, GPL-3.0, Rust+smithay): a scrollable-tiling
  Wayland compositor — windows arranged in columns on an infinite strip, per-monitor
  window strips, dynamic vertical workspaces. Status: stable for daily use; floating
  since 25.01; Xwayland via xwayland-satellite since 25.08; built-in screenshot UI,
  screencasting via xdg-desktop-portal-gnome, live-reloading config, works with screen
  readers, multi-monitor + mixed DPI from the start. Not a complete desktop environment
  — pairs with shells like DankMaterialShell/Noctalia. Repo layout includes niri-config,
  niri-ipc sub-crates, clippy.toml, flake.nix. For jupiterOS kiosks: niri is the compositor
  layer whose IPC surface (see [[ipc-niri-msg-niri]]) exposes idle/monitor/window
  state an agent could report to Home Assistant.'
---

GitHub - niri-wm/niri: A scrollable-tiling Wayland compositor. · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

Uh oh!

There was an error while loading. Please reload this page.

niri-wm

/

niri

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
1.1k

Star
27.4k

main

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History2,858 Commits

2,858 Commits
Folders and filesNameName
Last commit message
Last commit date

.github

.github

docs

docs

niri-config

niri-config

niri-ipc

niri-ipc

niri-visual-tests

niri-visual-tests

resources

resources

src

src

.gitignore

.gitignore

CONTRIBUTING.md

CONTRIBUTING.md

Cargo.lock

Cargo.lock

Cargo.toml

Cargo.toml

LICENSE

LICENSE

README.md

README.md

build.rs

build.rs

clippy.toml

clippy.toml

flake.lock

flake.lock

flake.nix

flake.nix

niri.spec.rpkg

niri.spec.rpkg

rustfmt.toml

rustfmt.toml

typos.toml

typos.toml

View all files

Repository files navigation

A scrollable-tiling Wayland compositor.

Getting Started | Configuration | Setup Showcase

About

Windows are arranged in columns on an infinite strip going to the right.
Opening a new window never causes existing windows to resize.

Every monitor has its own separate window strip.
Windows can never "overflow" onto an adjacent monitor.

Workspaces are dynamic and arranged vertically.
Every monitor has an independent set of workspaces, and there's always one empty workspace present all the way down.

The workspace arrangement is preserved across disconnecting and connecting monitors where it makes sense.
When a monitor disconnects, its workspaces will move to another monitor, but upon reconnection they will move back to the original monitor.

Features

Built from the ground up for scrollable tiling

Dynamic workspaces like in GNOME

An Overview that zooms out workspaces and windows

Built-in screenshot UI

Monitor and window screencasting through xdg-desktop-portal-gnome

You can block out sensitive windows from screencasts

Dynamic cast target that can change what it shows on the go

Touchpad and mouse gestures

Group windows into tabs

Configurable layout: gaps, borders, struts, window sizes

Gradient borders with Oklab and Oklch support

Background blur for windows and layer-shell surfaces

Animations with support for custom shaders

Live-reloading config

Works with screen readers

Video Demo

demo.mp4

Also check out these videos that showcase a lot of the niri functionality:

Niri Is My New Favorite Wayland Compositor by Brodie Robertson

How Is niri This Good? Live Demo + Config by Nick Janetakis

Status

Niri is stable for day-to-day use and does most things expected of a Wayland compositor.
Many people are daily-driving niri, and are happy to help in our Matrix channel.

Give it a try!
Follow the instructions on the Getting Started page.
Grab a desktop shell like DankMaterialShell or Noctalia (or build a more traditional setup): niri by itself is not a complete desktop environment.
Also check out awesome-niri, a list of niri-related links and projects.

Here are some points you may have questions about:

Multi-monitor: yes, a core part of the design from the very start. Mixed DPI works.

Fractional scaling: yes, plus all niri UI stays pixel-perfect.

NVIDIA: seems to work fine.

Floating windows: yes, starting from niri 25.01.

Input devices: niri supports tablets, touchpads, and touchscreens.
You can map the tablet to a specific monitor, or use OpenTabletDriver.
We have touchpad gestures, but no touchscreen gestures yet.

Wlr protocols: yes, we have most of the important ones like layer-shell, gamma-control, screencopy.
You can check on wayland.app at the bottom of each protocol's page.

Performance: while I run niri on beefy machines, I try to stay conscious of performance.
I've seen someone use it fine on an Eee PC 900 from 2008, of all things.

Xwayland: integrated via xwayland-satellite starting from niri 25.08.

Media

niri: Making a Wayland compositor in Rust · December 2024

My talk from the 2024 Moscow RustCon about niri, and how I do randomized property testing and profiling, and measure input latency.
The talk is in Russian, but I prepared full English subtitles that you can find in YouTube's subtitle language selector.

An interview with Ivan, the developer behind Niri · June 2025

An interview by a German tech podcast Das Triumvirat (in English).
We talk about niri development and history, and my experience building and maintaining niri.

A tour of the niri scrolling-tiling Wayland compositor · July 2025

An LWN article with a nice overview and introduction to niri.

Contributing

If you'd like to help with niri, there are plenty of both coding- and non-coding-related ways to do so.
See CONTRIBUTING.md for an overview.

Inspiration

Niri is heavily inspired by PaperWM which implements scrollable tiling on top of GNOME Shell.

One of the reasons that prompted me to try writing my own compositor is being able to properly separate the monitors.
Being a GNOME Shell extension, PaperWM has to work against Shell's global window coordinate space to prevent windows from overflowing.

Tile Scrollably Elsewhere

Here are some other projects which implement a similar workflow:

PaperWM: scrollable tiling on top of GNOME Shell.

karousel: scrollable tiling on top of KDE.

scroll and papersway: scrollable tiling on top of sway/i3.

Hyprland has a built-in scrolling layout.

Paneru and PaperWM.spoon: scrollable tiling on top of macOS.

Contact

Our main communication channel is a Matrix chat, feel free to join and ask a question: https://matrix.to/#/#niri:matrix.org

We also have a community Discord server: https://discord.gg/vT8Sfjy7sx

About
A scrollable-tiling Wayland compositor.
niri-wm.github.io/niri/
Topics
rustsmithaytiling-window-managerwaylandwayland-compositor
Resources
Readme
GPL-3.0 license
Contributing
Contributing
Activity
Custom properties
Stars
27.4k stars
Watchers
83 watching
Forks
1.1k forks
Report repository

Releases

Sponsor this project

Used by

Contributors

Languages

You can’t perform that action at this time.