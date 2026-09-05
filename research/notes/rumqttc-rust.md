---
title: rumqttc - Rust
id: rumqttc-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- rumqtt
- rumqttc
- crate-docs
- official-docs
created: '2026-09-02T04:02:29.641804Z'
updated: '2026-09-02T17:37:21.868535Z'
source: https://docs.rs/rumqttc/latest/rumqttc/
source_domain: docs.rs
fetched_at: '2026-09-02T04:02:24.741809Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'docs.rs page for rumqttc 0.25.1 (built 1 Sept 2026, Apache-2.0, repo bytebeamio/rumqtt,
  2.2k stars): pure-Rust async (tokio) MQTT client. Load-bearing feature claims from
  the crate front page: ''Eventloop orchestrates outgoing/incoming packets concurrently
  and handles the state''; ''Pings the broker when necessary and detects client side
  half open connections as well''; ''Automatic reconnections by just continuing the
  eventloop.poll()/connection.iter() loop''; ''Natural backpressure to client APIs
  during bad network''; queue-size-based flow control on outgoing packets; throttling
  of outgoing packets still a TODO. Only 35.34% of the crate is documented. AsyncClient::new(mqttoptions,
  cap) returns (client, eventloop); caller polls eventloop in a loop and keeps polling
  after errors to get auto-reconnect. Note for the agent project: the crate''s reconnect
  guarantee is only as good as the caller''s poll loop — if the poll task panics or
  stops, connectivity silently dies; and re-subscription after reconnect is NOT part
  of the advertised feature list.'
---

rumqttc - Rust

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
Crate rumqttc

Crate rumqttc Copy item pathSource Expand description

A pure rust MQTT client which strives to be robust, efficient and easy to use.
This library is backed by an async (tokio) eventloop which handles all the
robustness and and efficiency parts of MQTT but naturally fits into both sync
and async worlds as we’ll see

Let’s jump into examples right away

§A simple synchronous publish and subscribe

use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;
use std::thread;

let mut mqttoptions = MqttOptions::new("rumqtt-sync", "test.mosquitto.org", 1883);
mqttoptions.set_keep_alive(Duration::from_secs(5));

let (mut client, mut connection) = Client::new(mqttoptions, 10);
client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
thread::spawn(move || for i in 0..10 {
client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();
thread::sleep(Duration::from_millis(100));
});

// Iterate to poll the eventloop for connection progress
for (i, notification) in connection.iter().enumerate() {
println!("Notification = {:?}", notification);
}
§A simple asynchronous publish and subscribe

use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};
use std::time::Duration;
use std::error::Error;

let mut mqttoptions = MqttOptions::new("rumqtt-async", "test.mosquitto.org", 1883);
mqttoptions.set_keep_alive(Duration::from_secs(5));

let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();

task::spawn(async move {
for i in 0..10 {
client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).await.unwrap();
time::sleep(Duration::from_millis(100)).await;
}
});

loop {
let notification = eventloop.poll().await.unwrap();
println!("Received = {:?}", notification);
}

Quick overview of features

Eventloop orchestrates outgoing/incoming packets concurrently and handles the state

Pings the broker when necessary and detects client side half open connections as well

Throttling of outgoing packets (todo)

Queue size based flow control on outgoing packets

Automatic reconnections by just continuing the eventloop.poll()/connection.iter() loop

Natural backpressure to client APIs during bad network

In short, everything necessary to maintain a robust connection

Since the eventloop is externally polled (with iter()/poll() in a loop)
out side the library and Eventloop is accessible, users can

Distribute incoming messages based on topics

Stop it when required

Access internal state for use cases like graceful shutdown or to modify options before reconnection

§Important notes

Looping on connection.iter()/eventloop.poll() is necessary to run the
event loop and make progress. It yields incoming and outgoing activity
notifications which allows customization as you see fit.

Blocking inside the connection.iter()/eventloop.poll() loop will block
connection progress.

§FAQ

Connecting to a broker using raw ip doesn’t work

You cannot create a TLS connection to a bare IP address with a self-signed
certificate. This is a limitation of rustls.
One workaround, which only works under *nix/BSD-like systems, is to add an
entry to wherever your DNS resolver looks (e.g. /etc/hosts) for the bare IP
address and use that name in your code.

Re-exports§pub use tokio_native_tls;use-native-tlspub use tokio_rustls;use-rustls-no-providerpub use mqttbytes::v4::*;pub use mqttbytes::*;
Modules§mqttbytesmqttbytesv5
Structs§AsyncClientAn asynchronous client, communicates with MQTT EventLoop.ClientA synchronous client, communicates with MQTT EventLoop.ConnectionMQTT connection. Maintains all the necessary stateEventLoopEventloop with all the state of a connectionIterIterator which polls the EventLoop for connection progressMqttOptionsOptions to configure the behaviour of MQTT connectionMqttStateState of the mqtt connection.NetworkOptionsProvides a way to configure low level network connection configurationsProxyproxyRecvErrorError type returned by Connection::recv
Enums§ClientErrorClient ErrorConnectionErrorCritical errors during eventloop pollingEventEvents which can be yielded by the event loopOptionErrorurlOutgoingCurrent outgoing activity on the eventloopProxyAuthproxyProxyTypeproxyRecvTimeoutErrorError type returned by Connection::recv_timeoutRequestRequests by the client to mqtt event loop. Request are
handled one by one.StateErrorErrors during state handlingTlsConfigurationuse-native-tls or use-rustls-no-providerTLS configuration methodTlsErroruse-native-tls or use-rustls-no-providerTransportTransport methods. Defaults to TCP.TryRecvErrorError type returned by Connection::try_recv
Type Aliases§Incoming