---
title: 'MQTT sensors unavailable in 0.114.0bx, others stay unknown · Issue #38661
  · home-assistant/core · GitHub'
id: mqtt-sensors-unavailable-in-01140bx-others-stay-unknown-issue-38661-home-assista
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- mqtt-discovery
- discovery
- native-app-integration
- ha-issue
- known-issue
- availability-semantics
created: '2026-09-02T06:41:31.087053Z'
updated: '2026-09-02T17:37:22.318516Z'
source: https://github.com/home-assistant/core/issues/38661
source_domain: github.com
fetched_at: '2026-09-02T06:41:30.302990Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA core GitHub issue #38661 (Aug 2020, francisp2, closed, milestone 0.114.0,
  assignee emontnemery): after HA restart, ''A lot of my MQTT sensors are unavailable
  after a restart. The solution to get them back is to delete the entity, and resend
  the discovery message.'' Example discovery payload given (binary_sensor voordeur
  with device_class door, state_topic sensor/voordeur, unique_id, device identifiers).
  Broker was on a separate machine; Tasmota devices stayed ''unknown''. Historical
  evidence that HA-side restart recovery for MQTT entities depended on the device
  resending discovery — the era before retained-discovery handling matured. Confirms
  the long-standing design contract: discovery state is the device''s responsibility
  to (re)publish; HA does not cache entity config across restarts on its own. Also
  relevant as precedent for the mqtt-entities-become-unknown-on-reboot failure class.
  Provenance: official HA core issue tracker, verbatim description; resolution details
  not in the fetched page (closed with #38876 cross-reference, no fix narrative captured).'
---

MQTT sensors unavailable in 0.114.0bx, others stay unknown · Issue #38661 · home-assistant/core · GitHub

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

MQTT sensors unavailable in 0.114.0bx, others stay unknown #38661

New issueCopy link

New issueCopy link

Closed
#38876

Closed

MQTT sensors unavailable in 0.114.0bx, others stay unknown#38661

#38876

Copy link

Assignees

Labels

integration: mqtt

Milestone

0.114.0

Description

francisp2
opened on Aug 8, 2020

Issue body actions

The problem

A lot of my MQTT sensors are unavailable after a restart. The solution to get them back is to delete the entity, and resend the discovery message.

Environment

Home Assistant Core release with the issue:  0.114.0b0 and 0.114.0b2

Last working Home Assistant Core release (if known): 0.113.3

Operating environment (OS/Container/Supervised/Core): Supervised

Integration causing this issue:  MQTT

Link to integration documentation on our website: https://www.home-assistant.io/integrations/mqtt/

Problem-relevant configuration.yaml

No yaml, but an example of a discovery message :

topic :

homeassistant/binary_sensor/sensorvoordeur/config

Discovery message :

{"name": "voordeur", "device_class": "door", "state_topic": "sensor/voordeur", "unique_id": "voordeur", "device": {"identifiers": ["voordeur"],"name": "voordeur", "model": "Digoo door sensor", "manufacturer": "Digoo"}}

Many Tasmota devices stay 'unknown'

Traceback/Error logs

Nothing relevant in the log

Additional information

Both point to the same mqtt broker (on a separate machine)

0.114.0b2

https://img.techpowerup.org/200808/01140b2.png

[IMG]https://img.techpowerup.org/200808/01140b2.png[/IMG]

0.113.3

https://img.techpowerup.org/200808/01133.png

[IMG]https://img.techpowerup.org/200808/01133.png[/IMG]
Reactions are currently unavailable

Metadata
Metadata

Assignees

emontnemery

Labels

integration: mqtt

TypeNo type

Fields

Priority
None yet

ProjectsNo projects

Milestone

0.114.0

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.