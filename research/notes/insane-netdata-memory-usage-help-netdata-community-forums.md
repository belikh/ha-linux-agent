---
title: insane netdata memory usage - Help - Netdata Community Forums
id: insane-netdata-memory-usage-help-netdata-community-forums
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- netdata
- known-issue
- resource-footprint
- practitioner-forum
created: '2026-09-02T04:02:40.489716Z'
updated: '2026-09-02T17:37:22.035138Z'
source: https://community.netdata.cloud/t/insane-netdata-memory-usage/3342
source_domain: community.netdata.cloud
fetched_at: '2026-09-02T04:02:34.756309Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'Netdata maintainer-answered community thread (Oct 2022-Jan 2023) documenting
  real-world agent memory blowups: 200-1000+ MB used per daemon on Debian 10/11, 2.5
  GB reserved (1.4 GB used) on 16 GB hosts collecting ~7500 metrics, up to ~10 GB
  committed. Root causes identified by Netdata founder Costa_Tsaousis: dbengine indexes
  are memory-resident (the ''main offender''), a genuine ARAL allocator memory leak
  (fixed Oct 2022), plus defaults tuned for standalone experience. Recommended production-child
  config: [db] mode=alloc retention=1800 storage tiers=1, [ml] enabled=no, [health]
  enabled=no - yielding <50 MB RAM per child, with metrics streamed to Parents. dbengine
  v2 (merged Jan 2023, v1.37) cut default RAM to ~100 MB with 3 tiers / ~1 year retention;
  journal-file PR #13885 reduced dbengine memory 80-90% at cost of 15% speed. Also:
  backfilling/replication PR #13873 removed streaming gaps; centos 9 stream leak traced
  to ebpf.plugin (issue #14288). Directly quantifies the bloat-vs-footprint tradeoff
  a Rust single-binary agent (ha-linux-agent) must beat for jupiterOS fleet viability.'
---

insane netdata memory usage - Help - Netdata Community Forums

insane netdata memory usage

Help

agent

cave

October 3, 2022,  5:34pm

1

Debian 10/11, latest netdata stable from official netdata repository (1.36.1). Database on each server, web server is accessed through nginx proxy.

For me, a good monitoring software should have 2 main attributes:

collect required data in a correct way, then store/present/correlate that data etc. (doh)

not interfering (too much) with the system performance (cpu, memory, disk usage)

Clearly we cannot compare “old school” monitoring software like munin or cacti (5 min samples with tens of data samples) with netdata (1 sec samples with thousands of data samples). But …

Installed “default”, had some insane memory usage for netdata daemon. From 200-400 MB for regular servers to 700-1000+MB for kvm hypervisors. USED memory. And that’s just in a single month.

With some tunning, it’s a little better (marginal), but still far away from the < 100 MB “promised” in the documentation.

Examples:

hypervisors: 450-500 MB (since 14 sept)

almost unused nginx server (restarted days before)

netdata    564  0.6  8.4 648972 172568 ?       SNsl Oct01  16:58 /usr/sbin/netdata -D -P /var/run/netdata/netdata.pid

nginx proxy for netdata

netdata      837  0.9 24.6 1156840 246228 ?      SNsl Sep14 271:51 /usr/sbin/netdata -D -P /var/run/netdata/netdata.pid

almost unused nginx server

netdata      807  1.1 27.4 1724912 274908 ?      SNsl Sep10 363:39 /usr/sbin/netdata -D -P /var/run/netdata/netdata.pid

unused vpn server

netdata     1078  1.0 30.3 1593188 303828 ?      SNsl Sep14 284:23 /usr/sbin/netdata -D -P /var/run/netdata/netdata.pid

as a matter of fact, on some servers with memory pressure (i.e. database servers) netdata memory usage is “good” (under 100 MB); but overall it feels like ‘java’, with more memory available on servers, more memory is used by netdata

also “committed” values are insane, maximum is almost 10 gigs !!!; as you know “committed” memory data is to be watched especially on app servers (php, java etc.), being a “worst case scenario” for memory allocation; but netdata with 1-2 GB committed “screws up” every servers’ monitoring data, with 1-2 Gigs committed on its own, so the memory graphic on older monitoring systems becomes unusable

please tell me what am I doing wrong? how can I reduce the memory used & allocated (commited) by netdata?

the configuration follows:

[global]

run as user = netdata

process scheduling policy = idle # run with least priority

OOM score = 1000

enable metric correlations = no

[db]

update every = 1

mode = dbengine

storage tiers = 1

dbengine page cache size MB = 32

dbengine disk space MB = 256

[logs]

access = none

[ml]

enabled = no

[web]

bind to = *

allow connections from = localhost $ip_of_nginx

[registry]

enabled = no

registry to announce = $url_to_announce

[plugins]

commented #    ebpf = no

charts.d = no

fping = no

python.d = no

statsd = no

Christopher_Akritid1

October 3, 2022,  6:53pm

2

The used memory depends mainly on four things: Number of metrics collected, Resolution, Retention, Caching.

We just completed the prototype of a calculator you can grab from [Feat]: Tiering Calculator · Issue #601 · netdata/netdata-cloud · GitHub and play with, but I’m certain that in your case, it’s the number of metrics + the default retention of ~7-10 days at per second resolution.

I’m not sure where the 100MB claim is and I’d love the link so we can correct it, because it is obsolete. It assumes no data stored on disk, i.e. memory mode RAM  and very low retention.

What we strongly suggest for low resource systems is to have the dbengine on a parent outside your production infrastructure, e.g. on a management node. Configure each other Netdata with the minimum retention you want to have (maybe even set them to RAM mode) and just stream all their metrics to the parent. On that parent configure the dbengine tiering, to reach your ideal balance of memory usage, cache efficiency, data retention and resolution.

We are of course constantly working to reduce the required memory from the dbengine, but this is what’s currently available to you, plus of course the suggestions at How to optimize the Netdata Agent's performance | Learn Netdata

cave

October 3, 2022,  7:23pm

3

This is the link: How to optimize the Netdata Agent's performance | Learn Netdata (you’ve just mentioned it :-P)

Maybe it’s time to better explain (in the documentation) what agent means, and the implications of using a local database (in terms of memory usage).

In my case, keeping the dbengine at minimum gives me 1 - max 2 days of data retention (at 1 second resolution). That was the idea, keeping 1 sec interval for 1 day, and then exporting at 2-5-10 seconds interval to  external database (not yet implemented).

I will try to send data immediately (without dbengine), last time I didn’t like the calculated traffic estimation, but in this case like I’ve said, it’s a memory hog. Let’s hope it will improve netdata usage, if not, with regrets, I will have to search for another monitoring system which doesn’t hog the memory.

Btw (I’m not sure I understood the docs): in case that the “master” netdata is down (i.e. reboot or smth), the local data is lost or there is some retention period (i.e. “local” netdata will retry to send local data to the “master”)

thank you!

Christopher_Akritid1

October 3, 2022,  9:52pm

4

You have complete control over the memory Netdata will use. It will make the most efficient possible use of your memory based on what you ask it to do. We’re talking about a web server, data collection agent and time series database in one.  So try setting up a parent with all the bells and whistles and keep the children light.

I will update the first paragraph in the link (this page is really old) to contain this type of information.

jurgenhaas

October 6, 2022,  8:09am

5

I’m facing a similar issue where the agents are using 2.5GB of RAM a few seconds after restarting the service on nodes that only come with 16GB of RAM. I’ve now reconfigured netdata with the following values, without any effect, memory usage is still the same:
[global]
run as user = netdata
page cache size = 32
dbengine multihost disk space = 256

[web]
bind to = *

[logs]
debug log = none
error log = none
access log = none

[db]
mode = dbengine
storage tiers = 1
update every = 5
dbengine multihost disk space MB = 256
dbengine page cache size MB = 32

This node collects around 7500 metrics and the calculator tells me that this setting should then use 160MB, which would be OK. But why am I seeing 2.5GB?

cave

October 6, 2022,  6:33pm

6

One question still haunts me: why on memory pressured systems (like db servers, where almost all memory is allocated) netdata is below 100MB, as it should be (well … quoting obsolete docs)? And how can I duplicate this “feature” on other servers, of course, without allocating memory in bulk. (All servers have the same netdata configuration).

And second one: why netdata needs so many committed memory (iirc committed is allocated but not really used at the moment)?

cave

October 6, 2022,  6:38pm

7

pay attention to which memory you report: there is USED memory, and COMMITTED memory (allocated but not used); 2.5G is huge as used, indeed, but as committed I have servers with almost 10G committed (which is insane), but “only :-P” ~850M actually used; maybe you should paste the output from “ps auxwf | fgrep netdata.pid | fgrep -v grep” (if not debian based maybe the command should vary)

jurgenhaas

October 7, 2022,  6:58am

8

I know, the 2.5GB is reserved not used. But used memory is still 1.4GB, see the output from ps:
netdata  25418  2.7  8.6 2686348 1423652 ?     SNsl Oct06  38:00 /usr/sbin/netdata -D -P /var/run/netdata/netdata.pid

This is 9% of available RAM and netdata is the highest RAM consumer on such hosts - I have many of them.

And even worse: because of the allocation by netdata, the system is starting to swap out memory usage e.g. for mysql, which is the just really terrible.

We need to get back control over resource usage by netdata, I remember we recommended it to all clients because of low resource consumption, but that’s no longer true.

Costa_Tsaousis

October 8, 2022,  9:38pm

9

Hi all,

Thank you for bringing this up @cave !

I think we have failed to explain this properly and we also missed a couple of important things that could make things easier for you.

Based on this discussion, I created PR full memory tracking and profiling of Netdata Agent by ktsaou · Pull Request #13789 · netdata/netdata · GitHub to help our devs understand the situation in a better way.

Let me explain…

We ship Netdata Agent with default settings to give you a simple standalone setup. Something you can run without any configuration. Most of the settings and options are enabled since we want you to experience the full thing.

However, as you noticed, these settings are pushing Netdata way above 100MB of memory footprint.

On production systems, we suggest to setup one or more parents to offload the production systems.  Any Netdata Agent can become a parent. It is a simple configuration you can have it running in minutes. You can also have as many parents as necessary. The more the better. You don’t need to centralize everything to just one parent.

Once you have a couple of parents running (active-active clustering is supported for high availability) and you push metrics from your production systems, you can change a few settings on the production Netdata Agents to lower significantly their memory footprint.

Generally, on production systems you should use (in netdata.conf) something like this:
[db]
storage tiers = 1
mode = alloc
retention = 1800

[ml]
enabled = no

[health]
enabled = no

With these settings, netdata should not use more than 50MB of RAM. It will have just half an hour of data, which will be used to backfill the parents if the connection between them gets interrupted (we are currently working on this feature [backfilling] - most likely will be released by the end of the month).

There is a catch! If you had dbengine before, it will still be initialized and use memory for its indexes. On PR full memory tracking and profiling of Netdata Agent by ktsaou · Pull Request #13789 · netdata/netdata · GitHub I made netdata figure out that it does not need dbengine and shut it down on startup.  If it is a new installation, dbengine will be initialized but it will be empty, so it will not affect memory.

At the same PR I have posted a screenshot of my Netdata Agent, with a configuration of 3 storage tiers and 8GB storage in total. As you can see there, it uses 1GB of RAM. 1 GB on RAM vs 8  GB on DISK may seem a lot, but you have to take into account compression. The raw db size is about 30GB and it needs 1GB to index it. It is pretty decent.

However,  the same server, same metrics, when we apply the settings I gave above, the memory footprint becomes like this:

image1226×535 114 KB

Less than 50MB.

Keep in mind that Speed and Memory are the only things we can trade. More Speed means more Memory. Less Memory means less Speed.

For Netdata, Speed is the number one priority. For everything we do, we always sacrifice Memory to achieve ultimate Speed.

I agree however that Memory on production systems is important and we should be very sensitive about it.

So, we encourage you to setup streaming and move metric data to a parent and then on the children use a very small dbengine footprint or db mode alloc. Also, disable Health and ML (run these at the parents).

dbengine is the main offender for memory consumption. We designed it to be very fast, but it needs a lot of memory mainly because its index is in memory.

On the other side, to make Netdata Parents scale better, we have added the option dbengine page descriptors in file mapped memory = yes which sets up a swap like mechanism for the dbengine index, trading Disk I/O for Memory).  But we recommend to use this setting only on systems that are dedicated to Netdata, or systems you don’t really care about their disk I/O.

Costa_Tsaousis

October 10, 2022,  9:29am

10

And we found a memory leak in the way dbengine allocated memory, so we also had a leak magnifying the problem.

Array Allocator Memory Leak Fix

netdata:master ← ktsaou:aral-cleanup

opened 11:29PM - 09 Oct 22 UTC

ktsaou

+134
-97

- [x] ARAL now cleans up left-over mmap files
- [x] ARAL now uses the standard …double linked list instead of the custom one
- [x] ARAL now validates (with `-DNETDATA_INTERNAL_CHECKS=1`) that there is no page with free slots before allocating a new page
- [x] ARAL had a memory leak. It was reusing only one element per page. This made ARAL allocate way more memory than needed when dbengine was rotating databases.

Thank you for reporting this guys, and thank you very much for insisting on this issue.

We have some way to go to make dbengine a lot more memory efficient. I promise we will solve this problem entirely before the end of the year.

Costa_Tsaousis

October 10, 2022,  6:15pm

11

And this is a fix for the VM size.

ARAL optimal alloc size

netdata:master ← ktsaou:aral-optimal-alloc-size

opened 06:09PM - 10 Oct 22 UTC

ktsaou

+3
-1

We did several tests to find how RSS and VM size of netdata are affected by ARAL…, that allocates dbengine page descriptors.

We found that VM size grows significantly with the default ARAL max size of 10MB, compared to plain `mallloc()`.

On our tests we got the same VM size with malloc and ARAL, for max allocation size of up to 256k.
We decided to go with 64k as a safe option that should work as expected among different libc versions and operating systems.

Generally, VM size is not that important and we don’t have any direct means to control it. But we found out that we can tune a parameter in our code base, to lower the virtual memory footprint of Netdata Agent.

jurgenhaas

October 15, 2022,  2:53pm

12

Thank you! Dealing with the issue with this attitude is why I really love netdata.

Costa_Tsaousis

November 2, 2022, 10:59am

13

As promised, Backfilling/Replication of data between Netdata agents is already merged into Netdata. So, Netdata Parents should not have any gaps in their data anymore. That PR was: Replication of metrics (gaps filling) during streaming by vkalintiris · Pull Request #13873 · netdata/netdata · GitHub

The permanent dbengine memory footprint fix is in this PR: https://github.com/netdata/netdata/pull/13885

This PR introduces a new journal file for dbengine, which we use as a read-only swap file, indexing all the data of dbengine, without directly allocating memory for it. Based on our tests this reduces the memory footprint of dbengine by 80 - 90%, while sacrificing just 15% of its speed and 10% of disk space.

At the same time we are working on Netdata Cloud to use the new weights endpoint for anomaly rates which will allow us to merge PR: Remove anomaly rates chart. by vkalintiris · Pull Request #13763 · netdata/netdata · GitHub

ML doubles the data collection memory today. With the merge of this PR we will eliminate the need for additional data collection memory entirely when ML is enabled.

We expect both of them to be merged in the next few days and released next week with Netdata 1.37.

Juan_Jose_Garcia_Gon

November 10, 2022,  9:55pm

14

In my infrastructure, in those computers that are quite reduced in hardware, I have always optimized the memory by reducing dbengine multihost disk space although it has never been clear to me the difference with dbengine disk space   saludos @Christopher_Akritid1 !!

Costa_Tsaousis

January 10, 2023,  7:59pm

15

Happy New Year!

We have merged dbengine v2 to Netdata agent repo!

This provides a significant memory reduction for all use cases. A Netdata Agent running with default settings on a 64 bit machine should now use about 100MB of RAM, using 3 storage tiers, providing about 1 year of data retention (about half that memory when the agent runs on 32-bit machines, even less when it runs on IoT with less metrics collected).

There is a plethora of additional improvements added, especially related to memory footprint and performance when the agent runs at scale (parent node centralizing metrics from dozens/hundreds of other agents).

Tonight this version will be available at the nightly release channel for all users.

Thank you again for bringing this topic up. Your comments really steered the progress of the project.

Happy New Year!

Enjoy Netdata!

Pavel_Rekun

January 17, 2023,  3:21pm

16

still have memory leak issues.

using cloud + child agents. each agent goes up to 1.4 gb then the os starts killing processes - leak happens instantly after netdata start.

added another node and configured as parent and connected all child nodes + on child nodes i’ve deleted the dbengine to make sure it’s not loaded on startup - this worked for couple of days normally and now once again agents at 1.4gb ram.

As a side note - issues happens on centos 9 stream with latest version of netdata agent v1.37.0-148-g906fd2daf

on older machines running centos 8 with same agent v1.37.0-148-g906fd2daf - no leaks at all.

Please advise.

ilyam8

January 17, 2023,  7:15pm

17

Hi, @Pavel_Rekun. If you think there is an issue, please create a bug report.

Costa_Tsaousis

January 17, 2023,  7:42pm

18

Pavel_Rekun:

using cloud + child agents. each agent goes up to 1.4 gb then the os starts killing processes - leak happens instantly after netdata start.

Pavel, we really more info about this. So, please create a bug report. The bug report wizard asks for additional info which is vital for us to find the problem.

@ilyam8 please setup a Centos stream on our lab to check if we can reproduce the problem.

Pavel_Rekun

January 18, 2023, 11:07am

19

opened [Bug]: memory leak on centos 9 stream · Issue #14288 · netdata/netdata · GitHub

I did notice something on the machines where the leak happens. running “service netdata status” i get:
● netdata.service - Real time performance monitoring
Loaded: loaded (/usr/lib/systemd/system/netdata.service; enabled; preset: disabled)
Active: active (running) since Wed 2023-01-18 03:57:08 UTC; 7h ago
Process: 532607 ExecStartPre=/bin/mkdir -p /opt/netdata/var/cache/netdata (code=exited, status=0/SUCCESS)
Process: 532608 ExecStartPre=/bin/chown -R netdata /opt/netdata/var/cache/netdata (code=exited, status=0/SUCCESS)
Process: 532609 ExecStartPre=/bin/mkdir -p /run/netdata (code=exited, status=0/SUCCESS)
Process: 532611 ExecStartPre=/bin/chown -R netdata /run/netdata (code=exited, status=0/SUCCESS)
Main PID: 532612 (netdata)
Tasks: 78 (limit: 18660)
Memory: 1.4G
CPU: 10min 37.292s
CGroup: /system.slice/netdata.service
├─532612 /opt/netdata/bin/srv/netdata -P /run/netdata/netdata.pid -D
├─532615 /opt/netdata/bin/srv/netdata --special-spawn-server
├─532774 /opt/netdata/usr/libexec/netdata/plugins.d/go.d.plugin 1
├─532778 /opt/netdata/usr/libexec/netdata/plugins.d/apps.plugin 1
├─533185 /opt/netdata/usr/libexec/netdata/plugins.d/ebpf.plugin 1
└─572289 bash /opt/netdata/usr/libexec/netdata/plugins.d/tc-qos-helper.sh 1

Jan 18 03:57:15 servicesdb02 systemd-coredump[532992]: [🡕] Process 532770 (ebpf.plugin) of user 986 dumped core.
Jan 18 03:57:27 servicesdb02 [533141]: Does not have a configuration file inside `/opt/netdata/etc/netdata/ebpf.d.conf. It will try to load stock file.
Jan 18 03:57:27 servicesdb02 [533141]: Cannot read process groups configuration file '/opt/netdata/etc/netdata/apps_groups.conf'. Will try '/opt/netdata/usr/lib/netdata/conf.d/app>
Jan 18 03:57:27 servicesdb02 [533141]: PROCFILE: Cannot open file '/proc/532770/status'
Jan 18 03:57:27 servicesdb02 [533141]: Cannot open /proc/532770/status
Jan 18 03:57:28 servicesdb02 systemd-coredump[533161]: [🡕] Process 533141 (ebpf.plugin) of user 986 dumped core.
Jan 18 03:57:41 servicesdb02 [533185]: Does not have a configuration file inside `/opt/netdata/etc/netdata/ebpf.d.conf. It will try to load stock file.
Jan 18 03:57:41 servicesdb02 [533185]: Cannot read process groups configuration file '/opt/netdata/etc/netdata/apps_groups.conf'. Will try '/opt/netdata/usr/lib/netdata/conf.d/app>
Jan 18 03:57:41 servicesdb02 [533185]: PROCFILE: Cannot open file '/proc/533141/status'
Jan 18 03:57:41 servicesdb02 [533185]: Cannot open /proc/533141/status
lines 1-29/29 (END)

this line show on 2 machine where leak happens while on the 3rd one no leak and doesn't show this line.
Jan 18 03:57:28 servicesdb02 systemd-coredump[533161]: [🡕] Process 533141 (ebpf.plugin) of user 986 dumped core.

Related to the issue?

ilyam8

January 18, 2023,  6:24pm

20

@Pavel_Rekun can you try to disable epbf.plugin and see if that helps?

It is netdata.conf->[plugins]->ebpf. You need to uncomment the line and set it to no.

next page →

Related topics

Topic

Replies
Views
Activity

Netdata consuming high ram amount

Help

agent

29

8662

February 6, 2022

Netdata using lots of Memory

Help

agent

16

801

November 27, 2023

Reduce memory footprint of Netdata Agent

Help

agent
,
configuration

4

535

January 6, 2024

How to Monitor with Netdata. A crash course for Absolute Beginners

General

how-to

1

7970

June 22, 2021

Frequent crashes on netdata parent, high throughput / high child count

Help

agent
,
dbengine
,
configuration

13

1588

June 16, 2022

Powered by Discourse, best viewed with JavaScript enabled