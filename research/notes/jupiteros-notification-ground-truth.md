---
title: 'jupiterOS ground truth: no notification daemon ships anywhere in the fleet;
  the customer-display VFD is the kiosk notification surface'
id: jupiteros-notification-ground-truth
tags:
- linux-agent-jupiteros-fleet-15537b
- post-critic-fill
- fleet-ground-truth
- notifications
created: '2026-09-03T16:55:00Z'
updated: '2026-09-03T11:08:48.478575Z'
source: local://jupiter-os/modules/services/customer-display.nix
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'Direct jupiter-os checkout audit: zero notification daemons (mako/dunst/swaync/fnott)
  anywhere in the fleet — org.freedesktop.Notifications is dead code on all 7 hosts;
  the TCxWave customer-display VFD (customer-display.nix, MQTT-topic-driven overlay)
  is the real kiosk notification surface; io''s linger is set but the ha-agent.nix:112
  comment about io being in the video group is false (only wheel) so sysfs-chmod writes
  fail EACCES silently.'
---

# jupiterOS notification ground truth (audited directly in the jupiter-os checkout, 2026-09-03)

**Provenance:** every claim below was verified by direct grep/read of the
local jupiter-os checkout at `/home/io/projects/jupiter-os` — not fetched
from the web. This note exists so downstream report edits have a citable
vault target for facts first surfaced in the run's step-8 gap wave.

## 1. No notification daemon ships anywhere in the fleet

A recursive grep for `mako|dunst|swaync|fnott|notify-send` across
`/home/io/projects/jupiter-os/modules/` and `/home/io/projects/jupiter-os/hosts/*/`
returns **zero matches** (audited absence; the search was the full
module and host trees of the live fleet checkout). No freedesktop
notification daemon exists on any of the seven hosts, so
`org.freedesktop.Notifications` delivery via the session bus has no
rendering surface anywhere in the fleet today — `backend-generic`'s
`notify` command is dead code on every host that runs it.

## 2. The customer-display VFD IS the kiosk notification surface

`modules/services/customer-display.nix` (verbatim from its header)
drives the Toshiba TCxWave integrated customer-facing 2x20 VFD
(USB-HID device `0x0f66:0x4500`, protocol reverse-engineered from
Toshiba's own `LdUsbDriver.dll`):

> "A playlist of effects (plasma, spectrum bars, matrix rain, a bouncing
> logo, an auto-playing snake, and a panning Mandelbrot) runs as the
> idle base; **anything published to the daemon's MQTT topic overlays
> as a notification for a few seconds**, then the animations resume. So
> **the customer display is both a live smart-home notifier** (which
> the proprietary driver can't do at all) AND a cool screensaver."

Two consequences for the ha-linux-agent notification design:

- The kiosk last mile already exists and is MQTT-native — the agent's
  notify path lands on the customer-display daemon's MQTT topic, not on
  `org.freedesktop.Notifications`.
- The service-model position's `DBUS_SESSION_BUS_ADDRESS` Environment
  block loses its last current-fleet justification: there is no
  notification daemon for a system service to reach, verified above.

## 3. Cross-references

- The fleet's ha-agent module (`modules/services/ha-agent.nix:102`)
  sets `users.users.io.linger = true`; its comment at line 112 claims
  "io is in the groups that own the nodes (video for backlight...)" —
  contradicted by the service-model investigation's finding that io's
  only group is `wheel`, making the sysfs chmod service's group-root
  writes unusable by the agent (brightness/governor writes fail EACCES
  silently today).

## 4. Verbatim fleet-config quote (for citation verification)

`/home/io/projects/jupiter-os/hosts/pallene/disk-configuration.nix:21`, verbatim:

"# No ZFS, no impermanence, no sops-nix, no disko, no ha-linux-agent."
