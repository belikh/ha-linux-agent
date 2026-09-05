---
title: 'Documentation on $SYS/broker/mqtt/# topics is wrong · Issue #3726 · eclipse-mosquitto/mosquitto
  · GitHub'
id: documentation-on-sysbrokermqtt-topics-is-wrong-issue-3726-eclipse-mosquittomosqu
tags:
- linux-agent-jupiteros-fleet-15537b
- mosquitto
- known-issue
- birth-message
- ha-issue
- failure-notifications
- gap-07
- broker-config
created: '2026-09-02T17:05:55.173700Z'
updated: '2026-09-02T17:39:27.553273Z'
source: https://github.com/eclipse-mosquitto/mosquitto/issues/3726
source_domain: github.com
fetched_at: '2026-09-02T17:05:53.982732Z'
fetch_provider: builtin
status: review
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'mosquitto issue #3726 (Aug 25 2026, open, v2.1.2): the  documentation defect
  the D9 -check recommendation must respect. The mosquitto(8) man page lists topics
  like /broker/mqtt/connect/received under Broker Status that are never published
  — sys_tree.c excludes them explicitly (''Not published in , may be made available
  for plugins''). Practical consequence: an agent sensor template that expects /broker/mqtt/*
  topics will read nothing; the load-bearing monitoring topics are /broker/publish/messages/dropped,
  /broker/load/publish/dropped/+ and the store-vs-retailed pair. Filed against 2.1.2,
  still open.'
---

Documentation on $SYS/broker/mqtt/# topics is wrong · Issue #3726 · eclipse-mosquitto/mosquitto · GitHub

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

Documentation on $SYS/broker/mqtt/# topics is wrong #3726

New issueCopy link

New issueCopy link

Open
#3731

Open

Documentation on $SYS/broker/mqtt/# topics is wrong#3726

#3731

Copy link

Labels

Status: AvailableNo one has claimed responsibility for resolving this issue.No one has claimed responsibility for resolving this issue.

Description

mri-knx
opened on Aug 25, 2026

Issue body actions

(AI-free text)

https://mosquitto.org/man/mosquitto-8.html lists various topics like $SYS/broker/mqtt/connect/received in the section "Broker Status". These are never published, as of version 2.1.2.

In sys_tree.c, these topics are excluded explicitly from being published. The associated commit states

Not published in $SYS, may be made available for plugins.

So, the documentation is incorrect. I suggest to remove the unavailable topics.
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

Status: AvailableNo one has claimed responsibility for resolving this issue.No one has claimed responsibility for resolving this issue.

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
