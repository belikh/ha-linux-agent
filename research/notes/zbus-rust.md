---
title: zbus - Rust
id: zbus-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- source-code
- repo-source
- known-issue
- ha-linux-agent
- primary-source
- dbus
- rust
- api-docs
created: '2026-09-02T05:39:32.165371Z'
updated: '2026-09-05T10:51:21.961727Z'
source: https://docs.rs/zbus/latest/zbus/
source_domain: docs.rs
fetched_at: '2026-09-02T05:39:28.396251Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'docs.rs crate page for zbus 5.19.0 (MIT, published 09 August 2026, 93.65%
  documented, status: Stable): main subcrate of the zbus project providing the D-Bus
  API — ''It takes care of the establishment of a connection, the creation, sending
  and receiving of different kind of D-Bus messages (method calls, signals etc) for
  you.'' Pure-Rust with no C library dependency; supports tokio or any async runtime
  (async-io default, tokio optional), runs on Linux, FreeBSD, NetBSD, macOS, Windows
  (uds_windows), Android, and vsock (tokio-vsock) transports. Client pattern: #[proxy(interface,
  default_service, default_path)] trait MyGreeter { async fn say_hello(...) } generating
  MyGreeterProxy from Connection::session(). Deps include zbus_macros 5.19.0, zbus_names
  4.3.4, zvariant 5.14.0 — confirms current versioning for any ha-linux-agent D-Bus
  work.'
---

zbus - Rust

Docs.rs

zbus-5.19.0

zbus 5.19.0

Permalink

Docs.rs crate page

MIT

09 August 2026

Links

Repository

crates.io

Source

Owners

zeenix

Dependencies

async-broadcast ^0.7.0

normal

async-executor ^1.11.0

normal

optional

async-io ^2.3.2

normal

optional

async-lock ^3.3.0

normal

optional

async-process ^2.2.2

normal

optional

async-task ^4.7.1

normal

optional

async-trait ^0.1.80

normal

blocking ^1.6.0

normal

optional

enumflags2 ^0.7.9

normal

event-listener ^5.3.0

normal

futures-core ^0.3.30

normal

futures-lite ^2.6.0

normal

hex ^0.4.3

normal

ordered-stream ^0.2

normal

serde ^1.0.200

normal

serde_repr ^0.1.19

normal

tokio ^1.37.0

normal

optional

tracing ^0.1.40

normal

uuid ^1.8.0

normal

winnow ^1.0

normal

zbus_macros ^5.19.0

normal

zbus_names ^4.3.4

normal

zvariant ^5.14.0

normal

codspeed-criterion-compat ^5.0.0

dev

doc-comment ^0.3.3

dev

futures-util ^0.3.31

dev

ntest ^0.9.2

dev

tempfile ^3.10.1

dev

test-log ^0.2.16

dev

tokio ^1.37.0

dev

tracing-subscriber ^0.3.18

dev

tokio-vsock ^0.7

normal

optional

vsock ^0.5.0

normal

optional

async-recursion ^1.1.1

normal

libc ^0.2

normal

rustix ^1.1.2

normal

async-recursion ^1.1.1

normal

uds_windows ^1.1.0

normal

windows-sys ^0.61

normal

Versions

93.65%
of the crate is documented

Platform

aarch64-linux-android

x86_64-apple-darwin

x86_64-pc-windows-gnu

x86_64-unknown-freebsd

x86_64-unknown-linux-gnu

x86_64-unknown-netbsd

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
Crate zbus

Crate zbus Copy item pathSource Expand description

§zbus

This is the main subcrate of the zbus project, that provides the API to interact with D-Bus. It
takes care of the establishment of a connection, the creation, sending and receiving of different
kind of D-Bus messages (method calls, signals etc) for you.

Status: Stable.

§Getting Started

The best way to get started with zbus is the book, where we start
with basic D-Bus concepts and explain with code samples, how zbus makes D-Bus easy.

§Example code

We’ll create a simple D-Bus service and client to demonstrate the usage of zbus. Note that these
examples assume that a D-Bus broker is setup on your machine and you’ve a session bus running
(DBUS_SESSION_BUS_ADDRESS environment variable must be set). This is guaranteed to be the case on
a typical Linux desktop session.

§Service

A simple service that politely greets whoever calls its SayHello method:

use std::{error::Error, future::pending};
use zbus::{connection, interface};

struct Greeter {
count: u64
}

#[interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
// Can be `async` as well.
fn say_hello(&mut self, name: &str) -> String {
self.count += 1;
format!("Hello {}! I have been called {} times.", name, self.count)
}
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
let greeter = Greeter { count: 0 };
let _conn = connection::Builder::session()?
.name("org.zbus.MyGreeter")?
.serve_at("/org/zbus/MyGreeter", greeter)?
.build()
.await?;

// Do other things or go to wait forever
pending::<()>().await;

Ok(())
}

You can use the following command to test it:

$ busctl --user call org.zbus.MyGreeter /org/zbus/MyGreeter org.zbus.MyGreeter1 SayHello s "Maria"
s "Hello Maria! I have been called 1 times."
§Client

Now let’s write the client-side code for MyGreeter service:

use zbus::{Connection, Result, proxy};

#[proxy(
interface = "org.zbus.MyGreeter1",
default_service = "org.zbus.MyGreeter",
default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
async fn say_hello(&self, name: &str) -> Result<String>;
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<()> {
let connection = Connection::session().await?;

// `proxy` macro creates `MyGreaterProxy` based on `MyGreeter` trait.
let proxy = MyGreeterProxy::new(&connection).await?;
let reply = proxy.say_hello("Maria").await?;
println!("{reply}");

Ok(())
}
§Blocking API

While zbus is primarily asynchronous (since 2.0), blocking wrappers are provided for
convenience. Since zbus 5.0, blocking API can be disabled by disabling the blocking-api cargo
feature.

§Compatibility with async runtimes

zbus is runtime-agnostic and should work out of the box with different Rust async runtimes. However,
in order to achieve that, zbus spawns a thread per connection to handle various internal tasks. If
that is something you would like to avoid, you need to:

Use connection::Builder and disable the internal_executor flag.

Ensure the internal executor keeps ticking continuously.

Moreover, by default zbus makes use of async-io for all I/O, which also launches its own thread
to run its own internal executor.

§Special tokio support

Since tokio is the most popular async runtime, zbus provides an easy way to enable tight
integration with it without you having to worry about any of the above: Enabling the tokio feature:

# Sample Cargo.toml snippet.
[dependencies]
# Also disable the default `async-io` feature to avoid unused dependencies.
zbus = { version = "5", default-features = false, features = ["tokio"] }

That’s it! No threads launched behind your back by zbus (directly or indirectly) now and no need to
tick any executors etc. 😼

The tokio and async-io features are additive: when both are enabled (for example, because
another crate in your dependency tree enables tokio), zbus selects the runtime at run time — using
tokio when a tokio runtime is driving the current thread, and async-io otherwise. With only the
tokio feature enabled (i.e. async-io disabled), zbus must be used from a thread running a tokio
runtime, as there is no async-io fallback to drive its I/O.

This run-time selection applies to the async API. The blocking API (zbus::blocking) drives its
connections through its own block_on, which uses tokio whenever the tokio feature is enabled, so
those connections always run on tokio when that feature is on.

Note: On Windows, the async-io feature is currently required for UNIX domain socket support,
see the corresponding tokio issue on GitHub.

Re-exports§pub use address::Address;pub use message::Message;pub use connection as conn;pub use connection::Connection;pub use match_rule::MatchRule;pub use match_rule::OwnedMatchRule;pub use proxy::Proxy;pub use object_server::ObjectServer;pub use zbus_names as names;pub use zvariant;
Modules§addressD-Bus address handling.blockingThe blocking API.connectionConnection API.fdomatch_ruleBus match rule API.messageD-Bus Message.object_serverThe object server API.proxyThe client-side proxy API.
Structs§ExecutorA wrapper around the underlying runtime/executor.GuidA D-Bus server GUID.MessageStreamA stream::Stream implementation that yields Message items.OwnedGuidOwned version of Guid.
Enums§AuthMechanismAuthentication mechanismsErrorThe error type for zbus.
Traits§AsyncDropAsync equivalent of Drop.DBusErrorA trait that needs to be implemented by error types to be returned from D-Bus methods.
Type Aliases§ResultAlias for a Result with the error type zbus::Error.
Attribute Macros§interfaceAttribute macro for implementing a D-Bus interface.proxyAttribute macro for defining D-Bus proxies (using zbus::Proxy and
zbus::blocking::Proxy).
Derive Macros§DBusErrorDerive macro for implementing zbus::DBusError trait.

## Related

- [[d-bus]]
