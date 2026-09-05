---
title: Releasing Telegraf2Hassio for remote servers monitoring - Share your Projects!
  - Home Assistant Community
id: releasing-telegraf2hassio-for-remote-servers-monitoring-share-your-projects-home
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- home-assistant
- mqtt
- discovery
created: '2026-09-02T04:02:33.522150Z'
updated: '2026-09-02T17:37:21.915646Z'
source: https://community.home-assistant.io/t/releasing-telegraf2hassio-for-remote-servers-monitoring/406318
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T04:02:31.438310Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA community post (joaofl, March 28 2022) announcing Telegraf2Hassio: a
  HA add-on that bridges Telegraf''s native MQTT output into HA self-discoverable
  MQTT sensors WITHOUT needing InfluxDB or Grafana — the closest prior art to ha-linux-agent''s
  MQTT-discovery architecture. It translates Telegraf MQTT messages into HA discovery
  format so remote-server stats appear on the HA dashboard automatically; ships an
  example dashboard YAML and a minimal telegraf.conf. Caveats the author admits: only
  tested with one config, other Telegraf inputs/sensors unverified (''it is likely
  that other addons and sensors may work out of the box with this addon, but I cannot
  guarantee''). A Feb 2026 reply asks how to delete all created entities and start
  afresh — evidence that discovery-created entity lifecycle/cleanup remains an unsolved
  pain point in this pattern, directly relevant to ha-linux-agent''s entity lifecycle
  design.'
---

Releasing Telegraf2Hassio for remote servers monitoring - Share your Projects! - Home Assistant Community

Releasing Telegraf2Hassio for remote servers monitoring

Share your Projects!

mqtt

joaofl

(João)

March 28, 2022,  8:59pm

1

Telegraf2Hassio

This addon will let you display Telegraf stats of a running instance directly on you Home Assistant dashboard, using self discoverable MQTT sensors.

Differently from most Telegraf integrations approaches out there, this addon does not need InfluxDB neither Grafana dashboards to display Telegraf’s data. Instead, it translates Telegraf’s native MQTT messages into Home Assistant self-discoverable ones, such that it can detect and present your data with ease.

Find the link here: https://github.com/joaofl/hassio-addons/tree/master/telegraf2hassio

Dashboard example

Below an example dashboard I brought up real quick. I really hope to see some much cooler ones once some dedicated people start to play around with it.

dashboard-example1511×561 65 KB

Find also the source code to it here: example_dashboard.yaml

And the corresponding Telegraf config on my server side: telegraf.conf

Note that this is a reduced config file, only showing the uncommented lines of the original file by
cat /etc/telegraf/telegraf.conf | grep -v "#" | grep .

It is likely that other addons and sensors may work out of the box with this addon, but I cannot guarantee, since this is the only config I tested so far. If something goes wrong, feel free to make a PR and contribute to this addon

spry-salt

(Spry Salt)

February 7, 2026,  8:07am

2

This is great. I now have a need, however, to delete all the entities it created and start afresh. What’s the way to do that please?

Powered by Discourse, best viewed with JavaScript enabled