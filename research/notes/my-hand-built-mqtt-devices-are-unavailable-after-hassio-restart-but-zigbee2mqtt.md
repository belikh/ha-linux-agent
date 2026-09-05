---
title: My hand built MQTT devices are unavailable after hass.io restart but zigbee2mqtt
  ones are - Home Assistant OS - Home Assistant Community
id: my-hand-built-mqtt-devices-are-unavailable-after-hassio-restart-but-zigbee2mqtt
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- mqtt
- discovery
- retained-messages
- availability
- community-thread
created: '2026-09-02T04:02:29.656820Z'
updated: '2026-09-05T10:51:21.610081Z'
source: https://community.home-assistant.io/t/my-hand-built-mqtt-devices-are-unavailable-after-hass-io-restart-but-zigbee2mqtt-ones-are/201214
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T04:02:27.482121Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA community thread (June 2020, Steve Harper + Taras/123): canonical explanation
  of why hand-built MQTT devices go unavailable after hass.io restarts while zigbee2mqtt
  devices come back — zigbee2mqtt publishes ALL messages retained (config, state,
  availability), while hand-built ones don''t, so after HA re-subscribes post-restart,
  non-retained topics ''receive nothing (i.e. unavailable)''. Resolution trail: (1)
  YAML-defined MQTT entities can never become ''devices'' — the device registry linkage
  requires MQTT discovery + unique_id; (2) config topics must be published retained,
  otherwise ''an entity is registered in the entity registry but now it has no configuration
  information available''; (3) OP thought retain=True was set everywhere but one topic
  was missed — after fixing, restarts restored state. Also: command topics should
  NOT be published retained (a re-subscribing client would re-receive old commands).
  Shows the full retained-message discipline a fleet agent must implement: discovery
  config retained (+ expiry), state retained, availability retained, commands not
  retained.'
---

My hand built MQTT devices are unavailable after hass.io restart but zigbee2mqtt ones are - Home Assistant OS - Home Assistant Community

My hand built MQTT devices are unavailable after hass.io restart but zigbee2mqtt ones are

Installation

Home Assistant OS

mqtt

sharper

(Steve Harper)

June 1, 2020, 10:47pm

1

Guys,

I have tried looking for answers to my questions but have failed. Apologies if this has been covered elsewhere…

A few years ago I  hand crafted a Raspebrry PI based multi-zone irrigation system for my garden which I controlled from an inbuilt web interface on the device.

I have plans to expand my home automation so decided it would be far better if I used a home automation server and rewrote my irrigation device to work with that.  I chose hass.io as I wanted maximum flexibility and I liked the idea of a non-cloud based solution.

I rewrote my PI  based irrigation device (in python) to be an MQTT client publishing and subscribing to a set of topics that describe and control the state of each watering zone. I publish availiabilty and state topics and subscribe to a command topic for each zone.

All this I have working with HA 0.110.4 on very latest production hassos 4.8. running on (another) PI 3b. I am using the mosquito broker v 5.1 and all is working ‘tickty boo’ except for a couple of things.

I have manually created entries in my configuration.yaml for my irrigation switches as well as some sensors (inc temperature) that I have installed.  ( I may go for auto discovery once I have the basic set-up working but not yet). All of my hand-crafted ‘devices’  appear as entities within HA but not devices - why?

Incidentally I have also installed the zigbee2mqtt add on and have auto discovered a  zigbee (OSRAM smart+ plug) device. This correctly appears as a device within HA so I must be missing something!

in my irrigation device I publish and subscribe to the appropriate topics in the client.on_connect() callback which is called once the client-broker connection is established. I have coded my device client according to the paho-mqtt guidelines, using loop_start() and loop_stop() methods on the MQTT client class which runs a background threat so as to re-estabilsh the MQTT broker - client connection if it is lost.

During development I am forever restarting hass.io after editing configuration.yaml.  The presence of the zigbee2mqtt  Smart+ plug doesnt seem to be affected by these restarts of HA (and therefore the broker)  but all the ‘entities’ I have created for my irrigation server appear unavilable in HA. What am I doing wrong?

I am pretty sure I could overcome this issue by publishing availability of my devices evey 5 minutes from my irrigation server but this feels wrong. I am suprised that either the client.on_connect() callback isnt called again on reestablishing the connection or the loss of connection (which must be detected in  the irrigation client) isnt rippled up to my ‘application’ code so I could take steps to re-publish and subcribe to my topics.

Happy to share configurtion.yaml and or client MQTT code if required and thanks in advance for any help /guidance you can offer.

a.

123

(Taras)

June 2, 2020,  1:55am

2

sharper:

I may go for auto discovery once I have the basic set-up working but not yet). All of my hand-crafted ‘devices’ appear as entities within HA but not devices - why?

If you want to define devices, you will have to use MQTT Discovery. Whatever MQTT sensors, switches, lights, etc that you manually define in configuration.yaml (and related files) do not accept device-related data. It states that in the device option:

device

(map)(Optional)

Information about the device this binary sensor is a part of to tie it into the device registry. Only works through MQTT discovery and when  unique_id  is set.

If you wish, you can use scripts to create entities and devices via MQTT Discovery. I provided examples in these posts:

Adding MQTT Devices without discovery Configuration

Given that zigbee2mqtt supports Home Assistant’s MQTT Discovery, it’s advantageous to use it.
There is another option. You can continue to define the entity manually and use MQTT Discovery. Create a script that publishes the sensor’s configuration to the appropriate discovery topic.
For example, here’s a simple script that creates two binary_sensors via MQTT Discovery:
#script:
create_entities:
alias: Create entities via MQTT Discovery
sequence:
- service: mqtt.publish
…

Rapsberry Pi MQTT monitor Configuration

Instead of defining the sensors in YAML, you can use the following script to define them via MQTT Discovery. The advantage is that all of the sensors will be associated with a device (that’s the one thing you currently can’t do in YAML is define MQTT Sensors with a device). It’s a handy way to organize the sensors (they are all associated with a specific device) and also permits the creation of device-oriented automations.
Modify the following script to suit your environment’s requirements:
#s…

sharper:

my irrigation server appear unavilable in HA

It sounds like your irrigation server is not publishing its messages using the retained flag.

When a message is published to a topic with the retained flag, it instructs the MQTT Broker to store (retain) the message. Any MQTT client that subscribes to that topic will receive the retained message (even if the subscriber connects to the broker after the message was published).

If the message is not published with a retain flag, MQTT clients that subscribe to the topic after the message was published will receive nothing. Home Assistant is an MQTT client and, after a restart, it connects to the MQTT Broker and subscribes to all topics of interest. It receives all retained messages and for those topics whose messages were not retained, it receives nothing (i.e. unavailable).

Given that you are the author of the irrigation server’s software, you can easily change the code so the messages are published using the retained flag.

Zigbee2mqtt publishes retained messages (that’s why they are available upon restart). You can confirm that by connecting to the MQTT Broker using a client like MQTT Explorer which indicates which topics do/don’t have retained messages.

xAPPO

(Kevin Hawkins)

June 2, 2020,  2:01am

3

@123 Kudos for the detailed and helpful reply,  it’s nice when people take the time for these

sharper

(Steve Harper)

June 2, 2020,  7:28pm

4

Taras,

Many many thanks for your help! Using the details of your scripts I was easily able to get my config topics published from my python base Irrigation Server. MQTT discovery now works a treat! All my irrigation entities are now being associated with a newly created irrigation server device - thank you!

One point, I publish the (same) config topics every time my server boots - is that good practice?

However, I am still having difficulty with state persistence  my entities in HA over restarts of HA even though I an now setting retain-True whenever I publish avaialbility and state.

Using the most excellent MQTT Explorer app you pointed me to (thank you), I can see that  the last values I published to my topics are being preserved in the Mosquitto broker even across restarts of HA (the mosquitto broker is an add-on to HA)  but still the entities appear as unavailable in the HA Lovelace UI after a restart?

PS: The MQTT Explorer showed me the broker was persisting a whole range of topics and values from various itterations of my Irrigation Server and was also very useful in purging unused items .

TIA again for further assistance!

Steve

123

(Taras)

June 2, 2020,  7:39pm

5

sharper:

I publish the (same) config topics every time my server boots - is that good practice?

I don’t see anything wrong with that. However, they should be published as retained messages. Otherwise, when Home Assistant restarts, it is presented with the conundrum that an entity is registered in the entity registry but now it has no configuration information available via MQTT Discovery.

sharper:

I am still having difficulty with state persistence my entities in HA over restarts of HA even though I an now setting retain-True whenever I publish avaialbility and state.

Does rebooting the Irrigation Server fix it?

What are you publishing for availability?

sharper

(Steve Harper)

June 2, 2020,  7:49pm

6

Wow - thanks for the quick response!

I do publish the config with retain=True.

Rebooting the irrigation server does fix the persistence issue but the MQTT Explorer shows the retained messages even if I dont?

Payloads for available/not available are “online” and “offline” respectively.

Steve

123

(Taras)

June 2, 2020,  8:17pm

7

If you are publishing retained messages to all topics:

the entity’s discovery topic (i.e. its configuration)

the entity’s state_topic (i.e. its state value)

then when Home Assistant restarts and re-subscribes to those topics, it gets the entity’s configuration and its state. The only window of opportunity for the entity’s state to be unavailable is the moment before Home Assistant connects to the broker and re-subscribes to the entity’s state_topic (which is a very short period of time).

If you say it reports unavailable until you restart the Irrigation Server, that’s much longer than a mere moment in time and something else is causing it, possibly the availability_topic. Are offline and online published as retained messages?

sharper

(Steve Harper)

June 2, 2020,  8:44pm

8

Thank you so much - that sorted it - I thought I published everyting with retain=True but I was mistaken. I do now and it is all working as I would want it to!

You have helped me enormously - I am really grateful.

Steve

xAPPO

(Kevin Hawkins)

June 2, 2020, 11:56pm

9

Just as an aside if you publish ‘command’ payloads these should not be sent ‘retained’.

The reason is that if you re-subscribe to a broker then you would receive the commands again if they were retained - and they would be actioned again even if they were originally sent a long time ago.

sharper

(Steve Harper)

June 3, 2020,  9:53am

10

Thx xAPPO - I understand!

AJStubbsy

(Adam)

July 3, 2022,  9:51am

11

Can you, or maybe Taras, tell me how you specified everything with retain=True please? I have this code based on the script that Taras created. I thought I was specifying that the messages are retained but after I restart HA, the switch is unavailable, until I press it in my dashboard. Do I need to add retain=True in to the Payload as well?
service: mqtt.publish
data:
topic: homeassistant/switch/entertainment/config
retain: true
payload: |
{
"name": "Entertainment",
"unique_id": "entertainment",
"state_topic": "stat/entertainment/POWER",
"command_topic": "cmnd/entertainment/POWER",
"payload_on": "ON",
"payload_off": "OFF",
"state_on": "ON",
"state_off": "OFF"
}

123

(Taras)

July 3, 2022, 10:24am

12

AJStubbsy:

the switch is unavailable, until I press it in my dashboard

I believe that means when the physical switch publishes its state to stat/entertainment/POWER it isn’t publishing it as a retained message.

AJStubbsy

(Adam)

July 3, 2022, 10:31am

13

Ah! So it’s the device itself that needs to be investigated. I thought the broker would remember the state from HA. I’ll look in to that, thanks.

123

(Taras)

July 3, 2022, 10:35am

14

AJStubbsy:

I thought the broker would remember the state from HA

Home Assistant doesn’t publish to the state_topic (in this case it’s stat/entertainment/POWER), that’s the device’s task (to report its current state).

Guff666

(Gareth Howell)

July 23, 2022,  1:06pm

15

The broker only retains the state if the message had the retain flag set by the device that published the message.

Powered by Discourse, best viewed with JavaScript enabled