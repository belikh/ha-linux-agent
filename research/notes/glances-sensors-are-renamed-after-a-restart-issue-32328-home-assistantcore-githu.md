---
title: 'Glances sensors are renamed after a restart · Issue #32328 · home-assistant/core
  · GitHub'
id: glances-sensors-are-renamed-after-a-restart-issue-32328-home-assistantcore-githu
tags:
- linux-agent-jupiteros-fleet-15537b
- home-assistant
- availability
created: '2026-09-02T04:02:33.516722Z'
updated: '2026-09-05T10:51:21.628987Z'
source: https://github.com/home-assistant/core/issues/32328
source_domain: github.com
fetched_at: '2026-09-02T04:02:30.773396Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA core issue #32328 (opened Feb 28 2020 by DecentM, HA 0.106.0, Docker,
  closed with assignee fabaff, labelled ''stale''): after a restart, some Glances
  sensors get renamed with a ''_2'' suffix (e.g. machine_etc_hostname_used_percent
  → machine_etc_hostname_used_percent_2), making cards for containers_*, cpu_*, ram_*,
  swap_*, thread and total sensors disappear. Fix-at-the-time: remove and re-add the
  integration via the web UI — but the renaming recurs on the next restart. Root-cause
  class: entity-id churn from a REST polling integration whose unique-id handling
  changed in 0.106. Demonstrates a third distinct Glances failure mode — sensor identity
  instability across restarts — which a discovery-based agent with stable unique_ids
  per host metric avoids.'
---

Glances sensors are renamed after a restart · Issue #32328 · home-assistant/core · GitHub

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

Glances sensors are renamed after a restart #32328

New issueCopy link

New issueCopy link

Closed

Closed

Glances sensors are renamed after a restart#32328

Copy link

Assignees

Labels

integration: glancesstale

Description

DecentM
opened on Feb 28, 2020

Issue body actions

The problem

I set up monitoring for a bunch of systems with Home Assistant's Glances integration. This worked really well until I updated to 0.106, which introduced some changes to the integration. Now, when I restart HA, some sensors are renamed with a _2 after their name. This has happened with all sensors, but not all at once.

For example - first, the machine_etc_hostname_used_percent sensor was renamed to machine_etc_hostname_used_percent_2. Then today, I noticed that my cards for containers_*, cpu_*, ram_*, swap_*, thread and total disappeared because of this renaming.

This renaming is undone if I remove then re-add the integration using the web UI until the next restart. Then it happens again.

Environment

Home Assistant release with the issue: 0.106.0

Last working Home Assistant release (if known): 0.105.5

Operating environment (Hass.io/Docker/Windows/etc.): Docker

Integration causing this issue: Glances

Link to integration documentation on our website: https://www.home-assistant.io/integrations/glances/

Problem-relevant configuration.yaml

# This integration is configured through the web UI

Traceback/Error logs

Additional information
Reactions are currently unavailable

Metadata
Metadata

Assignees

fabaff

Labels

integration: glancesstale

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