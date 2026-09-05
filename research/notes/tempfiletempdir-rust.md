---
title: tempfile::tempdir - Rust
id: tempfiletempdir-rust
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- official-docs
- primary-source
- resource-footprint
- practitioner-guide
- testing
- rust
- api-docs
created: '2026-09-02T05:39:32.169471Z'
updated: '2026-09-05T10:51:21.944723Z'
source: https://docs.rs/tempfile/3.0.4/tempfile/fn.tempdir.html
source_domain: docs.rs
fetched_at: '2026-09-02T05:39:28.760041Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'docs.rs reference for tempfile 3.0.4 tempdir() (Stebalien, MIT/Apache-2.0,
  Sept 2018 — note: 3.x line is now much newer; the pinned 3.0.4 doc is the page actually
  fetched): ''The tempdir function creates a directory in the file system and returns
  a TempDir. The directory will be automatically deleted when the TempDir''s destructor
  is run.'' Creates inside std::env::temp_dir(), errors if creation fails, documents
  a Resource Leaking caveat (see TempDir docs). Example: dir = tempdir()?; File::create(dir.path().join(...));
  drop + dir.close()?. The standard primitive for hermetic fs-touching Rust unit/integration
  tests — real filesystem semantics without mocking std::fs.'
---

tempfile::tempdir - Rust

Docs.rs

tempfile-3.0.4

tempfile 3.0.4

Docs.rs crate page

MIT/Apache-2.0

14 September 2018

Links

Homepage

Repository

crates.io

Source

Owners

Stebalien

Dependencies

cfg-if ^0.1

normal

rand ^0.5

normal

remove_dir_all ^0.5

normal

redox_syscall ^0.1

normal

libc ^0.2.27

normal

winapi ^0.3

normal

Versions

Go to latest version

Platform

i686-apple-darwin

i686-pc-windows-msvc

i686-unknown-linux-gnu

x86_64-apple-darwin

x86_64-pc-windows-msvc

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

Function tempfile::tempdir[−][src]pub fn tempdir() -> Result<TempDir>

Create a new temporary directory.

The tempdir function creates a directory in the file system
and returns a TempDir.
The directory will be automatically deleted when the TempDirs
desctructor is run.

Resource Leaking

See the resource leaking docs on TempDir.

Errors

If the directory can not be created, Err is returned.

Examples

use tempfile::tempdir;
use std::fs::File;
use std::io::{self, Write};

// Create a directory inside of `std::env::temp_dir()`
let dir = tempdir()?;

let file_path = dir.path().join("my-temporary-note.txt");
let mut file = File::create(file_path)?;
writeln!(file, "Brian was here. Briefly.")?;

// `tmp_dir` goes out of scope, the directory as well as
// `tmp_file` will be deleted here.
drop(file);
dir.close()?;

Help

Keyboard Shortcuts?Show this help dialogSFocus the search field↑Move up in search results↓Move down in search results↹Switch tab⏎Go to active search result+Expand all sections-Collapse all sections

Search Tricks
Prefix searches with a type followed by a colon (e.g. fn:) to restrict the search to a given type.
Accepted types are: fn, mod, struct, enum, trait, type, macro, and const.
Search functions by type signature (e.g. vec -> usize or * -> vec)
Search multiple things at once by splitting your query with comma (e.g. str,u8 or String,struct:Vec,test)