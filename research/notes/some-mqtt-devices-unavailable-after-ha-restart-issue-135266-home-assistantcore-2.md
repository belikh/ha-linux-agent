---
title: 'Some MQTT devices unavailable after HA restart · Issue #135266 · home-assistant/core
  · GitHub'
id: some-mqtt-devices-unavailable-after-ha-restart-issue-135266-home-assistantcore-2
tags:
- linux-agent-jupiteros-fleet-15537b
created: '2026-09-02T04:27:55.686519Z'
updated: '2026-09-02T17:37:21.714570Z'
source: https://github.com/home-assistant/core/issues/135266#issuecomment
source_domain: github.com
fetched_at: '2026-09-02T04:27:55.684960Z'
fetch_provider: builtin
status: deprecated
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: Suggested by some-mqtt-devices-unavailable-after-ha-restart-issue-135266-home-assistantcore-g
  — need the maintainer r...
---

*Suggested by [[some-mqtt-devices-unavailable-after-ha-restart-issue-135266-home-assistantcore-g]] — need the maintainer resolution comments not captured in the issue body fetch*

Some MQTT devices unavailable after HA restart · Issue #135266 · home-assistant/core · GitHub

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

home-assistant

/

core

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
38.5k

Star
90.2k

Some MQTT devices unavailable after HA restart #135266

New issueCopy link

New issueCopy link

Closed

Closed

Some MQTT devices unavailable after HA restart#135266

Copy link

Assignees

Labels

integration: mqtt

Description

richardstone
opened on Jan 10, 2025

Issue body actions

The problem

After a Home Assistant restart, most of the MQTT devices connected through Z-Wave JS UI became available, but a couple of the very same type of device - Aeotec Home Energy Meter - Gen5, are still displayed as unavailable.

Within the Z-Wave JS UI MQTT settings I have retain enabled, and I have turned on retained discovery as well.

I have the discovery prefix set to a different topic than the MQTT prefix.

Connecting to the MQTT broker with MQTT explorer, I can see all the retained discovery messages under the discovery prefix, and I can see new messages arriving on the usual MQTT prefix, containing latest data from the Aeotec Home Energy Meters. With this, I can't see any issues with either Z-Wave JS UI or the MQTT broker I'm using.

In Home Assistant, after a restart, I see these devices discovered but as "Unavailable". At this point I can resolve the issue via various ways:

Restart the Z-Wave JS UI instance.

Delete the device from Home Assistant, then rediscovery the device manually in Z-Wave JS UI.

In Home Assistant MQTT settings, initiate a listen on the device topic, then the device becames available instantly

All three options are very inconvenient to do after every Home Assistant restart.

I have 115 MQTT devices under a single Home Assistant instance, resulting in 5374 entities in total.

Interesting thing is that after every HA restart, almost always the very same 8-10 Aeotec Home Energy Meters are the only devices that are unavailable amongst other devices like switches, climates, shutter controllers.

Could you please help me to further debug/resolve this issue?

What version of Home Assistant Core has the issue?

2024.8.2

What was the last working version of Home Assistant Core?

No response

What type of installation are you running?

Home Assistant Container

Integration causing the issue

mqtt

Link to integration documentation on our website

https://www.home-assistant.io/integrations/mqtt/

Diagnostics information

No response

Example YAML snippet

No response

Anything in the logs that might be useful for us?

No response

Additional information

No response
Reactions are currently unavailable

Metadata
Metadata

Assignees

bdraco

emontnemery

jbouwh

Labels

integration: mqtt

TypeNo type

Fields

Priority
None yet

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.