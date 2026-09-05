---
title: Agent Performance Optimization Guide | Configuration
id: agent-performance-optimization-guide-configuration
tags:
- linux-agent-jupiteros-fleet-15537b
- netdata
- official-docs
- resource-footprint
created: '2026-09-02T04:33:06.023063Z'
updated: '2026-09-05T10:51:21.830256Z'
source: https://learn.netdata.cloud/docs/netdata-agent/configuration/performance-optimization
source_domain: learn.netdata.cloud
fetched_at: '2026-09-02T04:33:06.019909Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'Official Netdata Agent Performance Optimization Guide (docs updated Mar
  2026): strategy matrix - Parent-Child architecture, disabling unneeded collectors,
  and reducing collection frequency cut CPU/RAM/disk-IO; adjusting retention and RAM
  mode cut RAM/disk-IO; turning off ML on Children cuts CPU only. Key semantics: only
  ACTIVE collectors consume resources (inactive plugins shut down automatically);
  Children in Parent-Child setups should run db mode = ram (no persistent local storage
  needed since they stream); ML belongs on Parents (''complete data picture'', more
  resources) with [ml] enabled = no on Children; shorter retention also means faster
  agent startup. Defaults being optimised away: per-second collection, auto app discovery,
  health alerting, per-metric ML training. This is the vendor''s own admission that
  out-of-the-box agent behaviour is too heavy for edge/fleet hosts - the exact tuning
  surface a purpose-built light agent (ha-linux-agent) eliminates by construction
  on jupiterOS.'
---

*Suggested by [[insane-netdata-memory-usage-help-netdata-community-forums]] — cited twice by Netdata staff as the optimization reference*

Agent Performance Optimization Guide | Configuration

Skip to main content

On this page

While Netdata Agents work seamlessly out-of-the-box with comprehensive monitoring, you can tune their configuration for better performance when needed.

Why optimize your Agent​

By default, your Netdata Agent provides:

Automatic Application Discovery: Continuously detects and monitors applications on your node

Real-time Metric Collection: Collects metrics every second

Health Monitoring: Actively tracks health status with built-in alerting

Machine Learning: Trains anomaly detection models for each metric (Anomaly Detection)

These features deliver comprehensive monitoring but consume system resources. You might need to optimize when running Agents on resource-constrained systems or when scaling your monitoring infrastructure.

note

See Resource Utilization for detailed Agent resource requirements.

How to optimize performance​

Here's how each optimization strategy reduces resource usage:
Optimization StrategyReduces CPUReduces RAMReduces Disk IOSet up Parent-Child architecture✓✓✓Disable unneeded collectors✓✓✓Reduce collection frequency✓✓Adjust metric retention✓✓Switch to RAM mode✓✓Turn off ML on Children✓

Set up Parent-Child architecture​

Transform your monitoring by using Parent nodes as centralization points. Parents collect and aggregate data from multiple Child nodes, significantly reducing the load on individual systems.

In this setup:

Children stream their metrics to Parents instead of storing everything locally

Parents handle data aggregation, storage, and dashboard queries

You access all metrics through the Parent nodes

tip

This architecture works especially well in production environments where you monitor many systems. Learn more in our Centralization Points documentation.

Disable unneeded collectors​

Reduce resource usage by turning off Plugins or Collectors you don't need.

Important

Only active collectors consume resources. Inactive plugins and collectors shut down automatically, so you only save resources by disabling those currently running and collecting metrics.

Follow our configuration guide to identify and disable specific collectors.

Reduce collection frequency​

Save CPU and disk IO by collecting metrics less frequently. If you don't need per-second precision, or if your Agent consumes too much CPU during periods of low dashboard activity, increase the collection interval.

This change:

Significantly reduces CPU usage

Decreases disk write operations

Maintains meaningful monitoring capabilities

Learn how to adjust collection frequency in our configuration guide.

Adjust metric retention​

Control memory and disk usage by changing how long your Agent stores historical data. Shorter retention periods mean:

Less RAM needed for in-memory metrics

Reduced disk space requirements

Faster Agent startup times

Configure retention settings using our database configuration guide.

Switch to RAM mode​

For IoT devices and Child nodes in Parent-Child setups, switch to RAM mode to eliminate disk operations entirely. This mode:

Stores all metrics in memory only

Eliminates disk IO for metric storage

Significantly reduces overall resource usage

tip

Since Child nodes stream metrics to Parents, they don't need persistent local storage. RAM mode is ideal for this use case.

Set up RAM mode following our database configuration guide.

Turn off ML on Children​

Optimize resource allocation by running Machine Learning only where it matters most. We recommend:

Enable ML on Parents: They have the complete data picture and typically more resources

Disable ML on Children: They focus on collecting and streaming metrics

To disable ML, edit your configuration using edit-config:

[ml]

enabled = no

tip

This configuration particularly benefits Child nodes, allowing them to focus on their primary role of collecting and streaming metrics to Parent nodes where ML analysis happens centrally.

Next steps​

Identify your needs: Determine whether you need optimization for resource constraints or architectural efficiency

Start with architecture: If monitoring multiple systems, implement Parent-Child setup first

Fine-tune individual Agents: Apply specific optimizations based on each system's role and resources

Monitor the impact: Use Netdata dashboards to confirm your optimizations haven't compromised monitoring visibility

Do you have any feedback for this page? If so, you can open a new issue on our netdata/learn repository.

Why optimize your Agent
How to optimize performance
Set up Parent-Child architecture
Disable unneeded collectors
Reduce collection frequency
Adjust metric retention
Switch to RAM mode
Turn off ML on Children
Next steps