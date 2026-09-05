---
title: Using Sensors - HASS.Agent Documentation
id: using-sensors-hassagent-documentation
tags:
- linux-agent-jupiteros-fleet-15537b
- hass-agent
- official-docs
- mqtt-discovery
- availability-semantics
created: '2026-09-02T04:18:30.117368Z'
updated: '2026-09-05T10:51:21.794115Z'
source: https://www.hass-agent.io/2.2/getting-started/sensors/
source_domain: www.hass-agent.io
fetched_at: '2026-09-02T04:18:30.116145Z'
fetch_provider: builtin
status: evergreen
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'HASS.Agent docs ''Using Sensors'' page (v2.2 docs; the /latest/getting-started/sensors/
  URL 302s to this). Windows-oriented but the sensor-configuration UX contract is
  transferable to a Linux agent: per-sensor TYPE + Name (entity_id, no spaces) + Update
  Interval (with a recommended default), optional friendly_name, and an AVAILABILITY
  CHECK toggle controlling whether HA treats the PC as online before showing last
  state (i.e. how HA should handle offline hosts — the same availability semantics
  a fleet-wide jupiterOS agent must decide per host). Sensors attach to the MQTT integration
  under the computer''s name; entity_id equals the configured Name. Docs also confirm
  the feature set surface: Quick Actions, Commands, Notifications with actions/images,
  Media Player, Webviews.'
---

Using Sensors - HASS.Agent Documentation

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

Using your sensor

Further Reading

Using Commands

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

Using your sensor

Further Reading

Getting Started with Sensors¶

One of the core features of HASS.Agent is the ability to send data about your pc to home asssistant for use in automations and dashboards. To get started with sensors you will want to open the "Sensors" tab of HASS.Agent.

Creating your first sensor¶

Required properties¶

TYPE: Select the type of sensor from the left, there are many options available, a full description is available in the setup guide.

Name: Enter the name for the entity in HA, this is the entity id so it must not contain spaces.

Update Interval: Sets the interval for how often the sensor updates, the recommended value is usually fine.

Example

Here is the config for a sensor that monitors the user state.

Extra config optional¶

Friendly Name: Add a "friendly name" that shows up in dashboards instead of the id, can have any normal characters.

Friendly Name of "Gpu Temp"

Availability check: Enable/Disable availability check, this tells HA whether to check if the computer is online before grabbing the last state. If you want your sensor to always display last state even if the computer has been off for a while turn this on.

Using your sensor¶

After clicking "Store and Activate" the sensors should be immediately picked up by Home Assistant, you can find them in any place you would normally find them. Each sensor/command will be attached to the mqtt integration under the name of your computer. You can also search for it in the entities page found below.

You can use these entities any way you normally would, yaml or interface. Note that the entity id is the "Name" you set in the sensor config.

Further Reading¶

Quick Actions – Interact with homeassistant entities and scripts from your taskbar or a hotkey!

Commands – Create commands homeassistant can run to do things on your computer!

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