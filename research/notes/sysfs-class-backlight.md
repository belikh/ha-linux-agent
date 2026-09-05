---
title: sysfs-class-backlight
id: sysfs-class-backlight
tags:
- linux-agent-jupiteros-fleet-15537b
- kernel-docs
- backlight
- sysfs
- abi-stable
created: '2026-09-02T07:39:17.027432Z'
updated: '2026-09-05T10:51:22.130419Z'
source: https://raw.githubusercontent.com/torvalds/linux/master/Documentation/ABI/stable/sysfs-class-backlight
source_domain: raw.githubusercontent.com
fetched_at: '2026-09-02T07:39:17.026356Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'The stable kernel ABI file for the backlight class (Documentation/ABI/stable/sysfs-class-backlight
  in torvalds'' tree) — the TRUE primary behind the 404''d docs.kernel.org/ABI/testing/sysfs-class-backlight
  URL in the batch (the testing/ path was removed; this is the stable ABI). Authoritative
  per-attribute contract, all under /sys/class/backlight/<backlight>/: bl_power (April
  2005, v2.6.12 — values compatible with FB_BLANK_*: 0 = power on, 4 = power off);
  brightness (April 2005, v2.6.12 — values 0..max_brightness, shows the level STORED
  IN THE DRIVER which ''may not be the actual brightness (see actual_brightness)'');
  actual_brightness (March 2006, v2.6.17 — ''Show the actual brightness by querying
  the hardware. Due to implementation differences in hardware this may not...'' [text
  continues: match the requested brightness]). Contact Richard Purdie; Users: HAL.
  Contract dates confirm long-term stability — the same interface since kernel 2.6.12,
  safe for an agent to build against. Note fetched from raw.githubusercontent.com;
  every attribute here matches the GPU/backlight doc note''s description (see-also:
  backlight-support-the-linux-kernel-documentation).'
---

*Suggested by [[backlight-support-the-linux-kernel-documentation]] — the stable ABI file the kernel backlight doc references; primary for the 404'd assigned URL docs.kernel.org/ABI/testing/sysfs-class-backlight*

What:		/sys/class/backlight//bl_power
Date:		April 2005
KernelVersion:	2.6.12
Contact:	Richard Purdie
Description:
Control BACKLIGHT power, values are compatible with
FB_BLANK_* from fb.h

- 0 (FB_BLANK_UNBLANK)   : power on.
- 4 (FB_BLANK_POWERDOWN) : power off
Users:		HAL

What:		/sys/class/backlight//brightness
Date:		April 2005
KernelVersion:	2.6.12
Contact:	Richard Purdie
Description:
Control the brightness for this . Values
are between 0 and max_brightness. This file will also
show the brightness level stored in the driver, which
may not be the actual brightness (see actual_brightness).
Users:		HAL

What:		/sys/class/backlight//actual_brightness
Date:		March 2006
KernelVersion:	2.6.17
Contact:	Richard Purdie
Description:
Show the actual brightness by querying the hardware. Due
to implementation differences in hardware this may not
match the value in 'brightness'. For example some hardware
may treat blanking differently or have custom power saving
features. Userspace should generally use the values in
'brightness' to make decisions.
Users:		HAL

What:		/sys/class/backlight//max_brightness
Date:		April 2005
KernelVersion:	2.6.12
Contact:	Richard Purdie
Description:
Maximum brightness for .
Users:		HAL

What:		/sys/class/backlight//type
Date:		September 2010
KernelVersion:	2.6.37
Contact:	Matthew Garrett
Description:
The type of interface controlled by .
"firmware": The driver uses a standard firmware interface
"platform": The driver uses a platform-specific interface
"raw": The driver controls hardware registers directly

In the general case, when multiple backlight
interfaces are available for a single device, firmware
control should be preferred to platform control should
be preferred to raw control. Using a firmware
interface reduces the probability of confusion with
the hardware and the OS independently updating the
backlight state. Platform interfaces are mostly a
holdover from pre-standardisation of firmware
interfaces.