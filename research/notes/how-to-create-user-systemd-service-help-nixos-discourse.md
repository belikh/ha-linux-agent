---
title: How to create user systemd service - Help - NixOS Discourse
id: how-to-create-user-systemd-service-help-nixos-discourse
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
created: '2026-09-02T04:04:41.179183Z'
updated: '2026-09-02T17:37:22.080431Z'
source: https://discourse.nixos.org/t/how-to-create-user-systemd-service/57671
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T04:04:37.183963Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'NixOS Discourse thread (Dec 2024) walking a beginner through systemd.user.services.<name>
  in configuration.nix. Captures the exact failure ladder of user services: (1) inspecting
  with plain ''systemctl status'' fails — must pass --user; (2) running ''systemctl
  --user'' as root (incl. via sudo -i) fails with ''Failed to connect to user scope
  bus: No medium found'' because root has no live user-manager bus; machinectl shell
  <user>@ is the correct way to inspect another user''s units; (3) setting serviceConfig.User
  inside a user unit fails — user services necessarily run as the owning user (''Failed
  to determine supplementary groups: Operation not permitted'', status=216/GROUP);
  (4) systemd.user.* installs the unit for ALL users under /run/current-system/sw/lib/systemd/user/;
  per-user installation requires home-manager (masking per user is manual); (5) home-manager
  collision errors when a manually-created unit file exists in ~/.config/systemd/user
  — fixed with ''home-manager switch -b backup''; (6) use store paths (${pkgs.coreutils}/bin/sleep)
  not /run/current-system paths, and %h instead of absolute home paths for portability.
  Concrete evidence that user services carry auth/bus/UID traps that a fleet agent
  design must either solve or sidestep by running as a system service.'
---

How to create user systemd service - Help - NixOS Discourse

How to create user systemd service

Help

kazimierzkrauze

December 20, 2024,  1:07pm

1

Hello!

I just installed NixOS on my laptop and began the adventure. I was trying to create a user service in configuration.nix file. I put this in my file:
systemd.user.services.onedrive-UMFC = {
enable = true;
description = "OneDrive Client for Linux running for jeansib";
after = [ "network-online.target" ];
wantedBy = [ "multi-user.target" ];
serviceConfig = {
ProtectSystem = "full";
ProtectHostname = "true";
ProtectKernelTunables = "true";
ProtectControlGroups = "true";
RestrictRealtime = "true";
ExecStartPre = ''/run/current-system/sw/bin/sleep'';
ExecStart = ''/run/current-system/sw/bin/onedrive --monitor --confdir=/home/jeansib/.config/oned>
User = "jeansib";
Group = "users";
Restart = "on-failure";
RestartSec = "3";
RestartPreventExitStatus = "126";
TimeoutStopSec = "90";
};
};

Then I ran nixos-rebuild switch and didn’t get any error.

But now, where is this service file stored? When I run systemctl status onedrive-UMFC I get an error saying that such unit doesn’t exist.

Can anyone help me please?

Miro

waffle8946

December 20, 2024,  7:12pm

2

Command is missing --user.

kazimierzkrauze

December 20, 2024,  7:30pm

3

Thank you, if I understand correctly I had to add --user flag to systemctl command. But when I do, I get this error:
[root@nixos:~]# systemctl --user list-units
Failed to connect to user scope bus via local transport: No medium found

[root@nixos:~]# systemctl status --user onedrive-UMFC
Failed to connect to user scope bus via local transport: No medium found

[root@nixos:~]#

What can that mean?

waffle8946

December 20, 2024,  7:42pm

4

Did you upgrade systemd recently and not reboot yet?

Also, you need to run this as a user that will run the service, it’s a user unit.

VTimofeenko

December 20, 2024,  7:54pm

5

systemctl --user shows the user’s services for the current user. You’re running that command as root and I presume you intend for the service to work in a regular user’s scope.

Run this command(systemctl --user list-units ) as your normal user.

About the error message in your post:

If you become root through sudo, systemctl --user will not work at all:
❯ sudo -i

root in ~
❯ systemctl --user status
Failed to connect to user scope bus via local transport: No medium found

You can get this working if you use machinectl shell <username>@ $(which bash) instead of sudo, but that’s most likely not what you want here.

Just for completeness sake, here's how to call systemctl --user in an interactive shell for a different user
❯ whoami
spacecadet

❯ sudo -i # start login session as root

❯ whoami
root

❯ systemctl --user status
Failed to connect to user scope bus via local transport: No medium found

❯ exit # back to the normal user
❯ machinectl shell root@ $(which bash)
# < auth >
❯ whoami
root
❯ systemctl --user status
WARNING: terminal is not fully functional
Press RETURN to continue
● uranium
State: running
Units: 177 loaded (incl. loaded aliases)
Jobs: 0 queued
Failed: 0 units
Since: Fri 2024-12-20 11:57:17 PST; 16s ago
systemd: 256.8
CGroup: /user.slice/user-0.slice/user@0.service
<the output>

Now from root:
❯ whoami
root

❯ machinectl shell spacecadet@ $(which bash)
Connected to the local host. Press ^] three times within 1s to exit session.

❯ systemctl --user status
● uranium
State: degraded
Units: 237 loaded (incl. loaded aliases)
Jobs: 0 queued
Failed: 1 units
<the output>

2 Likes

kazimierzkrauze

December 21, 2024,  8:29am

6

Thank you very much for your answers, I am deeply grateful.

I didn’t know of the --user option, I always used systemctl enable --now unit-name@user-name.service to enable and start user units.

I changed to unstable channel of Nix, but I rebooted.

Now I can see my unit running:
[jeansib@nixos:~]$ systemctl --user status onedrive-UMFC.service
● onedrive-UMFC.service - OneDrive Client for Linux running for jeansib
Loaded: loaded (/home/jeansib/.config/systemd/user/onedrive-UMFC.service; enabled; preset: ignored)
Active: activating (auto-restart) (Result: exit-code) since Sat 2024-12-21 09:18:33 CET; 1s ago
Invocation: bb4a32cc3ac7424c88aaebc395976f88
Process: 10834 ExecStartPre=/run/current-system/sw/bin/sleep (code=exited, status=216/GROUP)
Mem peak: 1.5M
CPU: 7ms
[jeansib@nixos:~]$

If you would help me further, I would appreciate that.

I see that there is some kind of group problem with this unit. I was tinkering a little bit with configuration and now it looks like this:
systemd.user.services.onedrive-UMFC = {
enable = true;
description = "OneDrive Client for Linux running for jeansib";
after = [ "network-online.target" ];
wantedBy = [ "default.target" ];
serviceConfig = {
ProtectSystem = "full";
ProtectHostname = "true";
ProtectKernelTunables = "true";
#      ProtectControlGroups = "true";
RestrictRealtime = "true";
ExecStartPre = ''/run/current-system/sw/bin/sleep'';
ExecStart = ''/run/current-system/sw/bin/onedrive --monitor --confdir=/home/jeansib/.config/onedrive/onedrive-UMFC/'';
User = "jeansib";
#      Group = "jeansib";
Restart = "on-failure";
RestartSec = "3";
RestartPreventExitStatus = "126";
TimeoutStopSec = "90";
};
};

First I changed group from “users” to “jeansib” and then I tried to eliminate all group lines thinking that maybe it somehow broke the unit.

When I run try to start the service I get this error:
[jeansib@nixos:~]$ systemctl --user start onedrive-UMFC.service
Job for onedrive-UMFC.service failed because the control process exited with error code.
See "systemctl --user status onedrive-UMFC.service" and "journalctl --user -xeu onedrive-UMFC.service" for details.
[jeansib@nixos:~]$

Running journalctl --user -xeu … gives me this:
gru 21 09:27:51 nixos (sleep)[11787]: onedrive-UMFC.service: Failed to determine supplementary groups: Operation not permitted
gru 21 09:27:51 nixos (sleep)[11787]: onedrive-UMFC.service: Failed at step GROUP spawning /run/current-system/sw/bin/sleep: Operation not permitted

symphorien

December 21, 2024,  2:09pm

7

You cannot set User on a user systemd service. It will necessarily run as your user.

kazimierzkrauze

December 21, 2024,  3:15pm

8

Okay, so this whole systemd block should be inside users.users.jeansib{} block?

waffle8946

December 21, 2024,  3:47pm

9

No. Using systemd.user.* hierarchy creates it for all (regular) users on the system. (More accurately it puts it in /run/current-system/sw/lib/systemd/user/ which is used by all systemd user sessions.)

If you want per-user control I believe you have to use something like home-manager at this time.

kazimierzkrauze

December 21, 2024,  5:18pm

10

Okay, but can I disable for specific user this unit?

TLATER

December 22, 2024,  6:13am

11

You could mask the unit in that user’s systemd directories, but you would still need to do that with home-manager (or - gasp - manually) afaik.

Regardless, they would have permissions to re-enable it - this really looks like the type of unit that should be done with home-manager in your home directory, so your user is the only user with any control over or insight into it.

Also, use %h instead of an absolute path for your home directory in systemd units, that way they are portable, and use ${package}/bin/binary (or add the packages to path and don’t use an absolute path) instead of /run/current-system, otherwise you’re relying on your config just happening to install those binaries.

kazimierzkrauze

December 22, 2024, 11:57am

12

Thank you for your kind answers! I decided to go with home-manager. This is my config:
{config, pkgs, ...}:
{
home = {
stateVersion = "24.11";
username = "jeansib";
homeDirectory = "/home/jeansib";
};

systemd.user.services.onedrive-UMFC = {
Unit = {
Description = "OneDrive client for Linux";
};
Service = {
ProtectSystem = "full";
ProtectHostname = true;
ProtectKernelTunables = true;
ProtectControlGroups = true;
RestrictRealtime = true;
Group = "users";
ExecStartPre = "sleep 15";
ExecStart= ''${pkgs.onedrive}/bin/onedrive --monitor --confdir=/home/jeansib/.config/onedrive/onedrive-UMFC'';
Restart = "on-failure";
RestartSec = 3;
# Do not restart the service if a --resync is required which is done via a 126 exit code
RestartPreventExitStatus = 126;
# Time to wait for the service to stop gracefully before forcefully terminating it
TimeoutStopSec = 90;
};
Install = {
WantedBy = [ "default.target" ];
};
};
}

But when I try to switch I get this error:
[jeansib@nixos:~]$ home-manager switch
/nix/store/hiwz93nvmz33h6ccy85ai37dng94dwb0-home-manager-generation
Starting Home Manager activation
Activating checkFilesChanged
Activating checkLinkTargets
Existing file '/home/jeansib/.config/systemd/user/onedrive-UMFC.service' is in the way of '/nix/store/rfg38jpl2msc7kllzvgdgqydrycvv269-home-manager-files/.config/systemd/user/onedrive-UMFC.service'
Please do one of the following:
- Move or remove the above files and try again.
- In standalone mode, use 'home-manager switch -b backup' to back up
files automatically.
- When used as a NixOS or nix-darwin module, set
'home-manager.backupFileExtension'
to, for example, 'backup' and rebuild.
[jeansib@nixos:~]$

What does it mean? I tried to add to /etc/nixos/configuration.nix line:
home-manager.backupFileExtension = "backup";

but that didn’t work…

kazimierzkrauze

December 22, 2024,  1:14pm

13

Guys! I did it! I think that there was somewhere still the unit I created with /etc/nixos/configuration.nix file, I had to disable it and run home-manager switch -b backup and now this unit works! But also I had to comment out ExecStartPre:
systemd.user.services.onedrive-UMFC = {
Unit = {
Description = "OneDrive client for Linux";
};
Service = {
ProtectSystem = "full";
ProtectHostname = true;
ProtectKernelTunables = true;
ProtectControlGroups = true;
RestrictRealtime = true;
Group = "users";
#      ExecStartPre = ''sleep 15'';
ExecStart= ''${pkgs.onedrive}/bin/onedrive --monitor --confdir=/home/kazimierzkrauze/.config/onedrive/onedrive-UMFC'';
Restart = "on-failure";
RestartSec = 3;
# Do not restart the service if a --resync is required which is done via a 126 exit code
RestartPreventExitStatus = 126;
# Time to wait for the service to stop gracefully before forcefully terminating it
TimeoutStopSec = 90;
};
Install = {
WantedBy = [ "default.target" ];
};
};

I tried to enter ‘’${pkgs.sleep}…‘’ but that didn’t work.

bwolf

December 22, 2024,  5:38pm

14

kazimierzkrauze:

I tried to enter ‘’${pkgs.sleep}…‘’ but that didn’t work.

Try "${pkgs.coreutils}/bin/sleep 15" instead.

kazimierzkrauze

December 22, 2024,  7:04pm

15

Thank you, that helped!

TLATER

December 23, 2024,  2:39am

16

kazimierzkrauze:

home-manager switch -b backup

This was what fixed it; the error message says you already put a systemd unit in your user’s directory manually, and home-manager refused to overwrite such files in case they contain something important.

Adding the -b flag just renames the file, so you probably have a subtly different, inert copy of the unit in there.

kazimierzkrauze

December 23, 2024,  5:30am

17

Well, that’s strange, I did try to use this command and the output was the same, it was not until I manually disabled this unit that home-manager switch worked. But maybe not… I found that sometimes I have to just try again and it works.

TLATER

December 23, 2024,  5:33am

18

The same message would indeed appear if your NixOS config created that file somehow, but I didn’t think NixOS modules touched user directories. Maybe systemd creates a symlink to the system-wide file?

kazimierzkrauze

December 23, 2024,  4:46pm

19

But if I declare in /etc/nixos/conf.nix user service then where is it stored?

waffle8946

December 23, 2024,  5:01pm

20

Did you run systemctl enable at any point?

next page →

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.