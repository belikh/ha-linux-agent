---
title: 'No keepalive (0) still allowed with max_keepalive set · Issue #2117 · eclipse-mosquitto/mosquitto
  · GitHub'
id: no-keepalive-0-still-allowed-with-max_keepalive-set-issue-2117-eclipse-mosquitto
tags:
- linux-agent-jupiteros-fleet-15537b
- reliability-failure-modes
- mqtt
created: '2026-09-02T05:06:16.296347Z'
updated: '2026-09-05T10:51:21.872831Z'
source: https://github.com/eclipse-mosquitto/mosquitto/issues/2117
source_domain: github.com
fetched_at: '2026-09-02T05:06:16.294917Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'eclipse-mosquitto/mosquitto issue #2117 (''No keepalive (0) still allowed
  with max_keepalive set'', opened 2021-03-07 by dnadlinger, CLOSED, milestone 2.0.9):
  the root-cause issue both Telegraf MQTT-output sources cite for their mandatory
  non-zero keep_alive. Report: even with max_keepalive set in mosquitto''s config,
  clients could still CONNECT with keepalive=0 and no Server Keep Alive was sent in
  the connack — unexpected behaviour vs the MQTT v5 spec and mosquitto docs. Fixed
  in mosquitto 2.0.9 per the milestone. Load-bearing detail for ha-linux-agent''s
  MQTT client design: (1) keepalive=0 (the rumqttc default!) is unsafe against mosquitto
  ≥2.0 — brokers may reject or mishandle it, so the agent must always negotiate an
  explicit keep-alive interval; (2) the citation target Telegraf''s docs link (github.com/eclipse/mosquitto/issues/2117)
  now 404s because the org was renamed to eclipse-mosquitto — showing how stale upstream
  links erode even official documentation''s provenance chain.'
---

*Suggested by [[telegraf-documentation-2]] — root-cause issue for the keep_alive requirement cited by both Telegraf MQTT sources; eclipse/mosquitto 404s, trying the eclipse-mosquitto org rename*

No keepalive (0) still allowed with max_keepalive set · Issue #2117 · eclipse-mosquitto/mosquitto · GitHub

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

eclipse-mosquitto

/

mosquitto

Public

Notifications
You must be signed in to change notification settings

Fork
2.6k

Star
11.2k

No keepalive (0) still allowed with max_keepalive set #2117

New issueCopy link

New issueCopy link

Closed

Closed

No keepalive (0) still allowed with max_keepalive set#2117

Copy link

Milestone

2.0.9

Description

dnadlinger
opened on Mar 7, 2021

Issue body actions

Even if max_keepalive is set in the config file, clients are still allowed to connect using 0 (i.e. no keepalive) without a Server Keep Alive being sent in the connect ack.

I'm not sure whether this is a bug, but it was certainly unexpected behaviour to me after reading the v5 spec and mosquitto documentation.
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

LabelsNo labels

No labels

TypeNo type

ProjectsNo projects

Milestone

2.0.9

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.