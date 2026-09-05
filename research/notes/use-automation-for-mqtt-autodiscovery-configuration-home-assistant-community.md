---
title: Use Automation for MQTT Autodiscovery - Configuration - Home Assistant Community
id: use-automation-for-mqtt-autodiscovery-configuration-home-assistant-community
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- ha-issue
- official-docs
- known-issue
- community-thread
- practitioner-forum
- mqtt-discovery
created: '2026-09-02T06:41:31.092392Z'
updated: '2026-09-05T10:51:22.047790Z'
source: https://community.home-assistant.io/t/use-automation-for-mqtt-autodiscovery/455475
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T06:41:31.045947Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA community thread (Aug 2022, INSTAR/Mike Polinowski, HasQT, luma): attempt
  to publish MQTT discovery config via an HA automation when a camera''s LWT indicates
  connection. Core finding: HA''s automation mqtt.publish action renders Jinja templates
  in the payload BEFORE publishing — so a discovery payload containing value_template:
  ''{{ value_json.val }}'' (an instruction intended for HA''s MQTT integration to
  evaluate LATER on the state topic) is evaluated immediately at publish time, errors
  on undefined value_json, and the message is never forwarded. OP''s workaround: write
  an external Python Paho client (github.com/mpolinowski/ha-mqtt-python) run from
  a shell script, bypassing HA templating entirely — ''My MQTT camera connects to
  the Home Assistant MQTT broker and is immediately added as a device''. HasQT contributes
  a bashio/mosquitto_pub pattern for add-on sensor publishing with escaped value_templates
  ''{{ value_json. | default() }}'' and notes the MQTT integration ''does not seem
  to like disabled sensors much though. If they are disabled from the script they
  can''t be deleted by the mqtt integration if they have never been enabled''. Direct
  lesson for ha-linux-agent: the agent publishes discovery JSON itself over MQTT (not
  through HA automations), so templates inside value_template fields are just data
  — but any tooling that publishes config via HA mqtt.publish must escape or avoid
  nested templates. Also documents entity naming via object_id and disabled-entity
  deletion quirks. Provenance: practitioner forum, first-hand build log.'
---

Use Automation for MQTT Autodiscovery - Configuration - Home Assistant Community

Use Automation for MQTT Autodiscovery

Configuration

mqtt

INSTAR

(Mike Polinowski)

August 30, 2022, 10:05am

1

What I try to accomplish

I want HA to publish a "config" update through the MQTT service when an MQTT device is connecting.

What is working

I can manually update the topic:
homeassistant/switch/in9408_garden/alarm_area_red/config

with the following JSON payload to use the MQTT auto-discovery to configure my device:
{
"device": {
"identifiers": "in9408_garden",
"manufacturer": "INSTAR Deutschland GmbH",
"model": "INSTAR 2k+ IN-9408 WLAN",
"name": "IN-9408 2k+ Garden",
"configuration_url": "http://192.168.2.115:80"
},
"availability": {
"topic": "cameras/115/status/testament",
"payload_available": "{\"val\":\"alive\"}",
"payload_not_available": "{\"val\":\"dead\"}"
},
"object_id":"in9408_garden_alarm_area_red",
"unique_id": "in9408_garden_alarm_area_red",
"name": "IN-9408 2k+ Garden Alarm Area Red",
"icon": "mdi:camera-metering-matrix",
"command_topic": "cameras/115/alarm/areas/red/enable",
"payload_on": "{\"val\":\"1\"}",
"payload_off": "{\"val\":\"0\"}",
"state_topic": "cameras/115/status/alarm/areas/red/enable",
"state_on": 1,
"state_off": 0,
"value_template": "{{ value_json.val }}",
"qos": 1
}

What doesn' t work

I want to add an automation that publishes the above update whenever my devices Last-Will topic indicates that the device was connected - and I am getting the following error message when I run the action:

2022-08-30_18-02744×723 64.3 KB

Anyone ran into this issue before?

HasQT

(Has)

August 30, 2022, 10:27am

2

Just guessing…

It might need to converted to a “literal” (or from a literal, idk)

“{{ value_json.val }}” change to
\"{{ value_json.val }}\"

INSTAR

(Mike Polinowski)

August 30, 2022, 11:09am

3

Hey Has,

thank you for your reply - but unfortunately I am still getting the same error:

2022-08-30_19-08758×217 21.3 KB

luma

August 30, 2022, 11:21am

4

This might be of help.  If you do a “Run Action” in the automation editor, it skips the triggers/conditions and just starts running your actions.  If your actions fundamentally require information from an incoming trigger (like, say, decoding an incoming JSON message), then your automation won’t work when fired manually.

edit: just to be clear, you’ll probably need to fire off that MQTT message in order to test your automation correctly.  Also, you’ll probably want a default value declared for your template so you don’t get errors if it gets triggered in some other way (like somebody clicked “run actions”).

INSTAR

(Mike Polinowski)

August 30, 2022, 11:38am

5

Hi luma,

is_defined sounded promising - I tried it and it does not change the error message. The script is self-contained. value_json.val is an instruction HA is supposed to use once the device is set up (which means that HA subscribed to the given MQTT topics).

When the script is run there is no value_json.val yet - this instruction probably needs to be send as type string. So I think Has has the right idea - but I am not sure why it does not work.

But I am also not sure what happens when I run this script - compared to what happens when I send the MQTT update manually

luma

August 30, 2022, 12:07pm

6

value_jason.val is undefined when you press “RUN ACTIONS” in the automation editor, because there was no JSON being sent to the automation.  It will never work when you press RUN ACTIONS for this reason.  If you add a default value you can get rid of that error, but it still isn’t going to work because once again you don’t have a JSON that is being sent to your automation for it to Do Things with when you press RUN ACTIONS.

So, give yourself a default, and also expect to manually send MQTT messages for your test/dev process because RUN ACTIONS will not do what you are expecting.

luma

August 30, 2022, 12:10pm

7

I should also note: it’s technically possible to capture the RUN ACTIONS via a condition {{ trigger.platform is none }}, so if you really want to test via RUN ACTIONS, you can create a separate code path that is followed when somebody presses that button in the automation editor.

INSTAR

(Mike Polinowski)

September 2, 2022, 10:15am

8

luma:

So, give yourself a default, and also expect to manually send MQTT messages for your test/dev process because RUN ACTIONS will not do what you are expecting.

Hi Luma,

I spend some thinking about this issue. And this is not the solution. Your option is for a script that - when run in production - will receive data that defines this variable. But this is not going to happen in my script.

What is going wrong

I define an action that says:

Here is an instruction.  DO NOT READ THIS. Just forward it to the MQTT broker. Someone will pick it up from there and will know what to do with it.

I run the script and get an error:

I Read the instruction. Don't know what it means. I am not going to forward it. Break.

What I would need is a way to mask the variable. So the script does not see it. But it has to be readable again for the HA MQTT auto-discovery service (that will pick it up from the broker). I thought maybe there is a way to use templating to generate the variable on run-time. So that it is not being executed. But I did not find a solution for it yet.

What I am doing now

I wrote a Python mqtt client.

I use a shell script to run the client (I had issue getting the Python file to run using python_scripts I guess it is because of the isolation/virtEnv).

The client then updates all the MQTT configuration topics.

And the shell script then becomes the action for my automation.

Result: My MQTT camera connects to the Home Assistant MQTT broker and is immediately added as a device with a bunch of functions:

2022-09-02_18-141026×857 94.1 KB

GitHub - mpolinowski/ha-mqtt-python: Paho MQTT Python client script for Home Assistant...

Paho MQTT Python client script for Home Assistant MQTT Auto-Discovery

This was an interesting deep-dive into HA for me  But it is a bit convoluted...

HasQT

(Has)

September 2, 2022, 11:08am

9

I have been playing around with this for my addon.

It is abit messy at the moment, but I can add the device sensors from the addon with bashio scripts

I cant get it to read my $mqtt_data yet… having trouble breaking the string up. But I can send all the sensors to mqtt.

The mqtt integration does not seem to like disabled sensors much though. If they are disabled from the script they can’t be deleted by the mqtt integration if they have never been enabled

You also may need to set you default() in your value templates
\"{{ value_json.$value | default() }}\",

"MQTT_Data": "Timestamp,SunRise,SunSet,InvSerial,InvName,InvTime,InvStatus,InvSwVer",

if bashio::var.has_value "$MQTT_Data" 'InvTime' ; then
value='InvTime'
describe='SMA Time Inverter'
mdi_icon='mdi:clock'
devClass=
stClass=
UoM=
EnabDef='true'
ent_cat='diagnostic'

bashio::log.info Setting Up "$value"

/usr/bin/mosquitto_pub -h "$MQTT_Host" -u "$MQTT_User" -P "$MQTT_Pass" -t homeassistant/sensor/sbfspot_"$PLANTNAME"/sbfspot_"$InvSerial""$value"/config -m "{\"name\": \"$describe\", \"state_topic\": \"homeassistant/sbfspot_$PLANTNAME/sbfspot_$InvSerial\", \"value_template\": \"{{ value_json.$value | default() }}\", \"unique_id\": \"$InvSerial"'_'"$value\", \"enabled_by_default\": \"$EnabDef\", \"entity_category\": \"$ent_cat\", \"icon\": \"$mdi_icon\", \"device\": { \"identifiers\": [\"$(bashio::addon.name)""-Sensors\"], \"name\": \"HAOS-SBFspot\", \"model\": \"$InvType\", \"manufacturer\": \"SMA\", \"sw_version\": \"$InvSwVer\" }}" "$debugMQTT" "$RoK"
else
bashio::log.yellow Skipping "${value}"
fi

if bashio::var.equals "${MQTT_Data}" 'SunRise' ; then
value='SunRise'
describe='SMA Sun Rise'
mdi_icon='mdi:weather-sunny'
devClass=
stClass=
UoM=
EnabDef='true'
ent_cat=

bashio::log.info Setting Up "$value"

/usr/bin/mosquitto_pub -h "$MQTT_Host" -u "$MQTT_User" -P "$MQTT_Pass" -t homeassistant/sensor/sbfspot_"$PLANTNAME"/sbfspot_"$InvSerial""$value"/config -m "{\"name\": \"$describe\", \"state_topic\": \"homeassistant/sbfspot_$PLANTNAME/sbfspot_$InvSerial\", \"value_template\": \"{{ value_json.$value | default() }}\", \"unique_id\": \"$InvSerial"'_'"$value\", \"enabled_by_default\": \"$EnabDef\", \"icon\": \"$mdi_icon\", \"device\": { \"identifiers\": [\"$(bashio::addon.name)""-Sensors\"], \"name\": \"HAOS-SBFspot\", \"model\": \"$InvType\", \"manufacturer\": \"SMA\", \"sw_version\": \"$InvSwVer\" }}" "$debugMQTT" "$RoK"
else
bashio::log.yellow Skipping "${value}"
fi

habuild/hassio/blob/16c0304bd7dbdf1af710134654a673778d46c375/ether-sbfspot/rootfs/usr/bin/sbfspot/mqttSensorConfig

#!/usr/bin/with-contenv bashio
# shellcheck shell=bash
#  Publish MQTT sensors for Config discovery.

#set -x

CONFIG_PATH=/data/options.json

PLANTNAME="$(jq --raw-output '.Plantname' $CONFIG_PATH)"
MQTT_Host="$(jq --raw-output '.MQTT_Host' $CONFIG_PATH)"
MQTT_User="$(jq --raw-output '.MQTT_User' $CONFIG_PATH)"
MQTT_Pass="$(jq --raw-output '.MQTT_Pass' $CONFIG_PATH)"
MQTT_Topic="$(jq --raw-output '.MQTT_Topic' $CONFIG_PATH)"
MQTT_Data="$(jq --raw-output '.MQTT_Data' $CONFIG_PATH)"
#  arg="$(jq --raw-output '.MQTT_Data' /data/options.json | tr ',' '\n')"

echo "$MQTT_Host"
echo "$MQTT_User"
#  echo "$MQTT_Pass"
echo "$MQTT_Topic"

This file has been truncated. show original

marcelod

(garliclord)

February 14, 2023, 12:11pm

10

This doesn’t fully answer your question since it doesn’t seem to work when evaluating the actual template, but from the HA docs in this page, the example they give for how a template should be passed shows this syntax:

"value_template": "{% raw %}{{ value|float }}{% endraw %}"

Wrapping the template in {% raw %} allows it to be passed onto the MQTT broker without being evaluated in the process. That said, I can’t seem to get any templates to work when using this.

mekaneck

(Rick)

April 2, 2023,  9:52am

11

Perhaps this post will help:

https://community.home-assistant.io/t/manually-add-mqtt-entities/264695/7

Powered by Discourse, best viewed with JavaScript enabled