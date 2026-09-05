---
title: 'GitHub - z-galaxy/zbus: Rust D-Bus crate. · GitHub'
id: github-z-galaxyzbus-rust-d-bus-crate-github
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- birth-message
- source-code
- windows-only
- dbus
- rust
- api-docs
created: '2026-09-02T05:39:32.160313Z'
updated: '2026-09-05T10:51:21.965763Z'
source: https://github.com/z-galaxy/zbus
source_domain: github.com
fetched_at: '2026-09-02T05:39:27.930539Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'z-galaxy/zbus (fork of the Rust D-Bus crate, 746 stars, 6,464 commits —
  note: canonical upstream is bus1/zbus, this URL 302s to the z-galaxy org mirror):
  ''A Rust API for D-Bus communication. The goal is to provide a safe and simple high-
  and low-level API akin to GDBus, that doesn''t depend on C libraries.'' Pure-Rust
  D-Bus stack — subcrates zbus (API + wire format + name types), zbus_macros (#[proxy],
  #[interface], #[derive(DBusError)]), zbus_xml, zbus_xmlgen, zbus_utils. Example
  service: #[interface(name = "org.zbus.MyGreeter1")] impl Greeter { fn say_hello(&mut
  self, name: &str) -> String }, served via connection::Builder::session()?.name(...)?.serve_at(...)?.build().await
  — testable with ''busctl --user call''. Assumes DBUS_SESSION_BUS_ADDRESS set. Relevant
  to ha-linux-agent: a systemd-hosted agent exposing D-Bus interfaces needs this crate''s
  session-vs-system bus handling, which fails on headless kiosks unless lingering
  or a session bus is guaranteed.'
---

GitHub - z-galaxy/zbus: Rust D-Bus crate. · GitHub

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

z-galaxy

/

zbus

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
160

Star
746

main

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History6,464 Commits

6,464 Commits
Folders and filesNameName
Last commit message
Last commit date

.githooks

.githooks

.github

.github

CI

CI

book

book

docs/superpowers/specs

docs/superpowers/specs

test_fixtures

test_fixtures

zbus

zbus

zbus_macros

zbus_macros

zbus_utils

zbus_utils

zbus_xml

zbus_xml

zbus_xmlgen

zbus_xmlgen

.codespellrc

.codespellrc

.commitlintrc.mjs

.commitlintrc.mjs

.editorconfig

.editorconfig

.gitignore

.gitignore

.mailmap

.mailmap

.rustfmt.toml

.rustfmt.toml

AGENTS.md

AGENTS.md

CLAUDE.md

CLAUDE.md

CONTRIBUTING.md

CONTRIBUTING.md

Cargo.lock

Cargo.lock

Cargo.toml

Cargo.toml

LICENSE

LICENSE

LICENSE-MIT

LICENSE-MIT

README.md

README.md

SECURITY.md

SECURITY.md

logo.png

logo.png

release-plz.toml

release-plz.toml

zbus-pixels.gif

zbus-pixels.gif

View all files

Repository files navigation

zbus

A Rust API for D-Bus communication. The
goal is to provide a safe and simple high- and low-level API akin to
GDBus, that doesn't depend on C
libraries.

The project is divided into the following subcrates:

zbus: The main subcrate. It provides the API to interact with D-Bus, the D-Bus wire
format (what used to be the zvariant crate) and the bus name types (what used to be
the zbus_names crate). With default-features = false you get the wire format and the name
types alone, without any of the D-Bus API.

zbus_macros: The procedural macros behind #[proxy], #[interface], #[derive(DBusError)]
and the wire-format derives. zbus re-exports all of them, so you rarely depend on it directly.

zbus_xml: API to handle D-Bus introspection description XML.

zbus_xmlgen: A developer tool to generate Rust code from D-Bus interface description XML.

zbus_utils: The D-Bus signature parser, name validators and derive-macro plumbing
shared by zbus and zbus_macros.

zgvariant is a sibling project. It implements GVariant, the format zbus itself dropped in
6.0. From its 2.0 release it builds on zbus_utils too, so its signature type is the same one
zbus uses; zgvariant 1.x depends on zvariant_utils 4.x — this crate under its old name — and its
signature type is a distinct one.

Getting Started

The best way to get started with zbus is the book, where we start
with basic D-Bus concepts and explain with code samples, how zbus makes D-Bus easy.

Example code

We'll create a simple D-Bus service and client to demonstrate the usage of zbus. Note that these
examples assume that a D-Bus broker is setup on your machine and you've a session bus running
(DBUS_SESSION_BUS_ADDRESS environment variable must be set). This is guaranteed to be the case on
a typical Linux desktop session.

Service

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

Client

Now let's write the client-side code for MyGreeter service:

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

// `proxy` macro creates `MyGreeterProxy` based on `Notifications` trait.
let proxy = MyGreeterProxy::new(&connection).await?;
let reply = proxy.say_hello("Maria").await?;
println!("{reply}");

Ok(())
}

Getting Help

If you need help in using these crates, are looking for ways to contribute, or just want to hang out
with the cool kids, please come chat with us in the
#zbus:matrix.org Matrix room. If something doesn't seem
right, please file an issue.

Security

If you discover a security vulnerability, please report it privately following our
Security Policy. We take security seriously and will respond promptly to reports.

Portability

Supported targets include Unix, Windows and macOS with Linux as the main target. Integration tests
of zbus crate currently require a session bus running on the build host.

License

MIT license LICENSE-MIT

Alternative Crates

dbus-rs relies on the battle tested libdbus C library to send and receive messages.
Companion crates add Tokio support, server builder without macros, and
code generation.

There are many other D-Bus crates out there with various levels of maturity and features.

About
Rust D-Bus crate.
Resources
Readme
License, MIT licenses found
Contributing
Contributing
Security policy
Security policy
Activity
Custom properties
Stars
746 stars
Watchers
3 watching
Forks
160 forks
Report repository

Releases

Sponsor this project

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.

## Related

- [[d-bus]]
