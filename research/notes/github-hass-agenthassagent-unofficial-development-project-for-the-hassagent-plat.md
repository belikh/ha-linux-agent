---
title: 'GitHub - hass-agent/HASS.Agent: Unofficial development project for the HASS.Agent
  platform. · GitHub'
id: github-hass-agenthassagent-unofficial-development-project-for-the-hassagent-plat
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- ha-linux-agent
- hass-agent
- windows-only
- comparative-benchmark
- mqtt-discovery
created: '2026-09-02T04:02:37.726814Z'
updated: '2026-09-02T17:37:21.978069Z'
source: https://github.com/hass-agent/HASS.Agent
source_domain: github.com
fetched_at: '2026-09-02T04:02:29.543820Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HASS.Agent (hass-agent org, 1.3k stars, fork continuing Sam/LAB02''s original)
  is a WINDOWS-ONLY .NET 8 Home Assistant companion client: notifications with actionable
  buttons, media player, quick actions with hotkeys, commands, sensors, WebView, and
  a Satellite Service that collects sensors/executes commands when no user is logged
  in. Entities auto-register in HA via MQTT integration. Crucially for the jupiterOS
  question it explicitly states ''What it''s not: A Linux/macOS client (at least yet)!''
  — cross-platform is only a ''hopeful wish'' for a future v3 rewrite, and it points
  Linux users to the official macOS companion app or IoPC. Release history confirms
  active maintenance (2.2.1 Jun 2025, 2.2.0 Jan 2025, 2.1.1) with the 2.2.0 breaking
  change removing LibreHardwareMonitor/WinRing0 for security, which permanently killed
  the GPU temperature sensor.'
---

GitHub - hass-agent/HASS.Agent: Unofficial development project for the HASS.Agent platform. · GitHub

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

hass-agent

/

HASS.Agent

Public

forked from LAB02-Research/HASS.Agent.Staging

Notifications
You must be signed in to change notification settings

Fork
69

Star
1.3k

main

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History414 Commits

414 Commits
Folders and filesNameName
Last commit message
Last commit date

.github

.github

assets

assets

src

src

.editorconfig

.editorconfig

.gitignore

.gitignore

LICENSE.md

LICENSE.md

README.md

README.md

View all files

Repository files navigation

HASS.Agent

HASS.Agent is a Windows-based client (companion) application for Home Assistant, developed in .NET 8.

Click here to download the latest installer.

HASS.Agent is completely free, and will always stay that way without restrictions.

Contents

Why?

Fork?

Functionality

Screenshots

Installation

Help and Documentation

Articles

What it's not

Helping Out

Credits and Licensing

Legacy

Why?

Quick note from Sam on the initial idea:

The main reason I built this is that I wanted to receive notifications on my PC, including images, and to quickly perform actions (e.g. to toggle a lamp). There weren't any software-based solutions for this, so I set out to build one myself.

There's no need to explain that we (and the Community overall) like the idea. That's why we're here, to continue the development in the Home Assistant spirit of integrating everything into open source smarthome world.

Fork?

The original HASS.Agent has been created by Sam.

Unfortunately due to some time constraints, they're not able to provide the constant support and feature updates. That's where we step in - trying to keep HASS.Agent bug free (dreams need to be big right?) and to introduce new features here and there!

Note on the organization and project name

"Why this project is named the same as original HASS.Agent, it's confusing"

Yes, I agree, you're right. The initial idea after Sam's disappearance was to continue the work and offer it to Sam once they're back.
Well, now that's more than unlikely since it's been quite a long time we last spoke with Sam.

Knowing what we do now, we've probably made a different decision but doing it now it'll only create more confusion. We have a full rewrite project going and it has been already decided the name is going to be altered.

Functionality

Summary of the core functions:

Notifications: receive notifications, show them using Windows builtin toast popups, attach images and receive input from them. Supports actionable notifications: add buttons so you can easily interact with Home Assistant, without having to open anything or ask user for an answer to a question.

This requires the installation of the HASS.Agent integration.

Media Player: use HASS.Agent as a mediaplayer device: see and control what's playing and send text-to-speech.

This requires the installation of the HASS.Agent integration.

Quick Actions: use a keyboard shortcut to quickly pull up a command interface, through which you can control Home Assistant entities - or, assign a keyboard shortcut to individual Quick Actions for even faster triggering.

Commands: control your PC (or other Windows based device) through Home Assistant using custom- or built-in commands.

Sensors: send your PC's sensors to Home Assistant to monitor every aspect of your device.

WebView: quickly show any website, anywhere - no browser required, for instance a HA dashboard.

Satellite Service: use the service to collect sensor data and execute commands, even when you're not logged in (not all commands/sensors are available for Satellite Service)

All entities are dynamically acquired from your Home Assistant instance.

Commands and sensors are automatically added to your Home Assistant instance via MQTT Integration

Screenshots

Notification examples:

WebView example, showing a dashboard when right-clicking the tray icon:

This is the Quick Action window you'll see when using the hotkey. This window automatically resizes to the amount of buttons you've added:

You can easily configure a new Quick Action, HASS.Agent will fetch your entities for you:

The sensors configuration screen:

Adding a new sensor is just as easy:

Easily manage the satellite service through HASS.Agent:

You'll be guided through the configuration options during onboarding:

Installation

Installing HASS.Agent is easy; just download the latest installer, run it and you're done! The installer is signed by us and won't download or do weird stuff - it just places everything where it should, and launches with the right parameter. (optionally installing .NET8)

After installing, the onboarding process will help you get everything configured, step by step. If you want an introduction into HASS.Agent, be sure to read the introduction docs.

Original HASS.Agent documentation is available here - please bear in mind however that it may not represent state of things present in this version.

Click here to download the latest installer

If you want to install manually, there are .zip packages available for every release. Read the manual for more info.

Help and Documentation

Stuck while installing or using HASS.Agent, need some help integrating the sensors/commands or have a great idea for the next version? There are a few channels through which you can reach out:

Github Tickets: Report bugs, feature requests, ideas, tips, ..

Documentation: Installation, configuration and usage documentation, as well as examples.

Discord: Get help with setting up and using HASS.Agent, report bugs or just talk about whatever.

Home Assistant forum: Bit of everything, with the addition that other HA users can help as well.

Starting from zero, and want to learn what HASS.Agent's about and how to start? Be sure to check the introduction article, and optionally the command basics.

EverythingSmartHome's youtube video is a great guide on the original HASS.Agent version: Control Your Windows PC With Home Assistant!. We recommend having a look at his other videos as well, great stuff!

If you want to help with the development of HASS.Agent, check out the Helping Out section for (translating) info.

Articles

Original HASS.Agent by Sam/LAB02 Research

Liam Alexander Colman from Home Assistant Guide was kind enough to write an article about HASS.Agent: Integrate Home Assistant with Windows using HASS.Agent. The website's full of useful articles, worth having a look :)

What it's not

A Linux/macOS client (at least yet)!

Without getting into much of the details, it's not as easy as you think.

With HASS.Agent "2.X" version it's basically impossible. We are thinking about cross-platform support for the "v3 rewrite" but as of now it's only a hopeful wish.

You can try the official companion app for macOS, or IoPC which runs on Linux.

Note: We haven't tested either and we do not track the development efforts.

Helping Out

The best way to help out is to test as much as you can (or even join the beta program), and report any weird or failing behavior by opening a ticket.

Same goes for sharing ideas for new (or improved) functionality! If you want, you can join on Discord to discuss your ideas.

Feature PR Submissions

While all feature PR submissions are welcome, please note that their merging is at sole discretion of the main developers.

When designing the new feature please take into account:

how they fit with the codebase/codestyle (yes, we are aware the codebase has it's flaws - hence the rewrite efforts)

is the feature agnostic enough and will benefit all users

the smaller the PR the better the chance of it being merged

AI Usage

Fully "vibe-coded" submissions will most likely be rejected.

While personal opinions on AI usage and it's impact can vary person-to-person, we'd rather put the effort writing code than reviewing fully AI generated and copy-pasted code.

Credits and Licensing

First and foremost, huge thanks for Sam for creating and maintaining the original HASS.Agent in their spare time! We wouldn't be here withot the spark that pushed them to write the first line of code ❤️

As of now, we do not accept any kind of donation/coffee :)

If you'd like however, you can support creator of original HASS.Agent:

Thanks to the entire team that's developing Home Assistant - such an amazing platform!

The initial development was boosed by sleevezipper's HASS Workstation Service. Thank you for sharing your hard work.

And a big thank you to all other packages:

CoreAudio, HotkeyListener, MQTTnet, Syncfusion, Octokit, Cassia, Grapevine, LibreHardwareMonitor, Newtonsoft.Json, Serilog, CliWrap, HADotNet, Microsoft.Toolkit.Uwp.Notifications, GrpcDotNetNamedPipes, gRPC, ByteSize.

Please consult their individual licensing if you plan to use any of their code.

Everything on the HASS.Agent platform is released under the MIT license.

Legacy

HASS.Agent is a .NET 8 application. If for some reason you can't install .NET 8, you can use the last .NET Framework 4.8 version also developed by Sam:

v2022.3.8

Per it's release time it was pretty feature complete if you just want commands, sensors, quickactions and notifications.

You'll need to have .NET Framework 4.8 installed on your PC, which you can download here.

Please note

.NET Framework version is considered legacy and unsupported

All issues/requests regarding the .NET Framework version of HASS.Agent will be closed

About
Unofficial development project for the HASS.Agent platform.
hass-agent.io
Resources
Readme
MIT license
Activity
Custom properties
Stars
1.3k stars
Watchers
18 watching
Forks
69 forks
Report repository

Releases

Packages

Contributors

Languages

You can’t perform that action at this time.