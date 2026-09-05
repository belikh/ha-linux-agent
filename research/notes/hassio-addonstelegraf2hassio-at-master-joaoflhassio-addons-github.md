---
title: hassio-addons/telegraf2hassio at master · joaofl/hassio-addons · GitHub
id: hassio-addonstelegraf2hassio-at-master-joaoflhassio-addons-github
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt-discovery
- comparative-benchmark
created: '2026-09-02T05:05:53.512611Z'
updated: '2026-09-02T17:37:22.151383Z'
source: https://github.com/joaofl/hassio-addons/tree/master/telegraf2hassio
source_domain: github.com
fetched_at: '2026-09-02T05:05:53.511212Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'Source tree of joaofl/hassio-addons telegraf2hassio (27 stars, 32 forks;
  dirs: resources, source, plus CHANGELOG.md, Dockerfile, README.md, build.json, config.yaml,
  run.sh) — the actual implementation behind the 2022 community-post prior art for
  ha-linux-agent''s architecture. README documents the add-on''s configuration schema:
  mqtt_broker/mqtt_port/mqtt_user/mqtt_pass, telegraf_topic (default ''telegraf/#''),
  calc_rate, log_level. Mechanism: the add-on subscribes to Telegraf''s native MQTT
  output and re-publishes each measurement as HA self-discoverable MQTT sensors —
  no InfluxDB or Grafana needed. calc_rate takes comma-separated Telegraf measurement
  names (host_sensor_device_uid_measurement format like ''myserver_net_enp2s0_12_bytes_recv'')
  and emits a derived _dt rate-of-change measurement announced via MQTT — manual per-sensor
  derivative configuration, exactly what an agent with native rate calculation would
  not need. Author caveat: only his own config tested, other Telegraf inputs/sensors
  unverified. Note: this is a Python add-on running INSIDE HA (add-on), i.e. a hub-side
  bridge pattern — each host still runs plain Telegraf; discovery logic and rate math
  live on the HA side, not the host. For ha-linux-agent: shows the bridge pattern''s
  config surface (broker creds + topic wildcard + per-sensor rate lists) that a per-host
  agent with built-in discovery eliminates.'
---

*Suggested by [[releasing-telegraf2hassio-for-remote-servers-monitoring-share-your-projects-home]] — actual source code of the telegraf2hassio bridge announced in the community post*

hassio-addons/telegraf2hassio at master · joaofl/hassio-addons · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

joaofl

/

hassio-addons

Public

Notifications
You must be signed in to change notification settings

Fork
32

Star
27

FilesExpand file tree

master

/
telegraf2hassio/Copy path

Directory actions

More optionsMore options

Directory actions

More optionsMore options

Latest commit

HistoryHistory

History

master

/
telegraf2hassio/Copy pathTop

Folders and filesNameName
Last commit message
Last commit date
parent directory
..

resources

resources

source

source

CHANGELOG.md

CHANGELOG.md

Dockerfile

Dockerfile

README.md

README.md

build.json

build.json

config.yaml

config.yaml

run.sh

run.sh

View all files

README.mdOutline

Telegraf2Hassio

This addon will let you display Telegraf stats of a running instance directly on you Home Assistant dashboard, using self discoverable MQTT sensors.

Differently from most Telegraf integrations approaches out there, this addon does not need InfluxDB neither Grafana dashboards to display Telegraf's data.
Instead, it translates Telegraf's native MQTT messages into Home Assistant self-discoverable ones, such that it can detect and present your data with ease.

Installation

To use this repository with your own Home Assistant setup please follow the official instructions on how to configure it.

Below the link to this addons source code @github https://github.com/joaofl/hassio-addons

Configuration

The available configuration options are as bellow. Make sure to edit
according to your setup:

options:
mqtt_broker: localhost
mqtt_port: 1883
mqtt_user: mqtt_user_here
mqtt_pass: mqtt_pass_here
telegraf_topic: telegraf/#
calc_rate: host_sensor_measurement_1,host_sensor_measurement_2
log_level: info

The calc_rate is an optional argument, but it allows to add calculated rate measurements on top of the measurements already provided by Telegraf.
For example, if you want know the data rate on a given ethernet port, then the calc_rate setting should look like below:

myserver_net_enp2s0_12_bytes_recv,nuvem_net_enp2s0_12_bytes_sent

where myserver is the Telegraf client name, net is the sensor name, enp2s0_12 is the device name, followed by its unique ID (12), and finally the measurement name bytes_sent.
Having added the settings above to calc_rate (adapted to your setup names), another measurement will be announced via MQTT, with the same name ending with _dt, containing the calculated rate of change for that specific measurement.
Multiple rate measurements can be added comma separated.

If you are not sure about the names to expect, start the addon, and check the logs after the first batch of data is received. It will show the host name, as well as of all sensors and measurements discovered.

Example dashboard

Below an example dashboard I brought up real quick. I really hope to see some much cooler ones once some dedicated people start to play around with it.

Find also the source code to it here: example_dashboard.yaml

And the corresponding Telegraf config on my server side: telegraf.conf
Note that this is a reduced config file, only showing the uncommented lines of the original file by cat /etc/telegraf/telegraf.conf | grep -v "#" | grep .

It is likely that other addons and sensors may work out of the box with this addon, but I cannot guarantee, since this is the only config I tested so far. If something goes wrong, feel free to make a PR and contribute to this addon :)

You can’t perform that action at this time.