---
title: 'cmd/tailscale/cli: build a mechanism to change the output from `--json` logs
  in the future · Issue #17619 · tailscale/tailscale · GitHub'
id: cmdtailscalecli-build-a-mechanism-to-change-the-output-from-json-logs-in-the-fut
tags:
- linux-agent-jupiteros-fleet-15537b
- known-issue
- ha-issue
- native-app-integration
- birth-message
- version-ground-truth
created: '2026-09-02T05:38:56.253519Z'
updated: '2026-09-02T17:37:22.224107Z'
source: https://github.com/tailscale/tailscale/issues/17619
source_domain: github.com
fetched_at: '2026-09-02T05:38:51.698387Z'
fetch_provider: builtin
status: review
type: note
tier: institutional
content_type: docs
deprecated: false
summary: 'Tailscale GitHub issue #17619 (opened Oct 23 2025 by alexwlchan, closed):
  because --json is a boolean flag, every CLI command''s JSON output is frozen — ''we''re
  stuck with whatever JSON output we originally decided, and we can''t change it or
  we''ll break downstream clients''. Proposal: make --json take a version argument
  (--json=4 / --json=N), where --json alone always yields v1; passing a non-existent
  N is an error; strictly additive changes stay allowed, removal or semantic changes
  to existing fields require a version bump. Motivation was stabilising lock-log JSON
  for #17613. Open design questions recorded: whether to embed ''json_format'': ''1''
  in the output and whether to warn on old versions. Directly relevant to ha-linux-agent
  reliability: any agent parsing ''tailscale status --json'' today depends on an explicitly
  unstable, frozen-by-compatibility-promise format — the official docs themselves
  mark it ''WARNING: format subject to change''.'
---

cmd/tailscale/cli: build a mechanism to change the output from `--json` logs in the future · Issue #17619 · tailscale/tailscale · GitHub

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

tailscale

/

tailscale

Public

Notifications
You must be signed in to change notification settings

Fork
3.2k

Star
36k

cmd/tailscale/cli: build a mechanism to change the output from --json logs in the future #17619

New issueCopy link

New issueCopy link

Closed

Closed

cmd/tailscale/cli: build a mechanism to change the output from --json logs in the future#17619

Copy link

Description

alexwlchan
opened on Oct 23, 2025

Issue body actions

The problem

We have CLI commands that print their output in JSON if you use --json (e.g. tailscale status, tailscale lock status).

Because --json is a boolean flag, this means we're stuck with whatever JSON output we originally decided, and we can't change it or we'll break downstream clients. This is annoying!

The ideal solution

The --json flag takes an argument, e.g. pass --json=4 to get the 4th version of the JSON output.

If we ever want to make a breaking change to the output, we can bump the version number and create a new output, but clients using the old output will continue to work as-is. (We can discuss exactly what "breaking" means; I imagine it means removing or changing the output of an existing fields. Strictly additive changes are fine.)

Next steps

I want to stabilise some JSON output for #17613, but I don't want to make this problem worse.

Here's my proposal: we introduce --json=N for Tailnet Lock with a v1 of the lock log JSON, with the following behaviour:

--json=N = you get version N of the output. Passing a non-existent N is an error.

--json = you get version 1 of the output, whatever it looked like when we started numbering the outputs.

This means existing scripts will work as-is, because we aren't changing the data they receive.

Other ideas:

Do we include the version number in the output? (e.g. "json_format": "1")

Do we print a warning when you're getting an older JSON version?

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