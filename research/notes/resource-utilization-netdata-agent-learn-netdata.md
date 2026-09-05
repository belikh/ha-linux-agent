---
title: Resource utilization | Netdata Agent | Learn Netdata
id: resource-utilization-netdata-agent-learn-netdata
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- official-docs
- netdata
- resource-footprint
created: '2026-09-02T04:02:40.483812Z'
updated: '2026-09-05T10:51:21.723384Z'
source: https://learn.netdata.cloud/docs/netdata-agent/resource-utilization
source_domain: learn.netdata.cloud
fetched_at: '2026-09-02T04:02:33.491503Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Official Netdata docs on agent resource footprint: CPU 1-5% of a single
  core with default settings (up to 5-20% in production), RAM 100-200 MB on an empty
  system rising to 250-350 MB in typical production, ~4 GiB disk by default (3 GiB
  metrics plus metadata), root required for install. Resource drivers: number of collected
  metrics, 1-second sample frequency, dbengine database mode and tier count (default
  3), ML training, streaming compression (zstd recommended). Canonical fleet pattern:
  Parent-Child streaming - run lightweight Child agents on edge hosts (db mode = ram
  eliminates child disk I/O) and centralise dbengine tiers/retention/ML on Parent
  nodes. Storage engine uses 32-bit float format with anomaly bit and fixed-step design.'
---

Resource utilization | Netdata Agent | Learn Netdata

Skip to main content

On this page

Netdata is designed to automatically adjust its resource consumption based on the specific workload.

Minimum system requirements​

A standalone Netdata Agent has a small footprint and runs comfortably on a minimal system. The table below shows Netdata's measured resource usage — follow the links for how each figure is derived.
ResourceNetdata's footprintCPU1%-5% of a single core with default settings; up to 5%-20% in productionRAM100-200 MB on an empty system; 250-350 MB in typical productionDisk~4 GiB by default (3 GiB metrics plus metadata), configurable per tierPrivilegesRoot on Linux, or Administrator on Windows, required for installation

For multi-node setups that centralize metrics on a Netdata Parent, resource needs scale with the number of Children and retention — see Parent Configuration Best Practices.

What affects resource usage​

This table shows the specific system resources affected by different Netdata features:
FeatureCPURAMDisk I/ODisk SpaceNetwork TrafficCollected metrics✓✓✓✓-Sample frequency✓-✓✓-Database mode and tiers-✓✓✓-Machine learning✓✓---Streaming✓✓--✓

Collected metrics

Impact: More metrics mean higher CPU, RAM, disk I/O, and disk space usage.

Optimization: To reduce resource consumption, consider lowering the number of collected metrics by disabling unnecessary data collectors.

Sample frequency

Impact: Netdata collects most metrics with 1-second granularity. This high frequency impacts CPU usage.

Optimization: Lowering the sampling frequency (e.g., 1-second to 2-second intervals) can halve CPU usage. Balance the need for detailed data with resource efficiency.

Database Mode

Impact: The default database mode, dbengine, compresses data and writes it to disk.

Optimization: In a Parent-Child setup, switch the Child's database mode to ram. This eliminates disk I/O for the Child.

Database Tiers

Impact: The number of database tiers directly affects memory consumption. More tiers mean higher memory usage.

Optimization: The default number of tiers is 3. Choose the appropriate number of tiers based on data retention requirements.

Machine Learning

Impact: Machine learning model training is CPU-intensive, affecting overall CPU usage.

Optimization: Consider disabling machine learning for less critical metrics or adjusting model training frequency.

Streaming Compression

Impact: Compression algorithm choice affects CPU usage and network traffic.

Optimization: Select an algorithm that balances CPU efficiency with network bandwidth requirements (e.g., zstd for a good balance).

Minimizing the resources used by Netdata Agents​

To optimize resource utilization, consider using a Parent-Child setup.

This approach involves centralizing the collection and processing of metrics on Parent nodes while running lightweight Children Agents on edge devices.

Maximizing the scale of Parent Agents​

Parents dynamically adjust their resource usage based on the volume of metrics received. However, for optimal query performance, you may need to dedicate more RAM.

Check RAM Requirements for more information.

Netdata's performance and scalability optimization techniques​

Minimal Disk I/O

Netdata directly writes metric data to disk, bypassing system caches and reducing I/O overhead. Additionally, its optimized data structures minimize disk space and memory usage through efficient compression and timestamping.

Compact Storage Engine

Netdata uses a custom 32-bit floating-point format tailored for efficient storage of time-series data, along with an anomaly bit. This, combined with a fixed-step database design, enables efficient storage and retrieval of data. Timestamp optimization further reduces storage overhead by storing timestamps at regular intervals.

For per-tier on-disk sample sizes, see Disk Requirements & Retention.

Intelligent Query Engine

Netdata prioritizes interactive queries over background tasks like machine learning and replication, ensuring optimal user experience, especially under heavy load.

Efficient Label Storage

Netdata uses pointers to reference shared label key-value pairs, minimizing memory usage, especially in highly dynamic environments.

Scalable Streaming Protocol

Netdata's streaming protocol enables the creation of distributed monitoring setups, where Children offload data processing to Parents, optimizing resource utilization.

Do you have any feedback for this page? If so, you can open a new issue on our netdata/learn repository.

Minimum system requirements
What affects resource usage
Minimizing the resources used by Netdata Agents
Maximizing the scale of Parent Agents
Netdata's performance and scalability optimization techniques