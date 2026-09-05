---
title: How to Monitor ZFS Pool Health and Status on Ubuntu
id: how-to-monitor-zfs-pool-health-and-status-on-ubuntu
tags:
- linux-agent-jupiteros-fleet-15537b
- known-issue
- smart
- storage-health
- node-exporter
- repo-source
- practitioner-guide
- linux-agent
created: '2026-09-02T05:38:56.234502Z'
updated: '2026-09-05T10:51:21.899347Z'
source: https://oneuptime.com/blog/post/2026-03-02-how-to-monitor-zfs-pool-health-and-status-on-ubuntu/view
source_domain: oneuptime.com
fetched_at: '2026-09-02T05:38:47.718951Z'
fetch_provider: builtin
status: evergreen
type: note
tier: practitioner
content_type: blog
deprecated: false
summary: 'OneUptime blog guide (Mar 2026, open-source post, technically validated
  May 2026) on ZFS pool health monitoring for Linux/Ubuntu. Key commands: zpool status
  (pool state), zpool list -H -o name/health/cap/frag (scriptable single-line per
  pool), zpool status -v (per-device READ/WRITE/CKSUM error columns), zpool iostat
  -v/-ql (I/O and latency percentiles), zfs list -r/-t (per-dataset and snapshot space).
  Includes a complete bash health-check script with thresholds: pool state != ONLINE
  is CRITICAL, capacity > 85% WARNING, fragmentation > 50% WARNING, any non-zero CKSUM
  error per device WARNING, ''errors:'' line != ''No known data errors'' WARNING —
  exits 0/1 and writes /var/run/zfs-alert for external monitoring, cron every 15 min.
  Recommends zed (ZFS Event Daemon, zfs-zed package, zed.rc email config) for event-driven
  alerts on pool state changes/scrub/resilver completion, and prometheus-node-exporter
  ZFS collector (node_zfs_* metrics incl. node_zfs_zpool_nread/nwritten per pool,
  /proc/spl/kstat/zfs/arcstats for ARC hit ratio, <80% hit ratio flags RAM constraint).
  Alert thresholds summary: capacity alert 80%/critical 90%, fragmentation monitor
  30%/address 50%, ARC hit ratio below 80%.'
---

How to Monitor ZFS Pool Health and Status on Ubuntu

Skip to main content

Sign
in
Sign up

Close menu

Enterprise

DevOps

SRE

Platform

Pricing

Docs

Request Demo

Self-Hosted

Trust Center

Support

Sign
up

Existing customer?
Sign in

Products

Explore the OneUptime platform

One platform for monitoring, observability & incident response.

⌘K

AI

Investigates incidents with AI and turns findings into fix pull requests for your review.

Essentials

Monitoring

Uptime & synthetic checks

Status Page

Communicate incidents to users

Incidents

Detect, manage & resolve

On-Call & Alerts

Smart routing & escalations

Scheduled Maintenance

Plan & communicate downtime

Observability

Observability

Logs, metrics & traces in one

Topology

Service, infra & network maps

Security Events

SIEM signals & Sigma detections

Logs

Fastest log ingest & search

Metrics

Application & infra metrics

Traces

Distributed request tracing

Exceptions

Error tracking & debugging

Profiles

CPU & memory profiling

RUM

Real user monitoring

Infrastructure

Services

Catalog every service you run

Kubernetes

Cluster & pod observability

Docker

Host & container observability

Podman

Host & container observability

Hosts

Auto-discovered server metrics

Proxmox

VE clusters, VMs & backups

AI / LLM Observability

Tokens, cost, traces & prompts

Ceph

Storage cluster health

Docker Swarm

Nodes, services, tasks & stacks

IoT Devices

Fleets, sensors & gateways

Network Devices

Switches, routers & firewalls

Serverless

Functions & cold starts

Cloud

AWS, GCP & Azure

Automation & Analytics

Workflows

No-code automation builder

Runbooks

Auto-trigger response steps

Dashboards

Custom data visualizations

No products found

100% Open Source

Self-host or use our cloud

↑
↓

↵
esc

How to Monitor ZFS Pool Health and Status on Ubuntu

Monitor ZFS pool health, capacity, I/O performance, and error states on Ubuntu using zpool commands, automated alerts, and integration with monitoring systems.

By @nawazdhandala

•
Mar 02, 2026
•

Reading time

Ubuntu

ZFS

Monitoring

Storage

Linux

On this page

ZFS is generally self-managing, but it needs monitoring to catch issues before they become problems. A pool approaching full capacity, a disk accumulating checksum errors, or a resilver that's been running for days all need human attention. This guide covers the monitoring commands and strategies for keeping ZFS pools healthy on Ubuntu systems.
Quick Health Check
The fastest way to assess all pools at once:sudo zpool status
For a healthy system, every pool shows ONLINE:  pool: tank
state: ONLINE
scan: scrub repaired 0 in 02:14:22 with 0 errors on Sun Mar  1 03:00:00 2026
config:

NAME        STATE     READ WRITE CKSUM
tank        ONLINE       0     0     0
mirror-0  ONLINE       0     0     0
sdb     ONLINE       0     0     0
sdc     ONLINE       0     0     0

errors: No known data errors
Interpreting pool states

StateMeaningONLINEFully healthy and operationalDEGRADEDOperational but redundancy reduced (disk failed/removed)FAULTEDPool is not accessible - too many failuresOFFLINEPool was manually taken offlineUNAVAILDevice not accessibleREMOVEDDevice was physically removed

Any state other than ONLINE requires immediate attention.
Capacity Monitoring# Quick capacity view

sudo zpool listNAME    SIZE  ALLOC   FREE  FRAG    CAP  DEDUP  HEALTH
tank    3.62T  1.82T  1.80T  12%    50%  1.00x  ONLINE
backup  7.27T  5.14T  2.13T  28%    70%  1.00x  ONLINE
Key columns:
ALLOC: Space currently used
FREE: Available space
FRAG: Fragmentation percentage (below 30% is healthy; above 50% can affect performance)
CAP: Capacity percentage used
Per-dataset space usagesudo zfs list -r tankNAME                   USED  AVAIL     REFER  MOUNTPOINT
tank                  1.82T  1.80T       96K  /tank
tank/web               450G  1.80T      450G  /var/www
tank/databases         890G  1.80T      890G  /var/lib
tank/backups           480G  1.80T      480G  /backup
Check snapshot space usagesudo zfs list -t all -o name,used,refer -r tank
Snapshots consuming large amounts of space show up in the USED column of their parent dataset.
Error Monitoring# Verbose status showing per-device error counts
sudo zpool status -v tank
Focus on the READ, WRITE, and CKSUM columns:        NAME        STATE     READ WRITE CKSUM
tank        ONLINE       0     0     0
mirror-0  ONLINE       0     0     0
sdb     ONLINE       0     0     0
sdc     ONLINE       0     5    12
A disk showing CKSUM errors (checksum errors) while data was being repaired from the mirror is concerning. If errors keep accumulating, the disk is likely failing.
Error interpretation
READ: Errors while reading from the device
WRITE: Errors while writing to the device
CKSUM: Checksum mismatches - data read didn't match its stored checksum
Even a few CKSUM errors without corresponding disk errors can be transient. Repeated or growing CKSUM errors indicate a failing disk, bad SATA/SAS cable, or controller issue.
Clear error counters# Clear after replacing hardware or investigating
sudo zpool clear tank

# Clear counters for a specific device
sudo zpool clear tank /dev/sdc
I/O Performance Monitoring# Real-time I/O statistics for all pools
sudo zpool iostat 2              capacity     operations     bandwidth
pool        alloc   free   read  write   read  write
----------  -----  -----  -----  -----  -----  -----
tank        1.82T  1.80T     42    128  3.21M  9.84M
backup      5.14T  2.13T      5     32   412K  2.12M# Per-vdev breakdown
sudo zpool iostat -v 2              capacity     operations     bandwidth
pool        alloc   free   read  write   read  write
----------  -----  -----  -----  -----  -----  -----
tank        1.82T  1.80T     42    128  3.21M  9.84M
mirror    1.82T  1.80T     21     64  1.61M  4.92M
sdb         -      -     21     64  1.61M  4.92M
sdc         -      -     21      0  1.61M      0
Latency statistics# Show latency percentiles
sudo zpool iostat -ql 5
This shows operation latency which helps identify slow disks.
Automated Health Monitoring Script
A comprehensive monitoring script that checks all common issues:sudo nano /usr/local/bin/zfs-health-check.sh#!/bin/bash
# Comprehensive ZFS health check
# Returns 0 if healthy, 1 if issues found

ALERT_FILE="/var/run/zfs-alert"
ISSUES=()

# Check each pool
while IFS= read -r pool; do
STATE=$(zpool list -H -o health "$pool")
CAP=$(zpool list -H -o cap "$pool" | tr -d '%')
FRAG=$(zpool list -H -o frag "$pool" | tr -d '%')

# Check pool health
if [ "$STATE" != "ONLINE" ]; then
ISSUES+=("CRITICAL: Pool $pool is in state $STATE")
fi

# Check capacity
if [ "$CAP" -gt 85 ]; then
ISSUES+=("WARNING: Pool $pool is ${CAP}% full (above 85% threshold)")
fi

# Check fragmentation
if [ "$FRAG" -gt 50 ]; then
ISSUES+=("WARNING: Pool $pool fragmentation is ${FRAG}% (above 50%)")
fi

# Check for data errors
ERROR_SUMMARY=$(zpool status "$pool" | grep "errors:" | grep -v "No known data errors")
if [ -n "$ERROR_SUMMARY" ]; then
ISSUES+=("WARNING: Pool $pool has errors: $ERROR_SUMMARY")
fi

# Check for devices with high error counts
while IFS= read -r line; do
CKSUM=$(echo "$line" | awk '{print $5}')
DEV=$(echo "$line" | awk '{print $1}')
if [ -n "$CKSUM" ] && [ "$CKSUM" -gt 0 ] 2>/dev/null; then
ISSUES+=("WARNING: Device $DEV in pool $pool has $CKSUM checksum errors")
fi
done < <(zpool status "$pool" | awk 'NR>7 && /\S/ {print}')

done < <(zpool list -H -o name)

# Report results
if [ ${#ISSUES[@]} -eq 0 ]; then
echo "OK: All ZFS pools healthy"
rm -f "$ALERT_FILE"
exit 0
else
echo "ZFS ALERTS:"
for issue in "${ISSUES[@]}"; do
echo "  - $issue"
done

# Write alert file for external monitoring
printf '%s\n' "${ISSUES[@]}" > "$ALERT_FILE"
exit 1
fisudo chmod +x /usr/local/bin/zfs-health-check.sh

# Test it
sudo /usr/local/bin/zfs-health-check.sh
Add to cron for regular checks:echo "*/15 * * * * root /usr/local/bin/zfs-health-check.sh | logger -t zfs-health" \
| sudo tee /etc/cron.d/zfs-health
Monitoring with zed (ZFS Event Daemon)
ZFS includes zed, the ZFS Event Daemon, which monitors ZFS events and can send email alerts:sudo apt install zfs-zed
sudo systemctl enable --now zfs-zed.service
Configure alerts:sudo nano /etc/zfs/zed.d/zed.rc# Email alerts for ZFS events
ZED_EMAIL_ADDR="[email protected]"
ZED_EMAIL_PROG="mail"
ZED_EMAIL_OPTS="-s '@subject@' @address@"

# Alert on these event classes
ZED_NOTIFY_INTERVAL_SECS=3600
ZED_NOTIFY_VERBOSE=0sudo systemctl restart zfs-zed.service
zed sends emails on pool state changes, scrub completion, resilver completion, and device errors.
Prometheus/Grafana Integration
For infrastructure monitoring, prometheus-node-exporter includes a ZFS collector that exposes ARC and per-pool I/O metrics:# Install the node exporter (includes a ZFS collector)
sudo apt install prometheus-node-exporter

# Start and enable
sudo systemctl enable --now prometheus-node-exporter

# Verify ZFS metrics are available
curl http://localhost:9100/metrics | grep node_zfs
Sample metrics:node_zfs_arc_size 8589934592
node_zfs_arc_hits 82371029
node_zfs_arc_misses 15892134
node_zfs_zpool_nread{zpool="tank"} 3.21e+09
node_zfs_zpool_nwritten{zpool="tank"} 9.84e+09
Add scrape config to Prometheus:# /etc/prometheus/prometheus.yml
scrape_configs:
- job_name: 'node'
static_configs:
- targets: ['localhost:9100']
ARC (Cache) Monitoring
The Adaptive Replacement Cache uses RAM for reads. Monitor its effectiveness:# View ARC statistics
cat /proc/spl/kstat/zfs/arcstats | grep -E "^(hits|misses|size|c_max)"hits                            4       82371029
misses                          4       15892134
size                            4    8589934592   # 8GB ARC
c_max                           4    8589934592   # 8GB max
A high hit ratio (hits / (hits + misses)) indicates the ARC is working effectively. Below 80% may indicate the working set exceeds available RAM.# Calculate hit ratio
awk '/^hits/{h=$3} /^misses/{m=$3} END{printf "ARC hit rate: %.1f%%\n", h/(h+m)*100}' \
/proc/spl/kstat/zfs/arcstats
Key Metrics to Track
Pool health - should always be ONLINE
Capacity - alert above 80%, critical above 90%
Fragmentation - monitor above 30%, address above 50%
CKSUM errors - any non-zero value warrants investigation
Scrub results - any "errors" in the scrub output
Resilver duration - excessively long resilvers indicate slow/stressed disks
ARC hit ratio - below 80% may indicate RAM constraints
Regular monitoring, combined with automated scrubs and ZED email alerts, gives a solid foundation for ZFS pool health management on Ubuntu.

Share this article

Nawaz Dhandala
Author

@nawazdhandala • Mar 02, 2026 •

Nawaz is building OneUptime with a passion for engineering reliable systems and improving observability.

GitHub

Technically validated

· May 19, 2026

View report

Help improve this post

Every OneUptime blog post is open source. Found a typo, an inaccuracy, or have a clearer way to explain something? Anyone can contribute — your edits make this post better for everyone who reads it next.

Edit this post on GitHub

Contributing guidelines

Open source

OneUptime is the Open-Source
Observability Platform

Your complete reliability stack unified: infrastructure monitoring, incident management, status pages, and APM. Open-source and self-hostable.

Get started for free

Request a demo

Status Page

Real-time status updates

Incidents

Detect and resolve fast

Monitoring

Monitor any resource

On-Call

Smart alert routing

Maintenance

Plan & communicate downtime

Logs

Fastest log ingest and search

Metrics

Performance insights

Traces

End-to-end distributed tracing

Exceptions

Catch and fix bugs early

Workflows

Automate any process

Dashboards

Visualize all your data

Kubernetes

Monitor K8s clusters

Profiles

CPU & memory profiling

AI

Detect, diagnose, and resolve incidents with AI-powered root cause analysis and code fixes.

We use cookies to enhance your browsing experience and provide
personalized content. By clicking "Accept," you consent to the use of cookies.

Our product uses both first-party and third-party cookies for session storage and for various other purposes.

Please note that disabling certain cookies may affect the functionality and performance of our product.

For more information about how we handle your data and cookies, please read our Privacy Policy.

By continuing to use our site without changing your cookie settings, you agree to our use of cookies as
described above. See our terms and our privacy policy

Accept
all
Reject all

Validation report

Technically reviewed for accuracy • May 19, 2026

Loading validation report…

Automated technical review

Close