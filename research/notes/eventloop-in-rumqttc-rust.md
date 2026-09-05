---
title: EventLoop in rumqttc - Rust
id: eventloop-in-rumqttc-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- rust
- rumqttc
- repo-source
- reconnect
- api-docs
- crate-docs
- primary-source
- availability
created: '2026-09-02T06:41:31.047374Z'
updated: '2026-09-05T10:51:22.025907Z'
source: https://docs.rs/rumqttc/latest/rumqttc/struct.EventLoop.html
source_domain: docs.rs
fetched_at: '2026-09-02T06:41:24.833334Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'docs.rs API reference for rumqttc 0.25.1 (built 2026-09-01, Apache-2.0,
  owner tekjar/bytebeamio): the EventLoop struct owns all per-connection state (mqtt_options,
  MqttState, pending VecDeque<Request>, optional Network, NetworkOptions). poll()
  yields the next notification/outgoing request, periodically pings the broker, and
  — critically for a fleet agent — continuing to poll automatically reconnects after
  disconnection; blocking the iteration is explicitly warned against. clean() moves
  unacked packets from the last session back into the eventloop for republish (MQTT
  spec requires QoS>=1 redelivery), drops the network connection, and clears keepalive
  timeout; the docs note it should be used only when the eventloop is blocked on network,
  and recommend setting AsyncClient channel capacity to 0 so the pending list length
  is managed properly. EventLoop is Send but !Sync. For ha-linux-agent this is the
  ground-truth reconnect/redelivery contract: the agent''s connection supervisor must
  keep polling through errors rather than tearing the loop down.'
---

EventLoop in rumqttc - Rust

Docs.rs

rumqttc-0.25.1

rumqttc 0.25.1

Permalink

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
EventLoop

rumqttc
Struct EventLoop Copy item pathSource pub struct EventLoop {
pub mqtt_options: MqttOptions,
pub state: MqttState,
pub pending: VecDeque<Request>,
pub network: Option<Network>,
pub network_options: NetworkOptions,
/* private fields */
}Expand description

Eventloop with all the state of a connection

Fields§§mqtt_options: MqttOptions

Options of the current mqtt connection
§state: MqttState

Current state of the connection
§pending: VecDeque<Request>

Pending packets from last session
§network: Option<Network>

Network connection to the broker
§network_options: NetworkOptions
Implementations§
Source§
impl EventLoop
Source
pub fn new(mqtt_options: MqttOptions, cap: usize) -> EventLoop

New MQTT EventLoop

When connection encounters critical errors (like auth failure), user has a choice to
access and update options, state and requests.
Source
pub fn clean(&mut self)

Last session might contain packets which aren’t acked. MQTT says these packets should be
republished in the next session. Move pending messages from state to eventloop, drops the
underlying network connection and clears the keepalive timeout if any.

NOTE: Use only when EventLoop is blocked on network and unable to immediately handle disconnect.
Also, while this helps prevent data loss, the pending list length should be managed properly.
For this reason we recommend setting AsycClient’s channel capacity to 0.

Source
pub async fn poll(&mut self) -> Result<Event, ConnectionError>

Yields Next notification or outgoing request and periodically pings
the broker. Continuing to poll will reconnect to the broker if there is
a disconnection.
NOTE Don’t block this while iterating
Source
pub fn network_options(&self) -> NetworkOptionsSource
pub fn set_network_options(
&mut self,
network_options: NetworkOptions,
) -> &mut Self
Auto Trait Implementations§
§
impl !Freeze for EventLoop§
impl !RefUnwindSafe for EventLoop§
impl !Sync for EventLoop§
impl !UnwindSafe for EventLoop§
impl Send for EventLoop§
impl Unpin for EventLoop§
impl UnsafeUnpin for EventLoop
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