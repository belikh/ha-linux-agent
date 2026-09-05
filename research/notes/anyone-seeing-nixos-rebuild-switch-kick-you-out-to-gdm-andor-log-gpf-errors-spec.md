---
title: 'Anyone seeing `nixos-rebuild switch` kick you out to GDM and/or log GPF errors?
  specifically: `users.you.linger = true` set and unstable channel? - Help - NixOS
  Discourse'
id: anyone-seeing-nixos-rebuild-switch-kick-you-out-to-gdm-andor-log-gpf-errors-spec
tags:
- linux-agent-jupiteros-fleet-15537b
- systemd
- rust
- nixos
- windows-only
- known-issue
- gap-03
- stc-ng
- user-unit-restart
- regression
created: '2026-09-02T16:29:19.120258Z'
updated: '2026-09-02T17:37:22.617478Z'
source: https://discourse.nixos.org/t/anyone-seeing-nixos-rebuild-switch-kick-you-out-to-gdm-and-or-log-gpf-errors-specifically-users-you-linger-true-set-and-unstable-channel/79578
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T16:29:14.227019Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'OVERTURNS the Feb-2023 ''switch never restarts user units'' claim for 2026
  NixOS: limwa (PR #517768 author of the trigger analysis, Aug 2026) states ''nixos-rebuild
  switch now restarts user units other than nixos-activation.service'' — a behaviour
  change from the switch-to-configuration-ng Rust rewrite. User services (niri.service,
  gnome-session-monitor.service) began restarting on switch, crashing sessions (GDM
  logout, Cinnamon terminal kills). bittner dates the regression precisely: first
  occurrence 2026-05-26, nixpkgs 26.05.20260515.d233902→26.05.20260523.64c08a7, systemd
  constant at 260.1, isolating commits 6ced06a1b/76c8d4509 (PR #517768); 7 hits in
  57 switches, fires only when a unit''s store path changes (flake/channel updates,
  not small edits). Teardown detail: stc-ng restarts active user targets incl. graphical-session.target
  (RefuseManualStart=yes/StopWhenUnneeded=yes — a one-way trip), and when the killed
  terminal takes stc-ng''s stdout with it, stc-ng exits 101 (Rust panic code) mid-activation,
  leaving docker/NetworkManager/home-assistant stopped (half-applied generation).
  Fixes are per-unit restartIfChanged=false (niri #519740, gnome-session-monitor #536457);
  bittner argues the list is open-ended. Workaround: run switch from TTY/SSH. Occurs
  with Linger=no too — the user-unit restart pass is independent of lingering.'
---

Anyone seeing `nixos-rebuild switch` kick you out to GDM and/or log GPF errors? specifically: `users.you.linger = true` set and unstable channel? - Help - NixOS Discourse

Anyone seeing `nixos-rebuild switch` kick you out to GDM and/or log GPF errors? specifically: `users.you.linger = true` set and unstable channel?

Help

daxodev

August 14, 2026,  2:22am

1

sometime in the last month or two I noticed switch now kicks me out to GDM screen. I finally had time to look today and I see there’s this line in my dmesg:
Aug 13 20:48:00 myhost kernel: traps: systemd-logind[1040] general protection fault ip:75d9102bab86 sp:7ffc6e210c00 error:0 in libsyst>

Anyone else already debugging this? I can share a lot more about my configuration but if this smells familiar/old-news, then it’s not urgent - I’ll just punt and subscribe there instead…

daxodev

August 14, 2026,  3:26pm

3

Is there a tracking bug upstream already? I thought it wasn’t an upstream bug and just something wrong with nixos-rebuild switch because if I do boot instead then I don’t trigger the GPF (and no issues upon reboot back to my new build either).

I’d be curious to learn how to pinpoint the root cause so I can report it to the right place.

jtojnar

August 14, 2026,  4:21pm

4

Possibly fixed by https://github.com/NixOS/nixpkgs/pull/536457

1 Like

daxodev

August 14, 2026,  5:33pm

5

Interesting, thanks! Is that related to GPF error though? Seems like maybe it’s just _avoiding_ the error (not dissimilar to my using boot cmd)?

crertel

August 14, 2026,  9:06pm

6

I haven’t noticed GDM, but I had noticed it killing all my terminal windows (on Cinnamon).

limwa

August 14, 2026, 11:37pm

7

I’m not sure about how it relates to your issue specifically, but the PR above (made by me) fixes part of an issue introduced by a nixos-rebuild change. Namely, the change was that nixos-rebuild switch now restarts user units other than nixos-activation.service.

Since that change, some user services (niri.service, gnome-session-monitor.service, there might be more) started restarting on nixos-rebuild switch and causing instability (like sessions crashing, etc.). Each of these services must be manually corrected (niri is done, gnome-session-monitor is in progress, others might still not even have a PR), so my advice is to check the user units that nixos-rebuild switch says will restart. If your session crashes during this, pipe the output to a file and read the file afterwards. It might be the case that another unit needs a similar fix.

1 Like

bittner

August 16, 2026,  3:45pm

8

I can add another data point, and I think it separates the two symptoms being discussed here.

I get the same “kicked out to GDM mid-switch” behaviour, but without linger (Linger=no, never set in my config) and without any systemd-logind GPF — there is exactly one general protection fault in my entire journal and it’s a gnome-shell one from an unrelated mutter bug. So at least in my case the logind GPF isn’t a precondition for the session teardown, which suggests the GPF and the teardown may be two separate things rather than cause and effect.

When it started

I could date it more or less precisely thanks to NixOS keeping old generations around. Correlating every switch-to-configuration switch in my journal against GNOME session teardowns, and discarding the ones that were actually me rebooting, I get seven genuine occurrences: the first on 2026-05-26, and none at all before that date. In each one the session goes down 2–5 seconds after the switch triggers the systemd daemon-reload.

The switch that first did it moved nixpkgs 26.05.20260515.d233902 → 26.05.20260523.64c08a7. systemd is 260.1 on both sides, so it isn’t a systemd bump. The only commits in that range touching the relevant code are from #517768:

6ced06a1b switch-to-configuration-ng: rework user-unit migration candidate selection

76c8d4509 switch-to-configuration-ng: honour X-* directives in user-unit migration pass

6ced06a1b broadened candidate selection so that “package-shipped units previously in lower-priority locations now become migration candidates when /etc gains them”. That matches the timing, and it also explains why this only fires sometimes: the unit’s store path has to actually change, which happens on flake/channel updates but not on small config edits. Since the regression landed I have hit it 7 times in 57 switches.

I use home-manager as a NixOS module, but the only unit it places in $XDG_CONFIG_HOME/systemd/user here is an inactive tray.target. Everything that gets stopped is package-shipped under /etc/systemd/user, so this does not appear to be the home-manager shadowing case that rule also covers.

What the teardown looks like

Timestamps below are relative to the switch requesting the daemon-reload:
T+0.000  systemd[1]:     Reload requested from ('.switch-to-conf')
T+0.233  systemd[1]:     Reloading finished in 232 ms.
T+0.389  systemd[user]:  Stopping PipeWire PulseAudio...
T+0.393  systemd[user]:  Stopping Virtual filesystem service...
T+0.410  systemd[user]:  Stopping GCR ssh-agent wrapper...
T+0.412  systemd[user]:  Stopping IBus Daemon for GNOME...
T+0.417  systemd[user]:  Started PipeWire PulseAudio.        <- the only one that comes back
T+0.427  systemd[user]:  Stopping Portal service (GTK/GNOME implementation)...
T+0.430  systemd[user]:  Stopping Accessibility services bus...
T+0.445  systemd[user]:  Stopped target GNOME Session.
T+0.446  systemd[user]:  Stopped target GNOME Session (session: gnome).
T+0.733  systemd[user]:  Stopped target Current graphical user session.
T+1.886  systemd[user]:  Stopped target Session services which should run early...
T+1.899  gnome-session:  Could not get unit for graphical-session-pre.target: NoSuchUnit

systemd[user] is my per-user manager. Around 150ms after the daemon-reload finishes it stops a batch of GNOME user services for restart, only pipewire-pulse comes back, and the session targets collapse behind them.

Once graphical-session.target goes down it cannot come back in that manager:
graphical-session.target      ActiveState=active  RefuseManualStart=yes  RefuseManualStop=no  StopWhenUnneeded=yes
graphical-session-pre.target  ActiveState=active  RefuseManualStart=yes  RefuseManualStop=no  StopWhenUnneeded=yes

StopWhenUnneeded=yes takes it down once its dependents stop, and RefuseManualStart=yes means nothing can bring it back up. So whatever the initial trigger among the restarted units, the session can’t recover from it.

On #536457

I don’t think it covers my case, though I’d be glad to be wrong. gnome-session-monitor.service does get restarted on my machine and its store path did change on the switch that broke — but it stops 1.4 seconds after the session targets have already collapsed:
T+0.445  Stopped target GNOME Session.
T+1.887  Stopping Monitor Session leader for GNOME Session...

so here it looks like a casualty rather than the trigger. Happy to test a build with that PR applied if it would help settle it.

RefuseManualStart is not checked on restart

This looks like why the existing guards miss the case. 76c8d4509 added guards to the migration pass a day before the rework, but none of the three applies to these targets:

RefuseManualStop skips active units. Both targets have RefuseManualStop=no, so this gives no protection.

RefuseManualStart skips stopped units. Both targets are active, so it is never consulted.

the hardcoded skip-list covers sysinit.target, basic.target, multi-user.target and graphical.target. Those are the system targets; the per-user session targets are not in it.

So an active unit is checked for permission to be stopped, but never for permission to be started again. For a target that declares RefuseManualStart=yes, that makes a restart a one-way trip.

Would it make sense to consult RefuseManualStart on the restart path as well, or to treat units bound to graphical-session.target as not restartable while a session is live? I am not familiar enough with switch-to-configuration-ng internals to judge which of those is the right shape.

Per-unit opt-outs may not scale

Between #519740 (niri) and #536457 (gnome-session-monitor), the pattern so far is per-unit restartIfChanged = false. Given that @crertel reports the same thing on Cinnamon, and that the units that die on my system are generic session pieces rather than one identifiable culprit, I suspect that list is open-ended.

The switch itself then aborts, which may be a separate bug

When the session goes down it takes the terminal the rebuild was launched from with it. switch-to-configuration-ng loses its stdout pipe and exits 101 — the Rust panic exit code — so activation aborts partway through and leaves behind whatever it had already stopped. On my last occurrence that was docker, home-assistant and NetworkManager, none of which came back. That is why I rebooted rather than simply logging in again, and it is arguably worse than the logout itself: a half-applied generation is a much less obvious state to be left in.

Running the switch from a TTY or over SSH avoids this, though it does not prevent the teardown. Would it make sense for switch-to-configuration-ng to tolerate EPIPE on stdout and carry on with the remaining activation steps? That would let a switch finish applying the generation even when it loses the terminal it was launched from, independently of how the user-unit question is resolved. I would be glad to put together a PR for that part if it sounds like a reasonable direction.

Setup

NixOS unstable 26.11, GNOME 50, mutter 50.4, kernel 6.18.44, ThinkPad X1 Yoga Gen 8 (Intel, i915), GDM with autologin, Linger=no, home-manager as a NixOS module. Happy to provide full journals or test patches.

4 Likes

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.