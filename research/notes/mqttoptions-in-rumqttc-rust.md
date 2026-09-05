---
title: MqttOptions in rumqttc - Rust
id: mqttoptions-in-rumqttc-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-mqtt-lifecycle-supervisor-spec
created: '2026-09-02T09:57:04.383659Z'
updated: '2026-09-05T10:51:22.197676Z'
source: https://docs.rs/rumqttc/0.24.0/rumqttc/struct.MqttOptions.html
source_domain: docs.rs
fetched_at: '2026-09-02T09:57:04.382246Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'rumqttc 0.24.0 MqttOptions API docs (the EXACT version pinned in ha-linux-agent''s
  Cargo.lock; built 18 July 2025, 33.87% documented). Full connection-config surface
  for the agent: MqttOptions::new(id, host, port); parse_url behind ''url'' feature
  (mqtt:// ssl:// ws:// prefixes); set_last_will(LastWill)/set_transport/set_keep_alive(Duration)
  — keep_alive pings broker ''if there is no other data exchange''; set_clean_session(bool)
  with documented semantics ''clean_session = true removes all the state from queues
  & instructs the broker to clean all the client state when client disconnects'' vs
  false = broker holds client state + ''Local queue state is also held to retransmit
  packets after reconnection''; PANICS if clean_session=false with empty client_id.
  Also set_max_packet_size(incoming, outgoing), set_credentials, set_request_channel_capacity,
  set_pending_throttle (outgoing message rate), set_inflight(u16) concurrent in-flight
  messages, set_manual_acks, set_proxy, set_request_modifier. No MQTT5 session-expiry/clean-start
  API in 0.24 — this is a v3.1.1-style session flag.'
---

MqttOptions in rumqttc - Rust

Docs.rs

rumqttc-0.24.0

rumqttc 0.24.0

Docs.rs crate page

Apache-2.0

18 July 2025

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

async-tungstenite ^0.25.0

normal

optional

bytes ^1.5

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

native-tls ^0.2.11

normal

optional

rustls-native-certs ^0.7.0

normal

optional

rustls-pemfile ^2.1.0

normal

optional

rustls-webpki ^0.102.2

normal

optional

thiserror ^1

normal

tokio ^1.36

normal

tokio-native-tls ^0.3.1

normal

optional

tokio-rustls ^0.25.0

normal

optional

url ^2

normal

optional

ws_stream_tungstenite ^0.13.0

normal

optional

bincode ^1.3.3

dev

color-backtrace ^0.5

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

33.87%
of the crate is documented

Go to latest version

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

rumqttc
Struct MqttOptionsCopy item pathSource pub struct MqttOptions { /* private fields */ }Expand description

Options to configure the behaviour of MQTT connection

Implementations§
Source§
impl MqttOptions
Source
pub fn new<S: Into<String>, T: Into<String>>(
id: S,
host: T,
port: u16,
) -> MqttOptions

Create an MqttOptions object that contains default values for all settings other than

id: A string to identify the device connecting to a broker

host: The broker’s domain name or IP address

port: The port number on which broker must be listening for incoming connections

let options = MqttOptions::new("123", "localhost", 1883);
Source
pub fn parse_url<S: Into<String>>(url: S) -> Result<MqttOptions, OptionError>

Creates an MqttOptions object by parsing provided string with the url crate’s
Url::parse(url) method and is only enabled when run using the “url” feature.

let options = MqttOptions::parse_url("mqtt://example.com:1883?client_id=123").unwrap();

NOTE: A url must be prefixed with one of either tcp://, mqtt://, ssl://,mqtts://,
ws:// or wss:// to denote the protocol for establishing a connection with the broker.

NOTE: Encrypted connections(i.e. mqtts://, ssl://, wss://) by default use the
system’s root certificates. To configure with custom certificates, one may use the
set_transport method.

ⓘlet mut options = MqttOptions::parse_url("mqtts://example.com?client_id=123").unwrap();
options.set_transport(Transport::tls_with_config(client_config.into()));
Source
pub fn broker_address(&self) -> (String, u16)

Broker address
Source
pub fn set_last_will(&mut self, will: LastWill) -> &mut SelfSource
pub fn last_will(&self) -> Option<LastWill>Source
pub fn set_transport(&mut self, transport: Transport) -> &mut SelfSource
pub fn transport(&self) -> TransportSource
pub fn set_keep_alive(&mut self, duration: Duration) -> &mut Self

Set number of seconds after which client should ping the broker
if there is no other data exchange
Source
pub fn keep_alive(&self) -> Duration

Keep alive time
Source
pub fn client_id(&self) -> String

Client identifier
Source
pub fn set_max_packet_size(
&mut self,
incoming: usize,
outgoing: usize,
) -> &mut Self

Set packet size limit for outgoing an incoming packets
Source
pub fn max_packet_size(&self) -> usize

Maximum packet size
Source
pub fn set_clean_session(&mut self, clean_session: bool) -> &mut Self

clean_session = true removes all the state from queues & instructs the broker
to clean all the client state when client disconnects.

When set false, broker will hold the client state and performs pending
operations on the client when reconnection with same client_id
happens. Local queue state is also held to retransmit packets after reconnection.

§Panic

Panics if clean_session is false when client_id is empty.

ⓘlet mut options = MqttOptions::new("", "localhost", 1883);
options.set_clean_session(false);
Source
pub fn clean_session(&self) -> bool

Clean session
Source
pub fn set_credentials<U: Into<String>, P: Into<String>>(
&mut self,
username: U,
password: P,
) -> &mut Self

Username and password
Source
pub fn credentials(&self) -> Option<(String, String)>

Security options
Source
pub fn set_request_channel_capacity(&mut self, capacity: usize) -> &mut Self

Set request channel capacity
Source
pub fn request_channel_capacity(&self) -> usize

Request channel capacity
Source
pub fn set_pending_throttle(&mut self, duration: Duration) -> &mut Self

Enables throttling and sets outoing message rate to the specified ‘rate’
Source
pub fn pending_throttle(&self) -> Duration

Outgoing message rate
Source
pub fn set_inflight(&mut self, inflight: u16) -> &mut Self

Set number of concurrent in flight messages
Source
pub fn inflight(&self) -> u16

Number of concurrent in flight messages
Source
pub fn set_manual_acks(&mut self, manual_acks: bool) -> &mut Self

set manual acknowledgements
Source
pub fn manual_acks(&self) -> bool

get manual acknowledgements
Source
pub fn set_proxy(&mut self, proxy: Proxy) -> &mut SelfSource
pub fn proxy(&self) -> Option<Proxy>Source
pub fn set_request_modifier<F, O>(&mut self, request_modifier: F) -> &mut Self
where
F: Fn(Request<()>) -> O + Send + Sync + 'static,
O: IntoFuture<Output = Request<()>> + 'static,
O::IntoFuture: Send,Source
pub fn request_modifier(
&self,
) -> Option<Arc<dyn Fn(Request<()>) -> Pin<Box<dyn Future<Output = Request<()>> + Send>> + Send + Sync>>
Trait Implementations§
Source§
impl Clone for MqttOptions
Source§
fn clone(&self) -> MqttOptions
Returns a duplicate of the value. Read more1.0.0 · Source§
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read moreSource§
impl Debug for MqttOptions
Source§
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read moreSource§
impl TryFrom<Url> for MqttOptions
Source§
type Error = OptionError
The type returned in the event of a conversion error.Source§
fn try_from(url: Url) -> Result<Self, Self::Error>
Performs the conversion.
Auto Trait Implementations§
§
impl !Freeze for MqttOptions§
impl !RefUnwindSafe for MqttOptions§
impl Send for MqttOptions§
impl Sync for MqttOptions§
impl Unpin for MqttOptions§
impl !UnwindSafe for MqttOptions
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
fn instrument(self, span: Span) -> Instrumented<Self>
Instruments this type with the provided Span, returning an
Instrumented wrapper. Read moreSource§
fn in_current_span(self) -> Instrumented<Self>
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
type Error = Infallible
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
fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>
where
S: Into<Dispatch>,
Attaches the provided Subscriber to this type, returning a
WithDispatch wrapper. Read moreSource§
fn with_current_subscriber(self) -> WithDispatch<Self>
Attaches the current default Subscriber to this type, returning a
WithDispatch wrapper. Read moreSource§
impl<T> ErasedDestructor for T
where
T: 'static,