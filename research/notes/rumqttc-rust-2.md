---
title: rumqttc - Rust
id: rumqttc-rust-2
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-mqtt-lifecycle-supervisor-spec
created: '2026-09-02T09:58:27.462579Z'
updated: '2026-09-02T17:37:22.470127Z'
source: https://docs.rs/rumqttc/0.24.0/rumqttc/index.html
source_domain: docs.rs
fetched_at: '2026-09-02T09:58:27.461294Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'rumqttc 0.24.0 crate front page (the pinned version). Feature claims verbatim:
  ''Automatic reconnections by just continuing the eventloop.poll()/connection.iter()
  loop'' and ''Natural backpressure to client APIs during bad network''; eventloop
  ''Pings the broker when necessary and detects client side half open connections
  as well''; ''Queue size based flow control on outgoing packets''; throttling of
  outgoing packets is a TODO. Critical operational contract: ''Looping on connection.iter()/eventloop.poll()
  is necessary to run the event loop and make progress'' and ''Blocking inside the
  connection.iter()/eventloop.poll() loop will block connection progress'' — the eventloop
  is externally polled, giving users access to internal state ''for use cases like
  graceful shutdown or to modify options before reconnection''. Re-exports mqttbytes
  v4 and v5 modules (mqttbytes::v5 exists in 0.24.0 for MQTT5 packet encoding) but
  MqttOptions is v4-centric. Sync Client and AsyncClient both provided; Client::new(mqttoptions,
  cap) takes a request-channel capacity.'
---

rumqttc - Rust

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

Crate rumqttcCopy item pathSource Expand description

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

Re-exports§pub use tokio_rustls;pub use mqttbytes::v4::*;pub use mqttbytes::*;
Modules§mqttbytesmqttbytesv5
Structs§AsyncClientAn asynchronous client, communicates with MQTT EventLoop.ClientA synchronous client, communicates with MQTT EventLoop.ConnectionMQTT connection. Maintains all the necessary stateEventLoopEventloop with all the state of a connectionIterIterator which polls the EventLoop for connection progressMqttOptionsOptions to configure the behaviour of MQTT connectionMqttStateState of the mqtt connection.NetworkOptionsProvides a way to configure low level network connection configurationsProxyRecvErrorError type returned by Connection::recv
Enums§ClientErrorClient ErrorConnectionErrorCritical errors during eventloop pollingEventEvents which can be yielded by the event loopOptionErrorOutgoingCurrent outgoing activity on the eventloopProxyAuthProxyTypeRecvTimeoutErrorError type returned by Connection::recv_timeoutRequestRequests by the client to mqtt event loop. Request are
handled one by one.StateErrorErrors during state handlingTlsConfigurationTLS configuration methodTlsErrorTransportTransport methods. Defaults to TCP.TryRecvErrorError type returned by Connection::try_recv
Type Aliases§Incoming