---
title: How to Monitor Storage Health with SMART and smartctl on RHEL
id: how-to-monitor-storage-health-with-smart-and-smartctl-on-rhel
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- ha-linux-agent
- smart
- storage-health
- practitioner-guide
created: '2026-09-02T04:02:40.532378Z'
updated: '2026-09-05T10:51:21.750093Z'
source: https://oneuptime.com/blog/post/2026-03-04-monitor-storage-health-smart-smartctl-rhel-9/view
source_domain: oneuptime.com
fetched_at: '2026-09-02T04:02:40.480647Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'OneUptime engineering blog (Mar 2026, open-source/technically-reviewed per
  their footer): practical smartmontools reference for fleet storage health. Commands:
  smartctl -i (SMART capability/enabled), -H (overall health PASSED/FAILED), -A (attributes),
  -a (full), -t short/long (self-tests), -l selftest; NVMe via /dev/nvme0n1 with key
  health fields percentage used, available spare, media and data integrity errors,
  critical warning. smartd config pattern: ''DEVICESCAN -a -o on -S on -n standby,q
  -s (S/../.././02|L/../../6/03) -W 4,45,55 -m root@'' - daily short/Saturday long
  self-tests, temperature trip at 4-degree change/45/55, email alerts; then ''systemctl
  enable --now smartd''. Includes a bash health-report script iterating /dev/sd? and
  /dev/nvme?n1. HDD warning attributes: Reallocated_Sector_Ct, Current_Pending_Sector,
  Offline_Uncorrectable, UDMA_CRC_Error_Count, Spin_Retry_Count (any nonzero/growth);
  SSD: Wear_Leveling_Count, Media_Wearout_Indicator. Directly transferable to ha-linux-agent''s
  SMART collector design: periodic smartctl -H/-A polling cadence and which attributes
  gate alerts.'
---

How to Monitor Storage Health with SMART and smartctl on RHEL

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

How to Monitor Storage Health with SMART and smartctl on RHEL

Learn how to use SMART monitoring and smartctl on RHEL to detect failing drives early and prevent data loss from hardware failures.

By @nawazdhandala

•
Mar 04, 2026
•

Reading time

RHEL

SMART

Smartctl

Storage

Monitoring

Disk Health

Linux

On this page

Hard drives and SSDs can fail suddenly, but many failures show warning signs first. Most storage devices support SMART (Self-Monitoring, Analysis, and Reporting Technology), which tracks internal health metrics that can help identify failing devices before complete failure. On RHEL, the smartctl tool lets you query these metrics and set up automated monitoring.
Installing smartmontoolssudo dnf install smartmontools
Checking SMART Support
Verify that a device supports SMART:sudo smartctl -i /dev/sda
Look for:SMART support is: Available - device has SMART capability.
SMART support is: Enabled
If SMART is available but not enabled:sudo smartctl -s on /dev/sda
Viewing SMART Health Summary
Quick health check:sudo smartctl -H /dev/sda
Output:SMART overall-health self-assessment test result: PASSED
If this shows FAILED, back up your data immediately.
Viewing All SMART Attributessudo smartctl -A /dev/sda
For HDDs, key attributes to watch:

AttributeWhat It MeansWarning SignReallocated_Sector_CtBad sectors remapped to spare areaAny value above 0Current_Pending_SectorSectors waiting to be remappedGrowing countOffline_UncorrectableSectors that could not be correctedAny value above 0UDMA_CRC_Error_CountCable or connection errorsGrowing countSpin_Retry_CountDisk spindle start failuresAny value above 0

For SSDs, watch:

AttributeWhat It MeansWarning SignWear_Leveling_CountVendor-specific SSD wear levelLow normalized value or worsening trendReallocated_Sector_CtBad NAND cells remappedGrowing countMedia_Wearout_IndicatorVendor-specific remaining SSD lifeApproaching 0, if supported by the drive

Viewing Complete SMART Informationsudo smartctl -a /dev/sda
This shows device info, health status, attributes, and error logs all at once.
Running SMART Self-Tests
Short Test (2-5 minutes)sudo smartctl -t short /dev/sda
Long Test (hours, depending on disk size)sudo smartctl -t long /dev/sda
Check Test Resultssudo smartctl -l selftest /dev/sda
NVMe Drive Monitoring
For NVMe drives, use the NVMe namespace device path:sudo smartctl -a /dev/nvme0n1
Key NVMe health metrics:
Percentage Used - How much of the SSD lifetime has been consumed
Available Spare - Remaining spare NAND blocks
Media and Data Integrity Errors - Should be 0
Critical Warning - Active warnings
Configuring Automated Monitoring with smartd
The smartd daemon monitors drives continuously and sends alerts. Edit the configuration:sudo vi /etc/smartmontools/smartd.conf
Basic configuration to monitor all drives and send email alerts:DEVICESCAN -a -o on -S on -n standby,q -s (S/../.././02|L/../../6/03) -W 4,45,55 -m [email protected]
This configuration:
-a - Monitors all SMART attributes
-o on - Enables automatic offline testing
-S on - Enables attribute autosave
-n standby,q - Does not spin up drives in standby
-s (S/../.././02|L/../../6/03) - Short test daily at 2 AM, long test Saturdays at 3 AM
-W 4,45,55 - Temperature monitoring (4-degree change, 45 warning, 55 critical)
-m [email protected] - Email alerts
Enable and start the service:sudo systemctl enable --now smartd
Monitoring Specific Drives
For more control, specify individual drives:/dev/sda -a -o on -S on -s (S/../.././02|L/../../6/03) -W 4,45,55 -m [email protected]
/dev/sdb -a -o on -S on -s (S/../.././02|L/../../6/03) -W 4,45,55 -m [email protected]
/dev/nvme0n1 -a -W 4,60,70 -m [email protected]
Checking smartd Logsjournalctl -u smartd -f
Creating a SMART Health Report Script#!/bin/bash
echo "=== SMART Health Report ==="
echo "Date: $(date)"
echo ""

for disk in /dev/sd? /dev/nvme?n1; do
[ -b "$disk" ] || continue
echo "--- $disk ---"
HEALTH=$(smartctl -H "$disk" 2>/dev/null | grep "result")
echo "Health: $HEALTH"
TEMP=$(smartctl -A "$disk" 2>/dev/null | grep -i temperature | head -1)
echo "Temperature: $TEMP"
ERRORS=$(smartctl -l error "$disk" 2>/dev/null | grep "No Errors Logged")
if [ -z "$ERRORS" ]; then
echo "WARNING: Errors found in log"
else
echo "Errors: None"
fi
echo ""
done
Summary
SMART monitoring with smartctl on RHEL is your first line of defense against unexpected drive failures. Check health regularly with smartctl -H, monitor key attributes with smartctl -A, run self-tests periodically, and configure smartd for automated monitoring with email alerts. Early detection of failing drives gives you time to replace hardware and migrate data before a complete failure occurs.

Share this article

Nawaz Dhandala
Author

@nawazdhandala • Mar 04, 2026 •

Nawaz is building OneUptime with a passion for engineering reliable systems and improving observability.

GitHub

Technically validated

· May 15, 2026

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

Technically reviewed for accuracy • May 15, 2026

Loading validation report…

Automated technical review

Close