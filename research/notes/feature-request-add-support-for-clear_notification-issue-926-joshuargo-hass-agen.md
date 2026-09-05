---
title: '[FEATURE REQUEST] Add support for `clear_notification` · Issue #926 · joshuar/go-hass-agent
  · GitHub'
id: feature-request-add-support-for-clear_notification-issue-926-joshuargo-hass-agen
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-adopt-vs-build-honest-verdict
- adopt-vs-build
created: '2026-09-02T12:04:22.035380Z'
updated: '2026-09-02T17:37:22.499120Z'
source: https://github.com/joshuar/go-hass-agent/issues/926
source_domain: github.com
fetched_at: '2026-09-02T12:04:22.034141Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'OPEN feature request (labelled enhancement, no assignee, no maintainer reply,
  opened by LinqLover on Aug 2, 2026): on go-hass-agent, the mobile_app clear_notification
  command ''simply sends a second notification instead'' of clearing an existing one,
  whereas it ''works nice using clear_notification'' on the user''s phone; requests
  parity with companion apps per https://companion.home-assistant.io/docs/notifications/notifications-basic#clearing
  — confirms go-hass-agent lacks clear_notification support as of Aug 2026.'
---

[FEATURE REQUEST] Add support for `clear_notification` · Issue #926 · joshuar/go-hass-agent · GitHub

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

[FEATURE REQUEST] Add support for clear_notification #926

New issueCopy link

New issueCopy link

Open

Open

[FEATURE REQUEST] Add support for clear_notification#926

Copy link

Labels

enhancementNew feature or requestNew feature or request

Description

LinqLover
opened on Aug 2, 2026

Issue body actions

I have an automation that sends a notification to my laptop and wants to clear it after an hour. On my phone this works nice using clear_notification, but using go-hass-agent, clear_notification simply sends a second notification instead.

It would be nice if clear_notification would work just like on other companion apps. See: https://companion.home-assistant.io/docs/notifications/notifications-basic#clearing
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