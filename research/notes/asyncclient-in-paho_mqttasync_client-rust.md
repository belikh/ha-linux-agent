---
title: AsyncClient in paho_mqtt::async_client - Rust
id: asyncclient-in-paho_mqttasync_client-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-rumqttc-dependency-fitness
- alternatives
- paho-mqtt
- api-docs
created: '2026-09-02T13:35:44.702922Z'
updated: '2026-09-02T17:37:22.580342Z'
source: https://docs.rs/paho-mqtt/latest/paho_mqtt/async_client/struct.AsyncClient.html
source_domain: docs.rs
fetched_at: '2026-09-02T13:35:44.701204Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'docs.rs paho-mqtt 0.14.0 AsyncClient API (docs built 05 July 2026): confirms
  the callback-driven reconnect/resubscribe pattern. Methods: reconnect() — ''Attempts
  to reconnect to the broker. This can only be called after a connection was initially
  made or attempted. It will retry with the same connect options''; reconnect_with_callbacks();
  set_connected_callback(FnMut(&AsyncClient)); set_connection_lost_callback(); set_disconnected_callback(FnMut(&AsyncClient,
  Properties, ReasonCode)); set_message_callback(); get_stream()/get_event_stream()
  (None signalled on disconnect; ''It''s a best practice to open the stream before
  connecting... When using persistent (non-clean) sessions, messages could arriving
  as soon as the connection is made''). Subscribe surface: subscribe, subscribe_many,
  subscribe_many_same_qos, subscribe_with_options, subscribe_many_with_options. There
  is NO automatic re-subscription method — the Paho pattern is: enable automatic reconnect
  in ConnectOptions, then re-subscribe from the connected callback (set_connected_callback)
  after each reconnect. tokio ^1.49 is an OPTIONAL dependency (runtime-agnostic core).'
---

*Suggested by [[github-eclipse-pahopahomqttrust-pahomqttrust-github]] — README claims Automatic Reconnect; AsyncClient docs needed to verify reconnect callbacks and re-subscription pattern*

AsyncClient in paho_mqtt::async_client - Rust

Docs.rs

paho-mqtt-0.14.0

paho-mqtt 0.14.0

Permalink

Docs.rs crate page

EPL-2.0

05 July 2026

Links

Homepage

Repository

crates.io

Source

Owners

fpagliughi

Dependencies

async-channel ^2.5

normal

crossbeam-channel ^0.5

normal

futures ^0.3

normal

futures-timer ^3.0

normal

libc ^0.2

normal

log ^0.4

normal

paho-mqtt-sys ^0.11

normal

thiserror ^2.0

normal

tokio ^1.49

normal

optional

ctrlc ^3.2

dev

env_logger ^0.11

dev

futures-util ^0.3

dev

lazy_static ^1.4

dev

serde ^1.0

dev

serde_json ^1.0

dev

smol ^2.0

dev

toml ^0.9

dev

Versions

100%
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

paho_mqtt::async_client
Struct AsyncClient Copy item pathSource pub struct AsyncClient { /* private fields */ }Expand description

An asynchronous MQTT connection client.

Implementations§
Source§
impl AsyncClient
Source
pub fn new<T>(opts: T) -> Result<AsyncClient>
where
T: Into<CreateOptions>,

Creates a new MQTT client which can connect to an MQTT broker.

§Arguments

opts The create options for the client.

Source
pub fn mqtt_version(&self) -> MqttVersion

Gets the most recent MQTT version for the client.

This is the version of the current connection, or the most recent
connection if currently disconnected. Before an initial connection
is made, this will report MQTT_VERSION_DEFAULT (0).
Source
pub fn mqtt_version_raw(&self) -> u32

The raw, integer value of the MQTT version
Source
pub fn user_data(&self) -> Option<&UserData>

Get access to the user-defined data in the client.

This returns a reference to a read/write lock around the user data so
that the application can access the data, as needed from any outside
thread or a callback.

Note that it’s up to the application to ensure that it doesn’t
deadlock the callback thread when accessing the user data.
Source
pub fn connect_options(&self) -> ConnectOptions

Gets a copy of the options used in the last connection attempt.
Source
pub fn connect<T>(&self, opts: T) -> ConnectToken
where
T: Into<Option<ConnectOptions>>,

Connects to an MQTT broker using the specified connect options.

§Arguments

opts The connect options. This can be None, in which case the
default options are used.

Source
pub fn connect_with_callbacks<FS, FF>(
&self,
opts: ConnectOptions,
success_cb: FS,
failure_cb: FF,
) -> ConnectToken
where
FS: Fn(&AsyncClient, u16) + Send + 'static,
FF: Fn(&AsyncClient, u16, i32) + Send + 'static,

Connects to an MQTT broker using the specified connect options.

§Arguments

opts The connect options

success_cb The callback for a successful connection.

failure_cb The callback for a failed connection attempt.

Source
pub fn reconnect(&self) -> ConnectToken

Attempts to reconnect to the broker.
This can only be called after a connection was initially made or
attempted. It will retry with the same connect options.
Source
pub fn reconnect_with_callbacks<FS, FF>(
&self,
success_cb: FS,
failure_cb: FF,
) -> ConnectToken
where
FS: Fn(&AsyncClient, u16) + Send + 'static,
FF: Fn(&AsyncClient, u16, i32) + Send + 'static,

Attempts to reconnect to the broker, using callbacks to signal
completion.
This can only be called after a connection was initially made or
attempted. It will retry with the same connect options.

§Arguments

success_cb The callback for a successful connection.

failure_cb The callback for a failed connection attempt.

Source
pub fn disconnect<T>(&self, opt_opts: T) -> Token ⓘ
where
T: Into<Option<DisconnectOptions>>,

Disconnects from the MQTT broker.

§Arguments

opt_opts Optional disconnect options. Specifying None will use
default of immediate (zero timeout) disconnect.

Source
pub fn disconnect_after(&self, timeout: Duration) -> Token ⓘ

Disconnect from the MQTT broker with a timeout.
This will delay the disconnect for up to the specified timeout to
allow in-flight messages to complete.
This is the same as calling disconnect with options specifying a
timeout.

§Arguments

timeout The amount of time to wait for the disconnect. This has
a resolution in milliseconds.

Source
pub fn is_connected(&self) -> bool

Determines if this client is currently connected to an MQTT broker.
Source
pub fn set_connected_callback<F>(&self, cb: F)
where
F: FnMut(&AsyncClient) + Send + 'static,

Sets the callback for when the connection is established with the broker.

§Arguments

cb The callback to register with the library. This can be a
function or a closure.

Source
pub fn remove_connected_callback(&self)

Removes the callback for when the conection is established
Source
pub fn set_connection_lost_callback<F>(&self, cb: F)
where
F: FnMut(&AsyncClient) + Send + 'static,

Sets the callback for when the connection is lost with the broker.

§Arguments

cb The callback to register with the library. This can be a
function or a closure.

Source
pub fn remove_connection_lost_callback(&self)

Removes the callback for when the connection is lost
Source
pub fn set_disconnected_callback<F>(&self, cb: F)
where
F: FnMut(&AsyncClient, Properties, ReasonCode) + Send + 'static,

Sets the callback for when a disconnect message arrives from the broker.

§Arguments

cb The callback to register with the library. This can be a
function or a closure.

Source
pub fn remove_disconnected_callback(&self)

Removes the callback for when a disconnect message is received from the broker.
Source
pub fn set_message_callback<F>(&self, cb: F)
where
F: FnMut(&AsyncClient, Option<Message>) + Send + 'static,

Sets the callback for when a message arrives from the broker.

§Arguments

cb The callback to register with the library. This can be a
function or a closure.

Source
pub fn remove_message_callback(&self)

Removes the callback for when a message arrives from the broker.
Source
pub fn try_publish(&self, msg: Message) -> Result<DeliveryToken>

Attempts to publish a message to the MQTT broker, but returns an
error immediately if there’s a problem creating or queuing the
message.

Returns a Publish Error on failure so that the original message
can be recovered and sent again.
Source
pub fn publish(&self, msg: Message) -> DeliveryToken ⓘ

Publishes a message to the MQTT broker.

Returns a Delivery Token to track the progress of the operation.
Source
pub fn subscribe<S, Q>(&self, topic: S, qos: Q) -> SubscribeToken
where
S: Into<String>,
Q: Into<QoS>,

Subscribes to a single topic.

§Arguments

topic The topic name

qos The quality of service requested for messages

Source
pub fn subscribe_with_options<S, Q, T, P>(
&self,
topic: S,
qos: Q,
opts: T,
props: P,
) -> SubscribeToken
where
S: Into<String>,
Q: Into<QoS>,
T: Into<SubscribeOptions>,
P: Into<Option<Properties>>,

Subscribes to a single topic with v5 options

§Arguments

topic The topic name

qos The quality of service requested for messages

opts Options for the subscription

props MQTT v5 properties

Source
pub fn subscribe_many<T, Q>(
&self,
topics: &[T],
qos: &[Q],
) -> SubscribeManyToken
where
T: AsRef<str>,
Q: Into<QoS> + Copy,

Subscribes to multiple topics simultaneously.

§Arguments

topics The collection of topic names

qos The quality of service requested for messages

Source
pub fn subscribe_many_same_qos<T>(
&self,
topics: &[T],
qos: i32,
) -> SubscribeManyToken
where
T: AsRef<str>,

Subscribes to multiple topics simultaneously using the same QoS
for all of them.

§Arguments

topics The collection of topic names

qos The quality of service requested for all messages

Source
pub fn subscribe_many_with_options<T, Q, P>(
&self,
topics: &[T],
qos: &[Q],
opts: &[SubscribeOptions],
props: P,
) -> SubscribeManyToken
where
T: AsRef<str>,
Q: Into<QoS> + Copy,
P: Into<Option<Properties>>,

Subscribes to multiple topics simultaneously with options.

§Arguments

topics The collection of topic names

qos The quality of service requested for messages

opts Subscribe options (one per topic)

props MQTT v5 properties

Source
pub fn unsubscribe<S>(&self, topic: S) -> Token ⓘ
where
S: Into<String>,

Unsubscribes from a single topic.

§Arguments

topic The topic to unsubscribe. It must match a topic from a
previous subscribe.

Source
pub fn unsubscribe_with_options<S>(&self, topic: S, props: Properties) -> Token ⓘ
where
S: Into<String>,

Unsubscribes from a single topic.

§Arguments

topic The topic to unsubscribe. It must match a topic from a
previous subscribe.

props MQTT v5 properties for the unsubscribe.

Source
pub fn unsubscribe_many<T>(&self, topics: &[T]) -> Token ⓘ
where
T: AsRef<str>,

Unsubscribes from multiple topics simultaneously.

§Arguments

topic The topics to unsubscribe. Each must match a topic from a
previous subscribe.

Source
pub fn unsubscribe_many_with_options<T>(
&self,
topics: &[T],
props: Properties,
) -> Token ⓘ
where
T: AsRef<str>,

Unsubscribes from multiple topics simultaneously.

§Arguments

topic The topics to unsubscribe. Each must match a topic from a
previous subscribe.

props MQTT v5 properties for the unsubscribe.

Source
pub fn start_consuming(&self) -> SyncReceiver<Option<Message>>

Starts the client consuming messages for a blocking (non-async) app.

This starts the client receiving messages and placing them into a
channel. It returns the receiving-end of the channel for the
application to get the messages.

This should normall be called before the client is connected,
especially if the application is requesting a persistent session.
With a clean session, this should be called before subscribing to
ensure that messages are not lost.
Source
pub fn stop_consuming(&self)

Stops the client from consuming messages.
Source
pub fn start_consuming_events<L>(
&mut self,
buffer_lim: L,
) -> SyncReceiver<Event>
where
L: Into<Option<usize>>,

Creates a futures stream for consuming events.

This will install an internal callback to receive the incoming
events from the client, and return the receive side of the channel.
The stream will stay open for the life of the client.

The stream will rely on a bounded channel with the given buffer
capacity if ‘buffer_sz’ is ‘Some’ or will rely on an unbounded channel
if ‘buffer_sz’ is ‘None’.

It’s a best practice to open the stream before connecting to the
server. When using persistent (non-clean) sessions, messages could
arriving as soon as the connection is made - even before the
connect() call returns.
Source
pub fn get_stream<L>(&mut self, buffer_lim: L) -> AsyncReceiver<Option<Message>>
where
L: Into<Option<usize>>,

Creates a futures stream for consuming messages.

This will install an internal callback to receive the incoming
messages from the client, and return the receive side of the channel.
The stream will stay open for the life of the client. If the client
gets disconnected, it will insert None into the channel to signal
the app about the disconnect.

The stream will rely on a bounded channel with the given buffer
capacity if ‘buffer_sz’ is ‘Some’ or will rely on an unbounded channel
if ‘buffer_sz’ is ‘None’.

It’s a best practice to open the stream before connecting to the
server. When using persistent (non-clean) sessions, messages could
arriving as soon as the connection is made - even before the
connect() call returns.
Source
pub fn get_event_stream<L>(&mut self, buffer_lim: L) -> AsyncReceiver<Event>
where
L: Into<Option<usize>>,

Creates a futures stream for consuming events.

This will install an internal callback to receive the incoming
events from the client, and return the receive side of the channel.
The stream will stay open for the life of the client.

The stream will rely on a bounded channel with the given buffer
capacity if ‘buffer_sz’ is ‘Some’ or will rely on an unbounded channel
if ‘buffer_sz’ is ‘None’.

It’s a best practice to open the stream before connecting to the
server. When using persistent (non-clean) sessions, messages could
arriving as soon as the connection is made - even before the
connect() call returns.
Source
pub fn stop_stream(&self)

Stops the client from streaming messages in.
Source
pub fn client_id(&self) -> String

Returns client ID used for client instance

Client ID is returned as a rust String as set in a
CreateOptionsBuilder for symmetry
Source
pub fn server_uri(&self) -> String

Returns server URI used for connection

Server URI is returned as a rust String as set in a
CreateOptionsBuilder for symmetry

Trait Implementations§
Source§
impl Clone for AsyncClient
Source§
fn clone(&self) -> AsyncClient
Returns a duplicate of the value. Read more1.0.0 (const: unstable) · Source§
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read moreSource§
impl Send for AsyncClientSource§
impl Sync for AsyncClient
Auto Trait Implementations§
§
impl !RefUnwindSafe for AsyncClient§
impl !UnwindSafe for AsyncClient§
impl Freeze for AsyncClient§
impl Unpin for AsyncClient§
impl UnsafeUnpin for AsyncClient
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
impl<T, U> Into<U> for T
where
U: From<T>,
Source§
fn into(self) -> U

Calls U::from(self).

That is, this conversion is whatever the implementation of
From<T> for U chooses to do.
Source§
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
Performs the conversion.