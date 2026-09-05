---
title: go-hass-agent command - github.com/joshuar/go-hass-agent - Go Packages
id: go-hass-agent-command-githubcomjoshuargo-hass-agent-go-packages
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- repo-source
- go-hass-agent
- stale-source
- architecture-history
created: '2026-09-02T04:02:37.763519Z'
updated: '2026-09-02T17:37:21.964450Z'
source: https://pkg.go.dev/github.com/joshuar/go-hass-agent
source_domain: pkg.go.dev
fetched_at: '2026-09-02T04:02:33.775779Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'pkg.go.dev page for the go-hass-agent module — STALE, pinned to v1.4.3 (published
  Jul 2023, flagged ''not the latest version'', 0 importers) versus the actual current
  release v14.15.1 in nixpkgs. Its historical value: documents the v1-era architecture
  (Fyne GUI tray app, -t/--terminal headless flag, systemd service in /usr/lib/systemd/system
  installed-but-not-enabled, internal packages: agent/config/device/hass/linux/request/tracker)
  and the v1 sensor set (location, active app, battery, network, memory/swap, disk,
  load averages, uptime, power profile, screen lock, ABRT). Shows the project''s evolution
  from tray-icon GUI app to headless web-configured agent — the exact trajectory ha-linux-agent
  is on. Reliability caveat: do not cite this for current behaviour.'
---

go-hass-agent command - github.com/joshuar/go-hass-agent - Go Packages

go-hass-agent

command

module

Version:
v1.4.3

Opens a new window with list of versions in this module.

Latest

Latest

This package is not in the latest version of its module.

Go to latest

Published: Jul  8, 2023

License: MIT

Opens a new window with license information.

Imports: 3

Opens a new window with list of imports.

Imported by: 0

Opens a new window with list of known importers.

Main

Versions

Licenses

Imports

Imported By

Details

Valid go.mod file

The Go module system was introduced in Go 1.11 and is the official dependency management
solution for Go.

Redistributable license

Redistributable licenses place minimal restrictions on how software can be used,
modified, and redistributed.

Tagged version

Modules with tagged versions give importers more predictable builds.

Stable version

When a project reaches major version v1 it is considered stable.

Learn more about best practices

Repository

github.com/joshuar/go-hass-agent

Links

Open Source Insights

README
¶

go-hass-app

A Home Assistant, native app
integration
for desktop/laptop devices.

🎉 Features

This app will add some sensors to a Home Assistant instance:

Device location.

Current active application and list of running applications.

Battery status (for example, laptop battery and any peripherals).

Network status (for example, network connection status, internal and external
IP addresses and Wi-Fi details where relevant).

Memory and swap usage (total/free/used).

Disk usage.

Load Averages.

Uptime.

Power profile.

Screen lock.

Problems detected by ABRT.

The code can be extended to add additional sensors. See the development
docs for details.

🤔 Use-cases

As examples of some of the things that can be done with the data published by this app:

Change your lighting depending on what active/running apps are on your
laptop/desktop. For example, you could set your lights dim or activate a scene
when you are gaming.

With your laptop plugged into a smart plug that is also controlled by Home
Assistant, turn the smart plug on/off based on the battery charge to
force a full charge/discharge cycle of the battery, extending its life over
leaving it constantly charged.

Like on mobile devices, create automations based on the location of your
laptop running this app.

Receive notifications from Home Assistant on your desktop/laptop.

See also the FAQ.

🤝 Compatibility

Currently, only Linux is supported. Though the code is designed to be extensible
to other operating systems. See the development docs for
details on how to extend for other operating systems.

⬇️ Installation

Head over to the releases
page and download the appropriate package for your operating system and/or
distribution:

For Fedora, use the .rpm.

For Ubuntu, use the .deb.

For Debian, use the .tar.xz.

For Arch, use the .tar.zst.

For other distributions not listed above, you can try the binary, or build it
yourself from source (see development). Note that while
Go is known for statically compiled binaries that "run anywhere", the Fyne UI
toolkit used by go-hass-agent makes use of shared libraries that may need to
be installed as well.

🖱️ Usage

go-hass-agent runs as a tray icon by default. It is operating system,
distribution and desktop-environment agnostic and should manifest itself in any
tray of any desktop environment.

First-run

On first-run, go-hass-agent will display a window where you will need to enter
some details, so it can register itself with a Home Assistant instance to be
able to report sensors and receive notifications.

You will need:

A long-lived access token. You can generate one on your account profile
page.

The web address (URL) on which a Home Assistant instance can be found.

go-hass-agent will try to auto-detect this for you, and you can select it in
the Auto-discovered servers list. Otherwise, you will need to select Use
Custom Server?, and enter the details manually in Manual Server Entry.

When you have entered all the details, click Submit and the agent should
start running and reporting sensors to the Home Assistant instance.

As alternative, you can register go-hass-agent on the command-line with by
running:
go-hass-agent register --token _TOKEN_ --server _URL_

You will need to provide a long-lived token _TOKEN_ and the URL of your Home
Assistant instance, _URL_.

Regular Usage

When running, go-hass-agent will appear as a device under the Mobile
App integration in your
Home Assistant instance. It should also report a list of sensors/entities you
can use in any automations, scripts, dashboards and other parts of Home
Assistant.

Running Headless

go-hass-agent can run in a “headless” mode, without any GUI elements, by
specifying the -t or --terminal command-line option. On Linux systems, There
is also a systemd service file that can be used for automatic start-up,
installed (but not activated by default) in /usr/lib/systemd/system.

🧑‍🤝‍🧑 Contributing

🏗️ Development

I would welcome your contribution! If you find any improvement or issue you want
to fix, feel free to send a pull request!

Some documentation for development can be found in
the development docs. There is information for developing
go-hass-agent for different operating systems as well as adding additional
sensors. This might help anyone to look to contribute, extend or fork this tool.

🌐 Translations

While this application does not have many points where text is displayed to
the end user (logging aside), translation is supported through the language
and message packages that are part of
golang.org/x/text.

I would welcome pull requests for translations!

🙌 Acknowledgements

The app icon is taken from the Home Assistant
project.

License

MIT

Expand ▾
Collapse ▴

Documentation
¶

There is no documentation for this package.

Source Files
¶

View all Source files

main.go

Directories
¶

Show internal

Expand all

Path
Synopsis

assets

trayicon

cmd

internal

agent

config

device

hass

hass/deviceClass

hass/sensorType

hass/stateClass

linux

request

tracker

tracker/mocks

translations

Click to show internal directories.

Click to hide internal directories.

Jump to

Close

Keyboard shortcuts

? : This menu

/ : Search site

f or F : Jump to

y or Y
: Canonical URL

Close

go.dev uses cookies from Google to deliver and enhance the quality of its services and to
analyze traffic. Learn more.

Okay