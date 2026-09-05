---
title: Systemd Units | Operating Systems | Learn Netdata
id: systemd-units-operating-systems-learn-netdata
tags:
- linux-agent-jupiteros-fleet-15537b
- netdata
- official-docs
- systemd
created: '2026-09-02T04:31:50.422787Z'
updated: '2026-09-05T10:51:21.819966Z'
source: https://learn.netdata.cloud/docs/collecting-metrics/collectors/operating-systems/systemd-units
source_domain: learn.netdata.cloud
fetched_at: '2026-09-02T04:31:50.418473Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'Official Netdata systemdunits collector docs (go.d.plugin, module systemdunits):
  monitors state of systemd units AND unit files; NO auto-detection support; default
  include selector ''*.service''; configurable skip_transient; optional unit-file
  state collection (collect_unit_files, cached at collect_unit_files_every=300s because
  querying unit-file enablement state adds system overhead). System bus request timeout
  default 1s; update_every default 1s. Ships 12 built-in alerts - one per unit type
  - named systemd_<type>_unit_failed_state on metrics systemd.<type>_unit_state. Config
  via go.d/systemdunits.conf YAML jobs with shell-glob include patterns (e.g. ''*.service'',
  ''*''), multi-instance jobs with unique names. UI configuration again paywalled
  (Netdata Cloud paid plan). This is the concrete incumbent spec ha-linux-agent''s
  zbus-based systemd module can be measured against: per-unit state metric + failed-state
  alert per unit type, with unit-file enablement as a separate, deliberately cheaper-cadence
  collection.'
---

*Suggested by [[systemd-units-monitoring-netdata]] — primary collector doc behind the marketing 101 page*

Systemd Units | Operating Systems | Learn Netdata

Skip to main content

On this page

Plugin: go.d.plugin
Module: systemdunits

Overview​

This collector monitors the state of Systemd units and unit files.

This collector is supported on all platforms.

This collector supports collecting metrics from multiple instances of this integration, including remote instances.

Default Behavior​

Auto-Detection​

This integration doesn't support auto-detection.

Limits​

The default configuration for this integration does not impose any limits on data collection.

Performance Impact​

The default configuration for this integration is not expected to impose a significant performance impact on the system.

Setup​

You can configure the systemdunits collector in two ways:
MethodBest forHow toUIFast setup without editing filesGo to Nodes → Configure this node → Collectors → Jobs, search for systemdunits, then click + to add a job.FileIf you prefer configuring via file, or need to automate deployments (e.g., with Ansible)Edit go.d/systemdunits.conf and add a job.

important

UI configuration requires paid Netdata Cloud plan.

Prerequisites​

No action required.

Configuration​

Options​

The following options can be defined globally: update_every, autodetection_retry.
Config options

GroupOptionDescriptionDefaultRequiredCollectionupdate_everyData collection frequency.1noautodetection_retryRecheck interval in seconds. Zero means no recheck will be scheduled.0notimeoutSystem bus requests timeout.1noUnitsincludeSystemd units selector.*.servicenoskip_transientIf set, skip data collection for systemd transient units.falsenoUnit Filescollect_unit_filesIf set to true, collect the state of installed unit files. Enabling this may increase system overhead.falsenocollect_unit_files_everyInterval for querying systemd about unit files and their enablement state, measured in seconds. Data is cached for this interval to reduce system overhead.300noinclude_unit_filesSystemd unit files selector.*.serviceno
include​
Systemd units matching the selector will be monitored.

Logic: (pattern1 OR pattern2)

Pattern syntax: shell file name pattern

Syntax:

includes:

- pattern1

- pattern2

include_unit_files​
Systemd unit files matching the selector will be monitored.

Logic: (pattern1 OR pattern2)

Pattern syntax: shell file name pattern

Syntax:

includes:

- pattern1

- pattern2

via UI​

Configure the systemdunits collector from the Netdata web interface:

Go to Nodes.

Select the node where you want the systemdunits data-collection job to run and click the ⚙ (Configure this node). That node will run the data collection.

The Collectors → Jobs view opens by default.

In the Search box, type systemdunits (or scroll the list) to locate the systemdunits collector.

Click the + next to the systemdunits collector to add a new job.

Fill in the job fields, then click Test to verify the configuration and Submit to save.

Test runs the job with the provided settings and shows whether data can be collected.

If it fails, an error message appears with details (for example, connection refused, timeout, or command execution errors), so you can adjust and retest.

via File​

The configuration file name for this integration is go.d/systemdunits.conf.

The file format is YAML. Generally, the structure is:

update_every: 1

autodetection_retry: 0

jobs:

- name: some_name1

- name: some_name2

You can edit the configuration file using the edit-config script from the
Netdata config directory.

cd /etc/netdata 2>/dev/null || cd /opt/netdata/etc/netdata

sudo ./edit-config go.d/systemdunits.conf

Examples​

Service units​

Collect state of all service type units.
Config

jobs:

- name: service

include:

- '*.service'

One specific unit​

Collect state of one specific unit.
Config

jobs:

- name: my-specific-service

include:

- 'my-specific.service'

All unit types​

Collect state of all units.
Config

jobs:

- name: my-specific-service-unit

include:

- '*'

Multi-instance​

Note: When you define multiple jobs, their names must be unique.

Collect state of all service and socket type units.
Config

jobs:

- name: service

include:

- '*.service'

- name: socket

include:

- '*.socket'

Alerts​

The following alerts are available:
Alert nameOn metricDescriptionAlert name systemd_service_unit_failed_state On metricsystemd.service_unit_stateDescriptionsystemd service unit in the failed stateAlert name systemd_socket_unit_failed_state On metricsystemd.socket_unit_stateDescriptionsystemd socket unit in the failed stateAlert name systemd_target_unit_failed_state On metricsystemd.target_unit_stateDescriptionsystemd target unit in the failed stateAlert name systemd_path_unit_failed_state On metricsystemd.path_unit_stateDescriptionsystemd path unit in the failed stateAlert name systemd_device_unit_failed_state On metricsystemd.device_unit_stateDescriptionsystemd device unit in the failed stateAlert name systemd_mount_unit_failed_state On metricsystemd.mount_unit_stateDescriptionsystemd mount unit in the failed stateAlert name systemd_automount_unit_failed_state On metricsystemd.automount_unit_stateDescriptionsystemd automount unit in the failed stateAlert name systemd_swap_unit_failed_state On metricsystemd.swap_unit_stateDescriptionsystemd swap unit in the failed stateAlert name systemd_scope_unit_failed_state On metricsystemd.scope_unit_stateDescriptionsystemd scope unit in the failed stateAlert name systemd_slice_unit_failed_state On metricsystemd.slice_unit_stateDescriptionsystemd slice unit in the failed stateAlert name systemd_timer_unit_failed_state On metricsystemd.timer_unit_stateDescriptionsystemd timer unit in the failed state

Metrics​

Metrics grouped by scope.

The scope defines the instance that the metric belongs to. An instance is uniquely identified by a set of labels.

Per unit​

These metrics refer to the systemd unit.

Labels:
LabelDescriptionunit_namesystemd unit name

Metrics:
MetricDescriptionDimensionsUnitsystemd.service_unit_stateService Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.socket_unit_stateSocket Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.target_unit_stateTarget Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.path_unit_statePath Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.device_unit_stateDevice Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.mount_unit_stateMount Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.automount_unit_stateAutomount Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.swap_unit_stateSwap Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.timer_unit_stateTimer Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.scope_unit_stateScope Unit Stateactive, inactive, activating, deactivating, failedstatesystemd.slice_unit_stateSlice Unit Stateactive, inactive, activating, deactivating, failedstate

Per unit file​

These metrics refer to the systemd unit file.

Labels:
LabelDescriptionunit_file_namesystemd unit file nameunit_file_typesystemd unit file type

Metrics:
MetricDescriptionDimensionsUnitsystemd.unit_file_stateUnit File Stateenabled, enabled-runtime, linked, linked-runtime, alias, masked, masked-runtime, static, disabled, indirect, generated, transient, badstate

Troubleshooting​

Debug Mode​

Important: Debug mode is not supported for data collection jobs created via the UI using the Dyncfg feature.

To troubleshoot issues with the systemdunits collector, run the go.d.plugin with the debug option enabled. The output
should give you clues as to why the collector isn't working.

Navigate to the plugins.d directory, usually at /usr/libexec/netdata/plugins.d/. If that's not the case on
your system, open netdata.conf and look for the plugins setting under [directories].

cd /usr/libexec/netdata/plugins.d/

Switch to the netdata user.

sudo -u netdata -s

Run the go.d.plugin to debug the collector:

./go.d.plugin -d -m systemdunits

To debug a specific job:

./go.d.plugin -d -m systemdunits -j jobName

Getting Logs​

If you're encountering problems with the systemdunits collector, follow these steps to retrieve logs and identify potential issues:

Run the command specific to your system (systemd, non-systemd, or Docker container).

Examine the output for any warnings or error messages that might indicate issues.  These messages should provide clues about the root cause of the problem.

System with systemd​

Use the following command to view logs generated since the last Netdata service restart:

journalctl _SYSTEMD_INVOCATION_ID="$(systemctl show --value --property=InvocationID netdata)" --namespace=netdata --grep systemdunits

System without systemd​

Locate the collector log file, typically at /var/log/netdata/collector.log, and use grep to filter for collector's name:

grep systemdunits /var/log/netdata/collector.log

Note: This method shows logs from all restarts. Focus on the latest entries for troubleshooting current issues.

Docker Container​

If your Netdata runs in a Docker container named "netdata" (replace if different), use this command:

docker logs netdata 2>&1 | grep systemdunits

Do you have any feedback for this page? If so, you can open a new issue on our netdata/learn repository.

Overview
Default Behavior
Auto-Detection
Limits
Performance Impact
Setup
Prerequisites
Configuration
Options
include
include_unit_files
via UI
via File
Examples
Service units
One specific unit
All unit types
Multi-instance
Alerts
Metrics
Per unit
Per unit file
Troubleshooting
Debug Mode
Getting Logs
System with systemd
System without systemd
Docker Container