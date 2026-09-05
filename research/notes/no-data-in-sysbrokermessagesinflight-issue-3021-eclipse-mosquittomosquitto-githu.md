---
title: 'No data in $SYS/broker/messages/inflight · Issue #3021 · eclipse-mosquitto/mosquitto
  · GitHub'
id: no-data-in-sysbrokermessagesinflight-issue-3021-eclipse-mosquittomosquitto-githu
tags:
- linux-agent-jupiteros-fleet-15537b
- known-issue
- source-code
- birth-message
- mosquitto
- ha-issue
created: '2026-09-02T17:03:39.601459Z'
updated: '2026-09-05T10:51:22.415539Z'
source: https://github.com/eclipse-mosquitto/mosquitto/issues/3021
source_domain: github.com
fetched_at: '2026-09-02T17:03:34.361489Z'
fetch_provider: builtin
status: evergreen
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'No data in $SYS/broker/messages/inflight · Issue #3021 · eclipse-mosquitto/mosquitto
  · GitHub'
---

No data in $SYS/broker/messages/inflight · Issue #3021 · eclipse-mosquitto/mosquitto · GitHub

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

No data in $SYS/broker/messages/inflight #3021

New issueCopy link

New issueCopy link

Closed

Closed

No data in $SYS/broker/messages/inflight#3021

Copy link

Description

fschlager
opened on Mar 19, 2024

Issue body actions

I'm running Mosquitto v2.0.15 in Kubernetes on Linux.

The documentation states that the following statistic is available:

$SYS/broker/messages/inflight

The number of messages with QoS>0 that are awaiting acknowledgments.

Yet when I try to read data from it it always comes back empty. Also a quick skim through the code gave no indication that this topic is ever being written to.

I'm trying to monitor this value to detect possible bottlenecks in our setup.

Am I missing something?
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

- [[mosquitto8-mosquitto-debian-unstable-debian-manpages]]
- [[databasec]]
- [[mosquittoconf-man-page-eclipse-mosquitto]]
