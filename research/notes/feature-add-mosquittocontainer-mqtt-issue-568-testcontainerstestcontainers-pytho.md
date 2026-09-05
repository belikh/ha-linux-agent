---
title: 'Feature: add MosquittoContainer (mqtt) · Issue #568 · testcontainers/testcontainers-python
  · GitHub'
id: feature-add-mosquittocontainer-mqtt-issue-568-testcontainerstestcontainers-pytho
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- native-app-integration
- known-issue
- mqtt-discovery
- retained-messages
- testing
created: '2026-09-02T05:39:32.173413Z'
updated: '2026-09-02T17:37:22.236042Z'
source: https://github.com/testcontainers/testcontainers-python/issues/568
source_domain: github.com
fetched_at: '2026-09-02T05:39:29.973574Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'testcontainers-python issue #568 (f18m, opened 10 May 2024, closed): proposal
  to add a MosquittoContainer class to testcontainers-python — ''I''m writing integration
  tests for a Python project that publishes messages to an MQTT broker... there is
  no specialization available for MQTT and for its most-used broker which is Mosquitto.''
  Labelled enhancement + contributions-welcome; the requester offered a PR (his project
  ha-alarm-raspy2mqtt publishes to MQTT). Evidence that Mosquitto test-container support
  is recent and thin across ecosystems: Python only gained it via community contribution,
  Rust''s testcontainers-modules mosquitto feature is likewise community-maintained
  — so ha-linux-agent''s Rust tests must pin an explicit eclipse-mosquitto image tag
  rather than rely on a rich maintained harness.'
---

Feature: add MosquittoContainer (mqtt) · Issue #568 · testcontainers/testcontainers-python · GitHub

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

testcontainers

/

testcontainers-python

Public

Notifications
You must be signed in to change notification settings

Fork
381

Star
2.3k

Feature: add MosquittoContainer (mqtt) #568

New issueCopy link

New issueCopy link

Closed

Closed

Feature: add MosquittoContainer (mqtt)#568

Copy link

Labels

🚀 enhancement🤝 contributions are welcome

Description

f18m
opened on May 10, 2024

Issue body actions

What are you trying to do?

I'm writing integration tests for a Python project (https://github.com/f18m/ha-alarm-raspy2mqtt) that publishes messages to an MQTT broker.

I would like to use testcontainers to create the integration tests and noticed that there is no specialization available for MQTT and for its most-used broker which is Mosquitto.

So I'd like to propose the addition of a MosquittoContainer class to testcontainers-python.

Why should it be done this way?

There are a number of helpers that could be provided out of the box to ease integration testing of MQTT applications.

I can provide a PR to add my own MosquittoContainer implementation if there is interest by the project maintainers...
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

🚀 enhancement🤝 contributions are welcome

TypeNo type

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.