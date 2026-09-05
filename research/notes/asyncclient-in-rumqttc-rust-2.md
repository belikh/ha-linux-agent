---
title: AsyncClient in rumqttc - Rust
id: asyncclient-in-rumqttc-rust-2
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-rumqttc-dependency-fitness
- docs
- rumqttc
- api-docs
created: '2026-09-02T13:17:49.608346Z'
updated: '2026-09-02T17:37:22.563767Z'
source: https://docs.rs/rumqttc/0.25.1/rumqttc/struct.AsyncClient.html
source_domain: docs.rs
fetched_at: '2026-09-02T13:17:49.606852Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'docs.rs rumqttc 0.25.1 struct.AsyncClient page (docs built 01 Sep 2026):
  the 0.25.1 AsyncClient API surface is method-for-method IDENTICAL to 0.24.0''s —
  new, from_senders, publish/try_publish, ack/try_ack, publish_bytes, subscribe/try_subscribe,
  subscribe_many/try_subscribe_many, unsubscribe/try_unsubscribe, disconnect/try_disconnect.
  0.25.1 adds NO auto-resubscribe, no reconnect-signal, no v5-session method on AsyncClient.
  Key diff vs 0.24.0 is under the hood: fixedbitset ^0.5.7 (QoS2 inflight tracking),
  tokio-stream/tokio-util, thiserror ^2.0.8, async-tungstenite ^0.29, ws_stream_tungstenite
  ^0.15. Documentation coverage only 35.34%. Conclusion for the dependency-fitness
  locus: upgrading 0.24.0 -> 0.25.1 changes nothing at the AsyncClient API level;
  the reconnect-resubscribe gap persists in both.'
---

AsyncClient in rumqttc - Rust

Docs.rs

rumqttc-0.25.1

rumqttc 0.25.1

Docs.rs crate page

Apache-2.0

01 September 2026

Links

Repository

crates.io

Source

Owners

tekjar

Dependencies

async-http-proxy ^1.2.5

normal

optional

async-tungstenite ^0.29.0

normal

optional

bytes ^1.5

normal

fixedbitset ^0.5.7

normal

flume ^0.11

normal

futures-util ^0.3

normal

http ^1.0.0

normal

optional

log ^0.4

normal

native-tls ^0.2.12

normal

optional

rustls-native-certs ^0.8.1

normal

optional

rustls-pemfile ^2.2.0

normal

optional

rustls-webpki ^0.102.8

normal

optional

thiserror ^2.0.8

normal

tokio ^1.36

normal

tokio-native-tls ^0.3.1

normal

optional

tokio-rustls ^0.26.0

normal

optional

tokio-stream ^0.1.16

normal

tokio-util ^0.7

normal

url ^2

normal

optional

ws_stream_tungstenite ^0.15.0

normal

optional

bincode ^1.3.3

dev

color-backtrace ^0.6.1

dev

matches ^0.1

dev

pretty_assertions ^1

dev

pretty_env_logger ^0.5

dev

serde ^1

dev

Versions

35.34%
of the crate is documented

Platform

x86_64-unknown-linux-gnu

Feature flags

docs.rs

About docs.rs
Badges
Builds
Metadata
Shorthand URLs
Download
Rustdoc JSON
Build queue
Privacy policy

Rust

Rust website

The Book

Standard Library API Reference

Rust by Example

The Cargo Guide

Clippy Documentation

Skip to main content
AsyncClient

rumqttc
Struct AsyncClient Copy item pathSource pub struct AsyncClient { /* private fields */ }Expand description

An asynchronous client, communicates with MQTT EventLoop.

This is cloneable and can be used to asynchronously publish,
subscribe through the EventLoop, which is to be polled parallelly.

NOTE: The EventLoop must be regularly polled in order to send, receive and process packets
from the broker, i.e. move ahead.

Implementations§
Source§
impl AsyncClient
Source
pub fn new(options: MqttOptions, cap: usize) -> (AsyncClient, EventLoop)

Create a new AsyncClient.

cap specifies the capacity of the bounded async channel.
Source
pub fn from_senders(request_tx: Sender<Request>) -> AsyncClient

Create a new AsyncClient from a channel Sender.

This is mostly useful for creating a test instance where you can
listen on the corresponding receiver.
Source
pub async fn publish<S, V>(
&self,
topic: S,
qos: QoS,
retain: bool,
payload: V,
) -> Result<(), ClientError>
where
S: Into<String>,
V: Into<Vec<u8>>,

Sends a MQTT Publish to the EventLoop.
Source
pub fn try_publish<S, V>(
&self,
topic: S,
qos: QoS,
retain: bool,
payload: V,
) -> Result<(), ClientError>
where
S: Into<String>,
V: Into<Vec<u8>>,

Attempts to send a MQTT Publish to the EventLoop.
Source
pub async fn ack(&self, publish: &Publish) -> Result<(), ClientError>

Sends a MQTT PubAck to the EventLoop. Only needed in if manual_acks flag is set.
Source
pub fn try_ack(&self, publish: &Publish) -> Result<(), ClientError>

Attempts to send a MQTT PubAck to the EventLoop. Only needed in if manual_acks flag is set.
Source
pub async fn publish_bytes<S>(
&self,
topic: S,
qos: QoS,
retain: bool,
payload: Bytes,
) -> Result<(), ClientError>
where
S: Into<String>,

Sends a MQTT Publish to the EventLoop
Source
pub async fn subscribe<S: Into<String>>(
&self,
topic: S,
qos: QoS,
) -> Result<(), ClientError>

Sends a MQTT Subscribe to the EventLoop
Source
pub fn try_subscribe<S: Into<String>>(
&self,
topic: S,
qos: QoS,
) -> Result<(), ClientError>

Attempts to send a MQTT Subscribe to the EventLoop
Source
pub async fn subscribe_many<T>(&self, topics: T) -> Result<(), ClientError>
where
T: IntoIterator<Item = SubscribeFilter>,

Sends a MQTT Subscribe for multiple topics to the EventLoop
Source
pub fn try_subscribe_many<T>(&self, topics: T) -> Result<(), ClientError>
where
T: IntoIterator<Item = SubscribeFilter>,

Attempts to send a MQTT Subscribe for multiple topics to the EventLoop
Source
pub async fn unsubscribe<S: Into<String>>(
&self,
topic: S,
) -> Result<(), ClientError>

Sends a MQTT Unsubscribe to the EventLoop
Source
pub fn try_unsubscribe<S: Into<String>>(
&self,
topic: S,
) -> Result<(), ClientError>

Attempts to send a MQTT Unsubscribe to the EventLoop
Source
pub async fn disconnect(&self) -> Result<(), ClientError>

Sends a MQTT disconnect to the EventLoop
Source
pub fn try_disconnect(&self) -> Result<(), ClientError>

Attempts to send a MQTT disconnect to the EventLoop

Trait Implementations§
Source§
impl Clone for AsyncClient
Source§
fn clone(&self) -> AsyncClient
Returns a duplicate of the value. Read more1.0.0 (const: unstable) · Source§
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read moreSource§
impl Debug for AsyncClient
Source§
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read more
Auto Trait Implementations§
§
impl Freeze for AsyncClient§
impl RefUnwindSafe for AsyncClient§
impl Send for AsyncClient§
impl Sync for AsyncClient§
impl Unpin for AsyncClient§
impl UnsafeUnpin for AsyncClient§
impl UnwindSafe for AsyncClient
Blanket Implementations§
Source§
impl<T> Any for T
where
T: 'static + ?Sized,
Source§
fn type_id(&self) -> TypeId
Gets the TypeId of self. Read moreSource§
impl<T> Borrow<T> for T
where
T: ?Sized,
Source§
fn borrow(&self) -> &T
Immutably borrows from an owned value. Read moreSource§
impl<T> BorrowMut<T> for T
where
T: ?Sized,
Source§
fn borrow_mut(&mut self) -> &mut T
Mutably borrows from an owned value. Read moreSource§
impl<T> CloneToUninit for T
where
T: Clone,
Source§
unsafe fn clone_to_uninit(&self, dest: *mut u8)
🔬This is a nightly-only experimental API. (clone_to_uninit)
Performs copy-assignment from self to dest. Read moreSource§
impl<T> From<T> for T
Source§
fn from(t: T) -> T

Returns the argument unchanged.
Source§
impl<T> Instrument for T
Source§
fn instrument(self, span: Span) -> Instrumented<Self> ⓘ
Instruments this type with the provided Span, returning an
Instrumented wrapper. Read moreSource§
fn in_current_span(self) -> Instrumented<Self> ⓘ
Instruments this type with the current Span, returning an
Instrumented wrapper. Read moreSource§
impl<T, U> Into<U> for T
where
U: From<T>,
Source§
fn into(self) -> U

Calls U::from(self).

That is, this conversion is whatever the implementation of
From<T> for U chooses to do.
Source§
impl<T> Same for T
Source§
type Output = T
Should always be SelfSource§
impl<T> ToOwned for T
where
T: Clone,
Source§
type Owned = T
The resulting type after obtaining ownership.Source§
fn to_owned(&self) -> T
Creates owned data from borrowed data, usually by cloning. Read moreSource§
fn clone_into(&self, target: &mut T)
Uses borrowed data to replace owned data, usually by cloning. Read moreSource§
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
Source§
type Error = !
The type returned in the event of a conversion error.Source§
fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error>
Performs the conversion.Source§
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
Source§
type Error = <U as TryFrom<T>>::Error
The type returned in the event of a conversion error.Source§
fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error>
Performs the conversion.Source§
impl<V, T> VZip<V> for T
where
V: MultiLane<T>,
Source§
fn vzip(self) -> VSource§
impl<T> WithSubscriber for T
Source§
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> ⓘ
where
S: Into<Dispatch>,
Attaches the provided Subscriber to this type, returning a
WithDispatch wrapper. Read moreSource§
fn with_current_subscriber(self) -> WithDispatch<Self> ⓘ
Attaches the current default Subscriber to this type, returning a
WithDispatch wrapper. Read more