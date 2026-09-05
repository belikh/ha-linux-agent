---
title: Mocking std::fs for unit tests - help - The Rust Programming Language Forum
id: mocking-stdfs-for-unit-tests-help-the-rust-programming-language-forum
tags:
- linux-agent-jupiteros-fleet-15537b
- known-issue
- ha-issue
- source-code
- native-app-integration
- practitioner-forum
- testing
created: '2026-09-02T05:39:32.153853Z'
updated: '2026-09-05T10:51:21.970079Z'
source: https://users.rust-lang.org/t/mocking-std-fs-for-unit-tests/22382
source_domain: users.rust-lang.org
fetched_at: '2026-09-02T05:39:26.735658Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Rust forum thread (arkaitzj, Nov 2018) on mocking std::fs for unit tests:
  std::fs is a module not a trait, so there is no direct substitution mechanism —
  ''Creating tempdirs and all that is fine for integration tests, but I don''t think
  I should be relying on local fs for unit tests.'' Accepted answer (mmmmib): use
  the tempfile crate''s tempdir (auto-deleted on scope exit, defaults to OS temp dir
  which is usually tmpfs/ramdisk on Linux), or abstract over std Read/Write/Seek traits
  for testable I/O. Canonical pattern for ha-linux-agent: hermetic unit tests via
  tempfile + trait abstraction rather than filesystem mocking.'
---

Mocking std::fs for unit tests - help - The Rust Programming Language Forum

Mocking std::fs for unit tests

help

arkaitzj

November 18, 2018,  3:28pm

1

Hi,

I have a piece of code that walks over some directories and reads some files.

I am trying to write some tests and I am struggling to find an appropriate fs mocking procedure.

All the fs access in Rust is built on top of std::fs which is a module, not a trait or any other kind of interface, so there is no real way to substitute std::fs by std::mock_fs or something like that for the tests.

Creating tempdirs and all that is fine for integration tests, but I don't think I should be relying on local fs for unit tests.

Somebody must have seen this issue before, how do people deal with this?

Different concrete implementations for a generic trait

mmmmib

November 18, 2018,  6:34pm

2

You might want to try the tempfile crate:

tempdir in tempfile - Rust

Create a new temporary directory.

With that you can create individual files or directories, and they will be deleted when the object goes out of scope. I think the crate also defaults to using the OS's designated temporary directories, e.g./tmp/ on linux, which is usually a tmpfs/ramdisk.

Alternatively, the types from std::fs implement several traits over which you might be able to abstract, e.g. Read, Write, Seek:

File in std::fs - Rust

A reference to an open file on the filesystem.

Related topics

Topic

Replies
Views
Activity

Testing FS Access

help

5

1540

July 6, 2020

How to write test cases for file related functions?

help

2

621

June 4, 2024

[crate request] please give us a tempdir crate for use in tests

help

9

874

April 22, 2021

Mocking in Rust & thread-local globals

help

8

3405

October 24, 2015

What's the recommended way of testing I/O functions?

help

3

4590

May 8, 2018

Powered by Discourse, best viewed with JavaScript enabled