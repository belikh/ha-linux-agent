---
title: 'Enabling persistent user instance systemd · Issue #3702 · NixOS/nixpkgs ·
  GitHub'
id: enabling-persistent-user-instance-systemd-issue-3702-nixosnixpkgs-github
tags:
- linux-agent-jupiteros-fleet-15537b
created: '2026-09-02T05:08:34.670451Z'
updated: '2026-09-02T17:37:22.172567Z'
source: https://github.com/NixOS/nixpkgs/issues/3702
source_domain: github.com
fetched_at: '2026-09-02T05:08:34.669045Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'nixpkgs issue #3702 (opened Aug 2014 by CMCDragonkai, closed via PR #260248):
  tracking issue for enabling persistent systemd user instances in NixOS. Original
  problem: user-specific systemd instances only exist while a session exists — the
  instance exits when the last session closes; loginctl enable-linger username makes
  it persistent so users can run unattended long-running services, but in 2014 no
  NixOS option covered it and logind extraConfig didn''t either. Requested a declarative
  user-management option that runs enable-linger idempotently on bootup and disable-linger
  when the setting is removed. 9-year lifespan (2014→2023) documents how long the
  linger gap persisted in NixOS; the tmpfiles workaround in the June 2023 discourse
  thread was the community stopgap until PR #260248 landed the real option. Historical
  provenance for why older jupiterOS host configs may still carry manual linger files
  or loginctl invocations.'
---

*Suggested by [[adding-nixos-option-for-systemd-user-lingering-development-nixos-discourse]] — discourse linger thread links nixpkgs issue #3702 as the upstream tracking issue for enabling persistent user systemd instances*

Enabling persistent user instance systemd · Issue #3702 · NixOS/nixpkgs · GitHub

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

NixOS

/

nixpkgs

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
20k

Star
26k

Enabling persistent user instance systemd #3702

New issueCopy link

New issueCopy link

Closed
#260248

Closed

Enabling persistent user instance systemd#3702

#260248

Copy link

Assignees

Labels

0.kind: enhancementAdd something new or improve an existing system.Add something new or improve an existing system.

Description

CMCDragonkai
opened on Aug 21, 2014

Issue body actions

Apparently user specific systemd instances only exist while there is a session for a specific user. If the last session exits, then the user specific systemd instance exits as well.

There's a way to make a user specific systemd instance persistent, so that users can run unattended long running services: https://wiki.archlinux.org/index.php/Systemd/User#Automatic_start-up_of_systemd_user_instances

Basically you need to activate this option: http://www.freedesktop.org/software/systemd/man/loginctl.html#enable-linger%20USER...

loginctl enable-linger username

However this option is not available in NixOS or NixPkgs, I searched the repo. Also the logind extraConfig doesn't cover it.

I think there might need to be a new option as part of the user management that makes the user instance systemd persistent, and run the above command on bootup and the opposite if the setting changes?

It should of course only run once if it has already ran, even though it is idempotent. I wonder how you can do this?

Right now I'm simply running it during when I pack the image.
Reactions are currently unavailable

Metadata
Metadata

Assignees

andir

edolstra

flokli

kloenk

Labels

0.kind: enhancementAdd something new or improve an existing system.Add something new or improve an existing system.

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.