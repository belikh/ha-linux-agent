---
title: niri/niri-ipc/src/socket.rs at main · niri-wm/niri · GitHub
id: niriniri-ipcsrcsocketrs-at-main-niri-wmniri-github
tags:
- linux-agent-jupiteros-fleet-15537b
- source-code
- birth-message
- niri
- windows-only
- failure-notifications
- gap-06
- ipc
- primary-source
created: '2026-09-02T16:29:19.138752Z'
updated: '2026-09-05T10:51:22.368201Z'
source: https://github.com/niri-wm/niri/blob/main/niri-ipc/src/socket.rs
source_domain: github.com
fetched_at: '2026-09-02T16:29:19.119108Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'niri-ipc/src/socket.rs (client library): SOCKET_PATH_ENV = ''NIRI_SOCKET''.
  Socket::connect() reads $NIRI_SOCKET and errors ''NIRI_SOCKET is not set, are you
  running this within niri?'' if absent. Crucially Socket::connect_to(path) accepts
  ANY explicit path — the client library does not mandate the env var; an external
  process that knows the absolute socket path (e.g. /run/user/1000/niri.wayland-1.2474.sock)
  can connect directly without inheriting session environment, subject only to filesystem
  permissions on the socket inode.'
---

niri/niri-ipc/src/socket.rs at main · niri-wm/niri · GitHub

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

niri-wm

/

niri

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
1.1k

Star
27.4k

FilesExpand file tree

main

/
socket.rsCopy path

Blame
More file actions

Blame
More file actions

Latest commit

HistoryHistory

History

101 lines (90 loc) · 3.26 KB

main

/
socket.rsCopy pathTop

File metadata and controls

Code

Blame

101 lines (90 loc) · 3.26 KB

Raw
Copy raw file
Download raw file
Open symbols panel
Edit and raw actions

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101

//! Helper for blocking communication over the niri socket.

use std::env;

use std::io::{self, BufRead, BufReader, Write};

use std::net::Shutdown;

use std::os::unix::net::UnixStream;

use std::path::Path;

use crate::{Event, Reply, Request};

/// Name of the environment variable containing the niri IPC socket path.

pub const SOCKET_PATH_ENV: &str = "NIRI_SOCKET";

/// Helper for blocking communication over the niri socket.

///

/// This struct is used to communicate with the niri IPC server. It handles the socket connection

/// and serialization/deserialization of messages.

pub struct Socket {

stream: BufReader<UnixStream>,

}

impl Socket {

/// Connects to the default niri IPC socket.

///

/// This is equivalent to calling [`Self::connect_to`] with the path taken from the

/// [`SOCKET_PATH_ENV`] environment variable.

pub fn connect() -> io::Result<Self> {

let socket_path = env::var_os(SOCKET_PATH_ENV).ok_or_else(|| {

io::Error::new(

io::ErrorKind::NotFound,

format!("{SOCKET_PATH_ENV} is not set, are you running this within niri?"),

)

})?;

Self::connect_to(socket_path)

}

/// Connects to the niri IPC socket at the given path.

pub fn connect_to(path: impl AsRef<Path>) -> io::Result<Self> {

let stream = UnixStream::connect(path.as_ref())?;

let stream = BufReader::new(stream);

Ok(Self { stream })

}

/// Sends a request to niri and returns the response.

///

/// Return values:

///

/// * `Ok(Ok(response))`: successful [`Response`](crate::Response) from niri

/// * `Ok(Err(message))`: error message from niri

/// * `Err(error)`: error communicating with niri

pub fn send(&mut self, request: Request) -> io::Result<Reply> {

let mut buf = serde_json::to_string(&request).unwrap();

buf.push('\n');

self.stream.get_mut().write_all(buf.as_bytes())?;

buf.clear();

self.stream.read_line(&mut buf)?;

let reply = serde_json::from_str(&buf)?;

Ok(reply)

}

/// Starts reading event stream [`Event`]s from the socket.

///

/// The returned function will block until the next [`Event`] arrives, then return it.

///

/// Use this only after requesting an [`EventStream`][Request::EventStream].

///

/// # Examples

///

/// ```no_run

/// use niri_ipc::{Request, Response};

/// use niri_ipc::socket::Socket;

///

/// fn main() -> std::io::Result<()> {

///     let mut socket = Socket::connect()?;

///

///     let reply = socket.send(Request::EventStream)?;

///     if matches!(reply, Ok(Response::Handled)) {

///         let mut read_event = socket.read_events();

///         while let Ok(event) = read_event() {

///             println!("Received event: {event:?}");

///         }

///     }

///

///     Ok(())

/// }

/// ```

pub fn read_events(self) -> impl FnMut() -> io::Result<Event> {

let Self { mut stream } = self;

let _ = stream.get_mut().shutdown(Shutdown::Write);

let mut buf = String::new();

move || {

buf.clear();

stream.read_line(&mut buf)?;

let event = serde_json::from_str(&buf)?;

Ok(event)

}

}

}

You can’t perform that action at this time.