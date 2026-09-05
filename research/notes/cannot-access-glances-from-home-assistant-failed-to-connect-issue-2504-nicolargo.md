---
title: 'Cannot access glances from Home Assistant.  "Failed to connect" · Issue #2504
  · nicolargo/glances · GitHub'
id: cannot-access-glances-from-home-assistant-failed-to-connect-issue-2504-nicolargo
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- home-assistant
- availability
created: '2026-09-02T04:02:33.526524Z'
updated: '2026-09-02T17:37:21.956458Z'
source: https://github.com/nicolargo/glances/issues/2504
source_domain: github.com
fetched_at: '2026-09-02T04:02:32.528313Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'Glances upstream issue #2504 (bert269, Jul 20 2023, closed): Glances API
  v3 on Ubuntu 22.04 (python3) serves fine in a browser and via direct API calls (http://IP:61208/api/3/mem/free
  returns data), but the HA Glances integration fails with ''Failed to Connect'' regardless
  of whether username/password fields are used. No labels, no assignee, no resolution
  recorded in the issue body. Fourth distinct Glances/HA integration failure mode:
  auth/connection handshake mismatches between the integration and the Glances webserver.
  Supports the research position that the Glances REST bridge is fragile at fleet
  scale — each host needs a correctly-configured authenticated webserver plus a matching
  HA config entry, with opaque failure modes when they drift.'
---

Cannot access glances from Home Assistant.  "Failed to connect" · Issue #2504 · nicolargo/glances · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

nicolargo

/

glances

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
1.8k

Star
33.5k

Cannot access glances from Home Assistant.  "Failed to connect" #2504

New issueCopy link

New issueCopy link

Closed

Closed

Cannot access glances from Home Assistant.  "Failed to connect"#2504

Copy link

Description

bert269
opened on Jul 20, 2023

Issue body actions

Installed Glances on Ubuntu 22.04 with python3.

Starting up fine:

When I access it from another Windows browser (http://192.168.1.210:61208) - I get the Glances display.

When I try to access data via the API: (http://192.168.1.210:61208/api/3/mem/free) I get a response back:

But then I try to install the HA integration and try to connect with the userid on the Ubuntu server, it fails with "Failed to Connect".

What is missing? Do I need to start the web-server with a userid/password? I also tried to setup the HA integration without the user and password field, but it still fails the same.
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

LabelsNo labels

No labels

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.