---
title: Using Commands - HASS.Agent Documentation
id: using-commands-hassagent-documentation
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- availability
- windows-only
- mqtt-discovery
created: '2026-09-02T06:41:40.119267Z'
updated: '2026-09-05T10:51:22.052733Z'
source: https://www.hass-agent.io/2.2/getting-started/commands/
source_domain: www.hass-agent.io
fetched_at: '2026-09-02T06:41:39.144554Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HASS.Agent (Windows companion app) docs — Creating your first Command: commands
  are HA entities (Button recommended for most; only some command types return state)
  configured with TYPE + Entity Type + Name (becomes the entity_id, no spaces) + optional
  friendly name. After ''Store and Activate'', commands are picked up immediately
  and each attaches under the MQTT integration named after the computer. Availability
  check option controls whether HA shows last state while the computer is offline.
  This is the Windows-side feature benchmark ha-linux-agent lacks: HA-dashbutton entities
  that trigger local actions (shutdown/restart, custom PowerShell) on the agent host.'
---

Using Commands - HASS.Agent Documentation

Skip to content

We need help! If you have experience in C# or Python we would love to have you help out.

Click here for more.

You're not viewing the latest version.

Click here to go to latest.

English

Deutsch

Help translating

Initializing search

HASS.Agent

Using your command

Further Reading

Sending Notifications

Setting up the Media Player

Using Webviews

Frequently Asked Questions

Troubleshooting

Setup

Sensors

Notifications

Media Player

Quick Actions

Helping Out

Developing the Integration

Developing the Documentation

Translating

Extra Resources

Changelog

Getting Help

Using your command

Further Reading

Creating your first Command¶

One of the main reasons for creating HASS.Agent is the ability to use commands and control your windows pc from home assistant automations and dashboards. This guide will show you how to make your first command, after that you can checkout the in-depth setup guide to create more complex commands.

Creating your first Command¶

Required properties¶

TYPE: Select the type of command from the left, there are many options available, a full description of each is available in the setup guide.

Entity Type: Select what entitiy you want in home assistant. This can be any of the options, but only certain commands will actually return states to HA. So we recommend just using Button for most commands.

Name: Enter the name for the entity in HA, this is the entity id so it must not contain spaces.

Example

Here is the config for a command that shuts down the computer.

Extra config optional¶

Add a "friendly name" that shows up in dashboards instead of the id, can have any normal characters.

Friendly Name of "Fully Shutdown"

Enable/Disable availability check, this tells HA whether to check if the computer is online before grabbing the last state. If you want your sensor to always display last state even if the computer has been off for a while turn this on.

Using your command¶

After clicking "Store and Activate" the commands should be immediately picked up by Home Assistant, you can find them in any place you would normally find them. Each command will be attached to the mqtt integration under the name of your computer. You can also search for it in the entities page found below.

You can use these entities any way you normally would, yaml or interface. Note that the entity id is the "Name" you set in the sensor config.

To test the command you can open the entities properties and click "PRESS" and it will trigger the command to run.

Further Reading¶

Quick Actions – Interact with homeassistant entities and scripts from your taskbar or a hotkey!

Sensors – Send data from your computer to homeassistant to create automations!

Notifications – Send notifications from homeassistant, including actions and images!

Media Player – Manage media on your computer and send text to speech!

Other Features – For a guide on all the features and config options in HASS.Agent.

Was this page helpful?

Thanks for your feedback!

Thanks for your feedback!

Back to top

Copyright © 2023-2026 HASS.Agent Team - Change cookie settings

Made with

Material for MkDocs

Cookie consent

We use cookies to fetch github information, recognize your repeated visits and preferences, as well as to measure the effectiveness of our documentation and whether users find what they're searching for. With your consent, you're helping us to make our documentation better.

Google Analytics

GitHub

Accept

Manage settings