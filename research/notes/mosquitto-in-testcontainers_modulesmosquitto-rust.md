---
title: Mosquitto in testcontainers_modules::mosquitto - Rust
id: mosquitto-in-testcontainers_modulesmosquitto-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- source-code
- repo-source
- mqtt-discovery
- storage-health
- testing
- api-docs
created: '2026-09-02T05:39:32.148411Z'
updated: '2026-09-02T17:37:22.244868Z'
source: https://docs.rs/testcontainers-modules/latest/testcontainers_modules/mosquitto/struct.Mosquitto.html
source_domain: docs.rs
fetched_at: '2026-09-02T05:39:25.845303Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'docs.rs API reference for testcontainers-modules 0.15.0 Mosquitto struct
  (feature-gated ''mosquitto'', MIT, built on testcontainers ^0.27.0): ''Module to
  work with Mosquitto inside of tests. Starts a MQTT broker without authentication.''
  Example: mosquitto::Mosquitto::default().start() via SyncRunner, then broker_url
  = format!("{}:{}", get_host().unwrap(), get_host_port_ipv4(1883).unwrap()). Documents
  the Image trait surface (ready_conditions, expose_ports, with_startup_timeout default
  60s, with_network, with_mapped_port, with_ready_conditions overrides via ImageExt)
  — the full configuration envelope available when spinning an ephemeral broker for
  Rust integration tests.'
---

Mosquitto in testcontainers_modules::mosquitto - Rust

Docs.rs

testcontainers-modules-0.15.0

testcontainers-modules 0.15.0

Permalink

Docs.rs crate page

MIT

23 June 2026

Links

Repository

crates.io

Source

Owners

DDtKey

mervyn-mccreight

Dependencies

parse-display ^0.10

normal

optional

rcgen ^0.14.5

normal

optional

serde ^1.0.217

normal

optional

serde_json ^1.0.138

normal

optional

testcontainers ^0.27.0

normal

alloy-network ^1.0.27

dev

alloy-provider ^1.0.27

dev

arrow-flight ^56.1.0

dev

async-nats ^0.42.0

dev

aws-config ^1.0.1

dev

aws-sdk-dynamodb ^1.2.0

dev

aws-sdk-s3 ^1.2.0

dev

aws-sdk-sqs ^1.2.0

dev

aws-types ^1.0.1

dev

azure_core ^0.30.1

dev

azure_storage ^0.21.0

dev

azure_storage_blobs ^0.21.0

dev

base64 ^0.22.1

dev

clickhouse ^0.13

dev

databend-driver ^0.28.2

dev

fantoccini ^0.21

dev

futures ^0.3

dev

k8s-openapi ^0.26

dev

kube ^2.0.1

dev

lapin ^3.0.0

dev

ldap3 ^0.11.5

dev

meilisearch-sdk ^0.29.1

dev

mongodb ^3.0.1

dev

mysql ^26.0.0

dev

native-tls ^0.2.12

dev

neo4rs ^0.8.0

dev

openssl-sys ^0.9.103

dev

oracle ^0.6.0

dev

postgres ^0.19.7

dev

pretty_env_logger ^0.5.0

dev

pulsar ^6.3

dev

rdkafka ^0.38.0

dev

redis ^0.32.2

dev

reqwest ^0.12.5

dev

retry ^2.0.0

dev

rqlite-rs ^0.6

dev

rustls ^0.23.2

dev

scylla ^1.0.0

dev

serde ^1.0.217

dev

serde_json ^1.0.138

dev

serial_test ^3.1.1

dev

surrealdb ^2.2.1

dev

tar ^0.4.40

dev

testcontainers ^0.27.0

dev

tiberius ^0.12.3

dev

tokio ^1

dev

tokio-util ^0.7.10

dev

tokio-zookeeper ^0.4.0

dev

vaultrs ^0.7.2

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
Mosquitto

testcontainers_modules::mosquitto
Struct Mosquitto Copy item pathSource pub struct Mosquitto { /* private fields */ }
Available on crate feature mosquitto only.Expand description

Module to work with Mosquitto inside of tests.

Starts a MQTT broker without authentication.

§Example

use testcontainers_modules::{mosquitto, testcontainers::runners::SyncRunner};

let mosquitto_instance = mosquitto::Mosquitto::default().start().unwrap();

let broker_url = format!(
"{}:{}",
mosquitto_instance.get_host().unwrap(),
mosquitto_instance.get_host_port_ipv4(1883).unwrap()
);
Trait Implementations§
Source§
impl Clone for Mosquitto
Source§
fn clone(&self) -> Mosquitto
Returns a duplicate of the value. Read more1.0.0 (const: unstable) · Source§
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read moreSource§
impl Debug for Mosquitto
Source§
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read moreSource§
impl Default for Mosquitto
Source§
fn default() -> Mosquitto
Returns the “default value” for a type. Read moreSource§
impl Image for Mosquitto
Source§
fn name(&self) -> &str
The name of the docker image to pull from the Docker Hub registry.Source§
fn tag(&self) -> &str
Implementations are encouraged to include a tag that will not change (i.e. NOT latest)
in order to prevent test code from randomly breaking because the underlying docker
suddenly changed.Source§
fn ready_conditions(&self) -> Vec<WaitFor>
Returns a list of conditions that need to be met before a started container is considered ready. Read moreSource§
fn cmd(&self) -> impl IntoIterator<Item = impl Into<Cow<'_, str>>>
Returns the CMD this image needs to be created with.Source§
fn env_vars(
&self,
) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)>
Returns the environment variables that needs to be set when a container is created.Source§
fn mounts(&self) -> impl IntoIterator<Item = &Mount>
Returns the mounts that needs to be created when a container is created.Source§
fn copy_to_sources(&self) -> impl IntoIterator<Item = &CopyToContainer>
Returns the files to be copied into the container at startup.Source§
fn entrypoint(&self) -> Option<&str>
Returns the entrypoint this image needs to be created with.Source§
fn expose_ports(&self) -> &[ContainerPort]
Returns the ports that needs to be exposed when a container is created. Read moreSource§
fn exec_after_start(
&self,
cs: ContainerState,
) -> Result<Vec<ExecCommand>, TestcontainersError>
Returns the commands that needs to be executed after a container is started i.e. commands
to be run in a running container. Read moreSource§
fn exec_before_ready(
&self,
cs: ContainerState,
) -> Result<Vec<ExecCommand>, TestcontainersError>
Returns commands that will be executed after the container has started, but before the
Image::ready_conditions are awaited for. Read more
Auto Trait Implementations§
§
impl Freeze for Mosquitto§
impl RefUnwindSafe for Mosquitto§
impl Send for Mosquitto§
impl Sync for Mosquitto§
impl Unpin for Mosquitto§
impl UnsafeUnpin for Mosquitto§
impl UnwindSafe for Mosquitto
Blanket Implementations§
Source§
impl<T> Any for T
where
T: 'static + ?Sized,
Source§
fn type_id(&self) -> TypeId
Gets the TypeId of self. Read moreSource§
impl<T, I> AsyncRunner<I> for T
where
T: Into<ContainerRequest<I>> + Send,
I: Image,
Source§
fn start<'async_trait>(
self,
) -> Pin<Box<dyn Future<Output = Result<ContainerAsync<I>, TestcontainersError>> + Send + 'async_trait>>
where
T: 'async_trait,
Starts the container and returns an instance of ContainerAsync.Source§
fn pull_image<'async_trait>(
self,
) -> Pin<Box<dyn Future<Output = Result<ContainerRequest<I>, TestcontainersError>> + Send + 'async_trait>>
where
T: 'async_trait,
Pulls the image from the registry.
Useful if you want to pull the image before starting the container.Source§
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
impl<T> FromRef<T> for T
where
T: Clone,
Source§
fn from_ref(input: &T) -> T
Converts to this type from a reference to the input type.Source§
impl<RI, I> ImageExt<I> for RI
where
RI: Into<ContainerRequest<I>>,
I: Image,
Source§
fn with_cmd(
self,
cmd: impl IntoIterator<Item = impl Into<String>>,
) -> ContainerRequest<I>
Returns a new ContainerRequest with the specified (overridden) CMD (Image::cmd). Read moreSource§
fn with_name(self, name: impl Into<String>) -> ContainerRequest<I>
Overrides the fully qualified image name (consists of {domain}/{owner}/{image}).
Can be used to specify a custom registry or owner.Source§
fn with_tag(self, tag: impl Into<String>) -> ContainerRequest<I>
Overrides the image tag. Read moreSource§
fn with_container_name(self, name: impl Into<String>) -> ContainerRequest<I>
Sets the container name.Source§
fn with_platform(self, platform: impl Into<String>) -> ContainerRequest<I>
Sets the platform the container will be run on. Read moreSource§
fn with_network(self, network: impl Into<String>) -> ContainerRequest<I>
Sets the network the container will be connected to.Source§
fn with_label(
self,
key: impl Into<String>,
value: impl Into<String>,
) -> ContainerRequest<I>
Adds the specified label to the container. Read moreSource§
fn with_labels(
self,
labels: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
) -> ContainerRequest<I>
Adds the specified labels to the container. Read moreSource§
fn with_env_var(
self,
name: impl Into<String>,
value: impl Into<String>,
) -> ContainerRequest<I>
Adds an environment variable to the container.Source§
fn with_host(
self,
key: impl Into<String>,
value: impl Into<Host>,
) -> ContainerRequest<I>
Adds a host to the container.Source§
fn with_hostname(self, hostname: impl Into<String>) -> ContainerRequest<I>
Configures hostname for the container.Source§
fn with_mount(self, mount: impl Into<Mount>) -> ContainerRequest<I>
Adds a mount to the container.Source§
fn with_copy_to(
self,
target: impl Into<CopyTargetOptions>,
source: impl Into<CopyDataSource>,
) -> ContainerRequest<I>
Copies data or a file/dir into the container. Read moreSource§
fn with_mapped_port(
self,
host_port: u16,
container_port: ContainerPort,
) -> ContainerRequest<I>
Adds a port mapping to the container, mapping the host port to the container’s internal port. Read moreSource§
fn with_ulimit(
self,
name: &str,
soft: i64,
hard: Option<i64>,
) -> ContainerRequest<I>
Adds a resource ulimit to the container. Read moreSource§
fn with_privileged(self, privileged: bool) -> ContainerRequest<I>
Sets the container to run in privileged mode.Source§
fn with_cap_add(self, capability: impl Into<String>) -> ContainerRequest<I>
Adds the capabilities to the containerSource§
fn with_cap_drop(self, capability: impl Into<String>) -> ContainerRequest<I>
Drops the capabilities from the container’s capabilitiesSource§
fn with_cgroupns_mode(self, cgroupns_mode: CgroupnsMode) -> ContainerRequest<I>
cgroup namespace mode for the container. Possible values are: Read moreSource§
fn with_userns_mode(self, userns_mode: &str) -> ContainerRequest<I>
Sets the usernamespace mode for the container when usernamespace remapping option is enabled.Source§
fn with_shm_size(self, bytes: u64) -> ContainerRequest<I>
Sets the shared memory size in bytesSource§
fn with_startup_timeout(self, timeout: Duration) -> ContainerRequest<I>
Sets the startup timeout for the container. The default is 60 seconds.Source§
fn with_working_dir(self, working_dir: impl Into<String>) -> ContainerRequest<I>
Sets the working directory. The default is defined by the underlying image, which in turn may default to /.Source§
fn with_log_consumer(
self,
log_consumer: impl LogConsumer + 'static,
) -> ContainerRequest<I>
Adds the log consumer to the container. Read moreSource§
fn with_host_config_modifier(
self,
modifier: impl Fn(&mut HostConfig) + Send + Sync + 'static,
) -> ContainerRequest<I>
Applies a custom modifier to the Docker HostConfig used for container creation. Read moreSource§
fn with_user(self, user: impl Into<String>) -> ContainerRequest<I>
Sets the user that commands are run as inside the container.Source§
fn with_readonly_rootfs(self, readonly_rootfs: bool) -> ContainerRequest<I>
Sets the container’s root filesystem to be mounted as read-onlySource§
fn with_security_opt(
self,
security_opt: impl Into<String>,
) -> ContainerRequest<I>
Sets security options for the containerSource§
fn with_ready_conditions(
self,
ready_conditions: Vec<WaitFor>,
) -> ContainerRequest<I>
Overrides ready conditions. Read moreSource§
fn with_health_check(self, health_check: Healthcheck) -> ContainerRequest<I>
Sets a custom health check for the container. Read moreSource§
fn with_open_stdin(self, open_stdin: bool) -> ContainerRequest<I>
Sets whether to keep stdin open for the container.Source§
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
impl<T> IntoEither for T
Source§
fn into_either(self, into_left: bool) -> Either<Self, Self>
Converts self into a Left variant of Either<Self, Self>
if into_left is true.
Converts self into a Right variant of Either<Self, Self>
otherwise. Read moreSource§
fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>
where
F: FnOnce(&Self) -> bool,
Converts self into a Left variant of Either<Self, Self>
if into_left(&self) returns true.
Converts self into a Right variant of Either<Self, Self>
otherwise. Read moreSource§
impl<T> IntoRequest<T> for T
Source§
fn into_request(self) -> Request<T>
Wrap the input message T in a tonic::RequestSource§
impl<T> IntoResult<T> for T
Source§
type Err = InfallibleSource§
fn into_result(self) -> Result<T, <T as IntoResult<T>>::Err>Source§
impl<T> IntoResult<T> for T
Source§
type Err = InfallibleSource§
fn into_result(self) -> Result<T, <T as IntoResult<T>>::Err>Source§
impl<L> LayerExt<L> for L
Source§
fn named_layer<S>(&self, service: S) -> Layered<<L as Layer<S>>::Service, S>
where
L: Layer<S>,
Applies the layer to a service and wraps it in Layered.Source§
impl<T> PolicyExt for T
where
T: ?Sized,
Source§
fn and<P, B, E>(self, other: P) -> And<T, P>
where
T: Sized + Policy<B, E>,
P: Policy<B, E>,
Create a new Policy that returns Action::Follow only if self and other return
Action::Follow. Read moreSource§
fn or<P, B, E>(self, other: P) -> Or<T, P>
where
T: Sized + Policy<B, E>,
P: Policy<B, E>,
Create a new Policy that returns Action::Follow if either self or other returns
Action::Follow. Read moreSource§
impl<T, I> SyncRunner<I> for T
where
T: Into<ContainerRequest<I>> + Send,
I: Image,
Source§
fn start(self) -> Result<Container<I>, TestcontainersError>
Starts the container and returns an instance of Container.Source§
fn pull_image(self) -> Result<ContainerRequest<I>, TestcontainersError>
Pulls the image from the registry.
Useful if you want to pull the image before starting the container.Source§
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
WithDispatch wrapper. Read more