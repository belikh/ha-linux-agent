---
title: 'GitHub - testcontainers/testcontainers-rs-modules-community: Community maintained
  modules for Testcontainers for Rust · GitHub'
id: github-testcontainerstestcontainers-rs-modules-community-community-maintained-mo
tags:
- linux-agent-jupiteros-fleet-15537b
- testing
- rust
- api-docs
created: '2026-09-02T06:19:42.547929Z'
updated: '2026-09-02T17:37:22.279637Z'
source: https://github.com/testcontainers/testcontainers-rs-modules-community
source_domain: github.com
fetched_at: '2026-09-02T06:19:42.546108Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'Upstream repo for the Rust testcontainers-modules crate (172 stars, 87 forks,
  273 commits): community-maintained Testcontainers modules where every module is
  a cargo feature. Documents the three key mechanics for ha-linux-agent''s test harness:
  (1) testcontainers is re-exported with aligned version so ''use testcontainers_modules::testcontainers::ImageExt''
  needs no separate dep; (2) SyncRunner behind the ''blocking'' feature for sync tests,
  AsyncRunner otherwise; (3) module defaults (image version/tag/env) are overridable
  via ContainerRequest — e.g. Redis::default().with_tag("6.2-alpine").with_env_var("REDIS_PASSWORD",
  ...) — same pattern applies to pinning an eclipse-mosquitto tag. Uses just + release-plz
  + git-cliff, MIT.'
---

*Suggested by [[mosquitto-in-testcontainers_modulesmosquitto-rust]] — upstream source repo for the Rust Mosquitto module pinned in the assigned docs.rs page*

GitHub - testcontainers/testcontainers-rs-modules-community: Community maintained modules for Testcontainers for Rust · GitHub

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

testcontainers

/

testcontainers-rs-modules-community

Public

Notifications
You must be signed in to change notification settings

Fork
87

Star
172

main

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History273 Commits

273 Commits
Folders and filesNameName
Last commit message
Last commit date

.github

.github

examples

examples

src

src

.editorconfig

.editorconfig

.gitignore

.gitignore

CHANGELOG.md

CHANGELOG.md

CONTRIBUTING.md

CONTRIBUTING.md

Cargo.toml

Cargo.toml

LICENSE

LICENSE

README.md

README.md

cliff.toml

cliff.toml

justfile

justfile

logo.svg

logo.svg

release-plz.toml

release-plz.toml

rustfmt.toml

rustfmt.toml

View all files

Repository files navigation

testcontainers-modules

Community maintained modules for testcontainers

Provides modules to use for testing components in accordance with testcontainers-rs.
Every module is treated as a feature inside this crate.

Usage

Depend on testcontainers-modules with necessary features (e.g postgres, minio and etc)

Enable blocking feature if you want to use modules within synchronous tests (feature-gate for SyncRunner)

Then start using the modules inside your tests with either AsyncRunner or SyncRunner

Simple example of using postgres module with SyncRunner (blocking and postgres features enabled):

use testcontainers_modules::{postgres, testcontainers::runners::SyncRunner};

#[test]
fn test_with_postgres() {
let container = postgres::Postgres::default().start().unwrap();
let host_ip = container.get_host().unwrap();
let host_port = container.get_host_port_ipv4(5432).unwrap();
}

Note: you don't need to explicitly depend on testcontainers as it's re-exported dependency
of testcontainers-modules with aligned version between these crates.
For example:

use testcontainers_modules::testcontainers::ImageExt;

You can also see examples
for more details.

How to override module defaults (version, tag, ENV-variables)

Just use RunnableImage:

use testcontainers_modules::{
redis::Redis,
testcontainers::{ContainerRequest, ImageExt}
};

/// Create a Redis module with `6.2-alpine` tag and custom password
fn create_redis() -> ContainerRequest<Redis> {
Redis::default()
.with_tag("6.2-alpine")
.with_env_var("REDIS_PASSWORD", "my_secret_password")
}

License

MIT license (LICENSE or http://opensource.org/licenses/MIT)

About
Community maintained modules for Testcontainers for Rust
docs.rs/crate/testcontainers-modules/
Topics
rusttestcontainerstestcontainers-rusttesting
Resources
Readme
MIT license
Contributing
Contributing
Activity
Custom properties
Stars
172 stars
Watchers
5 watching
Forks
87 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.