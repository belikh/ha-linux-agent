---
title: Releases · joshuar/go-hass-agent · GitHub
id: releases-joshuargo-hass-agent-github
tags:
- linux-agent-jupiteros-fleet-15537b
- go-hass-agent
- release-notes
- version-ground-truth
- reliability-failure-modes
created: '2026-09-02T04:37:53.724846Z'
updated: '2026-09-02T17:37:22.141695Z'
source: https://github.com/joshuar/go-hass-agent/releases
source_domain: github.com
fetched_at: '2026-09-02T04:37:53.723249Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'go-hass-agent releases ground truth: current release v14.15.1 (2026-08-09),
  confirming nixpkgs v14.15.1 is current and pkg.go.dev''s v1.4.3 is a stale artifact.
  Active ~monthly cadence through 2026 (v14.15.0 2026-07-25, v14.14.1 2026-06-23,
  back through v14.10.x). 26 release assets per version: rpm/deb/tar.zst/pkg.tar.zst
  across aarch64/armv6/armv7/x86_64, each with detached .sig files, cosign/GPG-signed
  via GitHub verified signatures. v14.15.1 fixes a CPU-usage-worker concurrent-map-read
  data race — exactly the class of bug that makes an agent intermittently unreliable.
  v14.15.0 adds NVIDIA and AMD GPU memory-usage sensors, PipeWire pw-dump tolerance
  for non-string node.nick values, and a sweep of fixes making disabled-sensor preferences
  respected BEFORE capability checks (disabled sensors were still doing privileged
  capability probes) plus clearer capability-error messages — fleet-relevant behaviour
  where per-host sensor toggles must actually short-circuit.'
---

*Suggested by [[github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device]] — release-level ground truth for current version and breaking changes*

Releases · joshuar/go-hass-agent · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

joshuar

/

go-hass-agent

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
31

Star
575

Releases: joshuar/go-hass-agent

Releases · joshuar/go-hass-agent

Release list

Previous Next

Jump to release

v14.15.1

v14.15.0

v14.14.1

v14.13.0

v14.12.0

v14.11.0

v14.10.5

v14.10.4

v14.10.3

v14.10.2

Previous Next

v14.15.1

v14.15.1

Latest

Latest

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

09 Aug 01:21

v14.15.1

55fafa8

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.15.1 (2026-08-09)

Bug Fixes

cpu: guard the usage worker's reading maps (4ad1c07)

cpu: guard the usage worker's reading maps (5b70152)

Performance Improvements

models: ⚡ more efficient message string generation (9bed8a5)

Assets
26

go-hass-agent-14.15.1-1-aarch64.pkg.tar.zst

sha256:42dcc68e09bf8ca61add687d5fc250ad5f6e3e29f420821814a1d9c7a7576085

9.28 MB
2026-08-09T01:23:31Z

go-hass-agent-14.15.1-1-aarch64.pkg.tar.zst.sig

sha256:dd44830eec38b41cd013859878255c2e2bfa0f5f3da185e18a40676dab95cafa

10.1 KB
2026-08-09T01:23:31Z

go-hass-agent-14.15.1-1-armv6.pkg.tar.zst

sha256:cf363d05981f572260375a048bf1c12020ad269cca658257d3ca1259204d630a

9.27 MB
2026-08-09T01:23:07Z

go-hass-agent-14.15.1-1-armv6.pkg.tar.zst.sig

sha256:6a41e5bf81c86e69304fde28ccc00a7261b5aff179f21234176cde19bb1cafbd

10 KB
2026-08-09T01:23:07Z

go-hass-agent-14.15.1-1-armv7.pkg.tar.zst

sha256:cd412d4f64b8e3464123d42bc89788642297062d114dbbc4d6becc6da7b9d07f

9.27 MB
2026-08-09T01:23:14Z

go-hass-agent-14.15.1-1-armv7.pkg.tar.zst.sig

sha256:0a3b9e166bba6461bc0f0844a168c49f3496139b1380ae71084ba5152a212431

10.1 KB
2026-08-09T01:23:14Z

go-hass-agent-14.15.1-1-x86_64.pkg.tar.zst

sha256:64022f9d332bb4ee7e64de87fda23adfea2edaf380c9252d04d00709f72695ae

10.3 MB
2026-08-09T01:22:58Z

go-hass-agent-14.15.1-1-x86_64.pkg.tar.zst.sig

sha256:8ebfd927b628a7d1a4f1de09e38e768019ffb800f81647237316f5b7aebda160

9.98 KB
2026-08-09T01:22:58Z

go-hass-agent-14.15.1-1.aarch64.rpm

sha256:8563a3cbb2a545dd39c71cfc79d9f4f3685a84bb0eb240a57f74f8a08e22f3f2

9.27 MB
2026-08-09T01:23:31Z

go-hass-agent-14.15.1-1.aarch64.rpm.sig

sha256:c97b85ffeaf5be67c94c8738363a6cc4bb094bafd9e0b270af3eb9467e7462cb

10 KB
2026-08-09T01:23:31Z

Source code (zip)

2026-08-09T01:20:36Z

Source code (tar.gz)

2026-08-09T01:20:36Z

Show all 26 assets

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.15.0

v14.15.0

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

25 Jul 05:43

v14.15.0

6fdeed4

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.15.0 (2026-07-25)

Features

gpu memory usage (a0fbdd7)

linux: (AMD) GPU memory usage (521ec76)

Bug Fixes

linux/disk: respect disabled preference before SMART capability check (0d27760)

linux/disk: respect disabled preference before SMART capability check (d52e70d)

linux/power: 🔧 separate success and error conditions when determining which power controls to expose (24a28f7)

linux/power: honor disabled preference for backlight control (a15b732)

linux/power: honor disabled preference for backlight control (a1d4898)

linux: improve capability error clarity and respect disabled pref for activity worker (7efe012)

linux: improve capability error clarity and respect disabled pref for activity worker (a9598e9)

pipewire: tolerate non-string node.nick values from pw-dump (0e285f2)

pipewire: tolerate non-string node.nick values from pw-dump (4494ffc)

pkg/linux/pipewire: 🔧 use any for undefined size element (81c27d7)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.14.1

v14.14.1

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

23 Jun 00:24

v14.14.1

5cfb543

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.14.1 (2026-06-22)

Bug Fixes

ci: 🔧 set APPVERSION as needed for nfpm packaging (d6f254f)

github: 🔧 fix missing version in build workflow (40b3a47)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.13.0

v14.13.0

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

19 Jun 23:57

v14.13.0

9c965ba

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.13.0 (2026-06-13)

Features

linux/media: ✨ improved webcam streaming (abf6153)

Assets
8

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.12.0

v14.12.0

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

07 Jun 04:52

v14.12.0

56b49e2

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.12.0 (2026-06-07)

Features

linux/media: ✨ monitor webcam usage via pipewire and video4linux (47fddc2)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.11.0

v14.11.0

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

29 Apr 22:19

v14.11.0

54a6e98

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.11.0 (2026-04-25)

Features

linux/location: ✨ expose location sensor on all device types but disable by default if not running on a laptop (b7c7d92)

Bug Fixes

linux/media: 🐛 volume control fixes (2befa10)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.10.5

v14.10.5

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

18 Apr 05:11

v14.10.5

57e0da3

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.10.5 (2026-04-18)

Miscellaneous Chores

release 14.10.5 (9bac9d6)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.10.4

v14.10.4

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

18 Apr 00:40

v14.10.4

7f5c6b2

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.10.4 (2026-04-18)

Bug Fixes

check disabled preference before D-Bus call in ABRT worker (c86f9a2)

check disabled preference before D-Bus call in ABRT worker (3fb98d4)

scheduler: 🐛 don't use outdated math package (caad369)

web/assets: 🐛 fix import of hyperscript (29892a0)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.10.3

v14.10.3

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

20 Mar 23:53

v14.10.3

38f5071

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.10.3 (2026-03-20)

Bug Fixes

linux/power: 🐛 fix logic for detecting and using ddcutil on Gnome for backlight control. fixes #802 (2dad9e6)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

v14.10.2

v14.10.2

Compare

Choose a tag to compare

Sorry, something went wrong.

Filter

Loading

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

No results found

View all tags

github-actions

released this

14 Mar 04:17

v14.10.2

75b792b

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

14.10.2 (2026-03-14)

Bug Fixes

logging: 🐛 fix writing logfile (116ee88)

Assets
26

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Previous 1 2 3 4 5 … 14 15 Next

Previous Next

You can’t perform that action at this time.