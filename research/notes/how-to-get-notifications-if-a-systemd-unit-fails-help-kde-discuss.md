---
title: How to get notifications if a SystemD unit fails - Help - KDE Discuss
id: how-to-get-notifications-if-a-systemd-unit-fails-help-kde-discuss
tags:
- linux-agent-jupiteros-fleet-15537b
- systemd
- known-issue
- practitioner-forum
- failure-notifications
created: '2026-09-02T06:42:41.460458Z'
updated: '2026-09-02T17:37:22.350853Z'
source: https://discuss.kde.org/t/how-to-get-notifications-if-a-systemd-unit-fails/5506
source_domain: discuss.kde.org
fetched_at: '2026-09-02T06:42:40.638325Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'KDE Discuss thread (Sept 2023) on surfacing failed systemd units as desktop
  notifications. Three approaches emerge: (1) jinliu''s bash loop — run ''systemctl
  status --failed'' (+ --user) every 5s into /run/user/1000, cmp against .old file,
  notify-send on change; (2) tubbadu''s Scriptinator plasmoid — same probe as init/periodic/onClick
  scripts emitting {PlasmoidStatusStart}...{PlasmoidIconStart}... tags; iterative
  fixes needed: ''systemctl status --failed'' output embeds timestamps so it ALWAYS
  differs between runs — must use ''systemctl --failed'' (list only) instead; also
  icon names vary by system (system-error vs computer-fail-symbolic), and a grep-for-failed
  guard is needed so fixed units don''t re-alert from output drift; (3) Herzenschein''s
  systemd-native answer: [Unit] OnFailure= (activate another unit on failure) and
  [Service] ExecStopPost= (arbitrary command after stop/failure, e.g. ExecStopPost=/usr/bin/notify-send
  ''Service X failed!''). Load-bearing for the agent design: the systemd-native mechanisms
  (OnFailure=, ExecStopPost=) are the event-driven pattern that avoids polling entirely
  — a fleet agent should subscribe to unit state changes (or use OnFailure drop-ins)
  rather than shell out to systemctl --failed on a timer; and if polling, systemctl
  --failed (stable list form) not systemctl status --failed (timestamped, always-differs).'
---

How to get notifications if a SystemD unit fails - Help - KDE Discuss

How to get notifications if a SystemD unit fails

Help

systemd

hook

September 26, 2023, 12:45pm

1

I just noticed that my backups, triggered by a SystemD timer, failed (a few times in a row) …and that got me thinking.

Why don’t SystemD (and similar) errors show in the System Notifications?

How can I have them show as either notifications or a list of failed units (e.g. as a plasmoid in the panel)?

jinliu

September 26, 2023,  1:34pm

2

Can be done with a bash script
#!/bin/bash

set -x

cd $XDG_RUNTIME_DIR
echo -n > systemd-status.old

while true; do
systemctl status --failed > systemd-status
systemctl --user status --failed >> systemd-status

if ! cmp -s systemd-status{,.old}; then
notify-send --app-name=systemd-monitor "$(cat systemd-status)"
fi

mv -f systemd-status{,.old}
sleep 5
done

hook

September 26, 2023,  2:36pm

3

That is pretty cool.

I was thinking something more KDE-ish and persistent perhaps.

But if no-one has a better suggestion, I might start off with the script, thanks.

hook

September 27, 2023,  2:06pm

4

I just found @tubbadu’s Scriptinator and I hope I can figure out how to use that to do what I want.

GitHub - tubbadu/Scriptinator: Behold! the Scriptinator! A plasmoid button to...

Behold! the Scriptinator! A plasmoid button to launch custom scripts on click and on mouse scroll, allowing you to change icon dynamically - GitHub - tubbadu/Scriptinator: Behold! the Scriptinator!...

To be brutally honest, I am a bit lost in the help of it. I will poke it around a bit if I can figure it out

hook

September 27, 2023,  2:15pm

5

Let’s see if this works:
#!/bin/bash

set -x

cd $XDG_RUNTIME_DIR
echo -n > systemd-status.old

systemctl status --failed > systemd-status
systemctl --user status --failed >> systemd-status

if ! cmp -s systemd-status{,.old}; then
# notify-send --app-name=systemd-monitor "$(cat systemd-status)"
echo "{PlasmoidStatusStart}attention{PlasmoidStatusEnd}"
echo "{PlasmoidIconStart}(dialog-error){PlasmoidIconEnd}"
echo "{PlasmoidTooltipStart}"
cat systemd-status
echo "{PlasmoidToolTipEnd}"
fi

mv -f systemd-status{,.old}

edit: fixed the script … I think … no I didn’t, at least it does not seem to work with Scriptinator

tubbadu

September 27, 2023,  2:35pm

6

wow someone is using my plasmoids!

you can use the Scriptinator hidden in the system tray (you have to enable it by rightclicking on the tray > configure system tray > entries > Scriprinator and set it to “Show when relevant”. Then you can use the script suggested above (perhaps without the while loop), adding the echo {PlasmoidStatusStart}insert new status here{PlasmoidStatusEnd}:
#!/bin/bash

set -x

cd $XDG_RUNTIME_DIR
echo -n > systemd-status.old

systemctl status --failed > systemd-
systemctl --user status --failed >> systemd-status

if ! cmp -s systemd-status{,.old}; then
notify-send --app-name=systemd-monitor "$(cat systemd-status)"
echo "{PlasmoidStatusStart}active{PlasmoidStatusEnd}"
echo "{PlasmoidIconStart}dialog-error{PlasmoidIconEnd}"
else
echo "{PlasmoidStatusStart}passive{PlasmoidStatusEnd}"
echo "{PlasmoidIconStart}dialog-ok{PlasmoidIconEnd}"
fi

mv -f systemd-status{,.old}

save this script somewhere and copy its location (let’s say /home/hook/Documents/systemd-status-scriptinator.sh). then open the system tray and right-click on Scriptinator > configure scriptinator. Let’s say you want to check for systemd errors every 30 seconds, and at startup.

set “Init script” and “periodic script” both to bash /home/hook/Documents/systemd-status-scriptinator.sh (or whatever path your script is). Set the timeout to 30 (or to whatever time you wish) then apply and click OK, and you should be done! Scriptinator will run the script every 30 seconds, checking for systemd error. If errors are found, it will appear in your panel with a red error icon, and it will instead stay quiet hidden in the tray with a “no problem” icon if no error is found. You can optionally set it to trigger the script also on click, so if you’re trying to solve the problem you don’t have to wait 30 seconds to understand if you fixed it or not

hope this helps! feel free to ask anything!

(disclaimer: I haven’t tested the script, it may not work as intended)

(disclaimer 2: I wrote scriptinator in my free time, so it has some problems and may not always work as it should. If you find any problem, feel free to report it and I’ll fix them as soon as I can work on it)

tubbadu

September 27, 2023,  2:45pm

7

make sure that dialog-error has no brackets (don’t do (dialog-error)), as scriptinator will only take what’s inside the {PlasmoidStatus___} tags and place it as icon, without verifying its existence (although it may be a cool feature, I can add it in the future)

echo "{PlasmoidStatusStart}attention{PlasmoidStatusEnd}"

setting it to attention will make it pulse forever, until a new status is set. You may want to add, in the OnClick script, a way to make it stop pulsing (for instance setting its status to active, so that it will still be visible, but without pulsing)

hook

September 27, 2023,  2:52pm

8

Thanks!

Scriptinator seems to work, but something in the script seems not to trigger the change. I just triggered the backup unit and it failed, but Scriptinator still shows a  even after I click it. (I did set it up at init, periodic and on-click).

tubbadu

September 27, 2023,  2:53pm

9

can you please try to run the script inside a terminal to see what the output is? So we can understand if the problem is the script or scriptinator itself

hook

September 27, 2023,  2:58pm

10

I think the main problem is that the script cleans up after itself. The script only reports the error the first time it was ran after a SystemD unit failed.

So if I run it directly after the unit fails, I get:
+ cd /run/user/1000
+ echo -n
+ systemctl status --failed
+ systemctl --user status --failed
+ cmp -s systemd-status systemd-status.old
+ echo '{PlasmoidStatusStart}attention{PlasmoidStatusEnd}'
{PlasmoidStatusStart}attention{PlasmoidStatusEnd}
+ echo '{PlasmoidIconStart}(dialog-error){PlasmoidIconEnd}'
{PlasmoidIconStart}(dialog-error){PlasmoidIconEnd}
+ echo '{PlasmoidTooltipStart}'
{PlasmoidTooltipStart}
+ cat systemd-status
× borgmatic.service - borgmatic backup
Loaded: loaded (/etc/systemd/system/borgmatic.service; static)
Active: failed (Result: exit-code) since Wed 2023-09-27 16:21:23 CEST; 12s ago
TriggeredBy: ● borgmatic.timer
Process: 30288 ExecStartPre=sleep 1m (code=exited, status=0/SUCCESS)
Process: 30503 ExecStart=systemd-inhibit --who=borgmatic --what=sleep:shutdown --why=Prevent interrupting scheduled backup /usr/bin/borgmatic --verbosity -2 --syslog-verbosity 1 (code=exited, status=1/FAILURE)
Main PID: 30503 (code=exited, status=1/FAILURE)
CPU: 803ms

sep 27 16:21:23 leza borgmatic[30504]: CRITICAL /etc/borgmatic/config.yaml: An error occurred
sep 27 16:21:23 leza borgmatic[30504]: CRITICAL backupserver: Error running actions for repository
sep 27 16:21:23 leza borgmatic[30504]: CRITICAL Remote: ssh: connect to host xmarksthespot.wheremymonkeyis.at port 22111: No route to host
Connection closed by remote host. Is borg working on the server?
sep 27 16:21:23 leza borgmatic[30504]: CRITICAL Command 'borg create --exclude-from /etc/borgmatic/excludes --exclude-caches --exclude-if-present .nobackup --info ssh://backup@xmarksthespot.wheremymonkeyis.at/./leza::{hostname}-{now:%Y-%m-%dT%H:%M:%S.%f} /etc /home /root/.borgmatic' returned non-zero exit status 2.
sep 27 16:21:23 leza borgmatic[30504]: CRITICAL
sep 27 16:21:23 leza borgmatic[30504]: CRITICAL Need some help? https://torsion.org/borgmatic/#issues
sep 27 16:21:23 leza systemd-inhibit[30503]: /usr/bin/borgmatic failed with exit status 1.
sep 27 16:21:23 leza systemd[1]: borgmatic.service: Main process exited, code=exited, status=1/FAILURE
sep 27 16:21:23 leza systemd[1]: borgmatic.service: Failed with result 'exit-code'.
sep 27 16:21:23 leza systemd[1]: Failed to start borgmatic backup.
+ echo '{PlasmoidToolTipEnd}'
{PlasmoidToolTipEnd}
+ mv -f systemd-status systemd-status.old

But any time after that I get only this:
+ cd /run/user/1000
+ echo -n
+ systemctl status --failed
+ systemctl --user status --failed
+ cmp -s systemd-status systemd-status.old
+ echo '{PlasmoidStatusStart}passive{PlasmoidStatusEnd}'
{PlasmoidStatusStart}passive{PlasmoidStatusEnd}
+ echo '{PlasmoidIconStart}dialog-ok{PlasmoidIconEnd}'
{PlasmoidIconStart}dialog-ok{PlasmoidIconEnd}
+ mv -f systemd-status systemd-status.old

(@jinliu had a different approach in mind when he wrote it and as stand-alone his worked fine. It’s my fault I am not skilled enough to figure out how to do it otherwise.)

tubbadu

September 27, 2023,  3:17pm

11

gothca, the problem as you say is that the script is done to throw a notification once when a new error is detected, and then wait for a new error. What we’re trying to achieve is instead a way to detect when the error is raised, and then keep the error icon until… well, until you notice it and fix it, I guess. We can do something like this:

at the beginning, an empty systemd-status.old file is created

periodically, it checks if the new systemd-status file is different from the old one

if nothing changed, it means no errors were raised, so it can just exit (leaving the same icon as before)

if it is different, then a new error appeared. we will then set the icon to the error one and the status to attention, until you (for example) click on it.

when you click on it, you are saying “all errors happened until now are now fixed”. So when clicked it will move the systemd-status file to .old, meaning that it will start listening for other errors. It should then set the “no error” icon now, and set the status to passive (or active if you still want to see it in the tray)

would this solution be good for you?

hook

September 27, 2023,  4:02pm

12

tubbadu:

What we’re trying to achieve is instead a way to detect when the error is raised, and then keep the error icon until… well, until you notice it and fix it, I guess. We can do something like this:

Exactly!

Your suggestion sounds really good.

I think maybe only the following:

tubbadu:

if nothing changed, it means no errors were raised, so it can just exit (leaving the same icon as before)

…would instead mean no new errors were raised. But I think the logic still works as intended.

tubbadu

September 27, 2023,  4:26pm

13

hook:

…would instead mean no new errors were raised. But I think the logic still works as intended.

yeah exactly!

so you can set up scriptinator to work like this:

init script: create an empty systemd-status.old:
cd $XDG_RUNTIME_DIR && echo -n > systemd-status.old

periodic script: check for new errors, and change scriptinator status if found:
cd $XDG_RUNTIME_DIR

systemctl status --failed > systemd-
systemctl --user status --failed >> systemd-status

if ! cmp -s systemd-status{,.old}; then
echo "{PlasmoidStatusStart}active{PlasmoidStatusEnd}"
echo "{PlasmoidIconStart}dialog-error{PlasmoidIconEnd}"
else
# do nothing
fi

onClick script: set scriptinator status to no-error and reset the systemd-status.old to the current situation
cd $XDG_RUNTIME_DIR
systemctl status --failed > systemd-status.old
systemctl --user status --failed >> systemd-status.old

echo "{PlasmoidStatusStart}passive{PlasmoidStatusEnd}" # set to active if you want the icon not to hide in the tray
echo "{PlasmoidIconStart}dialog-ok{PlasmoidIconEnd}"

hope this works!

hook

September 27, 2023,  6:06pm

14

Does not seem to work, I’m afraid.

There was an error in the second script, so I commented out 10th (else) line. I also s/systemd-/systemd-status in the 4th line.

Even then it does not seem to trigger Scriptinator.

Is this how it should be?

Screenshot_20230927_1947581060×1134 146 KB

hook

September 27, 2023,  9:09pm

15

Progress!

But then a new problem was that it triggers every time, because when it runs systemctl status --failed it records the timestamp when it ran. So it was always different from the old.

So I changed it to use the less chatty systemctl --failed instead.

While I was at it, I ran it past shellcheck --shell sh to make it work in sh too.

scriptinator_is_systemd_ok.sh is now:
#!/bin/sh

cd "$XDG_RUNTIME_DIR" || exit

systemctl --failed > systemd-status

if ! cmp --quiet systemd-status systemd-status.old
then
echo "{PlasmoidStatusStart}attention{PlasmoidStatusEnd}"
echo "{PlasmoidIconStart}system-error{PlasmoidIconEnd}"
echo "{PlasmoidTooltipStart}"
systemctl --failed | sed -e '/^$/,$d'
echo "{PlasmoidTooltipEnd}"
# else
# do nothing
fi

and scriptinator_systemd_is_ok_now.sh:
#!/bin/sh

cd "$XDG_RUNTIME_DIR" || exit

systemctl --failed > systemd-status.old

echo "{PlasmoidStatusStart}passive{PlasmoidStatusEnd}" # set to active if you want the icon not to hide in the tray
echo "{PlasmoidIconStart}system{PlasmoidIconEnd}"
echo "{PlasmoidTooltipStart}SystemD is running fine.{PlasmoidTooltipEnd}"

I also changed the settings to have “custom tooltip” enabled, and now it works pretty much like I want it to

Thank you both, @tubbadu and @jinliu !

For anyone else trying to set it up, this are the Scriptinator settings:

Screenshot_20230927_234550970×1044 141 KB

WilsonEPhillips

September 28, 2023, 12:07am

16

As a side note, if you tend to keep a session of htop running, you can monitor systemd units.

Screenshot_20230927_190427866×791 104 KB

jinliu

September 28, 2023,  3:52am

17

Thanks! I tried your scripts. It works fine except some minor issues:

My system doesn’t have “system” and “system-error” icons, so I use “system-run” and “computer-fail-symbolic” instead.

I don’t see the “SystemD is running fine.” tooltip. It shows “Behold! The Scriptinator!” instead. No idea why.

If I click the tray icon to dismiss it, then restart the failed systemd service, the icon would re-appear, since output of systemctl changed. So I modified scriptinator_is_systemd_ok.sh:

#!/bin/sh

cd "$XDG_RUNTIME_DIR" || exit

systemctl --failed > systemd-status

if ! cmp --quiet systemd-status systemd-status.old
then
if grep failed systemd-status
then
echo "{PlasmoidStatusStart}attention{PlasmoidStatusEnd}"
echo "{PlasmoidIconStart}system-error{PlasmoidIconEnd}"
echo "{PlasmoidTooltipStart}"
systemctl --failed | sed -e '/^$/,$d'
echo "{PlasmoidTooltipEnd}"
else
cp systemd-status systemd-status.old
fi
# else
# do nothing
fi

hook

September 28, 2023,  7:38am

18

WilsonEPhillips:

As a side note, if you tend to keep a session of htop running, you can monitor systemd units.

I started using btop and btm too, but that is a great tip, thanks!

hook

September 28, 2023,  7:44am

19

jinliu:

My system doesn’t have “system” and “system-error” icons, so I use “system-run” and “computer-fail-symbolic” instead.

Oooh, computer-fail is a good one!

jinliu:

I don’t see the “SystemD is running fine.” tooltip. It shows “Behold! The Scriptinator!” instead. No idea why.

I think that’s an issue with Scriptinator. I noticed that too. What seems to happen is that when you click i, it shows the correct tooltip for a really short time and, at least in my case, at the upper-left corner of the screen.

jinliu:

If I click the tray icon to dismiss it, then restart the failed systemd service, the icon would re-appear, since output of systemctl changed. So I modified scriptinator_is_systemd_ok.sh:

Yes, that is a limitation in my version, that I noticed too, but was too tired to figure out how to fix it. Your modification makes a lot of sense, thanks!

Herzenschein

September 28, 2023, 12:45pm

20

Uhh, I know this went in a different direction, but answering part of the original question:

hook:

How can I have [systemd timer failures] show as notifications?

There’s OnFailure (which lets you specify another service) and ExecStopPost (which lets you specify an arbitrary command). Something like this:
[Unit]
Description=My X service

[Service]
ExecStart=/usr/bin/false
ExecStopPost=/usr/bin/notify-send "Service X failed!"

next page →

Powered by Discourse, best viewed with JavaScript enabled