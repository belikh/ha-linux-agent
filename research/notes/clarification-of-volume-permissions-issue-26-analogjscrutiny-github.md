---
title: 'Clarification of volume permissions? · Issue #26 · AnalogJ/scrutiny · GitHub'
id: clarification-of-volume-permissions-issue-26-analogjscrutiny-github
tags:
- linux-agent-jupiteros-fleet-15537b
- smart
- nvme
- capabilities
- github-issue
created: '2026-09-02T07:45:24.611389Z'
updated: '2026-09-02T17:37:22.412076Z'
source: https://github.com/AnalogJ/scrutiny/issues/26
source_domain: github.com
fetched_at: '2026-09-02T07:45:24.610161Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'Scrutiny issue #26 (Sept 2020, closed; labels documentation+enhancement)
  — the primary evidence behind the README''s NVMe capability note. Comment thread
  establishes the kernel-level capability matrix for SMART polling: mrhotio found
  ''--cap-add SYS_RAWIO'' insufficient for NVMe — smartctl -a -j -d nvme /dev/nvme1
  returns ''Read NVMe Identify Controller failed: NVME_IOCTL_ADMIN_CMD: Permission
  denied'', exit_status 2 — while ''--cap-add SYS_ADMIN'' suffices (''does provide
  sufficient perms, but isn''t as restrictive''). Maintainer AnalogJ confirmed with
  Justin Cormack (Docker community slack) against kernel source: ''if you read the
  source code for NVME_IOCTL_ADMIN_CMD it requires CAP_SYS_ADMIN, so there is nothing
  you can do except grant that'' (nvme_user_cmd check in drivers/nvme/host/pci.c,
  v4.4 lines 1882/1940). Conclusion: CAP_SYS_RAWIO is enough for SATA/SCSI drives;
  CAP_SYS_ADMIN is REQUIRED for NVMe; mixed fleets need BOTH. Also: /run/udev mount
  is used for virtual-device filtering and WWN fallback device identification — without
  it Scrutiny falls back to serial numbers; 2023 comments add that container device
  mappings must use the bare /dev/nvmeN control node (not /dev/nvme0n1p1 partition
  paths — a user wasted days on exactly this mistake, matching the superuser thread''s
  control-vs-namespace-node distinction). Note body fetched only the issue shell;
  comment evidence retrieved via api.github.com.'
---

*Suggested by [[github-analogjscrutiny-hard-drive-smart-monitoring-historical-trends-real-world]] — README cites issue #26 for the NVMe SYS_ADMIN capability requirement — primary evidence for agent capability needs*

Clarification of volume permissions? · Issue #26 · AnalogJ/scrutiny · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

AnalogJ

/

scrutiny

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
296

Star
8.2k

Clarification of volume permissions? #26

New issueCopy link

New issueCopy link

Closed

Closed

Clarification of volume permissions?#26

Copy link

Labels

documentationImprovements or additions to documentationImprovements or additions to documentationenhancementNew feature or requestNew feature or request

Description

mrhotio
opened on Sep 18, 2020

Issue body actions

What is the reason for the volume /dev/disk having to need write permissions, instead of mounting it read-only?
Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

documentationImprovements or additions to documentationImprovements or additions to documentationenhancementNew feature or requestNew feature or request

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.