---
title: 'GitHub - racksync/hass-addons-prometheus-node-exporter: Expose Hardware Metrics
  with Prometheus Node-Exporter · GitHub'
id: github-racksynchass-addons-prometheus-node-exporter-expose-hardware-metrics-with
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- node-exporter
- home-assistant
- prometheus
created: '2026-09-02T04:02:40.511432Z'
updated: '2026-09-02T17:37:22.007212Z'
source: https://github.com/racksync/hass-addons-prometheus-node-exporter
source_domain: github.com
fetched_at: '2026-09-02T04:02:37.591019Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'GitHub README (racksync, current version 2025.11.1, fork of loganmarchione/hassos-addons):
  HAOS add-on packaging prometheus node_exporter to expose host hardware/OS metrics
  (CPU, memory, disk, network, hwmon temperatures) at :9100/metrics. Toggles per-collector
  (cpu, meminfo, diskstats, netdev, netstat, filesystem, loadavg, time, wifi, hwmon),
  ignore_mount_points, ignore_network_devices, cmdline_extra_args. Security posture:
  AppArmor, optional basic auth (bcrypt) and TLS certs; multi-arch amd64/aarch64/armv7;
  source-to-monorepo sync via GitHub Actions. Niche validation: 3 stars, 1 fork. Demonstrates
  the incumbent approach ha-linux-agent replaces - a containerised exporter needing
  an external Prometheus scraper, HAOS add-on distribution chain, and host-network
  privilege - versus a native Rust agent pushing via MQTT with zero external stack.'
---

GitHub - racksync/hass-addons-prometheus-node-exporter: Expose Hardware Metrics with Prometheus Node-Exporter · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

Uh oh!

There was an error while loading. Please reload this page.

racksync

/

hass-addons-prometheus-node-exporter

Public

forked from loganmarchione/hassos-addons

Notifications
You must be signed in to change notification settings

Fork
1

Star
3

main

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History198 Commits

198 Commits
Folders and filesNameName
Last commit message
Last commit date

.github/workflows

.github/workflows

node-exporter

node-exporter

.gitignore

.gitignore

.mdlrc

.mdlrc

README.md

README.md

repository.yaml

repository.yaml

View all files

Repository files navigation

Prometheus Node Exporter for Home Assistant

Source Repository - This repository contains the source code for the Prometheus Node Exporter add-on that automatically syncs to the Home Assistant Add-ons Suite.

Overview

This add-on exposes hardware and OS metrics for Prometheus monitoring. It collects comprehensive system statistics like CPU, memory, disk, and network usage from your Home Assistant host and makes them available through the Prometheus metrics format.

Features

Hardware Metrics: CPU, memory, disk usage, and temperature monitoring

Network Statistics: Real-time network interface monitoring

Security-First: AppArmor protection, minimal permissions, principle of least privilege

Configurable: Enable/disable specific collectors based on your needs

Prometheus Compatible: Standard metrics endpoint for integration with Prometheus/Grafana

Multi-Architecture: Support for amd64, aarch64, and armv7 systems

Architecture

This repository follows a source-to-monorepo architecture:

Source: Here (node-exporter/ directory) - Development and updates

Target: Home Assistant Add-ons Suite - Distribution to users

All changes made to the node-exporter/ directory are automatically validated and synced to the monorepo via GitHub Actions.

Installation

This add-on is available through the Home Assistant Add-ons Suite repository:

Add Repository to Home Assistant:

https://github.com/racksync/hass-addons-suite

Go to Settings → Add-ons → Add-on Store → ⋮ → Add Repository

Install Prometheus Node Exporter:

Find "Prometheus Node Exporter" in the store

Click INSTALL

Configure as needed (see Configuration section)

START the add-on

Configuration

Basic Setup

# Default configuration - works out of the box
log_level: "info"  # trace|debug|info|warn|error
enable_basic_auth: false
enable_tls: false

Advanced Configuration

# Enable/disable specific collectors
collectors:
cpu: true          # CPU usage and utilization
meminfo: true      # Memory statistics
diskstats: true    # Disk I/O statistics
netdev: true       # Network interface stats
netstat: true      # Network connection stats
filesystem: true   # Filesystem usage
loadavg: true      # System load average
time: true         # Current time metrics
wifi: false        # WiFi statistics (if applicable)
hwmon: true        # Hardware monitoring (temperature/fans)

# Ignore specific mount points or network devices
ignore_mount_points:
- "/tmp"
- "/run"

ignore_network_devices:
- "docker0"
- "veth*"

# Custom command line arguments for node_exporter
cmdline_extra_args: "--collector.disable-defaults --collector.cpu"

Security Options

# Enable HTTP Basic Authentication
enable_basic_auth: true
basic_auth_user: "your_username"
basic_auth_pass: "your_bcrypt_hash"

# Enable TLS/HTTPS
enable_tls: true
cert_file: "/ssl/fullchain.pem"
cert_key: "/ssl/privkey.pem"

Metrics Endpoint

Once running, the add-on exposes metrics at:

HTTP: http://your-home-assistant:9100/metrics

HTTPS (if TLS enabled): https://your-home-assistant:9100/metrics

With Auth: Include Basic Auth headers if enabled

Example Prometheus Configuration

scrape_configs:
- job_name: 'homeassistant-node-exporter'
static_configs:
- targets: ['your-home-assistant:9100']
metrics_path: '/metrics'
# Add authentication if enabled
basic_auth:
username: 'your_username'
password: 'your_password'

Development

Source Code Structure

node-exporter/
├── config.yaml          # Add-on configuration and schema
├── build.yaml           # Build configuration
├── Dockerfile           # Container image definition
├── CHANGELOG.md         # Version history and release notes
├── README.md           # This file
├── icon.png           # Add-on icon
├── logo.png           # Add-on logo
├── rootfs/            # Container filesystem
│   ├── etc/
│   │   ├── cont-init.d/
│   │   └── services.d/
│   └── run.sh
└── translations/
└── en.yaml        # English translations

Making Changes

Edit files in the node-exporter/ directory

Test configuration changes locally

Commit and push to this repository

GitHub Actions will automatically validate and sync to the monorepo

Automated Sync Process

Validation: Configuration files are validated before sync

Version Management: Automatic tagging with version information

Monorepo Update: Files are synced to racksync/hass-addons-suite

Release Creation: Automatic release tag creation

Security Considerations

AppArmor: Enabled for container isolation

Minimal Permissions: Only requests necessary system access

Principle of Least Privilege: Reduces attack surface

Authentication: Optional Basic Auth and TLS support

Network Access: Host network access required for system metrics

Support & Contributing

Issues: GitHub Issues in the monorepo

Discussions: Community support and feature requests

Contributions: Pull requests welcome in this source repository

Version

Current Version: 2025.11.1
Release: View in Add-ons Suite

License

This add-on follows the same licensing as the Home Assistant Add-ons Suite.

Maintained by: RACKSYNC CO., LTD. - ALL ABOUT AUTOMATION
Location: Bangkok, Thailand
Email: devops@racksync.com
Website: www.racksync.com
X (Twitter): @racksync
Facebook: racksync

About
Expose Hardware Metrics with Prometheus Node-Exporter
racksync.com
Topics
addonhasshomeassistantnode-exporterprometheus
Resources
Readme
Activity
Custom properties
Stars
3 stars
Watchers
0 watching
Forks
1 fork
Report repository

Releases

Packages

Contributors

Languages

You can’t perform that action at this time.