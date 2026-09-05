---
title: 'System design: architecting HA data flows with Telegraf, MQTT, InfluxDB and
  Grafana - Configuration - Home Assistant Community'
id: system-design-architecting-ha-data-flows-with-telegraf-mqtt-influxdb-and-grafana
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- home-assistant
- mqtt
- architecture
created: '2026-09-02T04:02:33.537408Z'
updated: '2026-09-02T17:37:21.929561Z'
source: https://community.home-assistant.io/t/system-design-architecting-ha-data-flows-with-telegraf-mqtt-influxdb-and-grafana/312723
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T04:02:33.482362Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HA community thread (adenoz + koying, June 2021) debating whether to route
  host metrics Telegraf→MQTT(Mosquitto)→HA→InfluxDB versus Telegraf→InfluxDB directly,
  with HA reading back via the InfluxDB integration. Asker''s motive for the MQTT
  path: metrics must trigger HA automations (e.g. network activity above threshold
  turns on a light), which Grafana alerting can''t do. koying''s counter-positions:
  (1) the MQTT hop is ''an unnecessary point of failure'' — he runs telegraf direct
  to InfluxDB and imports into HA; (2) the HA InfluxDB integration cannot import tags
  as attributes, limiting fidelity of read-back data; (3) HA''s InfluxDB export captures
  state changes, so attribute-level metric churn may be lost on the circuitous path.
  Architectural evidence for ha-linux-agent: the community has long wanted agent metrics
  to drive HA automations, and both halves of the fork (MQTT-for-automations vs DB-for-history)
  have documented weaknesses — an agent that publishes MQTT discovery sensors natively
  gets automations for free without the InfluxDB round-trip or the polling bridge.'
---

System design: architecting HA data flows with Telegraf, MQTT, InfluxDB and Grafana - Configuration - Home Assistant Community

System design: architecting HA data flows with Telegraf, MQTT, InfluxDB and Grafana

Configuration

mqtt,
configuration,
database

adenoz

June 2, 2021, 11:33am

1

I’ve been planning a home automation build for some time including most recently over a period of months being away from home with work. Home assistant is definitely going to be the platform I build on and around. Being unable to actually build anything yet and continue research has meant that I haven’t started building the first setup I came across. I’m aiming to build something quite robust for the long term. A little more complexity for a better overall system is fine with me.

But I’m now getting closer to execute and wanted to run some things regarding the overall system structure by the community here for advice before learning things the hard (slow) way. I want to get the data flows as good as I can from the start.

Firstly, for my sensors I’m aiming to use an entirely MQTT network using Mosquitto as the broker. I’m also aiming to use PostreSQL for the HA database rather than SQLite and feed HA data into InfluxDB.

I’m aiming to feed metrics (and eventually logs) from my router/firewall, desktops, laptops and servers into InfluxDB via Telegraf for use in both Grafana and HA. However here I get to my query.

I am considering sending the metrics (CPU use, HDD use, network activity, temps etc) from the computers from Telegraf via MQTT (which is possible) to Mosquitto which would then make it into both HA then InfluxDB. I could then use that data in both Grafana and HA. The main advantage I am considering for doing that rather than direct from Telegraf into InfluxDB is that I will be able to use that data in automations inside HA. I am thinking also that using MQTT could allow for better data assurance. I could set up alerts inside Grafana (for example a volume reaching 90% capacity or CPU above 90% for more than 1 minute etc), but that’s about it. With HA I could do much more beyond just alerting. And doing it inside Grafana means I’d also have configs/alerts in both HA and Grafana which would be very manageable but perhaps less than ideal?

So my question is, what am I missing when considering sending metrics from Telegraf via MQTT into Mosquitto then into HA and InfluxDB instead of just direct from Telegraf into InfluxDB? What assumptions have I made that are wrong? Clearly, I’ll be able to test this out when I start building the system but seeking to save lots of time by learning from others here. I’ve reviewed both the InfluxDB integration page and the page about State Objects and lots of other articles online that don’t discuss this specifically. I note in the InfluxDB page it says it transfers all State changes from HA into InfluxDB.

The main things that occur to me are that the data flow is more circular and therefore perhaps more prone to error. Perhaps some data from Telegraf will be lost or not compatible if going via MQTT then into Mosquitto then HA then InfluxDB due to some data not being a State change? What haven’t I considered? Obviously, setting this all up will take some work but just wanting to get the overall design and data flows optimised before getting my hands dirty.

Many thanks for considering this query. This is my first post here so apologies if it’s not in the right spot.

koying

(Chris B)

June 2, 2021, 11:39am

2

Could you clarify your reasoning behind going through MQTT for influxdb injection? Seems an unnecessary point of failure to me.

I go directly from telegraf to influxdb, then use the influxdb integration inside HA to “import” the data I need inside HA.

I also inject directly into influxdb from HA for some “long history” sensors, or sensors I want to plot via grafana.

adenoz

June 2, 2021, 11:46am

3

Hey thanks for your prompt response!

The main reason for my logic (perhaps flawed), was that I would like to be able to trigger automations.

I thought that going through MQTT then into Home Assistant would push the data in and trigger automations and that data would then be stored in InfluxDB for long term storage and graphing and analysis in Grafana.

A simple automation example could be that when network activity on a device goes above a threshold during certain hours, that a light will turn on.

But are you saying that if the data goes from a computer via Telegraf direct to InfluxDB that automations could still be triggered inside Home Assistant?

adenoz

June 2, 2021, 12:14pm

4

I hadn’t seen it before but just now found this great part of the home assistant site that discusses home assistant states from a data perspective. This leads me to believe that the InfluxDB integration only feeds state changes from HA to InfluxDB and not state attribute changes.(?) I think many metrics from Telegraf could be considered attribute changes and not overall state changes so they would not make it into InfluxDB via the proposed circuitous pathway.(?)

So in addition to what Chris has said, it seems going from Telegraf via MQTT through HA and into InfluxDB may not be optimal. Going direct from Telegraf into InfluxDB then seeking to use InfluxDB data to trigger automations inside HA should be workable. I must say I hadn’t considered data from InfluxDB that got there from Telegraf could be used in HA. I’d only considered the other direction. I’ll just need to confirm that is possible and identify any limitations.

koying

(Chris B)

June 2, 2021, 12:40pm

5

adenoz:

I’ll just need to confirm that is possible and identify any limitations

See InfluxDB - Home Assistant

It seems you cannot import tags as attributes, though

adenoz

June 2, 2021, 12:51pm

6

Hey wow. I think the penny has dropped on all of this. So I get now that I can have a database inside InfluxDB that is different to the home assistant one, that can have data being fed by Telegraf such as from a range of computers etc. And the data in that database can be used as home assistant sensors. And looking through the config details seems I should be able to trigger things like automations inside home assistant based on things happening on servers etc.

For some reason I just hadn’t consider data flowing in that direction. I had always just considered data flowing from home assistant to influxdb, not the other way around.

So thanks for clearing that up for me @koying !

Powered by Discourse, best viewed with JavaScript enabled