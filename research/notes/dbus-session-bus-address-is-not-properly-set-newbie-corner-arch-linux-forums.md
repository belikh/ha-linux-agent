---
title: DBus session bus address is not properly set / Newbie Corner / Arch Linux Forums
id: dbus-session-bus-address-is-not-properly-set-newbie-corner-arch-linux-forums
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- ha-linux-agent
- repo-map
created: '2026-09-02T04:04:41.189353Z'
updated: '2026-09-05T10:51:21.781150Z'
source: https://bbs.archlinux.org/viewtopic.php?id=221340
source_domain: bbs.archlinux.org
fetched_at: '2026-09-02T04:04:40.016940Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Arch forum thread (Dec 2016 - Feb 2017) diagnosing why DBUS_SESSION_BUS_ADDRESS
  is unset/mis-set for user-session processes. Failure patterns: (1) a user-manager
  dbus.service can be ACTIVE (running) as user@1000.service/dbus.service with dbus-daemon
  --session --address=systemd: while the environment variable DBUS_SESSION_BUS_ADDRESS
  is still EMPTY in the shell — service activation and env-var propagation are separate
  mechanisms; systemd must import the address into the user manager environment (systemctl
  --user set-environment / dbus.socket ExecStartPost), not merely run the daemon.
  (2) An ABSTRACT socket address like unix:abstract=/tmp/dbus-... comes from dbus-launch
  and indicates a stray per-session bus that is NOT the systemd user bus; fix is to
  remove the dbus-launch invocation from shell/xinitrc startup files. (3) Stale user
  unit overrides found under /etc/systemd/user/dbus.service or ~/.config/systemd/user
  shadow the correct /usr/lib/systemd/user/dbus.service and keep the address unset.
  Correct value on systemd hosts: DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus.
  Diagnostic sequence recommended: check systemctl status --user dbus.service, check
  XDG_RUNTIME_DIR=/run/user/1000, log in via plain TTY instead of lightdm to isolate.
  Directly relevant to a fleet agent that connects to the session bus: verify the
  env var AND socket path, and never self-launch dbus-launch.'
---

DBus session bus address is not properly set / Newbie Corner / Arch Linux Forums

Arch Linux

Home
Packages
Forums
Wiki
GitLab
Security
AUR
Download

Index

Rules

Search

Register

Login

You are not logged in.

Topics: Active | Unanswered

Index

» Newbie Corner

» DBus session bus address is not properly set

Pages: 1

#1 2016-12-31 01:58:21

ShionjiYuuko
Member

Registered: 2011-07-31
Posts: 67

DBus session bus address is not properly set

I've been trying PulseAudio equalizer qpaeq. It fails to launch with error
There was an error connecting to pulseaudio, please make sure you have the pulseaudio dbus module loaded, exiting...
Module equalizer sink and module dbus protocol is activated in /etc/pulse/default.pa
Tracing the error gives reveals that the cause is environment variable $DBUS_SESSION_BUS_ADDRESS being empty. Setting it manually to /run/user/$UID/dbus/user_bus_socket gives error
org.freedesktop.DBus.Error.ServiceUnknown: The name org.PulseAudio1 was not provided by any .service files
Result from systemctl --show-environment
DISPLAY=:0
HOME=/home/uname
JOURNAL_STREAM=8:14766
LANG=en_US.UTF-8
LC_COLLATE=C
LOGNAME=uname
MAIL=/var/spool/mail/uname
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin
SHELL=/usr/bin/zsh
USER=uname
XAUTHORITY=/home/uname/.Xauthority
XDG_RUNTIME_DIR=/run/user/1000
For login I'm using lightdm and awesome WM.
There's should be something wrong in the login.
How can I set the environment correctly? I've been checking all xinit files but there seems to be nothing that rewrites environment variables.
Thanks

始まりの荒野を独り もう歩き出してるらしい、僕は灰になるまで僕で有り続けたい
http://about.me/nnhnkn | http://identi.ca/nnhzkn

Offline

#2 2016-12-31 02:27:30

loqs
Member
Registered: 2014-03-06
Posts: 19,027

Re: DBus session bus address is not properly set

General_troubleshooting#Session_permissions
Also is DBUS_SESSION_BUS_ADDRESS still unset if you log directly into a terminal instead of through lightdm?
Also what is the output of
$ systemctl status --user dbus.service

Offline

#3 2016-12-31 09:19:56

Roken
Member

From: South Wales, UK
Registered: 2012-01-16
Posts: 1,366

Re: DBus session bus address is not properly set

I had a similar problem a while ago, and whilst I didn't trace it back to a specific setting, I fixed it by:
moving ~/.config to ~/.config.bak
This resulted in $DBUS_SESSION_BUS_ADDRESS being correctly set. Then I moved files from the .bak directory into the new .config as required.
https://bbs.archlinux.org/viewtopic.php … 3#p1662433 refers.

Ryzen 5900X 12 core/24 thread - RTX 3090 FE 24 Gb, Asus B550-F Gaming MB, 128Gb Corsair DDR4, Fractal Design Define 7 XL, 5 HD (2 NvME PCI, 4SSD) + 1 x optical.
Linux user #545703
/ is the root of all problems.

Offline

#4 2017-01-02 07:00:10

ShionjiYuuko
Member

Registered: 2011-07-31
Posts: 67

Re: DBus session bus address is not properly set

loqs wrote:

General_troubleshooting#Session_permissions
Also is DBUS_SESSION_BUS_ADDRESS still unset if you log directly into a terminal instead of through lightdm?
Also what is the output of
$ systemctl status --user dbus.service
- The local session is valid
- DBUS_SESSION_BUS_ADDRESS is unset in terminal
- dbus.service status
Loaded: loaded (/etc/systemd/user/dbus.service; enabled; vendor preset: enabled)
Active: active (running) since Mon 2017-01-02 15:47:40 JST; 5min ago
Main PID: 682 (dbus-daemon)
CGroup: /user.slice/user-1000.slice/user@1000.service/dbus.service
└─682 /usr/bin/dbus-daemon --session --address=systemd: --nofork --nopidfile --systemd-activation

Jan 02 15:47:40 alice-tan systemd[645]: Started D-Bus Message Bus.
Thanks.

始まりの荒野を独り もう歩き出してるらしい、僕は灰になるまで僕で有り続けたい
http://about.me/nnhnkn | http://identi.ca/nnhzkn

Offline

#5 2017-01-02 07:23:43

Scimmia
Fellow

Registered: 2012-09-01
Posts: 13,729

Re: DBus session bus address is not properly set

What is /etc/systemd/user/dbus.service? That should not exist.

Offline

#6 2017-01-29 19:04:58

Mixu
Member
Registered: 2012-12-03
Posts: 26

Re: DBus session bus address is not properly set

Hi! As the OP I can't open qpaeq and throws the same message.
Anybody can help me?
$ systemctl status --user dbus.service
dbus.service - D-Bus User Message Bus
Loaded: loaded (/usr/lib/systemd/user/dbus.service; static; vendor preset: enabled)
Active: active (running) since Sun 2017-01-29 19:31:15 CET; 24min ago
Docs: man:dbus-daemon(1)
Main PID: 3075 (dbus-daemon)
CGroup: /user.slice/user-1000.slice/user@1000.service/dbus.service
3075 /usr/bin/dbus-daemon --session --address=systemd: --nofork --nopidfile --sys
3221 /usr/lib/GConf/gconfd-2

Jan 29 19:31:15 MeowPC systemd[3024]: Started D-Bus User Message Bus.
Jan 29 19:31:15 MeowPC dbus-daemon[3075]: Successfully activated service 'org.freedesktop.syst
Jan 29 19:31:20 MeowPC dbus-daemon[3075]: Activating service name='org.gnome.GConf'
Jan 29 19:31:20 MeowPC dbus-daemon[3075]: Successfully activated service 'org.gnome.GConf'
echo $DBUS_SESSION_BUS_ADDRESS
unix:abstract=/tmp/dbus-iDskgvL60r,guid=aaaa474d48be6e7fc55dbc43588e34f3
$ pactl list
Module #26
Name: module-dbus-protocol
Argument:
Usage counter: n/a
Properties:
module.author = "Tanu Kaskinen"
module.description = "D-Bus interface"
module.version = "10.0"
I don't have any idea on what to look for or change

Offline

#7 2017-01-29 19:07:05

Scimmia
Fellow

Registered: 2012-09-01
Posts: 13,729

Re: DBus session bus address is not properly set

Mixu wrote:

echo $DBUS_SESSION_BUS_ADDRESS
unix:abstract=/tmp/dbus-iDskgvL60r,guid=aaaa474d48be6e7fc55dbc43588e34f3
This is wrong. Find where you're using dbus-launch and get rid of it.

Last edited by Scimmia (2017-01-29 19:07:20)

Offline

#8 2017-01-29 20:45:23

Mixu
Member
Registered: 2012-12-03
Posts: 26

Re: DBus session bus address is not properly set

Scimmia wrote:

Mixu wrote:

echo $DBUS_SESSION_BUS_ADDRESS
unix:abstract=/tmp/dbus-iDskgvL60r,guid=aaaa474d48be6e7fc55dbc43588e34f3
This is wrong. Find where you're using dbus-launch and get rid of it.
Thanks!
I did like this post and now it's working https://bbs.archlinux.org/viewtopic.php … 4#p1652394

Offline

#9 2017-02-02 19:02:42

ShionjiYuuko
Member

Registered: 2011-07-31
Posts: 67

Re: DBus session bus address is not properly set

Scimmia wrote:

What is /etc/systemd/user/dbus.service? That should not exist.
[Unit]
Description=D-Bus Message Bus
Requires=dbus.socket

[Service]
ExecStart=/usr/bin/dbus-daemon --session --address=systemd: --nofork --nopidfile --systemd-activation
ExecReload=/usr/bin/dbus-send --print-reply --session --type=method_call --dest=org.freedesktop.DBus / org.freedesktop.DBus.ReloadConfig
Removed the file, but $DBUS_SESSION_BUS_ADDRESS still empty.

始まりの荒野を独り もう歩き出してるらしい、僕は灰になるまで僕で有り続けたい
http://about.me/nnhnkn | http://identi.ca/nnhzkn

Offline

#10 2017-02-02 20:09:50

cirrus
Member

From: Glasgow Scotland
Registered: 2012-08-24
Posts: 341
Website

Re: DBus session bus address is not properly set

On my single user 4x Archlinux installs echo $DBUS_SESSION_BUS_ADDRESS shows
DBUS_SESSION_BUS_ADDRESS=/run/user/1000/bus
see if that helps, as mentioned
/tmp/dbus-iDskgvL60r,guid=aaaa474d48be6e7fc55dbc43588e34f3
looks wrong.

Last edited by cirrus (2017-02-02 20:12:18)

Ancestoral Clan https://cirrus.freevar.com/mclean.html

Offline

#11 2017-02-02 20:18:40

skottish
Forum Fellow

From: Here
Registered: 2006-06-16
Posts: 7,942

Re: DBus session bus address is not properly set

ShionjiYuuko wrote:

Scimmia wrote:

What is /etc/systemd/user/dbus.service? That should not exist.
[Unit]
Description=D-Bus Message Bus
Requires=dbus.socket

[Service]
ExecStart=/usr/bin/dbus-daemon --session --address=systemd: --nofork --nopidfile --systemd-activation
ExecReload=/usr/bin/dbus-send --print-reply --session --type=method_call --dest=org.freedesktop.DBus / org.freedesktop.DBus.ReloadConfig
Removed the file, but $DBUS_SESSION_BUS_ADDRESS still empty.
Check to see if you have the same file somewhere under ~/.local/share/systemd. I did (don't know how it got there), removed it, and everything is good.
EDIT: I believe that it was under ~/.config/systemd and not ~/.local.

Last edited by skottish (2017-02-14 14:08:57)

Offline

#12 2017-02-14 03:49:13

vimusov
Member
From: Yekaterinburg
Registered: 2017-02-14
Posts: 2
Website

Re: DBus session bus address is not properly set

Same problem with dbus. Some programs can not be started (gnome-terminal) or can not save settings (meld) because of error "dbus-launch: Autolaunch requested, but X11 support not compiled in". I tried all advice I found with no luck.
$ loginctl show-session $XDG_SESSION_ID | sort
Docked=yes
EnableWallMessages=no
HandleHibernateKey=hibernate
HandleLidSwitchDocked=ignore
HandleLidSwitch=suspend
HandlePowerKey=poweroff
HandleSuspendKey=suspend
HoldoffTimeoutUSec=30s
IdleAction=ignore
IdleActionUSec=30min
IdleHint=yes
IdleSinceHint=0
IdleSinceHintMonotonic=0
InhibitDelayMaxUSec=5s
InhibitorsMax=8192
KillUserProcesses=no
NAutoVTs=6
NCurrentInhibitors=0
NCurrentSessions=0
PreparingForShutdown=no
PreparingForSleep=no
RebootToFirmwareSetup=no
RemoveIPC=yes
RuntimeDirectorySize=3372564480
SessionsMax=8192
UserTasksMax=18446744073709551615

$ systemctl status --user dbus.service
Failed to connect to bus: No such file or directory

$ env | grep XDG
<empty output>

$ env | grep DBUS
<empty output>

$ ls /run/user/
<empty output>

$ stat /etc/systemd/user/dbus.service
stat: cannot stat '/etc/systemd/user/dbus.service': No such file or directory

$ stat ~/.local/share/systemd
stat: cannot stat '/home/rics/.local/share/systemd': No such file or directory
Also I tried to move ~/.config to ~/.config.bk, reboot system, restore it back and reboot again - no luck.
I read these topics:
General_troubleshooting#Session_permissions
D-Bus#Starting_the_user_session
/news/d-bus-now-launches-user-buses/
The only one interesting thing I found:

First, make sure you have a valid local session within X:
$ loginctl show-session $XDG_SESSION_ID
This should contain Remote=no and Active=yes in the output. If it does not, make sure that X runs on the same tty where the login occurred. This is required in order to preserve the logind session.
But I do not understand what does it mean: "make sure that X runs on the same tty where the login occurred"? How can I check that? Here is my process list, maybe it could help:
$ ps wf -A -o user,group,pid,tty,stat,state,args
USER     GROUP      PID TT       STAT S COMMAND
root     root         1 ?        Ss   S /sbin/init
root     root       217 ?        Ss   S /usr/lib/systemd/systemd-journald
root     root       244 ?        Ss   S /usr/lib/systemd/systemd-udevd
systemd+ systemd+   416 ?        Ssl  S /usr/lib/systemd/systemd-timesyncd
root     root       422 ?        Ss   S /usr/lib/systemd/systemd-logind
root     root       423 ?        Ss   S /usr/bin/nodm
root     root       431 tty7     Ssl+ S  \_ /usr/lib/xorg-server/Xorg :0 vt7 -nolisten tcp
rics     rics       473 ?        S    S  \_ /usr/bin/nodm
rics     rics       474 ?        S    S      \_ fluxbox
rics     rics       478 ?        S    S          \_ xxkb
rics     rics       513 ?        Ssl  S          \_ firefox
rics     rics       620 ?        SLsl S          \_ deadbeef
rics     rics       631 ?        Ssl  S          \_ telegram-desktop
rics     rics      4021 ?        Ssl  S          \_ thunderbird
rics     rics       426 ttyS0    Ss+  S /usr/bin/fspupsmon -i 10 -s 5 -u rics
dbus     dbus       427 ?        Ss   S /usr/bin/dbus-daemon --system --address=systemd: --nofork --nopidfile --systemd-activation
rics     rics       447 ?        Ss   S /usr/bin/yacron --user rics
root     root       448 ?        Ss   S /usr/bin/cupsd -l
nobody   nobody     449 ?        Ss   S /usr/bin/deadwood -f /etc/deadwood.conf
root     root       450 ?        Ss   S /usr/bin/sshd -D
rics     rics       452 ?        Ss   S /usr/bin/anonftp --port 21 --user rics --root /srv
root     root       453 tty1     Ss+  S /sbin/agetty --noclear tty1 linux
colord   colord     454 ?        Ssl  S /usr/lib/colord/colord
root     root       459 ?        Ss   S nginx: master process /usr/bin/nginx -g pid /run/nginx.pid; error_log stderr;
rics     rics       460 ?        S    S  \_ nginx: worker process
rics     rics       490 ?        S    S xbindkeys
systemd+ systemd+ 28446 ?        Ss   S /usr/lib/systemd/systemd-networkd
I use nodm and fluxbox. Here are my configs:
/etc/nodm.conf:

NODM_USER='rics'
NODM_X_OPTIONS='vt7 -nolisten tcp'
NODM_MIN_SESSION_TIME=60
NODM_RESTART_SESSION=0
NODM_XINIT='/usr/bin/xinit'
NODM_XSESSION='/home/rics/.xinitrc'

~/.xinitrc:

#!/bin/bash
exec xbindkeys &
exec xxkb &
setxkbmap -layout "us,ru" -option "grp:caps_toggle,terminate:ctrl_alt_bksp"
exec startfluxbox

~/.fluxbox/startup:

#!/bin/sh
slock
exec fluxbox
My colleague have been using lightdm and Cinnamon and they work flawlessly.
I almost lost hope to fix that problem. Please give me advice, I will provide any information needed.

Offline

#13 2017-02-14 13:15:10

seth
Member

From: Won't reply 2 private help req
Registered: 2012-09-03
Posts: 77,628

Re: DBus session bus address is not properly set

No, you don't.
Don't use nodm but autologin with getty - see eg. https://bbs.archlinux.org/viewtopic.php?id=220954
https://wiki.archlinux.org/index.php/Xi … X_at_login
https://wiki.archlinux.org/index.php/Au … al_console

How to upload text · How to boot w/o GUI · Disable Windows Fast-Start! · Fix your xinitrc

Offline

#14 2017-02-14 22:08:42

ewaller
Administrator

From: Pasadena, CA
Registered: 2009-07-13
Posts: 20,700

Re: DBus session bus address is not properly set

vimusov,
It has been reported that your post is a thread hijack.  I tend to agree.  Please start a new thread and, if you believe this thread to be relevant, you may link back to this one.

Nothing is too wonderful to be true, if it be consistent with the laws of nature -- Michael Faraday
The shortest way to ruin a country is to give power to demagogues.— Dionysius of Halicarnassus
---
How to Ask Questions the Smart Way

Offline

#15 2017-02-15 02:30:41

vimusov
Member
From: Yekaterinburg
Registered: 2017-02-14
Posts: 2
Website

Re: DBus session bus address is not properly set

ewaller wrote:

vimusov,
It has been reported that your post is a thread hijack.  I tend to agree.  Please start a new thread and, if you believe this thread to be relevant, you may link back to this one.
I am sorry about that. I thought it would be better to not start a new topic when there is an existing one with the same problem. I am going to do as you say.

Offline

Pages: 1

Index

» Newbie Corner

» DBus session bus address is not properly set

Board footer

Jump to

Newbie Corner
Installation
Kernel & Hardware
Applications & Desktop Environments
Laptop Issues
Networking, Server, and Protection
Multimedia and Games
Arch Linux Guided Installer
System Administration
Other Architectures

Announcements, Package & Security Advisories
Arch Discussion
Forum & Wiki discussion

Pacman & Package Upgrade Issues
[testing] Repo Forum
Creating & Modifying Packages
AUR Issues, Discussion & PKGBUILD Requests

GNU/Linux Discussion
Community Contributions
Programming & Scripting
Other Languages
Artwork and Screenshots

Atom topic feed

Powered by FluxBB

## Related

- [[d-bus]]
