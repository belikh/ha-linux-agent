---
title: 'All messages dropped when max_queued_messages 0 · Issue #2879 · eclipse-mosquitto/mosquitto
  · GitHub'
id: all-messages-dropped-when-max_queued_messages-0-issue-2879-eclipse-mosquittomosq
tags:
- linux-agent-jupiteros-fleet-15537b
- known-issue
- birth-message
- mosquitto
- ha-issue
- retained-messages
- gap-07
- broker-config
created: '2026-09-02T17:03:39.596526Z'
updated: '2026-09-05T10:51:22.433055Z'
source: https://github.com/eclipse-mosquitto/mosquitto/issues/2879
source_domain: github.com
fetched_at: '2026-09-02T17:03:33.307141Z'
fetch_provider: builtin
status: evergreen
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'mosquitto issue #2879 (Aug 2023, mosquitto 2.0.16): reproduces the drop
  mechanism with a literal broker log line. With max_queued_messages 0 + allow_anonymous,
  a plain mosquitto_sub/mosquitto_pub session logged ''Outgoing messages are being
  dropped for client auto-BBB3F4DC-...''. Demonstrates (a) the drop is silent from
  the subscriber''s perspective — no error reaches the client, only a broker-side
  NOTICE log; (b) a mosquitto.conf(5) doc quirk: the man page says max_queued_messages
  0 = ''no maximum'', but this user''s 2.0.16 dropped everything (the modern man page
  keeps ''0 = no maximum (not recommended)'' semantics; behaviour differed across
  versions — in 2.0.x, 0 combined with the QoS-0 path in db__ready_for_flight delivers-or-drops
  rather than queues). Independent of HA #135266: a second real-world witness that
  exceeding/zeroing the queue limit produces exactly the silent-drop signature, plus
  the log string to grep for.'
---

All messages dropped when max_queued_messages 0 · Issue #2879 · eclipse-mosquitto/mosquitto · GitHub

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

All messages dropped when max_queued_messages 0 #2879

New issueCopy link

New issueCopy link

Closed

Closed

All messages dropped when max_queued_messages 0#2879

Copy link

Description

Daedaluz
opened on Aug 21, 2023

Issue body actions

mosquitto.conf:

max_queued_messages 0
allow_anonymous true
listener 1883

start mosquitto:

$ docker run --rm -ti -p 1883:1883 -v $(PWD)/mosquitto.conf:/mosquitto/config/mosquitto.conf eclipse-mosquitto:2.0.16

subscribe:

$ mosquitto_sub -v -t "#"

publish:

$ mosquitto_pub -t "hello" -m "world"

Mosquitto log:

1692604552: mosquitto version 2.0.16 starting
1692604552: Config loaded from /mosquitto/config/mosquitto.conf.
1692604552: Opening ipv4 listen socket on port 1883.
1692604552: Opening ipv6 listen socket on port 1883.
1692604552: mosquitto version 2.0.16 running
1692604552: New connection from 172.17.0.1:33904 on port 1883.
1692604552: New client connected from 172.17.0.1:33904 as auto-BBB3F4DC-A2B8-A350-CAE6-B3B99F9DAC9B (p2, c1, k60).
1692604552: Outgoing messages are being dropped for client auto-BBB3F4DC-A2B8-A350-CAE6-B3B99F9DAC9B.
1692604553: New connection from 172.17.0.1:33906 on port 1883.
1692604553: New client connected from 172.17.0.1:33906 as auto-7B07F536-BDFB-4E81-F8FB-6D973493172F (p2, c1, k60).
1692604553: Outgoing messages are being dropped for client auto-7B07F536-BDFB-4E81-F8FB-6D973493172F.

nothing on the subscriber and the pub client hangs and eventually terminates with Error: Unknown error.
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

LabelsNo labels

No labels

TypeNo type

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.
## Related

- [[databasec]]
- [[mosquittoconf-man-page-eclipse-mosquitto]]
