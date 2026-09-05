---
title: go-hass-agent - MyNixOS
id: go-hass-agent-mynixos
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- go-hass-agent
- nixos
- nixpkgs
- fleet-deployment
created: '2026-09-02T04:02:37.752849Z'
updated: '2026-09-02T17:37:21.998988Z'
source: https://mynixos.com/nixpkgs/package/go-hass-agent
source_domain: mynixos.com
fetched_at: '2026-09-02T04:02:32.401181Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'go-hass-agent v14.15.1 is IN nixpkgs proper (pkgs/by-name/go/go-hass-agent/package.nix),
  maintained by Ethan Carter Edwards and nadir-ishiguro, MIT, not marked broken/insecure/unfree,
  and builds for all 24 Linux platforms in nixpkgs (x86_64, aarch64, armv7, riscv,
  mips, powerpc, s390x, loongarch64 etc). Directly answers the jupiterOS/NixOS fleet
  question: a mature Linux HA agent is already one system package line away — the
  build-vs-adopt decision for ha-linux-agent must be argued against this baseline.'
---

go-hass-agent - MyNixOS

package
go-hass-agent

Description

Home Assistant native app for desktop/laptop devices.

Go Hass Agent is an application to expose sensors, controls, and events from a device to Home Assistant. You can think of it as something similar to the Home Assistant companion app for mobile devices, but for your desktop, server, Raspberry Pi, Arduino, toaster, whatever. If it can run Go and Linux, it can run Go Hass Agent!
Out of the box, Go Hass Agent will report lots of details about the system it is running on. You can extend it with additional sensors and controls by hooking it up to MQTT. You can extend it even further with your own custom sensors and controls with scripts/programs.
You can then use these sensors, controls, or events in any automations and dashboards, just like the companion app or any other “thing” you've added into Home Assistant.

Metadata

Install

Version14.15.1

License
MIT

Status
BrokenNo
InsecureNo
UnfreeNo
UnsupportedNo

Sourcepkgs/by-name/go/go-hass-agent/package.nix:54

Homepagehttps://github.com/joshuar/go-hass-agent

Maintainers2 (2)

Ethan Carter Edwards

nadir-ishiguro

Platforms24 (24)
Linux
Show all

aarch64-linux

arc-linux

armv5tel-linux

armv6l-linux

armv7a-linux

armv7l-linux

i686-linux

loongarch64-linux

m68k-linux

microblaze-linux

microblazeel-linux

mips-linux

mips64-linux

mips64el-linux

mipsel-linux

powerpc-linux

powerpc64-linux

powerpc64le-linux

riscv32-linux

riscv64-linux

s390-linux

s390x-linux

sh4-linux

x86_64-linux