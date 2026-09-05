---
title: 'GitHub - inovex/mqtt_blackbox_exporter: Prometheus Exporter for MQTT monitoring
  · GitHub'
id: github-inovexmqtt_blackbox_exporter-prometheus-exporter-for-mqtt-monitoring-gith
tags:
- linux-agent-jupiteros-fleet-15537b
- primary-source
- mqtt
created: '2026-09-02T06:16:28.006341Z'
updated: '2026-09-02T17:37:22.275375Z'
source: https://github.com/inovex/mqtt_blackbox_exporter
source_domain: github.com
fetched_at: '2026-09-02T06:16:28.004776Z'
fetch_provider: builtin
status: review
type: note
tier: practitioner
content_type: code
deprecated: false
summary: 'inovex/mqtt_blackbox_exporter (Go, Apache-2.0, 83 stars, 99 commits, 24
  forks) — the concrete Prometheus exporter behind the Netdata ''MQTT blackbox'' pattern.
  Probe definition: subscribe to a topic, publish N messages to the same topic, count
  received messages — a publish/subscribe round-trip per probe. Config via config.yaml.dist,
  single binary or Docker image exposing /metrics on port 9214. Emitted metrics: probe_mqtt_completed_total
  and probe_mqtt_started_total (probe counters), probe_mqtt_duration_seconds histogram
  (round-trip latency buckets), probe_mqtt_messages_published_total and probe_mqtt_messages_received_total
  (per-broker counters — published-vs-received delta directly exposes message loss).
  All metrics labelled with broker URL (e.g. ssl://mqtt.example.net:8883) and probe
  name, so one exporter instance can supervise multiple brokers including TLS. Directly
  reusable as the broker-side synthetic check for a jupiterOS MQTT backbone: it would
  have caught the silent message-drop and retained-message regressions that a purely
  agent-side liveness sensor cannot see.'
---

*Suggested by [[mqtt-blackbox-monitoring-netdata]] — Netdata page cited this exporter as the actual MQTT blackbox probing tool*

GitHub - inovex/mqtt_blackbox_exporter: Prometheus Exporter for MQTT monitoring · GitHub

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

inovex

/

mqtt_blackbox_exporter

Public

Notifications
You must be signed in to change notification settings

Fork
24

Star
83

master

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History99 Commits

99 Commits
Folders and filesNameName
Last commit message
Last commit date

.github/workflows

.github/workflows

test

test

.gitignore

.gitignore

.goreleaser.yml

.goreleaser.yml

.travis.yml

.travis.yml

Dockerfile

Dockerfile

LICENSE

LICENSE

Makefile

Makefile

README.md

README.md

VERSION

VERSION

config.yaml.dist

config.yaml.dist

go.mod

go.mod

go.sum

go.sum

main.go

main.go

View all files

Repository files navigation

MQTT Blackbox Exporter

Tests MQTT messaging roundtrips (publish/subscribe on same topic).

Definition of roundtrip:

start subscriber on $topic

start publisher on $topic

publish $messages on $topic

receive $message on $topic

Build

$ mkdir -p ${GOPATH}/src/github.com/inovex/
$ git clone https://github.com/inovex/mqtt_blackbox_exporter.git ${GOPATH}/src/github.com/inovex/mqtt_blackbox_exporter/
$ cd ${GOPATH}/src/github.com/inovex/mqtt_blackbox_exporter/
$ make

This will build the mqtt_blackbox_exporter for all target platforms and write them to the build/ directory.

Binaries are provided on Github, see https://github.com/inovex/mqtt_blackbox_exporter.

Install

Place the binary somewhere in a PATH directory and make it executable (chmod +x mqtt_blackbox_exporter).

Configure

See config.yaml.dist for a configuration example.

Run

Native:

$ ./mqtt_blackbox_exporter -config.file config.yaml

Using Docker:

docker run --rm -it -p 9214:9214 -v ${PWD}/:/data/ inovex/mqtt_blackbox_exporter:<VERSION> -config.file /data/config.yaml

$ curl -s http://127.0.0.1:9214/metrics
...
# HELP probe_mqtt_completed_total Number of completed probes.
# TYPE probe_mqtt_completed_total counter
probe_mqtt_completed_total{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL"} 64

...

# HELP probe_mqtt_duration_seconds Time taken to execute probe.
# TYPE probe_mqtt_duration_seconds histogram
probe_mqtt_duration_seconds_bucket{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL",le="0.005"} 0
probe_mqtt_duration_seconds_bucket{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL",le="0.01"} 0
probe_mqtt_duration_seconds_sum{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL"} 50.09346619300002
probe_mqtt_duration_seconds_count{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL"} 64
...

# HELP probe_mqtt_messages_published_total Number of published messages.
# TYPE probe_mqtt_messages_published_total counter
probe_mqtt_messages_published_total{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL"} 640
...

# HELP probe_mqtt_messages_received_total Number of received messages.
# TYPE probe_mqtt_messages_received_total counter
probe_mqtt_messages_received_total{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL"} 640
...

# HELP probe_mqtt_started_total Number of started probes.
# TYPE probe_mqtt_started_total counter
probe_mqtt_started_total{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL"} 64
...

Release

The release is done automatically by goreleaser. To release a new version,
just set a git tag. Goreleaser run by Github Actions will create the binaries and upload them to the according Github release.

About
Prometheus Exporter for MQTT monitoring
Topics
exportermonitoringmqttprometheus
Resources
Readme
Apache-2.0 license
Activity
Custom properties
Stars
83 stars
Watchers
17 watching
Forks
24 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.