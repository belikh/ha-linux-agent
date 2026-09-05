---
title: 'openclaw gateway status fails on EC2/headless servers due to missing user-level
  systemd · Issue #11805 · openclaw/openclaw · GitHub'
id: openclaw-gateway-status-fails-on-ec2headless-servers-due-to-missing-user-level-s
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
created: '2026-09-02T04:04:41.184491Z'
updated: '2026-09-05T10:51:21.769029Z'
source: https://github.com/openclaw/openclaw/issues/11805
source_domain: github.com
fetched_at: '2026-09-02T04:04:38.601161Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'GitHub issue (Feb 2026, openclaw/openclaw #11805, closed): openclaw gateway
  status/install fail on headless EC2 with ''Error: systemctl --user unavailable:
  Failed to connect to bus: No medium found'' because user-level systemd is unavailable
  without a login session. Root-cause chain stated: user-level systemd requires a
  D-Bus session bus; the session bus requires XDG_RUNTIME_DIR; SSH sessions don''t
  set XDG_RUNTIME_DIR; and loginctl enable-linger must be enabled for the user. Workaround:
  ''sudo loginctl enable-linger $(whoami)'' + ''export XDG_RUNTIME_DIR=/run/user/$(id
  -u)'' then retry install. Suggested improvements directly applicable to any headless
  Linux agent: (A) detect the missing-XDG_RUNTIME_DIR scenario and print an actionable
  error; (B) support a --system flag installing to /etc/systemd/system as a system-level
  service for servers; (C) auto-set XDG_RUNTIME_DIR if /run/user/<uid> exists. States
  the failure mode affects all headless Linux deployments: EC2, GCP, Azure VMs, Docker
  containers, and any server accessed via SSH without a desktop session. Precedent
  for how the ha-linux-agent fleet agent should degrade gracefully when its user session
  bus is missing.'
---

openclaw gateway status fails on EC2/headless servers due to missing user-level systemd · Issue #11805 · openclaw/openclaw · GitHub

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

openclaw

/

openclaw

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
81.6k

Star
389k

openclaw gateway status fails on EC2/headless servers due to missing user-level systemd #11805

New issueCopy link

New issueCopy link

Closed

#54062

Blueflier/cognition-demo#15Blueflier/cognition-demo#15

Closed

openclaw gateway status fails on EC2/headless servers due to missing user-level systemd#11805

#54062

Blueflier/cognition-demo#15

Copy link

Assignees

Labels

bugSomething isn't workingSomething isn't working

Description

niceysam
opened on Feb 8, 2026

Issue body actions

Issue: openclaw gateway status fails on EC2/headless servers due to missing user-level systemd

Summary

On headless EC2 instances (and similar server environments), openclaw gateway status and openclaw gateway install fail with "Failed to connect to bus: No medium found" because user-level systemd (systemctl --user) is unavailable by default.

Environment

OS: Amazon Linux 2023 (6.1.159-182.297.amzn2023.x86_64)

OpenClaw: 2026.2.6-3

Node.js: v22.22.0

Architecture: x86_64 EC2 instance

Problem

Error Message

Gateway service check failed: Error: systemctl --user unavailable: Failed to connect to bus: No medium found

Runtime: unknown (Error: systemctl --user unavailable: Failed to connect to bus: No medium found)

systemd user services unavailable.
systemd user services are unavailable; install/enable systemd or run the gateway under your supervisor.

Root Cause

On EC2 and headless servers:

User-level systemd requires a D-Bus session bus

D-Bus session bus requires XDG_RUNTIME_DIR to be set

SSH sessions don't automatically set XDG_RUNTIME_DIR

loginctl enable-linger needs to be enabled for the user

Current Workaround

# 1. Enable linger for user persistence
sudo loginctl enable-linger $(whoami)

# 2. Set XDG_RUNTIME_DIR (add to ~/.bashrc)
export XDG_RUNTIME_DIR=/run/user/$(id -u)

# 3. Now openclaw gateway install works
openclaw gateway install --force

Suggested Improvements

Option A: Better Error Message + Documentation

Detect the missing XDG_RUNTIME_DIR scenario

Provide actionable fix instructions in the error message:

systemctl --user unavailable: D-Bus session bus not found.

On headless servers (EC2, etc.), run:
sudo loginctl enable-linger $(whoami)
export XDG_RUNTIME_DIR=/run/user/$(id -u)

Then retry: openclaw gateway install

Option B: Support System-Level systemd Service

Add --system flag to openclaw gateway install

Install to /etc/systemd/system/openclaw.service instead of user-level

Useful for servers where system-level services are preferred

Option C: Auto-detect and Set XDG_RUNTIME_DIR

If XDG_RUNTIME_DIR is unset but /run/user/<uid> exists, set it automatically

Would fix the issue transparently for most cases

Logs

{"0":"Gateway service check failed: Error: systemctl --user unavailable: Failed to connect to bus: No medium found","_meta":{"logLevelName":"ERROR"},"time":"2026-02-08T10:20:22.063Z"}
{"0":"systemd user services unavailable.","_meta":{"logLevelName":"ERROR"},"time":"2026-02-08T10:21:23.811Z"}

Related

This likely affects all headless Linux deployments:

AWS EC2

GCP Compute Engine

Azure VMs

Docker containers (already mentioned in current error message)

Any server accessed via SSH without a desktop session

Reactions are currently unavailable

Metadata
Metadata

Assignees

vincentkoc

Labels

bugSomething isn't workingSomething isn't working

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

## Related

- [[d-bus]]
