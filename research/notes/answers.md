---
title: answers
id: answers
tags:
- linux-agent-jupiteros-fleet-15537b
- nvme
- smart
- practitioner-forum
- stackexchange-api
created: '2026-09-02T06:56:17.891188Z'
updated: '2026-09-02T17:37:22.376543Z'
source: https://api.stackexchange.com/2.3/questions/1750390/answers?order=desc&sort=votes&site=superuser&filter=withbody
source_domain: api.stackexchange.com
fetched_at: '2026-09-02T06:56:17.890074Z'
fetch_provider: builtin
status: review
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'Superuser Q&A (1750390, Oct 2022, CC BY-SA 4.0) on running smartctl against
  NVMe: which device node to use. grawity''s accepted answer (score 12): /dev/nvme0
  is the raw CONTROL device node used to configure the hardware; /dev/nvme0n1 is the
  block-storage namespace (a logical division similar to partitions but at hardware
  level, analogous to SCSI LUNs; one NVMe device can have several namespaces with
  different encryption settings). SMART info is global to the device, so /dev/nvme0
  is the more appropriate node — but on current Linux BOTH control and block device
  nodes accept SMART ioctls, especially on consumer SSDs with a single namespace.
  mashuptwice (score 2): SMART checks the physical drive, not partitions — smartctl
  on /dev/sda vs /dev/sda1 yields identical output referencing /dev/sda. Design guidance
  for a fleet agent''s SMART collection: enumerate with smartctl --scan, prefer the
  char/control node (nvme0) for health ioctls, but either node works on current kernels;
  SMART is device-scoped, never partition-scoped. Fetched via api.stackexchange.com
  withbody filter because superuser.com returned 403 to the fetcher.'
---

{"items":[{"owner":{"account_id":20740,"reputation":521493,"user_id":1686,"user_type":"registered","accept_rate":88,"profile_image":"https://www.gravatar.com/avatar/ccfd70bcf7841c1cde0aa0d3e283402d?s=256&d=identicon&r=PG&f=y&so-version=2","display_name":"grawity","link":"https://superuser.com/users/1686/grawity"},"is_accepted":true,"score":12,"last_activity_date":1667237175,"last_edit_date":1667237175,"creation_date":1667227068,"answer_id":1750391,"question_id":1750390,"content_license":"CC BY-SA 4.0","body":"
/dev/nvme0 represents the raw device and is the "control" device node that you use to configure the hardware, while /dev/nvme0n1 represents the block-storage – or a chunk thereof. (Specifically, nvme0n1 is a namespace – a logical division similar to partitions but at "hardware" level, slightly similar to SCSI LUNs. A single NVMe device could in theory have several namespaces with e.g. different encryption settings.)\n
In general, SMART information (and any other kind of hardware status) is global to the device, so using the nvme0 node would be more appropriate, but in current Linux versions both the control device and the block-storage devices will accept SMART ioctls all the same. (Especially on consumer SSDs where only one namespace is ever going to be present.)\n
(Also: Those are not mountpoints – they're device nodes. The directory that something is mounted on becomes a mountpoint, such as /home.)\n"},{"owner":{"account_id":10676540,"reputation":3505,"user_id":718047,"user_type":"registered","profile_image":"https://i.sstatic.net/ZSkEO.png?s=256","display_name":"mashuptwice","link":"https://superuser.com/users/718047/mashuptwice"},"is_accepted":false,"score":2,"last_activity_date":1667227076,"creation_date":1667227076,"answer_id":1750392,"question_id":1750390,"content_license":"CC BY-SA 4.0","body":"
You seem to have a misconception of the capabilities of SMART.\n
SMART is not for checking partitions, but for checking hardware drives for their physical stats/errors. It will make no difference if you run smartctl on /dev/sda or /dev/sda1, the output will always reference the physical drive /dev/sda.\n"}],"has_more":false,"quota_max":300,"quota_remaining":298}