---
title: X-systemd.automount makes activation fail - Help - NixOS Discourse
id: x-systemdautomount-makes-activation-fail-help-nixos-discourse
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-fleet-service-model-by-host-class
created: '2026-09-02T09:48:55.753775Z'
updated: '2026-09-02T17:37:21.718308Z'
source: https://discourse.nixos.org/t/how-to-restart-systemd-user-services-on-nixos-rebuild-switch/54589
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T09:48:55.752302Z'
fetch_provider: builtin
status: deprecated
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: X-systemd.automount makes activation fail - Help - NixOS Discourse
---

X-systemd.automount makes activation fail - Help - NixOS Discourse

X-systemd.automount makes activation fail

Help

billy4479

October 19, 2024, 12:54pm

1

Hi all,

I’ve just found out about the x-systemd.automount option in fstab from Samba - NixOS Wiki so i thought I’d give it a try also for a local HDD I have installed on my system.

My config looks like this:
fileSystems."/mnt/HDD".options = [
"defaults"
"noauto"
"x-systemd.automount"
"x-systemd.idle-timeout=5m"
"x-systemd.device-timeout=1s"
"x-systemd.mount-timeout=1s"
];

But when I try to run nixos-rebuild switch I get the following error at the activation phase:
Error: Failed to open unit file /nix/store/h5qfbppr0s2yfdgxgr1fiswjcrx81hpf-nixos-system-computerone-24.11.20241014.a3c0b3b/etc/systemd/system/mnt-HDD.mount

Caused by:
No such file or directory (os error 2)
warning: error(s) occurred while switching to the new configuration

which makes the whole process fail.

Does anyone know how should I fix this?

Thanks!

1 Like

billy4479

October 19, 2024,  1:04pm

2

Update: rebooting has somehow fixed the issue (?)

During the reboot I’ve read on the tty something on systemd automount for HDD being successful so when I dropped into the desktop I tried to ls /mnt/HDD which got mounted correctly on the fly.

My flake also now builds and activates with no issues, no changes made.

I’m glad that the problem is fixed but I’m very confused on what happened.

There seems to be the same issue every time I edit the x-systemd.* options for any entry in fstab and they magically go away on reboot.

If someone has an explanation please let me know, thank you

2 Likes

omnibs

February 28, 2025,  4:22am

3

ugh, I’m getting the same thing on an sshfs entry using fileSystems.

did you figure out a way to unbork this?

billy4479

February 28, 2025,  8:47am

4

Sadly not, but it hasn’t given me any problems since.

If I recall correctly I rebooted rebuilt and rebooted again and somehow it worked, not sure why, not sure how.

It also seemed that I had to repeat the procedure every time I changed a mount option.

I’m sorry, I’m not able to help you any further than this

1 Like

bjackman

December 22, 2025,  8:51pm

5

I was getting a similar issue with NFS. I was able to work around it by dropping down to the systemd.mounts+systemd.automounts options as described here: NFS - NixOS Wiki

Caveat: I haven’t actually got my NFS setup working yet, but I think this is an orthogonal issue.

ElvishJerricco

December 22, 2025,  9:49pm

6

That’s the old wiki. Contributions should be made to the official one: https://wiki.nixos.org

Were you using 25.05? The issue with x-systemd.automount should be fixed in 25.11, so doing manual mount / automount units shouldn’t be necessary (and it’s generally recommended to prefer the fstab approach)

bjackman

December 23, 2025,  7:56pm

7

That’s the old wiki. Contributions should be made to the official one: https://wiki.nixos.org

Damn thanks, totally forgot about that situation

Were you using 25.05? The issue with x-systemd.automount should be fixed in 25.11

Nope I was using 25.11. The config is here: boxen/nixos_modules/pizza/default.nix at 47c3d67b9acccf527d0f27cfd7ee1b33e9970581 · bjackman/boxen · GitHub

bjackman

January 2, 2026, 12:57pm

8

@ElvishJerricco After seeing your comment I tried reverting to fileSystems and it was working, until I just tried this:

Comparing master...automount-reload · bjackman/boxen

Home Manage config. Contribute to bjackman/boxen development by creating an account on GitHub.

restarting sysinit-reactivation.target
reloading the following units: dbus.service, var-lib-filebrowser-data.automount
Failed to reload var-lib-filebrowser-data.automount: Job type reload is not applicable for unit var-lib-filebrowser-data.automount.
starting the following units: filebrowser.service

bjackman

January 2, 2026,  1:11pm

9

I was able to work around this issue like this:

Skip deploy-rs and deploy the configuration manually with  nixos-rebuild switch --flake .#pizza --sudo --target-host pizza.

(I guess I could also have just used deploy-rs’s --auto-rollback false).

On the target host, manually restart the automount unit with sudo systemctl restart var-lib-filebrowser-data.automount

Now I’m able to deploy with deploy-rs again.

AI claims that this is just that you can’t reload an automount unit. In that case I think this is just a bug here where we should be adding the unit to units_to_restart instead of units_to_reload.

The AI’s claim seems eminently plausible here but I can’t find any evidence to back it up, aside from my empirical experience.

ElvishJerricco

January 2, 2026,  6:14pm

10

Yea, I think you’re right that if an automount file system’s options change, then it tries to reload the automount unit, when it should be the mount unit that’s reloaded (and only if it’s currently active).

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.