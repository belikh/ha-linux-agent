---
title: Releases · eclipse-paho/paho.mqtt.rust · GitHub
id: releases-eclipse-pahopahomqttrust-github
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-rumqttc-dependency-fitness
- alternatives
- paho-mqtt
created: '2026-09-02T13:21:39.738887Z'
updated: '2026-09-02T17:37:22.575517Z'
source: https://github.com/eclipse-paho/paho.mqtt.rust/releases
source_domain: github.com
fetched_at: '2026-09-02T13:21:39.737037Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'eclipse-paho/paho.mqtt.rust releases page 1: v0.14.0 (26 Mar, latest) wraps
  Paho C v1.3.16, adds sync+async event streams where ''All events from the client
  flow through the stream: Connect, Connection Lost, Disconnected, Incoming Message'',
  tokio build feature, breaking MQTT v5 reason-code error changes, MSRV 1.75. Cadence
  shows active maintenance: v0.13.0 (21 Jan), v0.13.1 (19 Feb), v0.13.2 (27 Mar),
  v0.13.3 (28 Apr), v0.14.0 (26 Mar), sys-v0.10.3 (14 May, C23 compiler fix). Notably
  v0.12.3 (26 Oct prior year) ''fixed a number of bugs, including numerous issues
  with reconnecting to the broker ... crashes on reconnect callbacks'' — reconnect
  machinery specifically hardened by upstream.'
---

Releases · eclipse-paho/paho.mqtt.rust · GitHub

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

eclipse-paho

/

paho.mqtt.rust

Public

Notifications
You must be signed in to change notification settings

Fork
107

Star
588

Releases: eclipse-paho/paho.mqtt.rust

Releases · eclipse-paho/paho.mqtt.rust

Release list

Previous Next

Jump to release

Version 0.14.0

-sys Version 0.10.3

Version 0.13.3

Version 0.13.2

Version 0.13.1

Version 0.13.0

Version 0.12.5

Version 0.12.4

Version 0.12.3

Version 0.12.2

Previous Next

Version 0.14.0

Version 0.14.0

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

fpagliughi

released this

26 Mar 04:28

v0.14.0

db1f652

Updates for some performance increases, event streams, and fixes to MQTT v5 error reporting.

Support for Paho C v1.3.16

Improved performance and lower latency for connect and publish operations.

Added synchronous (blocking) and async event streams.

All events from the client flow through the stream:

Connect, Connection Lost, Disconnected, Incoming Message

Typeof Variable Byte Integer is u32 (not i32)

PropertyType enum now supports Ord, PartialOrd, and Hash traits

Added a tokio build feature for cargo, and a few tokio example apps.

Enabing the feature simply builds the tokio examples. (The library is still fully runtime agnostic).

Removed async-std from examples. Using smol instead.

Properties::byte_len and Properties::get_at no longer cast *const MQTTProperties to *mut before calling C functions that are correctly declared const in the bindings.

#244 Error handling for MQTT5 seems inconsistent

#264 Fix async MQTT5 error handling

Proper handling of v5 failures w/ Reason Codes

[Breaking] Reason Code Error variant now contains Properties from ACK packet

[Breaking] ACKs with a single reason code error generate an Error result, instead of an Ok() with a possible error code in it.

Subscribe Many & Unsubscribe Many still return Ok, possibly with a combination of success and error reason codes

Bumped MSRV to Rust v1.75

Bumped paho-mqtt-sys to v0.11

Various new clippy warnings fixed

#257 initialize mqtt version from create options instead of assuming

#261 Remove undefined property from Cargo.toml

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

-sys Version 0.10.3

-sys Version 0.10.3

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

fpagliughi

released this

14 May 23:48

sys-v0.10.3

ee4550b

Fixed the build of the C library for C23 compilers (gcc 15+, etc)

[#249)[https://github.com//pull/249) Fixed the Paho C build for C23

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Version 0.13.3

Version 0.13.3

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

fpagliughi

released this

28 Apr 16:44

v0.13.3

26b0c8c

Fixed TopicMapper

TopicMatcher implements From(HashMap) for the value type.

#243 TopicMatcher not matching parent when using '#' wildcard.

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Version 0.13.2

Version 0.13.2

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

fpagliughi

released this

27 Mar 20:56

v0.13.2

a7e1225

Minor updates to TopicFilter.

Added some methods to TopicFilter:

Queries has_wildcards() and num_fields().

TryFrom for &str and String

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Version 0.13.1

Version 0.13.1

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

fpagliughi

released this

19 Feb 15:33

v0.13.1

789d290

Bug fixes.

#240 Display trait for MqttVersion enumeration.

#241 Restored the ability to turn off the ssl feature.

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

❤️
1
Notgnoshi reacted with heart emoji

All reactions

❤️
1 reaction

1 person reacted

Version 0.13.0

Version 0.13.0

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

fpagliughi

released this

21 Jan 22:52

v0.13.0

999d7ff

Update for Paho C v1.3.14 with support for UNIX-domain sockets. It also includes:

Updated License to EPL-v2.0

Bumped MSRV to Rust v1.73.0

Bumped -sys to v0.10.0

Wrapping Paho C v1.3.14

Support for UNIX-domain sockets on local machine (*nix only)

build.rs builds optional UNIX sockets into Paho C on non-Windows systems (*nix)

Reworked the Error type

Remove Paho and PahoDescr errors. De-nested them into the top-level.

Parsing the error messages from PahoDescr for new error types.

Removed Paho error constants. Now errors can be matched easily/directly.

Token simplified to create an Option<Result<ServerResponse>> instead of individual components.

Created new enumeration types:

MqttVersion

ConnectReturnCode (for MQTT v3.x)

QoS

#181 Updated README.md with explanation on how to build for apple m1/x86_64

#216 Deref QoS pointers for SubscribeMany and UnsubscribeMany in server response

#224 Fix QoS 0 and 1 conversion

#236 Make from_c_parts only visible inside crate

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

🎉
4
Notgnoshi, krakjn, ralvescosta, and jspngh reacted with hooray emoji

All reactions

🎉
4 reactions

4 people reacted

Version 0.12.5

Version 0.12.5

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

fpagliughi

released this

25 May 13:51

v0.12.5

e6f4668

Some bug fixes and License update.

Updated the License to EPL-v2.0

Added some missing Eclipse Paho legal documents.

Added TopicMatcher::insert_many()

#216 Deref QoS pointers for SubscribeMany and UnsubscribeMany in server response

#215 Now subscribe_many() returns an error if slices not the same length. Also added subscribe_many_same_qos() to the clients.

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Version 0.12.4

Version 0.12.4

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

fpagliughi

released this

20 May 01:09

v0.12.4

a08a941

Fixes for topic matching

TopicMatcher

Fixed a number of corner cases

Iterator optimized

Added prune() and shrink_to_fit(), and get_key_value()

TopicFilter fixed corner cases

Added stand-alone topic_matches() and topic_matches_iter() functions from PR #228

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Version 0.12.3

Version 0.12.3

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

fpagliughi

released this

26 Oct 00:00

v0.12.3

cf953b9

Upgrade to Paho C v1.3.13 to fix a number of bugs, including numerous issues with reconnecting to the broker.

The -sys crate now wraps Paho C v1.3.13, fixing several issues, including crashes on reconnect callbacks.

Made the C logs less verbose

#203 AsyncClient::server_uri() getter.

#202 Fix disconnect timeout (from sec to ms)

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Version 0.12.2

Version 0.12.2

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

fpagliughi

released this

12 Sep 21:23

v0.12.2

15e33a6

#209 Forwarding trace/log statements from the Paho C library to the Rust logs

Minor cleanup of subscriber examples.

Assets
2

Loading

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Previous 1 2 3 Next

Previous Next

You can’t perform that action at this time.