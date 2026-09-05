---
title: RAM Utilization | Resource Utilization | Learn Netdata
id: ram-utilization-resource-utilization-learn-netdata
tags:
- linux-agent-jupiteros-fleet-15537b
- netdata
- official-docs
- resource-footprint
created: '2026-09-02T04:31:51.690276Z'
updated: '2026-09-02T17:37:22.116873Z'
source: https://learn.netdata.cloud/docs/netdata-agent/resource-utilization/ram
source_domain: learn.netdata.cloud
fetched_at: '2026-09-02T04:31:51.686133Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'Official Netdata RAM sizing doc (current, Netdata 2.1-era): default-database-tier
  agent needs about 16 KiB RAM per unique metric collected, independent of collection
  frequency; children default 100-200 MB depending on metric count. Parent cost model
  per metric: 1 KiB retained index + 20 KiB active collection (16 db + 4 collection
  structures) + 5 KiB if ML model = 26 KiB per actively-collected metric; per node:
  10 KiB index + 512 KiB reception + 512 KiB dispatch = 1034 KiB active. Worked example:
  2-Parent cluster with 1M active metrics from 500 nodes + 10M archived metrics from
  5000 nodes needs 35.7 GiB. Memory ballooning formula: memory = UNIQUE_METRICS x
  16KiB + CONFIGURED_CACHES (default 32 MiB); 1M series = ~16 GiB. Netdata 2.1 added
  [db].dbengine use all ram for caches and [db].dbengine out of memory protection
  (default 10% of system RAM, max 5 GiB) that auto-releases cache memory under pressure.
  Parents centralising systemd-journal logs should keep metrics and logs on separate
  disks; suggested page-cache sizing = 1/3 of available memory. Sizing model is the
  quantitative baseline for how many host metrics a jupiterOS agent can publish before
  a central collector chokes.'
---

*Suggested by [[insane-netdata-memory-usage-help-netdata-community-forums]] — official RAM sizing doc; thread cited resource docs twice*

RAM Utilization | Resource Utilization | Learn Netdata

Skip to main content

On this page

Using the default Database Tier configuration, Netdata needs about 16KiB per unique metric collected, independently of the data collection frequency.

Children​

Netdata by default should need 100MB to 200MB of RAM, depending on the number of metrics being collected.

This number can be lowered by limiting the number of Database Tiers or switching Database modes. For more information, check the Database section of our documentation.

Parents​
DescriptionScopeRAM RequiredNotesmetrics with retentiontime-series in the db1 KiBMetadata and indexesmetrics currently collectedtime-series collected20 KiB16 KiB for db + 4 KiB for collection structuresmetrics with Machine Learning Modelstime-series collected5 KiBThe trained models per dimensionnodes with retentionnodes in the db10 KiBMetadata and indexesnodes currently receivednodes collected512 KiBStructures and reception buffersnodes currently sentnodes collected512 KiBStructures and dispatch buffers

These numbers vary depending on metric name length, the average number of dimensions per instance and per context, the number and length of the labels added, the number of database tiers configured, the number of Machine Learning models maintained per metric and similar parameters. For most use cases, they represent the worst case scenario, so you may find out Netdata actually needs less than that.

Each metric currently being collected needs (1 index + 20 collection + 5 ml) = 26 KiB.  When it stops being collected, it needs 1 KiB (index).

Each node currently being collected needs (10 index + 512 reception + 512 dispatch) = 1034 KiB. When it stops being collected, it needs 10 KiB (index).

Example​

A Netdata cluster (two Parents) has one million currently collected metrics from 500 nodes, and 10 million archived metrics from 5000 nodes:
DescriptionEntriesRAM per EntryTotal RAMmetrics with retention11 million1 KiB10742 MiBmetrics currently collected1 million20 KiB19531 MiBmetrics with Machine Learning Models1 million5 KiB4883 MiBnodes with retention550010 KiB52 MiBnodes currently received500512 KiB256 MiBnodes currently sent500512 KiB256 MiBMemory required per node35.7 GiB

In highly volatile environments (like Kubernetes clusters), Database retention can significantly affect memory usage. Usually, reducing retention on higher Database Tiers helps to reduce memory usage.

Database Size​

Netdata supports memory ballooning to automatically adjust its Database memory size based on the number of time-series concurrently being collected.

The general formula, with the default configuration of Database Tiers, is:

memory = UNIQUE_METRICS x 16KiB + CONFIGURED_CACHES

The default CONFIGURED_CACHES is 32MiB.

To determine UNIQUE_METRICS for your Agent, query the /api/v3/info endpoint:

curl -s http://localhost:19999/api/v3/info | jq '.agents[0].metrics.collected'

This returns the number of unique time-series currently being collected.

For one million concurrently collected time-series (independently of their data collection frequency), the required memory is 16 GiB. In detail:

UNIQUE_METRICS = 1000000

CONFIGURED_CACHES = 32MiB

(UNIQUE_METRICS * 16KiB / 1024 in MiB) + CONFIGURED_CACHES =

( 1000000       * 16KiB / 1024 in MiB) + 32 MiB            =

15657 MiB =

about 16 GiB

Parents that also act as systemd-journal Logs centralization points​

Logs usually require significantly more disk space and I/O bandwidth than metrics. For optimal performance, we recommend to store metrics and logs on separate, independent disks.

Netdata uses direct-I/O for its Database to not pollute the system caches with its own data.

To optimize disk I/O, Netdata maintains its own private caches. The default settings of these caches are automatically adjusted to the minimum required size for acceptable metrics query performance.

systemd-journal on the other hand, relies on operating system caches for improving the query performance of logs. When the system lacks free memory, querying logs leads to increased disk I/O.

If you are experiencing slow responses and increased disk reads when metrics queries run, we suggest dedicating some more RAM to Netdata.

We frequently see that the following strategy gives the best results:

Start the Netdata Parent, send all the load you expect it to have and let it stabilize for a few hours. Netdata will now use the minimum memory it believes is required for smooth operation.

Check the available system memory.

Set the page cache in netdata.conf to use 1/3 of the available memory.

This will allow Netdata queries to have more caches, while leaving plenty of available memory of logs and the operating system.

In Netdata 2.1 we added the netdata.conf option [db].dbengine use all ram for caches and [db].dbengine out of memory protection.
Combining these two parameters is probably simpler to get best results:

[db].dbengine out of memory protection is by default 10% of total system RAM, but not more than 5GiB. When the amount of free memory is less than this, Netdata automatically starts releasing memory from its caches to avoid getting out of memory. On systemd-journal centralization points, set this to the amount of memory to be dedicated for systemd journal.

[db].dbengine use all ram for caches is by default no. Set it to yes to use all the memory except the memory given above.

With these settings, netdata will use all the memory available but leave the amount specified for systemd journal.

Do you have any feedback for this page? If so, you can open a new issue on our netdata/learn repository.

Children
Parents
Example
Database Size
Parents that also act as systemd-journal Logs centralization points