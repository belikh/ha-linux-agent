---
title: Security Model · niri-wm/niri Wiki · GitHub
id: security-model-niri-wmniri-wiki-github
tags:
- linux-agent-jupiteros-fleet-15537b
- systemd
- windows-only
- niri
- birth-message
- failure-notifications
- gap-06
- security-model
created: '2026-09-02T16:43:41.816692Z'
updated: '2026-09-02T17:37:22.652858Z'
source: https://github.com/niri-wm/niri/wiki/Security-Model
source_domain: github.com
fetched_at: '2026-09-02T16:43:41.148253Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'niri Security Model wiki (edited 2026-04-26): ''Niri assumes that programs
  running unsandboxed on the host are trusted'' — the IPC socket grants full session
  power: ''Anything with access to niri''s IPC socket can, among other things: Spawn
  a Wayland client which can do everything in the list above'' (screen recording via
  wlr-screencopy, input emulation, clipboard access, layer-shell overlay/password-spoof
  surfaces, killing/replacing lockscreens). Sandboxing untrusted clients requires:
  removing niri''s IPC socket, blocking host D-Bus, and a filtered (security-context)
  Wayland socket; Flatpak cited as one such sandbox. Direct security implication for
  a jupiterOS fleet agent running as User=io touching NIRI_SOCKET: same-UID processes
  have unrestricted IPC access by design — niri offers no per-connection authentication,
  only filesystem permission gating.'
---

Security Model · niri-wm/niri Wiki · GitHub

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

Security Model

Jump to bottom

github-actions[bot] edited this page Apr 26, 2026
·
2 revisions

Niri assumes that programs running unsandboxed on the host are trusted.

This is a reasonable assumption because programs running on the host have a wide variety of ways to get all access they need, even without niri.
For instance:

They can set $LD_PRELOAD in .bashrc or similar files to load an arbitrary library into all processes.

They can replace binaries in $PATH with malicious code.

They can interpose any socket in $XDG_RUNTIME_DIR, like Wayland, and do keylogging or record window contents.

They can scan the filesystem for secrets: SSH keys, password stores, etc.

They can connect to an unlocked keyring and steal credentials.

And so on and so forth.

Unsandboxed clients

Anything with access to niri's Wayland socket can, among other things:

Record the user's screen via wlr-screencopy.

Emulate input via wlr-virtual-pointer and virtual-keyboard.

Get the user's clipboard contents via wlr-data-control.

Create arbitrary fullscreen surfaces through wlr-layer-shell that can steal the user's input, pretend to be a password entry, or lock the user out of their session.

Kill a running lockscreen, create a new lock surface, and tell niri to unlock a locked session.

Anything with access to niri's IPC socket can, among other things:

Spawn a Wayland client which can do everything in the list above.

Anything with access to niri's D-Bus interfaces can, among other things:

Record the user's screen via the screencast interface.

Fully listen to and emulate input from the user's keyboard via the accessibility interface.

Also, while niri doesn't directly integrate Xwayland, it's worth reminding that anything with access to the X11 $DISPLAY (which comes both as a socket file on disk and as an abstract socket in the network namespace) can intercept and emulate all input and record the contents of any X11 windows on the same $DISPLAY (but not Wayland windows).

Running untrusted clients

Considering all of the above, for running untrusted clients, you need a proper sandbox that:

Removes niri's IPC socket.

Prevents D-Bus access to host services.

Uses a filtered Wayland socket.

For creating a filtered Wayland socket, you can use the security-context protocol which niri implements.
All unsafe protocols are made inaccessible through this filtered Wayland socket.

One sandbox that satisfies all of these criteria is the Flatpak sandbox.

Importantly, filtering just the Wayland socket (and leaving, for example, unrestricted D-Bus access) is not enough to prevent untrusted clients from doing bad things.

Lock screen

When the session is locked via ext-session-lock, most actions (keybindings) are automatically disabled.
Only a very small set of safe actions is allowed.
In particular, spawning will not work, with the exception of binds explicitly configured with allow-when-locked=true.

Importantly, the quit action is allowed—you can always quit niri, even when on a lock screen.
Therefore, you must ensure that quitting niri does not drop you into an unprotected TTY commandline.
Usually, a display manager, like GDM, will do this for you: when niri exits (via the quit bind or if it crashes), it'll put you back into a safe password prompt.

Other than quitting, the only way to exit a lock screen is for the lock screen client to tell niri to unlock the session.
If the lock screen client crashes, the session remains locked with a solid red background.
In this case, another lock screen client can take over (so you can start a fresh lock screen if it crashes, and still unlock your session).

Usage

Getting Started

Example systemd Setup

Important Software

Workspaces

Floating Windows

Tabs

Overview

Screencasting

Layer‐Shell Components

IPC, niri msg

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

Introduction

Input

Outputs

Key Bindings

Switch Events

Layout

Named Workspaces

Miscellaneous

Window Rules

Layer Rules

Animations

Gestures

Recent Windows

Debug Options

Include

Development

Design Principles

Developing niri

Documenting niri

Releasing niri

Fractional Layout

Redraw Loop

Animation Timing

Clone this wiki locally

You can’t perform that action at this time.

## Related

- [[d-bus]]
- [[ipc]]
