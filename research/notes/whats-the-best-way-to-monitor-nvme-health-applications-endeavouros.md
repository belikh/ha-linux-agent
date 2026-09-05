---
title: What’s the best way to monitor NVMe health? - Applications - EndeavourOS
id: whats-the-best-way-to-monitor-nvme-health-applications-endeavouros
tags:
- linux-agent-jupiteros-fleet-15537b
- smart
- storage-health
- nvme
- reliability-failure-modes
- windows-only
- community-thread
created: '2026-09-02T05:38:56.276117Z'
updated: '2026-09-05T10:51:21.903998Z'
source: https://forum.endeavouros.com/t/what-s-the-best-way-to-monitor-nvme-health/76296
source_domain: forum.endeavouros.com
fetched_at: '2026-09-02T05:38:56.227227Z'
fetch_provider: builtin
status: evergreen
type: note
tier: commentary
content_type: forum
deprecated: false
summary: 'EndeavourOS forum thread (Nov 10 2025, practitioners): user asks how to
  monitor NVMe health; community answers converge on nvme-cli (''sudo nvme smart-log
  /dev/nvme0'') and smartmontools'' smartctl (''sudo smartctl -a /dev/nvme0n1'', look
  for ''SMART overall-health self-assessment test result: PASSED'') as the standard
  tools, with links to the Arch Wiki S.M.A.R.T. page. Thread notes smartctl output
  is more user-friendly than nvme-cli. Endurance context: consumer NVMe TBW ratings
  150-600 TB; the OP''s drive rated 220 TB with 5.43 TB written (~1% used) — healthy
  despite the product being EOL''d. One user recommends Scrutiny (web-UI S.M.A.R.T.
  monitoring hub that wraps smartctl). Recovery guidance: physical/NAND-level failure
  is effectively unfixable; filesystem-level damage is fixable with proper disk checkers,
  but recurring filesystem errors can indicate underlying hardware failure — a practical
  distinction for agent alerting thresholds (media errors vs fs errors).'
---

What’s the best way to monitor NVMe health? - Applications - EndeavourOS

What’s the best way to monitor NVMe health?

General system

Applications

dirn

November 10, 2025,  4:06pm

1

Hi everyone,

First of all, apologies if this isn’t the right category — admins, please feel free to move it if needed.

Recently, I’ve been reading about how NVMe drives can sometimes fail without much warning, which got me thinking about health monitoring. What are the best tools or methods to keep an eye on NVMe drive health?

I came across a tool called nvme-cli and installed it — it lets me view the current state of my drive. Is this the right tool for monitoring NVMe health, or are there other (perhaps better) tools you’d recommend?

Also, if any early warning signs or bad symptoms show up, is there any way to fix or mitigate them?

Output from - sudo nvme smart-log /dev/nvme0

image542×365 65.3 KB

swh

November 10, 2025,  4:18pm

2

To my knowledge, it depends on how much TBW is specified by the disk manufacturer.

Normally between 150 and 600 TB.

cactux

November 10, 2025,  4:19pm

3

https://archlinux.org/packages/?name=smartmontools

https://wiki.archlinux.org/title/S.M.A.R.T.#smartctl

Another tool.

Noodly

November 10, 2025,  4:22pm

4

This might help…

I use  smartctl

It’s a tool for checking SMART data on all kinds of drives.

Install smartmontools If you don’t have it.

Check drive health: Use smartctl to get a general health status. You may need to find the device name with lsblk first.

sudo smartctl -a /dev/nvme0n1 (replace /dev/nvme0n1 with your device name).

Look for the “SMART overall-health self-assessment test result” or a similar line to see if it reports “PASSED”.

dirn

November 10, 2025,  4:40pm

5

swh:

To my knowledge, it depends on how much TBW is specified by the disk manufacturer.

Normally between 150 and 600 TB.

I need to check the manufacturer’s specifications to see the drive’s rated TBW. Based on what I can see, the current usage is only 1%.

@cactux / @Noodly, I’ve installed smartmontools — the output seems much more user-friendly compared to nvme-cli. Thanks.

image542×384 64.6 KB

My next question: if an error occurs, is it possible to fix it?

cactux

November 10, 2025,  4:46pm

6

I am far from knowledgeable in this field but if by error you mean a failure at hardware level, I doubt that there will be any fix for it. I may be wrong though.

Errors at filesystem/software level might be fixable. Depending on the filesystem there are some tools to be used to check the health and fix some issues.

If you get filesystem errors often, that may also be indicative of hardware failure.

The number of units written on your disk seems to indicate that it is still in its “infancy”.

thefrog

November 10, 2025,  4:51pm

7

dirn:

My next question: if an error occurs, is it possible to fix it?

That would be very dependent on what the “Error” is.

Physical Level damage is probably zero for the Average person and Will cost a bit money for those willing to try and get data back.

Filessystem damage is quite common and fortunately easily fixed with proper disk checkers.

dirn

November 10, 2025,  5:01pm

8

cactux:

The number of units written on your disk seems to indicate that it is still in its “infancy”.

Yes, it’s still considered brand new. I bought it along with my mini PC.

thefrog:

Physical Level damage is probably zero for the Average person and Will cost a bit money for those willing to try and get data back.

Filessystem damage is quite common and fortunately easily fixed with proper disk checkers.

Got it. Thanks for the answer @thefrog .

@swh, I’ve checked the drive’s specifications — the TBW is 220 TB.

image542×400 21.8 KB

Interestingly, it seems the product has already been marked as EOL

image1170×518 92.4 KB

I’ll mark @cactux’s answer as the solution

Thanks, everyone — really appreciate it!

swh

November 10, 2025,  5:08pm

9

dirn:

I’ve checked the drive’s specifications — the TBW is 220 TB.

So no worry then. You have 2.65 read and 5.43 written. It should last for a while longer.

moxdrox

November 10, 2025,  5:32pm

10

I like Scrutiny to monitor Drive Health. It also uses smartctl.

Scrutiny has also a nice WebUI

Noodly

November 10, 2025, 10:59pm

11

ooh, I’ll have to check that out. Thanks mate.

ricklinux

November 10, 2025, 11:23pm

12

Get the Western Digital Black. I’ve beat the shit out of my nvme drives and never had an issue. Lots of wiping out, reinstalls, bad shutdowns … you name it. Different file sysytems etc etc.

Powered by Discourse, best viewed with JavaScript enabled