---
title: MQTT Discovery - Availability - Home Assistant OS - Home Assistant Community
id: mqtt-discovery-availability-home-assistant-os-home-assistant-community
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- availability
- retained-messages
- community-thread
created: '2026-09-02T04:02:29.652112Z'
updated: '2026-09-05T10:51:21.605255Z'
source: https://community.home-assistant.io/t/mqtt-discovery-availability/206758
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T04:02:26.626428Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA community thread (June 2020, Taras/123 answering benlad): a discovery-config
  sensor with availability_topic remains ''unavailable'' (and the Gauge card errors
  ''Entity is non-numeric'') until a payload is ACTUALLY published to the availability
  topic — publishing state alone is not enough; both state and availability must arrive
  before the sensor shows a value. Taras''s key recommendation: ''In practice, both
  the temperature and the availability should be published as retained messages...
  If Home Assistant disconnects from the broker and then reconnects (like after a
  restart) it will immediately receive the stored values''. Without retained availability+state,
  every HA restart leaves the entity unavailable until the device next publishes.
  OP also notes their MQTT library silently failed to honour the retain flag — a real-world
  gotcha where the retained flag is set but not sent. Design takeaway for ha-linux-agent:
  publish availability and state topics retained so HA restarts restore instantly;
  verify retain actually reaches the broker.'
---

MQTT Discovery - Availability - Home Assistant OS - Home Assistant Community

MQTT Discovery - Availability

Installation

Home Assistant OS

benlad

(Benlad)

June 22, 2020,  7:37pm

1

I am trying to add availability to my discovered sensor.  The temperature is discovered fine with

Topic:
homeassistant/sensor/inkbird_f8300232744d/temperature1/config

Payload
{
"name":"inkbird_f8300232744d_temperature1",
"device_class":"temperature",
"state_topic":"inkbird_f8300232744d/temperatures",
"unit_of_measurement":"°C",
"unique_id":"inkbird_f8300232744d_temperature1",
"value_template":"{{value_json.temperature1}}"
}

If I publish a similar config with availability:
{
"name":"inkbird_f8300232744d_temperature1",
"device_class":"temperature",
"state_topic":"inkbird_f8300232744d/temperatures",
"availability_topic":"inkbird_f8300232744d/availability",
"payload_available":"Online",
"payload_not_available":"Offline",
"unit_of_measurement":"°C",
"unique_id":"inkbird_f8300232744d_temperature1",
"value_template":"{{value_json.temperature1}}"
}

I get this shown for my gauge in lovelace:

Any help will be appreciated!

123

(Taras)

June 22, 2020,  8:57pm

2

My guess is you haven’t published Online to inkbird_f8300232744d/availability.

I used your discovery topic and payload to create the sensor. Here’s how it appears in Developer Tools > States:

Screenshot from 2020-06-22 16-48-10862×72 9.28 KB

Its state is unavailable (which is certain to cause “Entity is non-numeric” for the Gauge card).

I then publish {"temperature": 23 } to  inkbird_f8300232744d/temperatures. However, the sensor’s reported state remains the same: unavailable

Why? Because it has not yet received information for the availability_topic.

Finally, I publish Online to inkbird_f8300232744d/availability. Now it knows the sensor’s availability status (Online) and its state (23) and displays the following:

Screenshot from 2020-06-22 16-50-36859×65 8.93 KB

In practice, both the temperature and the availability should be published as retained messages. It instructs the broker to store the values. If Home Assistant disconnects from the broker and then reconnects (like after a restart) it will immediately receive the stored values (as opposed to waiting for the next time the physical sensor reports its state and availability).

benlad

(Benlad)

June 22, 2020, 11:10pm

3

That looks like it.  For some reason I can’t get my MQTT library to publish a retained message.  I set the flag, but it doesn’t work.

I was thinking that availability offline might make the gauge disabled or similar, but I just get the error.  Now, I am not sure of the point!

123

(Taras)

June 22, 2020, 11:43pm

4

benlad:

I can’t get my MQTT library to publish a retained message

That will become inconvenient because whenever Home Assistant restarts and re-subscribes to the state_topic it will receive no payload (and the sensor will have no numeric value to report).

benlad:

thinking that availability offline might make the gauge disabled or similar

You might make the Gauge card part of a Conditional card. The condition would be the the sensor’s state is not unavailable.

benlad

(Benlad)

June 23, 2020,  1:03am

5

Thanks again for your help.

Powered by Discourse, best viewed with JavaScript enabled