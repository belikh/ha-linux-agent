---
title: WTH there is no windows and Linux HA app similar to macOS - Month of "What
  the heck?!" - Home Assistant Community
id: wth-there-is-no-windows-and-linux-ha-app-similar-to-macos-month-of-what-the-heck
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- mqtt
- home-assistant
- native-app-integration
- windows-only
- community-thread
- feature-gap
created: '2026-09-02T06:41:40.127273Z'
updated: '2026-09-02T17:37:22.756506Z'
source: https://community.home-assistant.io/t/wth-there-is-no-windows-and-linux-ha-app-similar-to-macos/810188
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T06:41:40.055701Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA Community ''Month of WTH'' thread (Dec 2024) asking why there is no official
  Windows/Linux HA companion app like the macOS one. Key facts from the discussion:
  (1) the macOS app largely comes from iOS app reuse, so bespoke Windows/Linux effort
  is much higher; (2) recommended Linux options named by community mods: glances,
  netdata, system bridge, system monitor; (3) HASS.Agent requires MQTT for ''most
  features'' — ''Without MQTT, only Quick Actions will work'' — though it can connect
  to HA via API for basic stuff with a long-lived token; (4) MQTT is an official HA
  addon channel (mosquitto) and one mod argues it''s ''more native than many things'';
  (5) HASS.Agent moved from LAB02-Research to hass-agent org; (6) HAuserSince2019
  notes the Mac App Store app is the iOS app (in a VM); (7) Hellis81 cites GamerClassN7/HA_Desktop_Companion
  as the no-MQTT option using the native HA API; (8) WallyR: the differentiator of
  an app over a browser is device sensors. Community-sourced feature demand signal
  for ha-linux-agent.'
---

WTH there is no windows and Linux HA app similar to macOS - Month of "What the heck?!" - Home Assistant Community

WTH there is no windows and Linux HA app similar to macOS

Month of "What the heck?!"

the-mentor

(TheMentor)

December 13, 2024,  7:59am

1

The Home Assistant app on MacOS is full of great sensors that enhances the Home Assistant integration.

I would love to have similar experience on Windows and Linux.

Maybe an electron app that gives more low level access to the os sensors?

Thanks

-Dm

Tinkerer

(aka DubhAd on GitHub)

December 13, 2024,  8:43am

2

For Windows do check out https://github.com/LAB02-Research/HASS.Agent

For Linux you’ve got glances, netdata, system bridge, and system monitor.

On either you’ve also got Open Hardware Monitor

Protoncek

December 13, 2024,  9:01am

3

Tinkerer:

For Windows do check out https://github.com/LAB02-Research/HASS.Agent

Actually, for Windows this one is more correct (newer):

https://github.com/hass-agent/HASS.Agent

LAB02 research stopped developing hass agent a while ago. If i’m not mistaken link must be manually added into HACS repositories for succesfull find of new version.

EDIT: Above link is for HA addon. Windows part is HERE

Tinkerer

(aka DubhAd on GitHub)

December 13, 2024,  9:24am

4

Protoncek:

Actually, for Windows this one is more correct (newer):

GitHub - hass-agent/HASS.Agent: Unofficial development project for the HASS.Agent platform.

Thanks, I used the sponsor link the the agent’s about dialog to find the repo, and the updated agent still links to the original repo… I should have spotted that.

the-mentor

(TheMentor)

December 13, 2024,  9:28am

5

I tried them both they are too complex and not even close to the simplicity of the macOS app thst works in a similar way to any companion app

The Hass.Agent requires mqtt etc and isn’t a native and simple integration

Protoncek

December 13, 2024, 10:12am

6

the-mentor:

The Hass.Agent requires mqtt etc

Not entirely true… mqtt is an option, altghouh some function won’t work without it:

You don’t have to have MQTT to use HASS.Agent; however, most features rely on it for bidirectional communication. Without MQTT, only Quick Actions will work.

It can (it does) connect to HA via API for basic stuff. There’s nothing complicated: after install there’s a wizard. Just create long-lived token in HA and enter it in agent.

But, mqtt is also nothing complicated - there’s addon in official addons (mosquitto)

the-mentor

(TheMentor)

December 13, 2024,  2:26pm

7

I personally hate mqtt  I don’t get it and I stay away from it.

Also mqtt is not a native way to integrate with home assistant in my opinion.

It would be nice to have a similar experience to the MacOS app

Protoncek

December 13, 2024,  3:08pm

8

I also “don’t quite get mqtt”, but i don’t have to, things just work when i do according to instructions.  In fact, i just never went deep into it to understand it, since i’ve never had to…

Native way… well, 70% of things in HA aren’t “native way”… (like hacs etc…) so it’s definitely more native than many things (at the end mosquitto is official addon)

Regarding mac: i’ve had it, but, personally, i hate it…all is done “too easy”…  it seems devs in apple think that we users are “stupid”, consequently they make all software “too easy”, without any real options but the ones they decide ”they are good for us…”.

I like to tinker, so i like software to be… well, “complicated”, or better said: open. So, it’s just a matter of personal opinion.

the-mentor

(TheMentor)

December 13, 2024,  3:22pm

9

What I’m saying is we have an official home assistant app for mac why shouldn’t we have official apps for the other desktop platforms

Tinkerer

(aka DubhAd on GitHub)

December 13, 2024,  4:25pm

10

All that needs is some Windows developers to step up and create it…

Participant

December 13, 2024,  5:29pm

11

the-mentor:

What I’m saying is we have an official home assistant app for mac why shouldn’t we have official apps for the other desktop platforms

Part of the reason is that MacOS is able to run the iOS companion app… I don’t want to say it comes for “free”, but the effort to support is much lower versus a bespoke app for Windows or Linux.

Still, with the Android Subsystem for Windows on its’ way out, I’d love if there were a native “companion app.”

Honestly, if it were just a native package to push sensor readings (and maybe execute local commands) over to HA, it’d be totally fine to still need a browser to view the interface.

HAuserSince2019

December 21, 2024,  2:55pm

12

I’m a mac user, but as the title suggest: I just found out I cannot find a windows app for my son’s windows computer

WallyR

(Wally)

December 21, 2024,  3:00pm

13

There is a hass.agent available.

Hass.agent is a third party app and not an “official release”.

And actually there is no app for MacOS.

Hellis81

(Hellis81)

December 21, 2024,  3:04pm

14

If you won’t want to use MQTT then there is also: GitHub - GamerClassN7/HA_Desktop_Companion: App which is using native HA Api to comunicate and report data to HA

Also since this is not about HA core then it’s not a WTH topic

HAuserSince2019

December 21, 2024,  4:24pm

15

There is an ap voor MacOs: ‎Home Assistant in de App Store

WallyR

(Wally)

December 21, 2024, 10:35pm

16

No, that is not for MacOS.

It is for iOS running in a VM.

HAuserSince2019

December 22, 2024,  8:37am

17

Well it’s allready there for many years since the intel processor age for Mac, not only iOS. Virtual or not, it’s the official app in the MacOS appstore. So we’re looking for a similair “official” app for windows. Not an unofficial one.

Protoncek

December 22, 2024,  8:51am

18

But there is “kind of” official addon: in Chrome, Edge’s store… although i rather use web page, it’s easier to have opened multiple pages.

Personally, i don’t see any usable value in such an app…except perhaps in case where a man has windows laptop on the wall as main HA panel (i do… , but edge with kiosk app is sufficient).

WallyR

(Wally)

December 22, 2024,  9:51am

19

The thing an app can provide over the browser is sensors from the device, so an app would be nice for all OSes.

The sensors might not all be the same as mobile devices, but that should be easily fixed.

Protoncek

December 22, 2024, 10:00am

20

Oh… yes, you have the point here. Personally however i don’t see which sensor i would find usefull from my PC, but that’s just me… (ok, perhaps battery charge from laptop)

For controlling devices from PC (like light control, radio…) i use hass.agent, which works perfectly.

next page →

Powered by Discourse, best viewed with JavaScript enabled