---
title: Releases · hass-agent/HASS.Agent · GitHub
id: releases-hass-agenthassagent-github
tags:
- linux-agent-jupiteros-fleet-15537b
- hass-agent
- release-notes
- reliability-failure-modes
created: '2026-09-02T04:02:37.734414Z'
updated: '2026-09-05T10:51:21.701876Z'
source: https://github.com/hass-agent/HASS.Agent/releases
source_domain: github.com
fetched_at: '2026-09-02T04:02:30.608139Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HASS.Agent release cadence and changelog detail, 2.1.1-beta1 (Oct 2023)
  through 2.2.1 (Jun 2025). Load-bearing reliability signals for the ha-linux-agent
  redesign: (1) many user-reported sensor/command regressions per release (MQTT autodiscovery
  not republished after reconnect #230, WMI-based sensors all stop when WMI restarts
  #205, device-rename changing MQTT entity IDs #388, LastActiveSensor wrong after
  ~25 days #440) — the failure modes of a MQTT + WMI sensor pipeline that jupiterOS''s
  agent must design against; (2) 2.2.0''s security-driven removal of LibreHardwareMonitor
  (WinRing0 kernel driver vulnerability) permanently broke GPU temp sensing rather
  than replacing it — evidence that privileged hardware-sensor access is a long-term
  liability; (3) ''use WebSocket'' MQTT option and onboarding MQTT connection test
  added 2.2.0; (4) Satellite Service (headless sensor collection without login) is
  an established pattern worth porting to the Linux agent. Release-signed with GPG,
  github-actions published, ~5 assets per release (installer/portable/satellite).'
---

Releases · hass-agent/HASS.Agent · GitHub

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

hass-agent

/

HASS.Agent

Public

forked from LAB02-Research/HASS.Agent.Staging

Notifications
You must be signed in to change notification settings

Fork
69

Star
1.3k

Releases: hass-agent/HASS.Agent

Releases · hass-agent/HASS.Agent

Release list

Previous Next

Jump to release

2.2.1

2.2.0

2.2.0-beta4

2.2.0-beta3

2.2.0-beta2

2.2.0-beta1

2.1.1

2.1.1-beta3

2.1.1-beta2

2.1.1-beta1

Previous Next

2.2.1

2.2.1

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

07 Jun 17:37

2.2.1

bb97a9f

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

As always, I'd much preferred this bugfix release to arrive sooner but also as always time was the main constraint - I'll improve at some point I promise :D

In case of problems, please create an issue on GitHub or reach out on discord.

Fixes:

Fixed LaunchURLCommand parameters not being treated explicitly (thanks to @juhavt for reporting) #428

Fixed MQTT test during onboarding using actual provided username & password (thanks to @jwidess and others for reporting) #429

Fixed KeyCommand not accepting "ESCAPE" key, since the configuration window would close (thanks to @arklev for reporting #431

Fixed WebViewCommand not being able to be configured on monitors with negative position values (thanks to @DyadicOne for reporting) #432

Fixed LastActiveSensor reporting wrong time after ~25 days (thank to @CrazyCoder for the PR!) #440

Fixed potential NullReferenceException when loading Satellite's Sensors/Commands configuration (thanks to @Bluscream for the PR!) #441

Fixed Hotkey detection issues and removed baked in dependency with nuget (thanks to @bcutter for reporting) #449

Fixed HADot.Net baked dependency with nuget #451

Fixed Virtual Desktop issues due to missing WinRT dll dependency (thanks to @madface303 for reporting and @AstralBlader for investigation!) #452

Contributors

CrazyCoder, Bluscream, and 7 other contributors

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

🎉
11
w00dwork, Seger85, Famku, RusikOk, bem13, NuclearTruck, gentoid, Lord-Memester, scmanjarrez, Nincodedo, and sreknob reacted with hooray emoji

All reactions

🎉
11 reactions

11 people reacted

2.2.0

2.2.0

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

19 Jan 10:18

2.2.0

8331d1b

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

Breaking changes, please read below.

Due to security concerns, we were forced to remove Libre Hardware Monitor library that allowed HASS.Agent to access hardware information. The breaking change in this is that starting with this release, the GPU Temperature Sensor is non-functioning.

It has been left present for backward compatibility reasons, however it will always return 0.

Please see the awesome writeup by @DarkAutumn for more details #43 (comment)

Please see Docs for how you can get GPU temperature information using the standalone version of Libre Hardware Monitor.

Some changes, some fixes and hopefully not a lot of bugs :)

Changes

Added support for Fan domain for Quick Actions (thanks to @axa88 for suggestion) #369

Added configuration backup that is done before migration from LAB02 version actually happens #377

Added WinFormsSleep command (version of sleep command using specifically WinForms API) (thanks to @felipecrs for suggestion) #378

Added ability to test MQTT connection during onboarding process (thanks to @jgstew for suggestion) #379

Added missing "use WebSocket" option to MQTT onboarding step #379

Added "MonitorSleepPowerPlan" command to turn off screen by temporairly modyfying the power plan (should not cause the device to go to sleep) #380

Added permission checks for both client (issues with administrative permissions) and service (not running as system) #383

Added suport for "HumanPresence" internal device sensor #281

Added new "NamedActiveWindow" binary sensor that can tell you if window containing specified name is currently active or not (thanks to @YpsilonTM for suggestion) #298

Added new "AccentColor" sensor that provides #RRGGBB values for Windows' accent colors (thanks to @kineticscreen for suggestion) #299

Added WebSocket support for MQTT connections (experimental :)) (thanks to @axelcypher for suggestion) #253

Added x86 (32bit) support to the project and the published artifacts (thanks to @DoplerGanger for suggestion) #322

Added ability to "hold" the KeyCommand for longer time by making it a switch type (thanks to @f-hicks for suggestion and testing) #325

Added support for selecting on which screen WebView will appear (thanks to @denisabt for the PR!) #246

Added support for button helpers (input_button) for quick actions (thanks to @drewsteinacher for the PR!) #199

Fixes:

Fixed rare issue where LocalAPI would not work due to device configuration missing #358

Fixed Home Assistant MQTT warning log regarding "object_id" deprecation (thanks to @Anto79-ops and @Momentum6890 for reporting) #372

Fixed device discovery messages not being properly removed when device name is changed #376

Fixed order of sensors and commands so they are sorted alphabetically by the first letter #381

Fixed onboarding donation message so it's clear all donations go towards original creator of HASS.Agent - Sam #384

Fixed MQTT entities changing ID after device rename #388

Fixed/changed Satellite Service installation logic so it limits possibility of upgrade failing silently (even though there was no UI error) and/or leaving the Libre Hardware Monitor files (.dll and .sys) which causes Windows Defender to still report them even though they are not actively used. #352

Fixed HASS.Agent crash when no WebView screen was explicitly configured (thanks to @cronner for reporting) #346

Fixed issue where user would be presented with "You're running the newest version" even though internet was unavailable  (thanks to @patienttruth for reporting) #247

Fixed relative URIs in the notifications not opening properly when clicked on (thanks to @felipecrs for reporting) #294

Fixed typo in the onboarding screen (thanks to @epipenRX for reporting) #300

Fixed quick actions window not scaling properly with the system scale (thanks to @MathisP75 for reporting) #301

Fixed repeating audio sensors error log when no default input or output is present because it was removed (thanks to @floriangagnard for reporting) #302

Fixed (once and for all?) issues with hotkey and german keyboard layouts by changing the default one to Shift+Control+Q (thanks to @FatalMerlin for reporting and to @f-rakete for help in fixing) #303

Fixed issue where InnoSetup installer parameters would not be passed on to the Satellite Service installer (thanks to @KrX3D for reporting) #306

Fixed satellite service missing AudioManager initisation, causing AudioSensor not to work properly (thanks to @Dvvarf for reporting) #326

Fixed HASS.Agent refusing to start after bluetooth dongle was removed with RadioCommand configured (thanks to @barrelltitor for reporting) #333

Fixed note regarding Home Assistant long lived token generation and minor wording tweak (thanks to @SVNKoch for the PR!) #156

Fixed the tray icon being blurry when using RDP extensively (thanks to @whc2001 for the PR!) #249

Fixed code typo in printer sensor's code (thanks to @Selim042 for the PR!) #318

Fixed issue where error of one internal device sensor would cause issues with configuration of others #338

Fixed update notification being shown at startup even though general update notifications were disabled (thanks to @stavismed for reporting) #239

Other:

Updated readme file (part 1) #385

Removed dependency on LibreHardwareMonitor that uses WinRing0 (thanks to all people that reported this, especially for @DarkAutumn for all the research!) #280

Moved project from .net6 to .net8 (thanks to @ricoroodenburg for suggestion) #284

Updated dependencies #304

Github Action artifact version update and build target version bump #225

Device renaming

For best results with renaming devices, this version should work with newest integration version: https://github.com/hass-agent/HASS.Agent-Integration/releases/tag/2.1.2

Contributors

Dvvarf, jgstew, and 26 other contributors

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

🎉
7
Pipitapi, theodorx7, george9816, atax112, opkelde, nuaawmy, and xiaolei0125 reacted with hooray emoji
❤️
21
EnriqueLB, Kyrvi, michalowskil, Bjk8kds, taidjn, Splayn3D, whc2001, Moldiz, charming-byte, makgit, and 11 more reacted with heart emoji

All reactions

🎉
7 reactions

❤️
21 reactions

24 people reacted

2.2.0-beta4

2.2.0-beta4

Pre-release

Pre-release

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

27 Nov 11:06

2.2.0-beta4

3e0077a

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

BREAKING CHANGES STARTING WITH 2.2.0-beta1

As with all beta versions, please remember to backup your configuration.

Automatic configuration backup is coming, at least I'm working on it :)

Breaking change:

Please read 2.2.0-beta1 release information for breaking change details https://github.com/hass-agent/HASS.Agent/releases/tag/2.2.0-beta1

Unless Home Assistant forces my hand with some MQTT changes this is the last beta release of 2.2.0 :)

Some fixes and additions here and there.

Best paired with integration beta - https://github.com/hass-agent/HASS.Agent-Integration/releases/tag/d2.1.2-beta1

Fixes in comparison to 2.2.0-beta3:

Fixed rare issue where LocalAPI would not work due to device configuration missing #358

Fixed Home Assistant MQTT warning log regarding "object_id" deprecation (thanks to @Anto79-ops and @Momentum6890 for reporting) #372

Fixed device discovery messages not being properly removed when device name is changed #376

Fixed order of sensors and commands so they are sorted alphabetically by the first letter #381

Fixed onboarding donation message so it's clear all donations go towards original creator of HASS.Agent - Sam #384

Fixed MQTT entities changing ID after device rename #388

Changes in comparison to 2.2.0-beta3:

Added support for Fan domain for Quick Actions (thanks to @axa88 for suggestion) #369

Added configuration backup that is done before migration from LAB02 version actually happens #377

Added WinFormsSleep command (version of sleep command using specifically WinForms API) (thanks to @felipecrs for suggestion) #378

Added ability to test MQTT connection during onboarding process (thanks to @jgstew for suggestion) #379

Added missing "use WebSocket" option to MQTT onboarding step #379

Added "MonitorSleepPowerPlan" command to turn off screen by temporairly modyfying the power plan (should not cause the device to go to sleep) #380

Added permission checks for both client (issues with administrative permissions) and service (not running as system) #383

Misc in comparison to 2.2.0-beta3:

Updated readme file (part 1) #385

Contributors

jgstew, axa88, and 3 other contributors

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

❤️
9
EnriqueLB, kali-777, lluisd, jwidess, GriYanc, StewieGriffin, BubiBalboa, Moldiz, and theodorx7 reacted with heart emoji

All reactions

❤️
9 reactions

9 people reacted

2.2.0-beta3

2.2.0-beta3

Pre-release

Pre-release

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

18 Sep 19:14

2.2.0-beta3

bdabd9d

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

BREAKING CHANGES STARTING WITH 2.2.0-beta1

As with all beta versions, please remember to backup your configuration.

Automatic configuration backup is coming, at least I'm working on it :)

Breaking change:

Please read 2.2.0-beta1 release information for breaking change details https://github.com/hass-agent/HASS.Agent/releases/tag/2.2.0-beta1

Few more of those and we'll have a release per day ;)

Apologies for so many in the short time span - removal of Libre Hardware Monitor library (the one causing your Windows Defender to complain all the time) is creating a lot of noise. All those betas are to try and smooth things out for full 2.2.0 release.

Once again, huge thanks to everyone helping and testing!

Fixes in comparison to 2.2.0-beta2:

Changed Satellite Service installation logic so it limits possibility of upgrade failing silently (even though there was no UI error) and/or leaving the Libre Hardware Monitor files (.dll and .sys) which causes Windows Defender to still report them even though they are not actively used. #352

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

🎉
8
DN0000, Turbo-Pascal, DennisGaida, Bjk8kds, CSymes, tort32, EnriqueLB, and opkelde reacted with hooray emoji
❤️
6
jouster1974, salt-peter431, theodorx7, rvanmaanen, opkelde, and PBrunot reacted with heart emoji

All reactions

🎉
8 reactions

❤️
6 reactions

13 people reacted

2.2.0-beta2

2.2.0-beta2

Pre-release

Pre-release

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

16 Sep 21:08

2.2.0-beta2

5877b0b

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

BREAKING CHANGES STARTING WITH 2.2.0-beta1

As with all beta versions, please remember to backup your configuration.

Automatic configuration backup is coming, at least I'm working on it :)

Breaking change:

Please read 2.2.0-beta1 release information for breaking change details https://github.com/hass-agent/HASS.Agent/releases/tag/2.2.0-beta1

Fixes in comparison to 2.2.0-beta1:

Fixed HASS.Agent crash when no WebView screen was explicitly configured #346

Small and quick release but since this bug causes crash of HASS.Agent I wanted it fixed as soon as possible :)

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

👍
3
theodorx7, EnriqueLB, and opkelde reacted with thumbs up emoji

All reactions

👍
3 reactions

3 people reacted

2.2.0-beta1

2.2.0-beta1

Pre-release

Pre-release

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

14 Sep 16:00

2.2.0-beta1

cd2a560

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

BREAKING CHANGES - PLEASE READ BELOW BEFORE INSTALLATION

As with all beta versions, please remember to backup your configuration.

Automatic configuration backup is coming, at least I'm working on it :)

Breaking change:

Due to security concerns, we were forced to remove Libre Hardware Monitor library that allowed HASS.Agent to access hardware information. The breaking change in this is that starting with this release, the GPU Temperature Sensor is non-functioning.

It has been left present for backward compatibility reasons, however it will always return 0.

Please see the awesome writeup by @DarkAutumn for more details #43 (comment)

Please see Docs for how you can get GPU temperature information using the standalone version of Libre Hardware Monitor.

Features:

Added suport for "HumanPresence" internal device sensor #281

Added new "NamedActiveWindow" binary sensor that can tell you if window containing specified name is currently active or not (thanks to @YpsilonTM for suggestion) #298

Added new "AccentColor" sensor that provides #RRGGBB values for Windows' accent colors (thanks to @kineticscreen for suggestion) #299

Added WebSocket support for MQTT connections (experimental :)) (thanks to @axelcypher for suggestion) #253

Added x86 (32bit) support to the project and the published artifacts (thanks to @DoplerGanger for suggestion) #322

Added ability to "hold" the KeyCommand for longer time by making it a switch type (thanks to @f-hicks for suggestion and testing) #325

Added support for selecting on which screen WebView will appear (thanks to @denisabt for the PR!) #246

Added support for button helpers (input_button) for quick actions (thanks to @drewsteinacher for the PR!) #199

Fixes::

Fixed issue where user would be presented with "You're running the newest version" even though internet was unavailable  (thanks to @patienttruth for reporting) #247

Fixed relative URIs in the notifications not opening properly when clicked on (thanks to @felipecrs for reporting) #294

Fixed typo in the onboarding screen (thanks to @epipenRX for reporting) #300

Fixed quick actions window not scaling properly with the system scale (thanks to @MathisP75 for reporting) #301

Fixed repeating audio sensors error log when no default input or output is present because it was removed (thanks to @floriangagnard for reporting) #302

Fixed (once and for all?) issues with hotkey and german keyboard layouts by changing the default one to Shift+Control+Q (thanks to @FatalMerlin for reporting and to @f-rakete for help in fixing) #303

Fixed issue where InnoSetup installer parameters would not be passed on to the Satellite Service installer (thanks to @KrX3D for reporting) #306

Fixed satellite service missing AudioManager initisation, causing AudioSensor not to work properly (thanks to @Dvvarf for reporting) #326

Fixed HASS.Agent refusing to start after bluetooth dongle was removed with RadioCommand configured (thanks to @barrelltitor for reporting) #333

Fixed note regarding Home Assistant long lived token generation and minor wording tweak (thanks to @SVNKoch for the PR!) #156

Fixed the tray icon being blurry when using RDP extensively (thanks to @whc2001 for the PR!) #249

Fixed code typo in printer sensor's code (thanks to @Selim042 for the PR!) #318

Fixed issue where error of one internal device sensor would cause issues with configuration of others #338

Fixed update notification being shown at startup even though general update notifications were disabled (thanks to @stavismed for reporting) #239

General changes:

Removed dependency on LibreHardwareMonitor that uses WinRing0 (thanks to all people that reported this, especially for @DarkAutumn for all the research!) #280

Moved project from .net6 to .net8 (thanks to @ricoroodenburg for suggestion) #284

Updated dependencies #304

Github Action artifact version update and build target version bump #225

Contributors

Dvvarf, DarkAutumn, and 21 other contributors

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

👍
11
ManuVice, Dhtjf, danishru, KrX3D, DyadicOne, DN0000, GriYanc, DominicMuir, theodorx7, EnriqueLB, and opkelde reacted with thumbs up emoji
🎉
5
DN0000, GriYanc, patienttruth, CSymes, and opkelde reacted with hooray emoji
❤️
2
GriYanc and opkelde reacted with heart emoji

All reactions

👍
11 reactions

🎉
5 reactions

❤️
2 reactions

13 people reacted

2.1.1

2.1.1

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

05 Feb 18:40

2.1.1

fd615f1

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

Fixes, fixes and fixes - in case of any issues please create and issue on GitHub or ask on Discord :)

Unless a major bug happens, next release is going to be probably "2.2.0" where we tackle the security updates.

Fixes:

MQTT autodiscovery messages were not republished after connection was lost and recovered (thanks to @whc2001 for reporting) #230

URI attached to a notification button wouldn't open properly #229

Satellite Service exe path is now properly enclosed in double quotes according to good security practices (thanks to @yakidd for reporting) #221

Media playback via Music Assistant wasn't working correctly (thanks to @felipecrs and @whc2001 for reporting, providing a lot of information and testing) #220

PowershellCommand argument handling where passing arguments containing spaces/quotes would cause the command to fail (thanks @greghesp for reporting) #204

Restarting "Windows Management Instrumentation (WMI)" while HASS.Agent is running causes all WMI based sensors to stop functioning (thanks to @jack5mikemotown for reporting) #205

Media player now respects the user-provided value in mute service call (thanks to @TarheelGrad1998 for reporting) #185

Notification image not being shown when relative "/media/pics/image.png" was used instead of full URL (thanks to @iankaufmann for reporting) #186

UI not displaying properly on high-DPI devices with scaling other than 100% (thanks to @IsaacInsoll for reporting) #187

Some of the documentations links were pointing to non-existing pages (thanks to @SVNKoch for the PR!) #159

Pressing ESC while having Alt+Tab pressed caused HASS.Agent windows to close (thanks to @SVNKoch for reporting and @denisabt for the PR!) #161

Fixed SetVolumeCommand ignoring provided value and not handling float values properly (thanks to @wynandtredoux and @drueppler for reporting)

Fixed internal AudioManager not properly clamping values if provided one was below 0 or above 100

Note

There are two additonal fixes I sneaked in between beta3 and full release - #230 and #229

Contributors

greghesp, iankaufmann, and 10 other contributors

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

🎉
12
Pel1can111, whc2001, Marcel0024, rodrigomoyano11, Fogh, DannAberHardt, patienttruth, Liborsaf, aboisleux, realgooseman, and 2 more reacted with hooray emoji
❤️
8
ohhai-kthxbai, j-inc, patienttruth, Liborsaf, ZackEndboss, realgooseman, Joshndroid, and opkelde reacted with heart emoji
🚀
5
FLab-Projects, ftt-prod, rodrigomoyano11, Liborsaf, and opkelde reacted with rocket emoji

All reactions

🎉
12 reactions

❤️
8 reactions

🚀
5 reactions

18 people reacted

2.1.1-beta3

2.1.1-beta3

Pre-release

Pre-release

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

20 Jan 19:33

2.1.1-beta3

2a61f58

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

This time it's actually last of betas for 2.1.1 :D

Fixes:

Satellite Service exe path is now properly enclosed in double quotes according to good security practices (thanks to @yakidd for reporting) #221

Media playback via Music Assistant wasn't working correctly (thanks to @felipecrs and @whc2001 for reporting, providing a lot of information and testing) #220

PowershellCommand argument handling where passing arguments containing spaces/quotes would cause the command to fail (thanks @greghesp for reporting) #204

Restarting "Windows Management Instrumentation (WMI)" while HASS.Agent is running causes all WMI based sensors to stop functioning (thanks to @jack5mikemotown for reporting) #205

Note:

Includes 2.1.1-beta1 and 2.1.1-beta2 changes of course.

Contributors

greghesp, yakidd, and 3 other contributors

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

👍
2
ManuVice and Liborsaf reacted with thumbs up emoji
❤️
2
RonnieDilli and Liborsaf reacted with heart emoji

All reactions

👍
2 reactions

❤️
2 reactions

3 people reacted

2.1.1-beta2

2.1.1-beta2

Pre-release

Pre-release

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

26 Oct 17:40

2.1.1-beta2

1385b32

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

Probably last of 2.1.1 betas :)

Fixes:

Media player now respects the user-provided value in mute service call (thanks to @TarheelGrad1998 for reporting) #185

Notification image not being shown when relative "/media/pics/image.png" was used instead of full URL (thanks to @iankaufmann for reporting) #186

UI not displaying properly on high-DPI devices with scaling other than 100% (thanks to @IsaacInsoll for reporting) #187

Some of the documentations links were pointing to non-existing pages (thanks to @SVNKoch for the PR!) #159

Pressing ESC while having Alt+Tab pressed caused HASS.Agent windows to close (thanks to @SVNKoch for reporting and @denisabt for the PR!) #161

Note:

Includes 2.1.1-beta1 changes of course.

Contributors

iankaufmann, denisabt, and 3 other contributors

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

👍
3
Sythsaz, profucius, and Metzlmane reacted with thumbs up emoji

All reactions

👍
3 reactions

3 people reacted

2.1.1-beta1

2.1.1-beta1

Pre-release

Pre-release

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

08 Oct 17:22

2.1.1-beta1

5fc4112

This commit was created on GitHub.com and signed with GitHub’s verified signature.

GPG key ID: B5690EEEBB952194

Verified

Learn about vigilant mode.

Trying to keep the promise of smaller but more frequent updates :)

First beta of 2.1.1:

Fixes:

Fixed SetVolumeCommand ignoring provided value and not handling float values properly (thanks to @wynandtredoux and @drueppler for reporting)

Fixed internal AudioManager not properly clamping values if provided one was below 0 or above 100

Note

The installer version will still say 2.1.0 - this will be fixed with beta2 :)

Contributors

wynandtredoux and drueppler

Assets
5

Loading

Uh oh!

There was an error while loading. Please reload this page.

❤️
5
Nincodedo, ManuVice, lichenophile, iankaufmann, and Anto79-ops reacted with heart emoji

All reactions

❤️
5 reactions

5 people reacted

Previous 1 2 3 Next

Previous Next

You can’t perform that action at this time.