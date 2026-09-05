---
title: Event in rumqttc - Rust
id: event-in-rumqttc-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-rumqttc-dependency-fitness
- docs
- rumqttc
- api-docs
created: '2026-09-02T13:21:38.359127Z'
updated: '2026-09-05T10:51:22.309603Z'
source: https://docs.rs/rumqttc/0.24.0/rumqttc/enum.Event.html
source_domain: docs.rs
fetched_at: '2026-09-02T13:21:38.358043Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'docs.rs rumqttc 0.24.0 enum.Event page: Event has exactly two variants —
  Incoming(Incoming) and Outgoing(Outgoing). There is NO Reconnect variant in 0.24.0
  (Event::Reconnect only exists in unmerged PR #1052). Consequently a reconnect is
  only observable as a second Event::Incoming(Packet::ConnAck) in the poll() loop;
  applications wanting re-subscription must count CONNACKs or match incoming ConnAck
  packets themselves.'
---

Event in rumqttc - Rust

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
Enum EventCopy item pathSource pub enum Event {
Incoming(Incoming),
Outgoing(Outgoing),
}Expand description

Events which can be yielded by the event loop

Variants§
§
Incoming(Incoming)§
Outgoing(Outgoing)
Trait Implementations§
Source§
impl Clone for Event
Source§
fn clone(&self) -> Event
Returns a duplicate of the value. Read more1.0.0 · Source§
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read moreSource§
impl Debug for Event
Source§
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read moreSource§
impl PartialEq for Event
Source§
fn eq(&self, other: &Event) -> bool
Tests for self and other values to be equal, and is used by ==.1.0.0 · Source§
fn ne(&self, other: &Rhs) -> bool
Tests for !=. The default implementation is almost always sufficient,
and should not be overridden without very good reason.Source§
impl Eq for EventSource§
impl StructuralPartialEq for Event
Auto Trait Implementations§
§
impl !Freeze for Event§
impl RefUnwindSafe for Event§
impl Send for Event§
impl Sync for Event§
impl Unpin for Event§
impl UnwindSafe for Event
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