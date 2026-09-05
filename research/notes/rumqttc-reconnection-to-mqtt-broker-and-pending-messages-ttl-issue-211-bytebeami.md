---
title: '[rumqttc] Reconnection to MQTT broker and pending messages TTL · Issue #211
  · bytebeamio/rumqtt · GitHub'
id: rumqttc-reconnection-to-mqtt-broker-and-pending-messages-ttl-issue-211-bytebeami
tags:
- linux-agent-jupiteros-fleet-15537b
- rumqtt
- rumqttc
- reconnect
- pending-queue
- known-issue
created: '2026-09-02T04:02:29.637324Z'
updated: '2026-09-05T10:51:21.585563Z'
source: https://github.com/bytebeamio/rumqtt/issues/211
source_domain: github.com
fetched_at: '2026-09-02T04:02:24.329902Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'rumqtt issue #211 (opened Dec 2020, still OPEN, labelled stale/blocked):
  rumqttc buffers messages during broker/network outage and replays pending messages
  on reconnect — ''rumqttc does a good job of keeping messages, trying to reconnect...
  and sending the pending messages when it reconnects'' — but there is NO way to (a)
  get a disconnect event/callback (user asks for closures in MqttOptions or at least
  a warn log; suggests detecting missing PingResp after timeout), and (b) set a TTL
  or garbage-collect stale pending messages, so arbitrarily old telemetry can be published
  on reconnect. User wanted a 30s TTL to avoid publishing stale data; notes timestamp-in-message
  is an anti-pattern because it needs clock sync across devices. Directly relevant
  to ha-linux-agent reliability: a fleet agent that publishes host telemetry must
  bound the age of queued messages or HA will see stale sensor values after any network
  partition, and it cannot rely on rumqttc to notify it of disconnects.'
---

[rumqttc] Reconnection to MQTT broker and pending messages TTL · Issue #211 · bytebeamio/rumqtt · GitHub

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

bytebeamio

/

rumqtt

Public

Notifications
You must be signed in to change notification settings

Fork
342

Star
2.2k

[rumqttc] Reconnection to MQTT broker and pending messages TTL #211

New issueCopy link

New issueCopy link

Open

Open

[rumqttc] Reconnection to MQTT broker and pending messages TTL#211

Copy link

Labels

staleNot moving forward; blockedNot moving forward; blocked

Description

dsferruzza
opened on Dec 29, 2020

Issue body actions

Hi!

I'm trying to use rumqttc in the context of an Actix Web app (and it seems very nice, thanks for that!).

I made a PoC where rumqttc is used from an Actix actor, which holds the client's state and can receive messages from HTTP handlers (for example, to publish MQTT messages).

Note: I'm using rumqttc 0.2.0 as 0.3.0 needs tokio 0.3 (I'm hopping that both rumqttc and actix-web will rely on tokio 1.0 in a near future).

When I shutdown my MQTT broker (or the network link between my app and my broker) while publishing messages, rumqttc does a good job of keeping messages, trying to reconnect to the broker and sending the pending messages when it reconnects.

This is quite cool, but I feel like I need a bit more in my context:

I can intercept ConnAck messages that means my app just reconnected to the broker, but is there any way to determine that it disconnected? Maybe switching some state when no PingResp is received after a timeout? I guess it could be useful to be able to provide closures in MqttOptions that would be called when this kind of events happen(?) Or at least a warn log message from rumqttc?

In my use case, I really don't want messages older than ~30 seconds to be published when app reconnects to broker. I don't think this can be handled by having timestamps inside messages because it would need time synchronization between every device that publish these messages or subscribes to them... Is there any way to specify a TTL when publishing messages or periodically "garbage collect" pending messages? (Maybe this is an anti-pattern, but I cannot find a way to do this)

Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

staleNot moving forward; blockedNot moving forward; blocked

TypeNo type

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.