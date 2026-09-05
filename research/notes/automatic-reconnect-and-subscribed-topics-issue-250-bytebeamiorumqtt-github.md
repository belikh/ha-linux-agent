---
title: 'Automatic reconnect and subscribed topics? · Issue #250 · bytebeamio/rumqtt
  · GitHub'
id: automatic-reconnect-and-subscribed-topics-issue-250-bytebeamiorumqtt-github
tags:
- linux-agent-jupiteros-fleet-15537b
- rumqtt
- rumqttc
- reconnect
- subscription
- known-issue
created: '2026-09-02T04:02:29.632726Z'
updated: '2026-09-05T10:51:21.614318Z'
source: https://github.com/bytebeamio/rumqtt/issues/250
source_domain: github.com
fetched_at: '2026-09-02T04:02:23.381755Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'rumqtt issue #250 (opened Feb 2021, still OPEN, labelled in-pipeline as
  of #1052): rumqttc''s eventloop reconnects automatically after connection errors,
  but subscriptions are NOT re-established after reconnect — the user observes ''no
  more incoming data'' from subscribed topics after ConnAck with session_present:
  false, and must manually re-call client.subscribe() in the error branch of the poll
  loop. Reproduction: pull the network cable with keep_alive=5s. This is the exact
  failure mode ha-linux-agent would hit on broker/HA restarts with clean sessions:
  poll() continues, reconnect succeeds, but the agent goes deaf unless it re-subscribes
  on every reconnect. Workaround shown: on Err(e) from eventloop.poll(), call client.subscribe(...)
  again. Load-bearing for any rumqttc-based agent: the eventloop does NOT own re-subscription;
  the caller must.'
---

Automatic reconnect and subscribed topics? · Issue #250 · bytebeamio/rumqtt · GitHub

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

Automatic reconnect and subscribed topics? #250

New issueCopy link

New issueCopy link

Open
#1052

Open

Automatic reconnect and subscribed topics?#250

#1052

Copy link

Labels

in-pipelinePlanned to be resolved in a future releasePlanned to be resolved in a future release

Description

plaes
opened on Feb 22, 2021

Issue body actions

This might be duplicate to #211

rumqttc seems to recover/reconnect properly after connection error, but after the reconnect, I'm not seeing any data coming in from subscribed topics:

... data coming in from topics...
Error = MqttState(Io(Custom { kind: ConnectionAborted, error: "connection closed by peer" }))
Incoming = ConnAck(ConnAck { session_present: false, code: Success })
... no more incoming data...

This is basically a modified async example. To trigger the issue, just jank the network cable or disable wireless a bit (in case broker is on another machine)...

#[tokio::main]
async fn main() {

let mut mqttoptions = MqttOptions::new("test-subscribe", "mqtt-server", 1883);
mqttoptions.set_keep_alive(5);

let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
// Subscribe to everything...
client.subscribe("#", QoS::AtMostOnce).await.unwrap();

loop {
match eventloop.poll().await {
Ok(Event::Incoming(Packet::Publish(p))) => {
println!("Incoming = {:?}, {:?}", p.topic, p.payload);
},
Ok(Event::Incoming(Packet::PingResp)) |
Ok(Event::Outgoing(Outgoing::PingReq)) => {},
Ok(Event::Incoming(i)) => {
println!("Incoming = {:?}", i);
},
Ok(Event::Outgoing(o)) => {
println!("Outgoing = {:?}", o);
},
Err(e) => {
println!("Error = {:?}", e);
// XXX: Here I have to re-subscribe to the topics :(
client.subscribe("#", QoS::AtMostOnce).await.unwrap();
}
}
}
}

Reactions are currently unavailable

Metadata
Metadata

AssigneesNo one assigned

Labels

in-pipelinePlanned to be resolved in a future releasePlanned to be resolved in a future release

TypeNo type

ProjectsNo projects

MilestoneNo milestone

RelationshipsNone yet

DevelopmentNo branches or pull requests

Issue actions
Open in GitHub Copilot app

You can’t perform that action at this time.