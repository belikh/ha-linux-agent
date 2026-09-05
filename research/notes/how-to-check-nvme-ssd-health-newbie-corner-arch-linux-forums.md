---
title: How to check NVMe SSD health? / Newbie Corner / Arch Linux Forums
id: how-to-check-nvme-ssd-health-newbie-corner-arch-linux-forums
tags:
- linux-agent-jupiteros-fleet-15537b
- smart
- home-assistant
- nvme
- birth-message
- practitioner-forum
created: '2026-09-02T06:42:41.453933Z'
updated: '2026-09-05T10:51:22.088066Z'
source: https://bbs.archlinux.org/viewtopic.php?id=248157
source_domain: bbs.archlinux.org
fetched_at: '2026-09-02T06:42:39.088275Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Arch forum thread (Aug 2019): user on Thinkpad T470 with Intel SSDPEKKF256G7L
  NVMe asks for non-destructive health checking. Moderator V1del: smartctl supports
  NVMe drives (recent versions); nvme-cli manpages are key — nvme-device-self-test,
  nvme-self-test-log, nvme-smart-log; vendor-specific commands may expose more; fsck
  is filesystem-level (useful regardless of block device); everything listed is non-destructive
  by default; SMART/self-tests usually runnable during normal operation while fsck/badblocks
  need unmounted FS. User''s verbatim nvme intel-id-ctrl /dev/nvme0 -H output shows
  an INTEL SSDPEKKF256G7L with ''Device Self-test Not Supported'' (oacs bit 4:0, edstt/dsto
  = 0) and ''health: healthy'' at the end of the identify output, wctemp 343K / cctemp
  353K (70°C/80°C), 5 power states (9.00W operational down to 0.005W non-operational).
  Lesson for an agent''s SMART collection design: NVMe health is NOT uniform — device
  self-test capability is optional in the NVMe spec (oacs/dsto bits), so an agent
  that assumes smartctl -t or self-test commands work will fail on consumer NVMe drives
  like this Intel 600p; nvme smart-log (controller-level, always available) is the
  reliable floor.'
---

How to check NVMe SSD health? / Newbie Corner / Arch Linux Forums

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

» How to check NVMe SSD health?

Pages: 1

#1 2019-08-02 11:08:59

maxgiggs
Member
Registered: 2014-06-05
Posts: 32

How to check NVMe SSD health?

Hi everyone!
Recently I had some errors while copying some files on GNOME/Files (the error message was a generic "input/output error", or something like that). Moreover, If I tried to calculate the md5sum of these files, I got the same error.
Therefore I started to suspect that my SSD (on NVMe, my system is a Thinkpad T470) was damaged and could fail in the recent future. How can I check if it is damaged? How may I get a report on its health status?
I am very confused about the tools I am supposed to use (fsck, badblock, smartctl, nvme-cli?). For example, it is not clear to me if SMART is supported by NVMe drives.
The most important thing is that I need a **non destructive** diagnostic tool, because the drive in question is my system root and contains all of my data.
In synthesis, is it possible to check non destructively if there is data corruption, if my drive is damaged and if there are signs of failure?
Thanks in advance!

Offline

#2 2019-08-02 11:53:12

V1del
Forum Moderator

Registered: 2012-10-16
Posts: 25,352

Re: How to check NVMe SSD health?

Smartctl has support for NVME drives in recent times, it can tell you whether SMART attributes are supported and what it reads.
Alternatively see the following nvme cli commands' manpages:
https://www.mankier.com/package/nvme-cli
In particular
https://www.mankier.com/1/nvme-device-self-test
https://www.mankier.com/1/nvme-self-test-log
https://www.mankier.com/1/nvme-smart-log
And note the potential vendor specific commands that might contain more information.
fsck is a filesystem check tool and useful regardless of underlying block device.
In general all of the tools you list here are non-destructive by default or have options to be non destructive, check relevant man pages.
Any further information will need the exact model of the drive, as supports here can differ depending on vendor.

Last edited by V1del (2019-08-02 11:57:26)

Offline

#3 2019-08-02 12:11:05

maxgiggs
Member
Registered: 2014-06-05
Posts: 32

Re: How to check NVMe SSD health?

Thanks for the hints on the nvme-cli! One of the (many) things that is not clear to me is whether or not these test (as well as fsck) should be performed while the drive is unmounted
The SSD should be the 256 GB Intel SSD 600p Series (at least `lscpi`say so, while `nvme list` says INTEL SSDPEKKF256G7L which should be the same). Any further indication would be of invaluable help!

Last edited by maxgiggs (2019-08-02 12:14:11)

Offline

#4 2019-08-02 12:18:44

V1del
Forum Moderator

Registered: 2012-10-16
Posts: 25,352

Re: How to check NVMe SSD health?

smartctls and self tests should usually be doable during normal operation. fsck should be done unmounted, badblocks probably as well (though I don't have much experience with it)

Offline

#5 2019-08-02 12:34:51

maxgiggs
Member
Registered: 2014-06-05
Posts: 32

Re: How to check NVMe SSD health?

It seems that self-test are not supported:
#nvme intel-id-ctrl /dev/nvme0 -H

NVME Identify Controller:
vid       : 0x8086
ssvid     : 0x8086
sn        : BTPY72030PF7256D
mn        : INTEL SSDPEKKF256G7L
fr        : 121P
rab       : 6
ieee      : 5cd2e4
cmic      : 0
[3:3] : 0	ANA not supported
[2:2] : 0	PCI
[1:1] : 0	Single Controller
[0:0] : 0	Single Port

mdts      : 5
cntlid    : 1
ver       : 10200
rtd3r     : 249f0
rtd3e     : 13880
oaes      : 0
[9:9] : 0	Firmware Activation Notices Not Supported
[8:8] : 0	Namespace Attribute Changed Event Not Supported

ctratt    : 0
[5:5] : 0	Predictable Latency Mode Not Supported
[4:4] : 0	Endurance Groups Not Supported
[3:3] : 0	Read Recovery Levels Not Supported
[2:2] : 0	NVM Sets Not Supported
[1:1] : 0	Non-Operational Power State Permissive Not Supported
[0:0] : 0	128-bit Host Identifier Not Supported

rrls      : 0
oacs      : 0x7
[8:8] : 0	Doorbell Buffer Config Not Supported
[7:7] : 0	Virtualization Management Not Supported
[6:6] : 0	NVMe-MI Send and Receive Not Supported
[5:5] : 0	Directives Not Supported
[4:4] : 0	Device Self-test Not Supported
[3:3] : 0	NS Management and Attachment Not Supported
[2:2] : 0x1	FW Commit and Download Supported
[1:1] : 0x1	Format NVM Supported
[0:0] : 0x1	Security Send and Receive Supported

acl       : 4
aerl      : 7
frmw      : 0x12
[4:4] : 0x1	Firmware Activate Without Reset Supported
[3:1] : 0x1	Number of Firmware Slots
[0:0] : 0	Firmware Slot 1 Read/Write

lpa       : 0x3
[3:3] : 0	Telemetry host/controller initiated log page Not Suporrted
[2:2] : 0	Extended data for Get Log Page Not Supported
[1:1] : 0x1	Command Effects Log Page Supported
[0:0] : 0x1	SMART/Health Log Page per NS Supported

elpe      : 63
npss      : 4
avscc     : 0
[0:0] : 0	Admin Vendor Specific Commands uses Vendor Specific Format

apsta     : 0x1
[0:0] : 0x1	Autonomous Power State Transitions Supported

wctemp    : 343
cctemp    : 353
mtfa      : 20
hmpre     : 0
hmmin     : 0
tnvmcap   : 0
unvmcap   : 0
rpmbs     : 0
[31:24]: 0	Access Size
[23:16]: 0	Total Size
[5:3] : 0	Authentication Method
[2:0] : 0	Number of RPMB Units

edstt     : 0
dsto      : 0
fwug      : 0
kas       : 0
hctma     : 0
[0:0] : 0	Host Controlled Thermal Management Not Supported

mntmt     : 0
mxtmt     : 0
sanicap   : 0
[2:2] : 0	Overwrite Sanitize Operation Not Supported
[1:1] : 0	Block Erase Sanitize Operation Not Supported
[0:0] : 0	Crypto Erase Sanitize Operation Not Supported

hmminds   : 0
hmmaxd    : 0
nsetidmax : 0
anatt     : 0
anacap    : 0
[7:7] : 0	Non-zero group ID Not Supported
[6:6] : 0	Group ID does not change
[4:4] : 0	ANA Change state Not Supported
[3:3] : 0	ANA Persistent Loss state Not Supported
[2:2] : 0	ANA Inaccessible state Not Supported
[1:1] : 0	ANA Non-optimized state Not Supported
[0:0] : 0	ANA Optimized state Not Supported

anagrpmax : 0
nanagrpid : 0
sqes      : 0x66
[7:4] : 0x6	Max SQ Entry Size (64)
[3:0] : 0x6	Min SQ Entry Size (64)

cqes      : 0x44
[7:4] : 0x4	Max CQ Entry Size (16)
[3:0] : 0x4	Min CQ Entry Size (16)

maxcmd    : 0
nn        : 1
oncs      : 0x1e
[7:7] : 0	Virtualization Management Not Supported
[6:6] : 0	Timestamp Not Supported
[5:5] : 0	Reservations Not Supported
[4:4] : 0x1	Save and Select Supported
[3:3] : 0x1	Write Zeroes Supported
[2:2] : 0x1	Data Set Management Supported
[1:1] : 0x1	Write Uncorrectable Supported
[0:0] : 0	Compare Not Supported

fuses     : 0
[0:0] : 0	Fused Compare and Write Not Supported

fna       : 0x4
[2:2] : 0x1	Crypto Erase Supported as part of Secure Erase
[1:1] : 0	Crypto Erase Applies to Single Namespace(s)
[0:0] : 0	Format Applies to Single Namespace(s)

vwc       : 0x1
[0:0] : 0x1	Volatile Write Cache Present

awun      : 0
awupf     : 0
nvscc     : 0
[0:0] : 0	NVM Vendor Specific Commands uses Vendor Specific Format

nwpc      : 0
[2:2] : 0	Permanent Write Protect Not Supported
[1:1] : 0	Write Protect Until Power Supply Not Supported
[0:0] : 0	No Write Protect and Write Protect Namespace Not Supported

acwu      : 0
sgls      : 0
[1:0]  : 0	Scatter-Gather Lists Not Supported

mnan      : 0
subnqn    :
ioccsz    : 0
iorcsz    : 0
icdoff    : 0
ctrattr   : 0
[0:0] : 0	Dynamic Controller Model

msdbd     : 0
ps    0 : mp:9.00W operational enlat:5 exlat:5 rrt:0 rrl:0
rwt:0 rwl:0 idle_power:- active_power:-
ps    1 : mp:4.60W operational enlat:30 exlat:30 rrt:1 rrl:1
rwt:1 rwl:1 idle_power:- active_power:-
ps    2 : mp:3.80W operational enlat:30 exlat:30 rrt:2 rrl:2
rwt:2 rwl:2 idle_power:- active_power:-
ps    3 : mp:0.0700W non-operational enlat:10000 exlat:300 rrt:3 rrl:3
rwt:3 rwl:3 idle_power:- active_power:-
ps    4 : mp:0.0050W non-operational enlat:2000 exlat:10000 rrt:4 rrl:4
rwt:4 rwl:4 idle_power:- active_power:-
ss      : 0
health  : healthy
bl      :
and neither smartctl tests:
#smartctl -c /dev/nvme0

smartctl 7.0 2018-12-30 r4883 [x86_64-linux-5.2.5-arch1-1-ARCH] (local build)
Copyright (C) 2002-18, Bruce Allen, Christian Franke, www.smartmontools.org

=== START OF INFORMATION SECTION ===
Firmware Updates (0x12):            1 Slot, no Reset required
Optional Admin Commands (0x0007):   Security Format Frmw_DL
Optional NVM Commands (0x001e):     Wr_Unc DS_Mngmt Wr_Zero Sav/Sel_Feat
Maximum Data Transfer Size:         32 Pages
Warning  Comp. Temp. Threshold:     70 Celsius
Critical Comp. Temp. Threshold:     80 Celsius

Supported Power States
St Op     Max   Active     Idle   RL RT WL WT  Ent_Lat  Ex_Lat
0 +     9.00W       -        -    0  0  0  0        5       5
1 +     4.60W       -        -    1  1  1  1       30      30
2 +     3.80W       -        -    2  2  2  2       30      30
3 -   0.0700W       -        -    3  3  3  3    10000     300
4 -   0.0050W       -        -    4  4  4  4     2000   10000

Supported LBA Sizes (NSID 0x1)
Id Fmt  Data  Metadt  Rel_Perf
0 +     512       0         0

Offline

#6 2019-08-02 12:42:55

maxgiggs
Member
Registered: 2014-06-05
Posts: 32

Re: How to check NVMe SSD health?

By the way, dmesg is showing these creepy messages
[  892.973888] print_req_error: critical medium error, dev nvme0n1, sector 96673920 flags 80700
[  892.994821] print_req_error: critical medium error, dev nvme0n1, sector 96674016 flags 0
[  893.012260] print_req_error: critical medium error, dev nvme0n1, sector 96674016 flags 0
[  893.029305] print_req_error: critical medium error, dev nvme0n1, sector 96674016 flags 0

Offline

#7 2019-08-02 12:50:56

V1del
Forum Moderator

Registered: 2012-10-16
Posts: 25,352

Re: How to check NVMe SSD health?

For proper smart data you will likely want to go down to the namespace level, e.g. nvme0n1 should be more telling.

Offline

#8 2019-08-02 12:52:19

maxgiggs
Member
Registered: 2014-06-05
Posts: 32

Re: How to check NVMe SSD health?

V1del wrote:

For proper smart data you will likely want to go down to the namespace level, e.g. nvme0n1 should be more telling.
Unfortunately I get the same output.

Offline

#9 2019-08-02 19:28:11

maxgiggs
Member
Registered: 2014-06-05
Posts: 32

Re: How to check NVMe SSD health?

In the end, `fsck -vcck /device/partition` was the most helpful among the listed commands. As expected, there were multiple read errors. However, now I can open or copy the corrupted files and I expect no more read errors from the damaged blocks: the situation is mitigated.
I wonder what was the cause of these error and if I should substitute my SSD. I noticed that I had the `discard` option in my fstab, could it be the origin of this premature SSD failures?

Offline

#10 2019-08-02 22:41:30

V1del
Forum Moderator

Registered: 2012-10-16
Posts: 25,352

Re: How to check NVMe SSD health?

Potentially, though it could also just be filesystem issues, it's not recommended to enable discards on the filesystem level, use a weekly fstrim instead.
Edit: Just saw this now, but the -c option just prints the capabilities, AFAIK intel SSDs will still log SMART data even without explicitly triggering tests, e.g. if I run smartctl -a instead I get
smartctl 7.0 2018-12-30 r4883 [x86_64-linux-5.2.4-arch1-1-ARCH] (local build)
Copyright (C) 2002-18, Bruce Allen, Christian Franke, www.smartmontools.org

=== START OF INFORMATION SECTION ===
Model Number:                       INTEL SSDPEDMW012T4
Serial Number:                      CVCQ511500BL1P2BGN
Firmware Version:                   8EV10135
PCI Vendor/Subsystem ID:            0x8086
IEEE OUI Identifier:                0x5cd2e4
Controller ID:                      0
Number of Namespaces:               1
Namespace 1 Size/Capacity:          1’200’243’695’616 [1.20 TB]
Namespace 1 Formatted LBA Size:     512
Local Time is:                      Sun Aug  4 13:29:12 2019 CEST
Firmware Updates (0x02):            1 Slot
Optional Admin Commands (0x0006):   Format Frmw_DL
Optional NVM Commands (0x0006):     Wr_Unc DS_Mngmt
Maximum Data Transfer Size:         32 Pages

Supported Power States
St Op     Max   Active     Idle   RL RT WL WT  Ent_Lat  Ex_Lat
0 +    25.00W       -        -    0  0  0  0        0       0

Supported LBA Sizes (NSID 0x1)
Id Fmt  Data  Metadt  Rel_Perf
0 +     512       0         2
1 -     512       8         2
2 -     512      16         2
3 -    4096       0         0
4 -    4096       8         0
5 -    4096      64         0
6 -    4096     128         0

=== START OF SMART DATA SECTION ===
SMART overall-health self-assessment test result: PASSED

SMART/Health Information (NVMe Log 0x02)
Critical Warning:                   0x00
Temperature:                        29 Celsius
Available Spare:                    100%
Available Spare Threshold:          10%
Percentage Used:                    1%
Data Units Read:                    1’905’455 [975 GB]
Data Units Written:                 2’031’890 [1.04 TB]
Host Read Commands:                 580’899’533
Host Write Commands:                377’068’472
Controller Busy Time:               22
Power Cycles:                       1’556
Power On Hours:                     13’506
Unsafe Shutdowns:                   41
Media and Data Integrity Errors:    0
Error Information Log Entries:      0

Error Information (NVMe Log 0x01, max 64 entries)
No Errors Logged

Last edited by V1del (2019-08-04 11:40:38)

Offline

#11 2020-01-01 19:22:53

probackup-nl
Member
From: Delft
Registered: 2017-11-15
Posts: 93
Website

Re: How to check NVMe SSD health?

V1del wrote:

For proper smart data you will likely want to go down to the namespace level, e.g. nvme0n1 should be more telling.
For the Intel SSD 600P Series (rev 03) NVMe I don't see any difference using smartctl between nvme0n1 and nvme0:
# /usr/bin/smartctl -A /dev/nvme0n1
smartctl 7.0 2018-12-30 r4883 [x86_64-linux-4.20.3-arch1-1-ARCH] (local build)
Copyright (C) 2002-18, Bruce Allen, Christian Franke, www.smartmontools.org

=== START OF SMART DATA SECTION ===
SMART/Health Information (NVMe Log 0x02)
Critical Warning:                   0x00
Temperature:                        25 Celsius
Available Spare:                    100%
Available Spare Threshold:          10%
Percentage Used:                    46%
Data Units Read:                    367,779 [188 GB]
Data Units Written:                 309,008,384 [158 TB]
Host Read Commands:                 1,919,941
Host Write Commands:                5,932,814,525
Controller Busy Time:               21,589
Power Cycles:                       99
Power On Hours:                     2,824
Unsafe Shutdowns:                   59
Media and Data Integrity Errors:    1
Error Information Log Entries:      1
Warning  Comp. Temperature Time:    0
Critical Comp. Temperature Time:    0

# /usr/bin/smartctl -A /dev/nvme0
smartctl 7.0 2018-12-30 r4883 [x86_64-linux-4.20.3-arch1-1-ARCH] (local build)
Copyright (C) 2002-18, Bruce Allen, Christian Franke, www.smartmontools.org

=== START OF SMART DATA SECTION ===
SMART/Health Information (NVMe Log 0x02)
Critical Warning:                   0x00
Temperature:                        25 Celsius
Available Spare:                    100%
Available Spare Threshold:          10%
Percentage Used:                    46%
Data Units Read:                    367,779 [188 GB]
Data Units Written:                 309,008,388 [158 TB]
Host Read Commands:                 1,919,941
Host Write Commands:                5,932,814,577
Controller Busy Time:               21,589
Power Cycles:                       99
Power On Hours:                     2,824
Unsafe Shutdowns:                   59
Media and Data Integrity Errors:    1
Error Information Log Entries:      1
Warning  Comp. Temperature Time:    0
Critical Comp. Temperature Time:    0

Last edited by probackup-nl (2020-01-01 19:23:27)

Offline

Pages: 1

Index

» Newbie Corner

» How to check NVMe SSD health?

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