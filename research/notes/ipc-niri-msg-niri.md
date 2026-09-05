---
title: IPC, niri msg - niri
id: ipc-niri-msg-niri
tags:
- linux-agent-jupiteros-fleet-15537b
- niri
- ipc
- event-stream
- kiosk
created: '2026-09-02T06:55:24.256571Z'
updated: '2026-09-02T17:37:22.772736Z'
source: https://niri-wm.github.io/niri/IPC.html
source_domain: niri-wm.github.io
fetched_at: '2026-09-02T06:55:24.255415Z'
fetch_provider: builtin
status: evergreen
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'niri IPC documentation (niri-wm.github.io/niri/IPC.html): the compositor
  exposes a UNIX domain socket at $NIRI_SOCKET for programmatic control. Protocol:
  write a JSON request on a single line (newline or flush+shutdown), read a JSON reply
  wrapped in Ok/Err; niri msg --json is a thin wrapper over the socket (testable with
  socat). Event Stream mode (since niri 0.1.9): the event-stream request makes niri
  continuously stream events into the connection until closed — "designed to give
  you the complete current state up-front, then follow up with updates to that state",
  so consumer state can never desync and no polling is needed; updates are atomic
  where reasonable (documented edge case: transient workspace-id staleness when events
  race). Compatibility contract: JSON output is stable (fields/variants not renamed,
  non-optional fields not removed; new fields arrive — handle unknowns gracefully);
  human-readable output is NOT stable. niri-ipc sub-crate is versioned with niri itself,
  not semver-stable. The exact mechanism a jupiterOS kiosk agent would consume for
  window/monitor/idle state — and a model of a well-specified machine-readable IPC
  with a stability promise.'
---

*Suggested by [[home-niri-wmniri-wiki-github]] — niri wiki TOC lists 'IPC, niri msg' page; the event-stream socket is the primary interface a jupiterOS kiosk agent would consume for idle/display/window state*

IPC, niri msg - niri

Skip to content

Initializing search

GitHub

Application-Specific Issues

Nvidia

Xwayland

Gestures

Fullscreen and Maximize

Window Effects

Packaging niri

Integrating niri

Security Model

Accessibility

Name and Logo

FAQ

Configuration

Development

IPC, niri msg

You can communicate with the running niri instance over an IPC socket.
Check niri msg --help for available commands.

The --json flag prints the response in JSON, rather than formatted.
For example, niri msg --json outputs.

Tip

If you're getting parsing errors from niri msg after upgrading niri, make sure that you've restarted niri itself.
You might be trying to run a newer niri msg against an older niri compositor.

Event Stream#

Since: 0.1.9

While most niri IPC requests return a single response, the event stream request will make niri continuously stream events into the IPC connection until it is closed.
This is useful for implementing various bars and indicators that update as soon as something happens, without continuous polling.

The event stream IPC is designed to give you the complete current state up-front, then follow up with updates to that state.
This way, your state can never "desync" from niri, and you don't need to make any other IPC information requests.

Where reasonable, event stream state updates are atomic, though this is not always the case.
For example, a window may end up with a workspace id for a workspace that had already been removed.
This can happen if the corresponding workspaces-changed event arrives before the corresponding window-changed event.

To get a taste of the events, run niri msg event-stream.
Though, this is more of a debug function than anything.
You can get raw events from niri msg --json event-stream, or by connecting to the niri socket and requesting an event stream manually.

You can find the full list of events along with documentation here.

Programmatic Access#

niri msg --json is a thin wrapper over writing and reading to a socket.
When implementing more complex scripts and modules, you're encouraged to access the socket directly.

Connect to the UNIX domain socket located at $NIRI_SOCKET in the filesystem.
Write your request encoded in JSON on a single line, followed by a newline character, or by flushing and shutting down the write end of the connection.
Read the reply as JSON, also on a single line.

You can use socat to test communicating with niri directly:

$ socat STDIO "$NIRI_SOCKET"
"FocusedWindow"
{"Ok":{"FocusedWindow":{"id":12,"title":"t socat STDIO /run/u ~","app_id":"Alacritty","workspace_id":6,"is_focused":true}}}

The reply is an Ok or an Err wrapping the same JSON object as you get from niri msg --json.

For more complex requests, you can use socat to find how niri msg formats them:

$ socat STDIO UNIX-LISTEN:temp.sock
# then, in a different terminal:
$ env NIRI_SOCKET=./temp.sock niri msg action focus-workspace 2
# then, look in the socat terminal:
{"Action":{"FocusWorkspace":{"reference":{"Index":2}}}}

You can find all available requests and response types in the niri-ipc sub-crate documentation.

Backwards Compatibility#

The JSON output should remain stable, as in:

existing fields and enum variants should not be renamed

non-optional existing fields should not be removed

However, new fields and enum variants will be added, so you should handle unknown fields or variants gracefully where reasonable.

The formatted/human-readable output (i.e. without --json flag) is not considered stable.
Please prefer the JSON output for scripts, since I reserve the right to make any changes to the human-readable output.

The niri-ipc sub-crate (like other niri sub-crates) is not API-stable in terms of the Rust semver; rather, it follows the version of niri itself.
In particular, new struct fields and enum variants will be added.