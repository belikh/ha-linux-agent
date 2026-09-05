---
title: 'Glances integration loses connection and doesn''t automatically recover ·
  Issue #110551 · home-assistant/core · GitHub'
id: glances-integration-loses-connection-and-doesnt-automatically-recover-issue-1105
tags:
- linux-agent-jupiteros-fleet-15537b
- home-assistant
- availability
created: '2026-09-02T04:02:33.506979Z'
updated: '2026-09-02T17:37:21.947871Z'
source: https://github.com/home-assistant/core/issues/110551
source_domain: github.com
fetched_at: '2026-09-02T04:02:29.014560Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA core issue #110551 (opened Feb 14 2024 by spikeygg, core-2024.2.1, HAOS,
  closed as NOT PLANNED, labelled ''stale''): the Glances integration on multiple
  machines randomly stops receiving data — last datapoint recorded then flatline ''static''
  — with zero related log entries, recurring for months. Workaround: manually hit
  ''reload'' on the integration in the HA UI, which restores data flow for several
  days until it silently dies again. No assignee, no fix, closed not-planned. Evidence
  for the research thesis that HA''s Glances polling integration is unreliable for
  fleet monitoring — silent failure with no self-recovery and no diagnostics, exactly
  the class of problem a purpose-built agent with MQTT LWT/availability + push telemetry
  avoids.'
---

Glances integration loses connection and doesn't automatically recover · Issue #110551 · home-assistant/core · GitHub

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

Glances integration loses connection and doesn't automatically recover #110551

New issueCopy link

New issueCopy link

Closed as not planned

Closed as not planned

Glances integration loses connection and doesn't automatically recover#110551

Copy link

Labels

integration: glancesstale

Description

spikeygg
opened on Feb 14, 2024

Issue body actions

The problem

I have glances on a few machines in my house. There are one or two of them exhibiting this problem where the integration just (seemingly randomly) stops receiving the data. You can see in this plot that it happened this morning around midnight on the Plexbox:

it recorded the last datapoint at 12:51:03 and then static.

I've been watching this happen for a few months. The solution: I go into the integration and just hit the 'reload' on this machine and it starts working again for several days until it happens again.

I checked the logs for detail around this period and there are no entries for Glances.

What version of Home Assistant Core has the issue?

core-2024.2.1

What was the last working version of Home Assistant Core?

No response

What type of installation are you running?

Home Assistant OS

Integration causing the issue

Glances

Link to integration documentation on our website

https://www.home-assistant.io/integrations/glances

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

AssigneesNo one assigned

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