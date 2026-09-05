---
title: 'Glances integration sometimes stops reporting current data · Issue #170384
  · home-assistant/core · GitHub'
id: glances-integration-sometimes-stops-reporting-current-data-issue-170384-home-ass
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- home-assistant
- availability
created: '2026-09-02T04:02:33.511412Z'
updated: '2026-09-02T17:37:21.951870Z'
source: https://github.com/home-assistant/core/issues/170384
source_domain: github.com
fetched_at: '2026-09-02T04:02:29.885816Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA core issue #170384 (opened May 12 2026 by michaelherger, core-2026.5.1,
  HA Container, still OPEN, labelled integration: glances): the Glances integration
  on a connected Linux host works fine for a while, then suddenly stops updating —
  flat-line charts — occasionally recovering on its own before stopping again. No
  obviously related HA log lines; no diagnostics available for the integration. Reporter''s
  historical data suggests it was working until May 1st/2nd and suspects the 2026.05
  release broke it. No assignee yet. Second independent confirmation (12 years of
  issue history apart from #110551, 2024→2026) that Glances polling silently flatlines
  without errors or self-recovery — the reliability gap that motivates replacing pull-based
  Glances with a purpose-built push agent for jupiterOS fleet monitoring.'
---

Glances integration sometimes stops reporting current data · Issue #170384 · home-assistant/core · GitHub

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

Glances integration sometimes stops reporting current data #170384

New issueCopy link

New issueCopy link

Open

Open

Glances integration sometimes stops reporting current data#170384

Copy link

Labels

integration: glances

Description

michaelherger
opened on May 12, 2026

Issue body actions

The problem

I've configured the Glances integration to report system data from a connected Linux host. The integration usually works fine for a while, then suddenly stops to update. I'd get flat line charts:

Every now and then it would go back to work, just to stop after a while again.

I haven't found any obviously related line in the HA log file.

What version of Home Assistant Core has the issue?

core-2026.5.1

What was the last working version of Home Assistant Core?

No response

What type of installation are you running?

Home Assistant Container

Integration causing the issue

Glances

Link to integration documentation on our website

https://www.home-assistant.io/integrations/glances/

Diagnostics information

Not available for this integration?

Example YAML snippet

Anything in the logs that might be useful for us?

Additional information

I don't have the version with which this broke. But looking at the historical data it seems that it was working fine up until May 1st/2nd (see above screenshot). Earlier data is looking good. I often do update timely. So it might have been the 2026.05 release which broke this?
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

integration: glances

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