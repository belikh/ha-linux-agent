---
title: mosquitto | Databases | Learn Netdata
id: mosquitto-databases-learn-netdata
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- netdata
- official-docs
- mqtt
- mosquitto
created: '2026-09-02T04:02:40.516704Z'
updated: '2026-09-05T10:51:21.737341Z'
source: https://learn.netdata.cloud/docs/collecting-metrics/collectors/databases/mosquitto
source_domain: learn.netdata.cloud
fetched_at: '2026-09-02T04:02:38.335196Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Official Netdata docs for its Mosquitto MQTT broker collector: implemented
  NOT as a native MQTT client but via the generic go.d prometheus module scraping
  a separate mosquitto_exporter over HTTP (10s default interval). Auto-detects local
  exporter ports; supports remote instances; per-job options include url, timeout,
  expected_prefix guard, selector allow/deny filters, Prometheus-compatible metric_relabel_configs
  (with histogram-corruption rejection), curated exporter profiles (auto/exact/combined/none
  modes), max_time_series=2000 global and 200 per-metric caps, HTTP auth/mTLS, and
  Virtual Node (vnode) association for mapping remote-host jobs. UI-based collector
  config requires a PAID Netdata Cloud plan; file config via go.d/prometheus.conf.
  Relevant contrast: ha-linux-agent already talks MQTT natively (rumqttc) and could
  collect broker stats ( topics) or use this collector''s design as the reference
  for metric shaping/limits if jupiterOS wants broker telemetry.'
---

mosquitto | Databases | Learn Netdata

Skip to main content

On this page

Plugin: go.d.plugin
Module: prometheus

Overview​

Keep an eye on Mosquitto MQTT broker metrics for efficient IoT message transport and performance.

Metrics are gathered by periodically sending HTTP requests to mosquitto exporter.

This collector is supported on all platforms.

This collector supports collecting metrics from multiple instances of this integration, including remote instances.

Default Behavior​

Auto-Detection​

By default, it detects instances running on the local host by trying to connect to known ports that are allocated to exporters.

Limits​

The default configuration for this integration does not impose any limits on data collection.

Performance Impact​

The default configuration for this integration is not expected to impose a significant performance impact on the system.

Setup​

You can configure the prometheus collector in two ways:
MethodBest forHow toUIFast setup without editing filesGo to Nodes → Configure this node → Collectors → Jobs, search for prometheus, then click + to add a job.FileIf you prefer configuring via file, or need to automate deployments (e.g., with Ansible)Edit go.d/prometheus.conf and add a job.

important

UI configuration requires paid Netdata Cloud plan.

Prerequisites​

Install Exporter​

Install mosquitto exporter by following the instructions mentioned in the exporter README.

Configuration​

Options​

The following options can be defined globally: update_every, autodetection_retry.
Config options

GroupOptionDescriptionDefaultRequiredCollectionupdate_everyData collection interval (seconds).10noautodetection_retryAutodetection retry interval (seconds). Set 0 to disable.0noTargeturlTarget endpoint URL.yestimeoutHTTP request timeout (seconds).10noexpected_prefixIf set, the job's check passes only when at least one post-job, pre-profile metric name starts with this prefix. Guards against scraping an unexpected endpoint; profile-owned relabeling cannot satisfy it.noCustomizationappApplication name used as the app segment of chart contexts (prometheus.<app>.<metric>). When unset, it is taken from a matched profile, otherwise it falls back to the job name.noFiltersselectorTime series selector (filter).noLimitsmax_time_seriesGlobal time series limit applied after job and profile relabeling. If the final output exceeds it, the data is not processed.2000nomax_time_series_per_metricPer-metric time series limit applied to final metric families. Metrics exceeding it are skipped.200noCustomizationfallback_typeJob-level fallback type overrides for untyped metrics.norelabelingJob-owned Prometheus-compatible metric relabeling, applied before profile selection.noprofilesCurated, exporter-specific chart profiles with optional untyped classification, profile-owned normalization, and scoped fallback-chart policy. User profiles may constrain unmatched fallback charts; stock profiles preserve unknown future families. Disable profiles with mode none.autonoHTTP AuthusernameUsername for Basic HTTP authentication.nopasswordPassword for Basic HTTP authentication.nobearer_token_filePath to a file containing a bearer token (used for Authorization: Bearer).noTLStls_skip_verifySkip TLS certificate and hostname verification (insecure).nonotls_caPath to CA bundle used to validate the server certificate.notls_certPath to client TLS certificate (for mTLS).notls_keyPath to client TLS private key (for mTLS).noProxyproxy_urlHTTP proxy URL.noproxy_usernameUsername for proxy Basic HTTP authentication.noproxy_passwordPassword for proxy Basic HTTP authentication.noRequestmethodHTTP method to use.GETnobodyRequest body (e.g., for POST/PUT).noheadersAdditional HTTP headers (one per line as key: value).nonot_follow_redirectsDo not follow HTTP redirects.nonoforce_http2Force HTTP/2 (including h2c over TCP).nonoVirtual NodevnodeAssociates this data collection job with a Virtual Node.no
selector​
This option allows you to filter out unwanted time series. Only metrics matching the selector will be collected.

Logic: (pattern1 OR pattern2) AND !(pattern3 or pattern4)

Pattern syntax: selector.

Option syntax:

selector:

allow:

- pattern1

- pattern2

deny:

- pattern3

- pattern4

fallback_type​
This job option allows you to process untyped metrics as Counter or Gauge instead of ignoring them.
Classification uses the post-job, pre-profile metric name. Profile relabeling preserves the selected
type but cannot create or change it by renaming the final metric.
Selected profiles may provide exporter-owned fallback_type defaults inside their own match scope.
Job gauge rules take precedence over job counter rules, and both job rule sets take precedence over
every profile rule. Use them for deployment-specific overrides rather than exporter behavior that
belongs in a profile. Keep patterns narrow: a broad job rule such as gauge: ['*'] overrides profile
counter classifications. Blank patterns and patterns with leading or trailing whitespace are rejected.

Metric name pattern syntax: shell file name pattern.

Option syntax:

fallback_type:

counter:

- metric_name_pattern1

- metric_name_pattern2

gauge:

- metric_name_pattern3

- metric_name_pattern4

relabeling​
A list of job-owned relabeling blocks, applied after selector and before profile selection. Each block
applies a list of Prometheus metric_relabel_configs rules to the metrics whose name matches match.
Profiles may own the same block format for exporter normalization after selection. See the
relabeling reference for
the full action set and more examples.

match: Netdata simple patterns matched against the full metric name — including any
_bucket/_sum/_count suffix, so prefer globs like app_lat* over an exact app_lat
(space-separated; * matches any sequence, ? any character, a leading ! negates). Use * to
target every metric. Required.

metric_relabel_configs: Prometheus relabel rules (source_labels, separator, regex, modulus,
target_label, replacement, action), applied in order to the scraped samples before charts are
built.

Relabeling that would corrupt a histogram or summary — splitting it, dropping a component, mutating the
le/quantile label, or merging two families — is rejected.

relabeling:

- match: 'http_*'

metric_relabel_configs:

- source_labels: [code]

regex: '(\d)\d\d'

target_label: code_class

replacement: '${1}xx'

profiles​
Profiles ship curated charts for recognized exporters -- see the
profile format for the file format and how
to author your own. profiles.mode selects them:

auto (default): every profile whose match hits at least one scraped metric.

exact: only the profiles named in mode_exact.entries (each must match, or the job fails its
check).

combined: auto plus the profiles named in mode_combined.entries.

none: no profiles — generic autogen charts only (the pre-profile behavior).

Selection uses post-job, pre-profile family names. A selected profile may carry fallback_type rules
that classify untyped scalar families inside its match scope and relabeling blocks that normalize
matching source families automatically before chart routing. Job fallback policy takes precedence;
conflicting profile rules use the same ordering as normalization. Each original family is
processed only by the first applicable profile normalizer: profile-name order in auto, configured
entry order in exact, and configured entries followed by remaining auto profiles in name order in
combined. Later profile pipelines do not see the family. All selected templates consume the same
final names and labels; the collector does not create a private metric stream per profile.
Only the block matching the selected mode (mode_exact or mode_combined) is read; entries under the
other block are ignored. Metrics not covered by an authored profile chart keep their generic autogen
charts unless an applicable profile autogen.selector rejects them. Every selector is limited to its
profile's match scope; when scopes overlap, every applicable selector must accept the series. This
changes fallback charts only; use selector or a relabeling drop rule to discard samples.
Stock profiles leave unknown future families eligible for generic fallback; closed fallback selectors are a
user-owned deployment policy, not a stock-profile authoring pattern.

profiles:

mode: exact

mode_exact:

entries:

- name: haproxy

via UI​

Configure the prometheus collector from the Netdata web interface:

Go to Nodes.

Select the node where you want the prometheus data-collection job to run and click the ⚙ (Configure this node). That node will run the data collection.

The Collectors → Jobs view opens by default.

In the Search box, type prometheus (or scroll the list) to locate the prometheus collector.

Click the + next to the prometheus collector to add a new job.

Fill in the job fields, then click Test to verify the configuration and Submit to save.

Test runs the job with the provided settings and shows whether data can be collected.

If it fails, an error message appears with details (for example, connection refused, timeout, or command execution errors), so you can adjust and retest.

via File​

The configuration file name for this integration is go.d/prometheus.conf.

The file format is YAML. Generally, the structure is:

update_every: 1

autodetection_retry: 0

jobs:

- name: some_name1

- name: some_name2

You can edit the configuration file using the edit-config script from the
Netdata config directory.

cd /etc/netdata 2>/dev/null || cd /opt/netdata/etc/netdata

sudo ./edit-config go.d/prometheus.conf

Examples​

Basic​

Note: Change the port of the monitored application on which it provides metrics.

A basic example configuration.

jobs:

- name: local

url: http://127.0.0.1:9090/metrics

Read metrics from a file​

An example configuration to read metrics from a file.
Config

# use "file://" scheme

jobs:

- name: myapp

url: file:///opt/metrics/myapp/metrics.txt

HTTP authentication​

Note: Change the port of the monitored application on which it provides metrics.

Basic HTTP authentication.
Config

jobs:

- name: local

url: http://127.0.0.1:9090/metrics

username: username

password: password

HTTPS with self-signed certificate​

Note: Change the port of the monitored application on which it provides metrics.

Do not validate server certificate chain and hostname.
Config

jobs:

- name: local

url: https://127.0.0.1:9090/metrics

tls_skip_verify: yes

Multi-instance​

Note: When you define multiple jobs, their names must be unique.
Note: Change the port of the monitored application on which it provides metrics.

Collecting metrics from local and remote instances.
Config

jobs:

- name: local

url: http://127.0.0.1:9090/metrics

- name: remote

url: http://192.0.2.1:9090/metrics

Metric relabeling​

Derive a code_class label (2xx, 4xx, ...) on metrics named http_*.
Config

jobs:

- name: local

url: http://127.0.0.1:9090/metrics

relabeling:

- match: 'http_*'

metric_relabel_configs:

- source_labels: [code]

regex: '(\d)\d\d'

target_label: code_class

replacement: '${1}xx'

Rename labels that collide with Netdata's reserved labels​

When these metrics are re-exported in Prometheus format, Netdata adds its own instance,
family, chart, and dimension labels. If the scraped endpoint already uses one of those
names, the re-export emits a duplicate label and a downstream Prometheus rejects the scrape.
Rename the colliding labels to avoid it (the use case the former label_prefix option served).
Config

jobs:

- name: coredns

url: http://127.0.0.1:9153/metrics

relabeling:

- match: '*'

metric_relabel_configs:

- regex: '(instance|family)'

action: labelmap

replacement: 'coredns_$1'

- regex: '(instance|family)'

action: labeldrop

Alerts​

There are no alerts configured by default for this integration.

Metrics​

This collector has built-in grouping logic based on the type of metrics.
MetricChartDimension(s)AlgorithmGaugefor each label setone, the metric nameabsoluteCounterfor each label setone, the metric nameincrementalSummary (quantiles)for each label set (excluding 'quantile')for each quantileabsoluteSummary (sum and count)for each label setthe metric nameincrementalHistogram (buckets)for each label set (excluding 'le')for each bucketincrementalHistogram (sum and count)for each label setthe metric nameincremental

Untyped metrics (have no '# TYPE') processing:

As Counter or Gauge depending on pattern match when 'fallback_type' is used.

As Counter if it has suffix '_total'.

As Summary if it has 'quantile' label.

As Histogram if it has 'le' label.

The rest are ignored.

Troubleshooting​

Debug Mode​

Important: Debug mode is not supported for data collection jobs created via the UI using the Dyncfg feature.

To troubleshoot issues with the prometheus collector, run the go.d.plugin with the debug option enabled. The output
should give you clues as to why the collector isn't working.

Navigate to the plugins.d directory, usually at /usr/libexec/netdata/plugins.d/. If that's not the case on
your system, open netdata.conf and look for the plugins setting under [directories].

cd /usr/libexec/netdata/plugins.d/

Switch to the netdata user.

sudo -u netdata -s

Run the go.d.plugin to debug the collector:

./go.d.plugin -d -m prometheus

To debug a specific job:

./go.d.plugin -d -m prometheus -j jobName

Getting Logs​

If you're encountering problems with the prometheus collector, follow these steps to retrieve logs and identify potential issues:

Run the command specific to your system (systemd, non-systemd, or Docker container).

Examine the output for any warnings or error messages that might indicate issues.  These messages should provide clues about the root cause of the problem.

System with systemd​

Use the following command to view logs generated since the last Netdata service restart:

journalctl _SYSTEMD_INVOCATION_ID="$(systemctl show --value --property=InvocationID netdata)" --namespace=netdata --grep prometheus

System without systemd​

Locate the collector log file, typically at /var/log/netdata/collector.log, and use grep to filter for collector's name:

grep prometheus /var/log/netdata/collector.log

Note: This method shows logs from all restarts. Focus on the latest entries for troubleshooting current issues.

Docker Container​

If your Netdata runs in a Docker container named "netdata" (replace if different), use this command:

docker logs netdata 2>&1 | grep prometheus

Disappearing or sparse metrics not clearing alerts​

The Prometheus collector detects metrics that disappear from a successful scrape response. Generated charts
and individual dimensions expire after their configured successful-cycle lifetime. An expired chart or
dimension makes its alerts REMOVED; this is not a normal CLEAR transition and does not send a recovery
notification. Export an explicit normal value (for example 0) whenever an alert needs a reliable recovery
transition. A failed scrape does not advance the expiry lifetime; use the generic collector collection-failure
alert to detect that separate condition.

Do you have any feedback for this page? If so, you can open a new issue on our netdata/learn repository.

Overview
Default Behavior
Auto-Detection
Limits
Performance Impact
Setup
Prerequisites
Install Exporter
Configuration
Options
selector
fallback_type
relabeling
profiles
via UI
via File
Examples
Basic
Read metrics from a file
HTTP authentication
HTTPS with self-signed certificate
Multi-instance
Metric relabeling
Rename labels that collide with Netdata's reserved labels
Alerts
Metrics
Troubleshooting
Debug Mode
Getting Logs
System with systemd
System without systemd
Docker Container
Disappearing or sparse metrics not clearing alerts