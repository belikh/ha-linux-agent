---
title: sysinfo - Rust
id: sysinfo-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- source-code
- rust
- repo-source
- known-issue
- rust-crate
- hardware-telemetry
- crate-docs
created: '2026-09-02T06:42:41.443812Z'
updated: '2026-09-02T17:37:22.341596Z'
source: https://docs.rs/sysinfo/latest/sysinfo/
source_domain: docs.rs
fetched_at: '2026-09-02T06:42:37.181877Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'docs.rs crate documentation for sysinfo 0.39.6 (released 2026-07-09, MIT,
  owner GuillaumeGomez; min rustc 1.95). Rust crate for system information on Android,
  FreeBSD, NetBSD, iOS, Linux, macOS, Raspberry Pi, Windows — unsupported OSes return
  empty values, checkable via IS_SUPPORTED_SYSTEM constant. Key API surface: System
  (CPU, memory, processes, load avg; refresh_cpu_usage/refresh_specifics), Components
  (temperatures via hwmon), Disks, Networks (per-interface received/transmitted totals
  plus per-refresh deltas). Critical usage model: structs are DIFF-BASED — must call
  refresh methods and keep ONE System instance alive rather than recreating it (CPU
  usage needs a previous measurement); MINIMUM_CPU_UPDATE_INTERVAL is the minimum
  sleep between CPU usage refreshes. Performance: prefer refresh_specifics(...) over
  refresh_all; set_open_files_limit(0) if the host process needs many FDs. Docker/WSL
  caveat: virtual Linux systems get no host hardware info via /sys/class/hwmon or
  /sys/class/thermal, so Components queries may return nothing. serde feature enables
  serialising all sysinfo types (structs: CGroupLimits, Motherboard, Product, LoadAvg,
  InterfaceOperationalState per RFC2863, etc.). This is the canonical crate for the
  agent''s local telemetry gathering; the diff-based refresh model matches a long-running
  MQTT agent''s design.'
---

sysinfo - Rust

Docs.rs

sysinfo-0.39.6

sysinfo 0.39.6

Permalink

Docs.rs crate page

MIT

09 July 2026

Links

Repository

crates.io

Source

Owners

GuillaumeGomez

Dependencies

memchr ^2.5

normal

optional

rayon ^1.8

normal

optional

serde ^1.0.190

normal

optional

bstr ^1.9.0

dev

itertools ^0.14.0

dev

serde_json ^1.0

dev

tempfile ^3.9

dev

objc2-core-foundation ^0.3.2

normal

optional

objc2-io-kit ^0.3.2

normal

optional

libc ^0.2.173

normal

objc2-open-directory ^0.3.2

normal

optional

ntapi ^0.4

normal

optional

windows >=0.62, <0.63

normal

optional

Versions

99.22%
of the crate is documented

Platform

i686-pc-windows-msvc

i686-unknown-linux-gnu

x86_64-apple-darwin

x86_64-pc-windows-msvc

x86_64-unknown-freebsd

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
Crate sysinfo

Crate sysinfo Copy item pathSource Expand description

§sysinfo

sysinfo is a crate used to get a system’s information.

§Supported OSes

It currently supports the following OSes (alphabetically sorted):

Android

FreeBSD

NetBSD

iOS

Linux

macOS

Raspberry Pi

Windows

You can still use sysinfo on non-supported OSes, it’ll simply do nothing and always return
empty values. You can check in your program directly if an OS is supported by checking the
IS_SUPPORTED_SYSTEM constant.

The minimum-supported version of rustc is 1.95.

§Usage

If you want to migrate from an older version, don’t hesitate to take a look at the
CHANGELOG and at the
migration guide.

⚠️ Before any attempt to read the different structs’ information, you need to update them to
get up-to-date information because for most of them, it works on diff between the current value
and the old one.

Which is why, it’s much better to keep the same instance of System around instead of
recreating it multiple times.

You have an example into the examples folder. You can run it with cargo run --example simple.

Otherwise, here is a little code sample:

use sysinfo::{
Components, Disks, Networks, System,
};

// Please note that we use "new_all" to ensure that all lists of
// CPUs and processes are filled!
let mut sys = System::new_all();

// First we update all information of our `System` struct.
sys.refresh_all();

println!("=> system:");
// RAM and swap information:
println!("total memory: {} bytes", sys.total_memory());
println!("used memory : {} bytes", sys.used_memory());
println!("total swap  : {} bytes", sys.total_swap());
println!("used swap   : {} bytes", sys.used_swap());

// Display system information:
println!("System name:             {:?}", System::name());
println!("System kernel version:   {:?}", System::kernel_version());
println!("System OS version:       {:?}", System::os_version());
println!("System host name:        {:?}", System::host_name());

// Number of CPUs:
println!("NB CPUs: {}", sys.cpus().len());

// Display processes ID, name and disk usage:
for (pid, process) in sys.processes() {
println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
}

// We display all disks' information:
println!("=> disks:");
let disks = Disks::new_with_refreshed_list();
for disk in &disks {
println!("{disk:?}");
}

// Network interfaces name, total data received and total data transmitted:
let networks = Networks::new_with_refreshed_list();
println!("=> networks:");
for (interface_name, data) in &networks {
println!(
"{interface_name}: {} B (down) / {} B (up)",
data.total_received(),
data.total_transmitted(),
);
// If you want the amount of data received/transmitted since last call
// to `Networks::refresh`, use `received`/`transmitted`.
}

// Components temperature:
let components = Components::new_with_refreshed_list();
println!("=> components:");
for component in &components {
println!("{component:?}");
}

Please remember that to have some up-to-date information, you need to call the equivalent
refresh method. For example, for the CPU usage:

use sysinfo::System;

let mut sys = System::new();

loop {
sys.refresh_cpu_usage(); // Refreshing CPU usage.
for cpu in sys.cpus() {
print!("{}% ", cpu.cpu_usage());
}
// Sleeping to let time for the system to run for long
// enough to have useful information.
std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
}

By default, sysinfo uses multiple threads. However, this can increase the memory usage on some
platforms (macOS for example). The behavior can be disabled by setting default-features = false
in Cargo.toml (which disables the multithread cargo feature).

§Good practice / Performance tips

Most of the time, you don’t want all information provided by sysinfo but just a subset of it.
In this case, it’s recommended to use refresh_specifics(...) methods with only what you need
to have much better performance.

Another issue frequently encountered: unless you know what you’re doing, it’s almost all the
time better to instantiate the System struct once and use this one instance through your
program. The reason is because a lot of information needs a previous measure to be computed
(the CPU usage for example). Another example why it’s much better: in case you want to list
all running processes, sysinfo needs to allocate all memory for the Process struct list,
which takes quite some time on the first run.

If your program needs to use a lot of file descriptors, you’d better use:

sysinfo::set_open_files_limit(0);

as sysinfo keeps a number of file descriptors open to have better performance on some
targets when refreshing processes.

§Running on Raspberry Pi

It’ll be difficult to build on Raspberry Pi. A good way-around is to cross-build, then send the
executable to your Raspberry Pi.

First install the arm toolchain, for example on Ubuntu:

> sudo apt-get install gcc-multilib-arm-linux-gnueabihf

Then configure cargo to use the corresponding toolchain:

cat << EOF > ~/.cargo/config
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF

Finally, cross compile:

rustup target add armv7-unknown-linux-gnueabihf
cargo build --target=armv7-unknown-linux-gnueabihf
§Linux on Docker & Windows Subsystem for Linux (WSL)

Virtual Linux systems, such as those run through Docker and Windows Subsystem for Linux (WSL), do
not receive host hardware information via /sys/class/hwmon or /sys/class/thermal. As such,
querying for components may return no results (or unexpected results) when using this library on
virtual systems.

§Use in binaries running inside the macOS or iOS Sandbox/stores

Apple has restrictions as to which APIs can be linked into binaries that are distributed through the app store.
By default, sysinfo is not compatible with these restrictions. You can use the apple-app-store
feature flag to disable the Apple prohibited features. This also enables the apple-sandbox feature.
In the case of applications using the sandbox outside of the app store, the apple-sandbox feature
can be used alone to avoid causing policy violations at runtime.

§How it works

I wrote a blog post you can find here which explains how sysinfo extracts information
on the different systems.

§Running tests

Because we’re looking at system information, some tests have a better chance to succeed when there is
a limited number of parallel running tests. To ensure they all pass, use:

cargo test -- --test-threads=1
§C interface

It’s possible to use this crate directly from C. Take a look at the Makefile and at the
examples/simple.c file.

To build the C example, just run:

> make
> ./simple
# If needed:
> LD_LIBRARY_PATH=target/debug/ ./simple
§Benchmarks

You can run the benchmarks locally with rust nightly by doing:

> cargo bench
§Donations

If you appreciate my work and want to support me, you can do it with
github sponsors or with
patreon.

With the serde feature enabled, you can then serialize sysinfo types. Let’s see an example with serde_json:

use sysinfo::System;

let mut sys = System::new_all();
// First we update all information of our `System` struct.
sys.refresh_all();

println!("{}", serde_json::to_string(&sys).unwrap());
Structs§CGroupLimitsContains memory limits for a cgroup.ComponentGetting a component temperature information.ComponentsInteracting with components.CpuContains all the methods of the Cpu struct.CpuRefreshKindUsed to determine what you want to refresh specifically on the Cpu type.DiskStruct containing a disk information.DiskRefreshKindUsed to determine what you want to refresh specifically on the Disk type.DiskUsageType containing read and written bytes.DisksDisks interface.GidA group id wrapping a platform specific type.GroupType containing group information.GroupsInteracting with groups.IpNetworkIP networks address for network interface.LoadAvgA struct representing system load average value.MacAddrMAC address for network interface.MemoryRefreshKindUsed to determine which memory you want to refresh specifically.MotherboardThis type allows to retrieve motherboard-related information.NetworkDataGetting volume of received and transmitted data.NetworksInteracting with network interfaces.PidProcess ID.ProcessStruct containing information of a process.ProcessRefreshKindUsed to determine what you want to refresh specifically on the Process type.ProductThis type allows to retrieve product-related information.RefreshKindUsed to determine what you want to refresh specifically on the System type.SystemType containing system’s information such as processes, memory and CPU.UidA user id wrapping a platform specific type.UserType containing user information.UsersInteracting with users.
Enums§DiskKindEnum containing the different supported kinds of disks.InterfaceOperationalStateThe operational state of some interface based on IfOperStatus
from RFC2863.IpNetworkFromStrErrorError type returned from MacAddr::from_str implementation.KillErrorEnum describing possible Process::kill_and_wait errors.MacAddrFromStrErrorError type returned from MacAddr::from_str implementation.ProcessStatusEnum describing the different status of a process.ProcessesToUpdateThis enum allows you to specify if you want all processes to be updated or just
some of them.SignalAn enum representing signals on UNIX-like systems.ThreadKindEnum describing the different kind of threads.UpdateKindThis enum allows you to specify when you want the related information to be updated.
Constants§IS_SUPPORTED_SYSTEMReturns true if this OS is supported. Please refer to the
crate-level documentation to get the list of supported OSes.MINIMUM_CPU_UPDATE_INTERVALThis is the minimum interval time used internally by sysinfo to refresh the CPU time.SUPPORTED_SIGNALSReturns the list of the supported signals on this system (used by
Process::kill_with).
Functions§get_current_pidReturns the pid for the current process.set_open_files_limitThis function is only used on Linux targets, when the system feature is enabled. In other
cases, it does nothing and returns false.