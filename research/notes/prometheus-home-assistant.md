---
title: Prometheus - Home Assistant
id: prometheus-home-assistant
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- home-assistant
- official-docs
- prometheus
- metrics-export
created: '2026-09-02T04:02:40.501931Z'
updated: '2026-09-02T17:37:22.011893Z'
source: https://www.home-assistant.io/integrations/prometheus
source_domain: www.home-assistant.io
fetched_at: '2026-09-02T04:02:35.850539Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'Official Home Assistant docs for the core Prometheus integration (since
  HA 0.49, ~3565 active installs, maintainer @knyar): exposes HA ENTITY metrics in
  Prometheus text format at /api/prometheus - NOT host/OS metrics. Pull model: an
  external Prometheus server scrapes HA on an interval (60s in examples) with a long-lived
  access token; no push. Config: namespace prefix, entity/domain/glob include-exclude
  filters, override_metric mapping, requires_auth (default true). Exports only 20
  domains (alarm_control_panel, automation, binary_sensor, climate, cover, counter,
  device_tracker, fan, geo_location, humidifier, input_boolean, input_number, light,
  lock, number, person, sensor, switch, update, water_heater) plus area_info/entity_info/floor_info
  info-metrics for label joins; unavailable/unknown entities are unexported. Marked
  LEGACY integration. Contrast for the jupiterOS agent design: ha-linux-agent''s MQTT
  push of host metrics into HA entities is a different lane than this pull exporter
  - both need a bridge for host metrics, which this integration deliberately does
  not provide.'
---

Prometheus - Home Assistant

On this page

Configuration

Configure Filter

Full Example

Metrics in unavailable or unknown states

Supported metrics

Info metrics

Related topics

Prometheus

The Prometheus integrationIntegrations connect and integrate Home Assistant with your devices, services, and more. [Learn more] exposes metrics in a format which Prometheus can read.

Configuration

To use the prometheus integrationIntegrations connect and integrate Home Assistant with your devices, services, and more. [Learn more] in your installation, add the following to your configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] file.
After changing the configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] file, restart Home Assistant to apply the changes.

# Example configuration.yaml entry
prometheus:

Configuration Variables

Looking for your configuration file?

namespace string (Optional)

The “namespace” that will be assigned to all the Prometheus metrics. This is the prefix of the metric name. E.g., having myhass as the namespace will cause the device tracker metrics to be myhass_device_tracker_state, the switch metrics to be myhass_switch_state and so on. The default is to not add any prefix to the metrics name. (available in version 0.73.0 and later)

filter list (Optional)

Filtering directives for the integrations which should be included or excluded from recording. (Configure Filter)

exclude_entities list (Optional)

The list of entity ids to be excluded from recording.

exclude_entity_globs list (Optional)

Exclude all entities matching a listed pattern (e.g., sensor.weather_*).

exclude_domains list (Optional)

The list of domains to be excluded from recording.

include_entities list (Optional)

The list of entity ids to be included from recordings. If set, all other entities will not be recorded. Values set by the exclude_* option will prevail.

include_entity_globs list (Optional)

Include all entities matching a listed pattern (e.g., sensor.weather_*). If set, all other entities will not be recorded. Values set by the exclude_* option will prevail.

include_domains list (Optional)

The list of domains to be included from recordings. If set, all other entities will not be recorded. Values set by the exclude_* option will prevail.

default_metric string (Optional)

Metric name to use when an entity doesn’t have a unit.

Default:
uses the entity id of the entity

override_metric string (Optional)

Metric name to use instead of unit or default metric. This will store all data points in a single metric.

component_config string (Optional)

This attribute contains integration-specific override values. See Customizing devices and services for format.

override_metric string (Optional)

Metric name to use instead of unit or default metric. This will store all data points in a single metric.

component_config_domain string (Optional)

This attribute contains domain-specific integration override values. See Customizing devices and services for format.

override_metric string (Optional)

Metric name to use instead of unit or default metric. This will store all data points in a single metric.

component_config_glob string (Optional)

This attribute contains integration-specific override values. See Customizing devices and services for format.

override_metric string (Optional)

Metric name to use instead of unit or default metric. This will store all data points in a single metric.

requires_auth boolean (Optional, default: true)

This makes authentication optional for the /api/prometheus endpoint.

Configure Filter

By default, no entity will be excluded. To limit which entities are being exposed to Prometheus, you can use the filter parameter.

# Example filter to include specified domains and exclude specified entities
prometheus:
filter:
include_domains:
- alarm_control_panel
- light
include_entity_globs:
- binary_sensor.*_occupancy
exclude_entities:
- light.kitchen_light

Filters are applied as follows:

No filter

All entities included

Only includes

Entity listed in entities include: include

Otherwise, entity matches domain include: include

Otherwise, entity matches glob include: include

Otherwise: exclude

Only excludes

Entity listed in exclude: exclude

Otherwise, entity matches domain exclude: exclude

Otherwise, entity matches glob exclude: exclude

Otherwise: include

Domain and/or glob includes (may also have excludes)

Entity listed in entities include: include

Otherwise, entity listed in entities exclude: exclude

Otherwise, entity matches glob include: include

Otherwise, entity matches glob exclude: exclude

Otherwise, entity matches domain include: include

Otherwise: exclude

Domain and/or glob excludes (no domain and/or glob includes)

Entity listed in entities include: include

Otherwise, entity listed in exclude: exclude

Otherwise, entity matches glob exclude: exclude

Otherwise, entity matches domain exclude: exclude

Otherwise: include

No Domain and/or glob includes or excludes

Entity listed in entities include: include

Otherwise: exclude

The following characters can be used in entity globs:

* - The asterisk represents zero, one, or multiple characters
? - The question mark represents zero or one character

Full Example

Advanced configuration example:

# Advanced configuration.yaml entry
prometheus:
namespace: hass
component_config_glob:
sensor.*_hum:
override_metric: humidity_percent
sensor.*_temp:
override_metric: temperature_c
sensor.temperature*:
override_metric: temperature_c
sensor.*_bat:
override_metric: battery_percent
filter:
include_domains:
- sensor
exclude_entity_globs:
- sensor.weather_*

You can then configure Prometheus to fetch metrics from Home Assistant by adding to its scrape_configs configuration.

# Example Prometheus scrape_configs entry
- job_name: "hass"
scrape_interval: 60s
metrics_path: /api/prometheus

# Legacy api password
params:
api_password: ['PASSWORD']

# Long-Lived Access Token
bearer_token: "your.longlived.token"

scheme: https
static_configs:
- targets: ['HOSTNAME:8123']

Replace your.longlived.token with a Home Assistant generated token.

The format to configure the bearer token has changed in Prometheus 2.26, so if you have a newer version, you can use this configuration sample:

# Example Prometheus scrape_configs entry (For version 2.26+)
- job_name: "hass"
scrape_interval: 60s
metrics_path: /api/prometheus

# Long-Lived Access Token
authorization:
credentials: "your.longlived.token"

scheme: https
static_configs:
- targets: ['HOSTNAME:8123']

When looking into the metrics on the Prometheus side, there will be:

All Home Assistant domains, which can be easily found through the common namespace prefix, if defined.

The client library provided metrics, which are a bunch of process_* and also a single pseudo-metric python_info which contains (not as value but as labels) information about the Python version of the client, that is, the Home Assistant Python interpreter.

Typically, you will only be interested in the first set of metrics.

Metrics in unavailable or unknown states

When the Prometheus exporter starts (typically when Home Assistant starts), all non-excluded entities in an unavailable or unknown state are not exported until they are available and known.

If an available entity goes into state unavailable or unknown, then it will automatically be unexported and return again automatically when available and known.

Note

To filter out these stale values, entity_available could be used in a query or recording rule. For example:

- record: "known_temperature_c"
expr: "temperature_c unless entity_available == 0"

This use of unless (which can be slow to compute) is no longer necessary, but will continue to work.

Supported metrics

Metrics are exported only for the following domains:

alarm_control_panel, automation, binary_sensor, climate, cover, counter, device_tracker, fan, geo_location, humidifier, input_boolean, input_number, light, lock, number, person, sensor, switch, update, water_heater

Info metrics

The Prometheus exporter additionally exports several info metrics: area_info, entity_info and floor_info (prefixed by the namespace if configured) for each area, entity and floor configured in your system. You can do a join across metrics to then get the labels from these onto the individual sensors if you want to use a metric of that hierarchy in a query. For example, to show temperature sensors averaged per area you might do:

avg by (area) (
sensor_temperature_celsius
* on(entity) group_left(area)
entity_info
)

Related topics

Configuration file

Help us improve our documentation

Suggest an edit to this page, or provide/view feedback for this page.

Edit

Provide feedback

View pending feedback

The Prometheus  integration  was introduced in Home Assistant 0.49, and it's used by

3565 active installations.

Its IoT class is Assumed State.

💾 Legacy integration

View source on GitHub

View known issues

View feature requests

Integration owners

We are incredibly grateful to the following contributors who currently maintain this integration:

@knyar

Categories

History

On this page

Configuration

Configure Filter

Full Example

Metrics in unavailable or unknown states

Supported metrics

Info metrics

Related topics

Back to top