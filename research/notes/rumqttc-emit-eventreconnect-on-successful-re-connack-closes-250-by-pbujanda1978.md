---
title: 'rumqttc: emit Event::Reconnect on successful re-CONNACK (closes #250) by pbujanda1978
  · Pull Request #1052 · bytebeamio/rumqtt · GitHub'
id: rumqttc-emit-eventreconnect-on-successful-re-connack-closes-250-by-pbujanda1978
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-rumqttc-dependency-fitness
- pull-request
- reconnect
created: '2026-09-02T13:17:47.998786Z'
updated: '2026-09-05T10:51:22.296779Z'
source: https://github.com/bytebeamio/rumqtt/pull/1052
source_domain: github.com
fetched_at: '2026-09-02T13:17:47.997017Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'PR #1052 (bytebeamio/rumqtt) ''rumqttc: emit Event::Reconnect on successful
  re-CONNACK (closes #250)'' by pbujanda1978, opened May 25 2026, status OPEN/unmerged.
  Tracks has_connected_before in EventLoop; after 2nd+ successful CONNACK pushes synthetic
  Event::Reconnect so the caller re-subscribes in its poll() loop. Does NOT auto-resubscribe
  inside the library — rumqttc does not remember subscriptions; the app must call
  client.subscribe() on Event::Reconnect. Scope: v5 module only (v4 deferred to follow-up).
  Validated against rumqttd, rmqtt, mosquitto. Problem statement is the canonical
  articulation of the #250 footgun: with clean_session=true (or session expiry), a
  transient network blip leaves the process running but silently receiving no publishes.
  Maintainer-side comment from thehouseisonfire shows fork-based alternative without
  new Event variant.'
---

rumqttc: emit Event::Reconnect on successful re-CONNACK (closes #250) by pbujanda1978 · Pull Request #1052 · bytebeamio/rumqtt · GitHub

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

Open

rumqttc: emit Event::Reconnect on successful re-CONNACK (closes #250)#1052

pbujanda1978 wants to merge 1 commit into
bytebeamio:mainbytebeamio/rumqtt:mainfrom
pbujanda1978:fix-event-reconnect-resubscribepbujanda1978/rumqtt:fix-event-reconnect-resubscribeCopy head branch name to clipboard

Conversation

pbujanda1978

commented

May 25, 2026

Copy link

Copy Markdown

Problem

After a transport drop and successful reconnect, the broker has a fresh session. With clean_session = true (or session expiry), every subscription the client had is gone. rumqttc itself doesn't remember them either, so the application is silent on the new session until it re-subscribes — but it has no signal that a reconnect happened.

This is #250: a long-standing rumqttc footgun where a transient network blip leaves a process running but unable to receive any publishes, with no error in the event loop (the connection is "fine" again, just empty of subscriptions).

Fix

Track has_connected_before in the EventLoop. After the second (and later) successful CONNACK, push a synthetic Event::Reconnect into the state events queue so the caller's poll() loop sees it.

The initial CONNACK is unchanged (Reconnect is not emitted on first connect, only subsequent ones), so existing callers that subscribe once on startup keep working — they just have to add the Reconnect arm to keep working across reconnects.

Caller pattern

match eventloop.poll().await {
Ok(Event::Reconnect) => {
for (topic, qos) in &my_subs {
client.subscribe(topic, *qos).await?;
}
}
Ok(Event::Incoming(packet)) => { /* ... */ }
Ok(Event::Outgoing(_)) => {}
Err(e) => { /* ... */ }
}

Validation

Tested against multiple brokers (rumqttd, rmqtt, mosquitto): a broker restart that previously left the client silently dead now fires Event::Reconnect within seconds, the caller re-subscribes, and message flow resumes without process restart.

Notes

Scope: only v5 here — the same fix applies to v4; happy to add it in this PR or a follow-up, whichever you prefer.

No public API breaks (new enum variant only); downstream pattern matches that don't include Reconnect will get a warning rather than an error if they have a wildcard arm.

Closes #250

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

👍
1
dfrommi reacted with thumbs up emoji

All reactions

👍
1 reaction

rumqttc: emit Event::Reconnect on successful re-CONNACK

…

96c16b6

After a transport drop and successful reconnect, the broker has a
fresh session. With clean_session = true (or session expiry), every
subscription the client had is gone. rumqttc itself doesn't remember
them either, so the application is silent on the new session until it
re-subscribes — but it has no signal that a reconnect happened.

This is issue bytebeamio#250: a long-standing rumqttc footgun where a transient
network blip leaves a process running but unable to receive any
publishes, with no error in the event loop (the connection is "fine"
again, just empty of subscriptions).

Fix: track has_connected_before in the EventLoop. After the second
(and later) successful CONNACK, push a synthetic Event::Reconnect
into the state events queue so the caller's poll() loop sees it.
The caller pattern is:

match eventloop.poll().await {
Ok(Event::Reconnect) => {
for (topic, qos) in &my_subs {
client.subscribe(topic, *qos).await?;
}
}
...
}

The initial CONNACK is unchanged (Reconnect is NOT emitted on first
connect, only subsequent ones), so existing callers that subscribe
once on startup keep working — they just have to add the Reconnect
arm to keep working across reconnects.

Validated against multiple brokers (rumqttd, rmqtt, mosquitto): a
broker restart that previously left the client silently dead now
fires Event::Reconnect within seconds, the caller re-subscribes,
and message flow resumes without process restart.

Closes bytebeamio#250

thehouseisonfire

commented

May 25, 2026

Copy link

Copy Markdown

I’ve added an example showing how to achieve similar behavior in my fork, although I chose not to introduce an additional Event enum variant.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

This file contains hidden or bidirectional Unicode text that may be interpreted or compiled differently than what appears below. To review, open the file in an editor that reveals hidden Unicode characters.
Learn more about bidirectional Unicode characters

Show hidden characters

Sign up for free
to join this conversation on GitHub.
Already have an account?
Sign in to comment

Reviewers

No reviews

Assignees

No one assigned

Labels

None yet

Projects

None yet

Milestone

No milestone

Development

Successfully merging this pull request may close these issues.

Automatic reconnect and subscribed topics?

Uh oh!

There was an error while loading. Please reload this page.

2 participants

Add this suggestion to a batch that can be applied as a single commit.This suggestion is invalid because no changes were made to the code.Suggestions cannot be applied while the pull request is closed.Suggestions cannot be applied while viewing a subset of changes.Only one suggestion per line can be applied in a batch.Add this suggestion to a batch that can be applied as a single commit.Applying suggestions on deleted lines is not supported.You must change the existing code in this line in order to create a valid suggestion.Outdated suggestions cannot be applied.This suggestion has been applied or marked resolved.Suggestions cannot be applied from pending reviews.Suggestions cannot be applied on multi-line comments.Suggestions cannot be applied while the pull request is queued to merge.Suggestion cannot be applied right now. Please check back later.

You can’t perform that action at this time.