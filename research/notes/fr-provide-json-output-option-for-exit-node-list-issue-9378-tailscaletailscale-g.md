---
title: 'FR: Provide JSON output option for ''exit-node list'' · Issue #9378 · tailscale/tailscale
  · GitHub'
id: fr-provide-json-output-option-for-exit-node-list-issue-9378-tailscaletailscale-g
tags:
- linux-agent-jupiteros-fleet-15537b
- known-issue
- node-exporter
- ha-issue
- native-app-integration
- birth-message
created: '2026-09-02T05:38:56.244963Z'
updated: '2026-09-02T17:37:22.207062Z'
source: https://github.com/tailscale/tailscale/issues/9378
source_domain: github.com
fetched_at: '2026-09-02T05:38:49.278540Z'
fetch_provider: builtin
status: review
type: note
tier: institutional
content_type: docs
deprecated: false
summary: 'Tailscale GitHub issue #9378 (opened Sep 13 2023 by geoffeg, closed): feature
  request for JSON output on ''tailscale exit-node list'', which at the time (v1.48.1)
  only printed column-based human output, forcing SwiftBar plugin authors to write
  regex parsers. Labels: T0 New feature, P1 Nuisance, L2 FewLikelihood, mullvad. Issue
  is now closed, implying exit-node list gained structured/JSON output in later versions
  (confirmed indirectly by the CLI reference page documenting ''tailscale exit-node
  list --filter=<country>'' alongside ''suggest''). Relevant to any Linux agent consuming
  exit-node state: parse via JSON where available rather than column text.'
---

FR: Provide JSON output option for 'exit-node list' · Issue #9378 · tailscale/tailscale · GitHub

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

FR: Provide JSON output option for 'exit-node list' #9378

New issueCopy link

New issueCopy link

Closed

Closed

FR: Provide JSON output option for 'exit-node list'#9378

Copy link

Labels

L2 FewLikelihoodLikelihoodP1 NuisancePriority levelPriority levelT0 New featureIssue typeIssue typefrFeature requestFeature requestmullvad

Description

geoffeg
opened on Sep 13, 2023

Issue body actions

What are you trying to do?

I'm trying to write a SwiftBar plugin to display the hostname of the exit node I'm using, as well as a list of available exit nodes when I select the menubar item. 'exit-node list' does not provide an option for JSON output like some other tailscale subcommands do.

How should we solve this?

Provide a JSON output option for tailscale exit-node list

What is the impact of not solving this?

Writing a regex to parse the column-based output of exit-node list

Anything else?

Currently using tailscale 1.48.1
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

L2 FewLikelihoodLikelihoodP1 NuisancePriority levelPriority levelT0 New featureIssue typeIssue typefrFeature requestFeature requestmullvad

TypeNo type

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.