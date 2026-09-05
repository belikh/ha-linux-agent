---
title: '[FEATURE REQUEST] Smart notifications with action buttons · Issue #667 · joshuar/go-hass-agent
  · GitHub'
id: feature-request-smart-notifications-with-action-buttons-issue-667-joshuargo-hass
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-adopt-vs-build-honest-verdict
- adopt-vs-build
created: '2026-09-02T12:04:18.864783Z'
updated: '2026-09-05T10:51:22.231647Z'
source: https://github.com/joshuar/go-hass-agent/issues/667
source_domain: github.com
fetched_at: '2026-09-02T12:04:18.863413Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'OPEN feature request (labelled enhancement, no assignee, no maintainer reply,
  opened by flaper87 on Dec 2, 2025 against go-hass-agent v14.2.0 on Arch Linux):
  user set up a notification that ''would require action... be able to confirm/deny
  the notification so HA will take actions based on that'', but the Confirm/Dismiss
  action buttons defined in the HA ''Confirm Notification'' script template do not
  appear — ''We should see the buttons to confirm / dismiss the notification just
  like for mobile devices''; confirms go-hass-agent cannot render actionable-notification
  buttons.'
---

[FEATURE REQUEST] Smart notifications with action buttons · Issue #667 · joshuar/go-hass-agent · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

joshuar

/

go-hass-agent

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
30

Star
575

[FEATURE REQUEST] Smart notifications with action buttons #667

New issueCopy link

New issueCopy link

Open

Open

[FEATURE REQUEST] Smart notifications with action buttons#667

Copy link

Labels

enhancementNew feature or requestNew feature or request

Description

flaper87
opened on Dec 2, 2025

Issue body actions

Go Hass Agent Version

Go Hass Agent: v14.2.0

Describe the bug

I just setup Go HASS Agent and I'm configuring some notifications that would require action. The goal is to be notified on my laptop and be able to confirm/deny the notification so HA will take actions based on that.

To Reproduce

Steps to reproduce the behavior:

Create a script using the Confirm Notification template

Add a Confirm / Dismiss action

Execute

Expected behavior

We should see the buttons to confirm / dismiss the notification just like for mobile devices

Screenshots

Logs

Nothing significant in the logs, no debug info about the message received.

Desktop (please complete the following information):

OS: Linux

Distribution Archlinux

Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

enhancementNew feature or requestNew feature or request

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.