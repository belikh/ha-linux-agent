---
title: MQTT - Home Assistant
id: mqtt-home-assistant
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- mqtt
- home-assistant
- discovery
- availability
- official-docs
- locus-mqtt-lifecycle-supervisor-spec
created: '2026-09-02T04:02:29.616700Z'
updated: '2026-09-02T17:37:21.873181Z'
source: https://www.home-assistant.io/integrations/mqtt/
source_domain: www.home-assistant.io
fetched_at: '2026-09-02T04:02:21.576229Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'Official Home Assistant MQTT integration reference. Load-bearing details
  for any agent implementing MQTT discovery: (1) HA publishes birth message ''online''
  to homeassistant/status on MQTT integration start, LWT ''offline'' on shutdown/crash;
  devices should subscribe and re-send discovery payloads on birth, with random delay
  recommended to avoid broker IO spikes. (2) On HA restart all discovered MQTT items
  WITH unique_id become unavailable until a new discovery message; items without unique_id
  are not added at startup. (3) Two discovery-restoration approaches: birth-triggered
  rediscovery vs retained discovery messages — retained messages can create ''ghost
  entities'' and HA supports message_expiry_interval (MQTT 5) to bound retention.
  (4) Availability: per-entity availability_topic/availability list with payload_available
  (''online'' default) and payload_not_available (''offline''), availability_mode
  all/any/latest; broker publishes LWT when client loses connection. (5) last_reported
  attribute only updates when force_update is set. (6) MQTT subentries (new) allow
  device context + entities per subentry. (7) ActiveMQ and RabbitMQ MQTT plugin explicitly
  unsupported — retention broken.'
---

MQTT - Home Assistant

On this page

Configuration

Setting up a broker

Broker configuration

Other broker configuration settings

Configure MQTT options

Change MQTT discovery options

Discovery options

Birth and last will messages

Testing your setup

Naming of MQTT Entities

Grouping entities

MQTT Discovery

Discovery messages

Discovery messages and availability

Using Availability topics

Discovery examples with component discovery

Support by third-party tools

Manual configured MQTT items

YAML configuration listed per item

YAML configuration keyed and bundled by {domain}

Entity state updates

The last reported state attribute

Using Templates

Examples

List of actions

Logging

Event event_mqtt_reloaded

Removing the integration

MQTT

MQTT (aka MQ Telemetry Transport) is a machine-to-machine or “Internet of Things” connectivity protocol on top of TCP/IP. It allows extremely lightweight publish/subscribe messaging transport.

Configuration

To add the MQTT service to your Home Assistant instance, use this My button:

Manual configuration steps

If the above My button doesn’t work, you can also perform the following steps
manually:

Browse to your Home Assistant instance.

Go to  Settings > Devices & services.

In the bottom right corner, select the
Add Integration button.

From the list, select MQTT.

Follow the instructions on screen to complete the setup.

MQTT devices and entities can be set up through MQTT discovery or added manually via YAML or subentries.

Configuration of MQTT components via MQTT discovery

Alarm control panel

Binary sensor

Button

Camera

Cover

Climate (HVAC)

Date

Date Time

Device tracker

Device trigger

Event

Fan

Humidifier

Image

Infrared

Lawn mower

Light

Lock

Notify

Number

Scene

Select

Sensor

Siren

Switch

Update

Tag scanner

Text

Time

Vacuum

Valve

Water heater

Configuration of MQTT components via YAML

Alarm control panel

Binary sensor

Button

Camera

Climate (HVAC)

Cover

Date

Date Time

Device tracker

Event

Fan

Humidifier

Image

Infrared

Lawn mower

Light

Lock

Notify

Number

Scene

Select

Sensor

Siren

Switch

Text

Time

Update

Vacuum

Valve

Water heater

Configuration of MQTT components via Subentries

Alarm control panel

Binary sensor

Button

Climate (HVAC)

Cover

Date

Date/Time

Fan

Image

Light

Lock

Notify

Number

Select

Sensor

Siren

Switch

Text

Time

Valve

Water heater

To add an MQTT device via a Subentry, follow these steps:

Go to Settings > Devices & services.

Select the MQTT integration.

Add a subentry via Settings > Devices & services, select , and select Add MQTT device.

A device context and one or more entities can be added to the subentry.

Your first step to get MQTT and Home Assistant working is to choose a broker.

The easiest option is to install the official Mosquitto Broker app for Home Assistant (formerly known as Mosquitto Broker add-on). You can choose to set up and configure this app automatically when you set up the MQTT integration. Home Assistant will automatically generate and assign a safe username and password, and no further attention is required. This also works if you have already set up this app yourself in advance.
You can set up additional logins for your MQTT devices and services using the Mosquitto app configuration.

Important

When MQTT is set up with the official Mosquitto MQTT broker app, the broker’s credentials are generated and kept secret. If the official Mosquitto MQTT broker needs to be re-installed, make sure you save a copy of the app user options, like the additional logins. After re-installing the app, the MQTT integration will automatically update the new password for the re-installed broker. It will then reconnect automatically.

Alternatively, you can use a different MQTT broker that you configure yourself, ensuring it is compatible with Home Assistant.

Setting up a broker

While public MQTT brokers are available, the easiest and most private option is running your own.

The recommended setup method is to use the Mosquitto MQTT broker app.

Warning

Neither ActiveMQ MQTT broker nor the RabbitMQ MQTT Plugin are supported, use a known working broker like Mosquitto instead.
There are at least two issues with the ActiveMQ MQTT broker which break MQTT message retention.

Broker configuration

MQTT broker settings are configured when the MQTT integration is first set up and can be changed later if needed.

Add the MQTT integration, then provide your broker’s hostname (or IP address) and port and (if required) the username and password that Home Assistant should use. To change the settings later, follow these steps:

Go to Settings > Devices & services.

Select the MQTT integration.

Reconfigure the MQTT broker settings via Settings > Devices & services, select , and select Reconfigure.

MQTT subentries can also be reconfigured. Additional entities can be added, or an entity can be removed from the sub entry. Each MQTT subentry holds one MQTT device. The MQTT device must have at least one entity.

Important

If you experience an error message like Failed to connect due to exception: [SSL: CERTIFICATE_VERIFY_FAILED] certificate verify failed, then check the collapse section Other settings and set Broker certificate validation to Auto.

Other broker configuration settings

Additional broker configuration options include setting a custom client ID, configuring a client certificate and key for authentication, and enabling TLS validation of the broker’s certificate to ensure a secure connection. To access these settings, open the MQTT broker settings, and expand Other settings.

Alternative client ID

You can set a custom MQTT client ID, this can help when debugging. Mind that the client ID must be unique. Leave this settings default if you want Home Assistant to generate a unique ID.

Keep alive

The time in seconds between sending keep alive messages for this client. The default is 60 seconds. The keep alive setting should be at least 15 seconds.

Broker certificate validation

To enable a secure connection to the broker, the broker certificate should be validated. If your broker uses a trusted certificate, then choose Auto. This validates the broker certificate against the bundled certificate authority (CA) certificates. If a self-signed certificate is used, select Custom. A custom PEM- or DER-encoded CA certificate can be uploaded. Select Next to show the control to upload the CA certificate.
If the server certificate does not match the hostname then validation will fail. To allow a connection without the verification of the hostname, turn the Ignore broker certificate validation switch on.

MQTT Protocol

Home Assistant requires that your broker supports MQTT protocol version 5. Most MQTT brokers, like the official “Mosquitto broker” App will support MQTT protocol version 5.

Securing the connection

With a secure broker connection, it is possible to use a client certificate for authentication. To set the client certificate and private key, turn on the option Use a client certificate and select Next to reveal file upload controls. A client certificate and the corresponding private key must be uploaded together. Both client certificate and private key must be either PEM- or DER-encoded. If the private key is encrypted with a password, ensure you supply the correct password when uploading the client certificate and key files.

Using WebSockets as transport

You can select websockets as the transport method if your MQTT broker supports it. When you select websockets and select Next, you will be able to add a WebSockets path (default is /) and WebSockets headers (optional). The target WebSockets URI ws://{broker}:{port}{ws_path} is built with the broker, port, and ws_path (WebSocket path) settings.
To configure the WebSocket’s headers, supply a valid JSON dictionary string. For example, { "Authorization": "token" , "x-header": "some header"}. The default transport method is tcp. The WebSockets transport can be secured using TLS and optionally using user credentials or a client certificate.

Note

A configured client certificate will only be active if broker certificate validation is enabled.

Configure MQTT options

To change the options, follow these steps:

Go to Settings > Devices & services.

Select the MQTT integration.

Select Configure, then Re-configure MQTT.

To open the MQTT options page, select Next.

Change MQTT discovery options

The MQTT discovery options can be changed by following these steps:

Go to Settings > Devices & services.

Find the MQTT integration and select it.

To open the MQTT discovery options page, select the Configure MQTT Options button.

Discovery options

MQTT discovery is enabled by default. Discovery can be turned off. The prefix for the discovery topic (default homeassistant) can be changed here as well.
See also MQTT Discovery section

Birth and last will messages

Home Assistant’s MQTT integration supports so-called Birth and Last Will and Testament (LWT) messages. The former is used to send a message after the service has started, and the latter is used to notify other clients about a disconnected client. Note that the LWT message will be sent both in case of a clean disconnect (for example, Home Assistant shutting down) and in case of an unclean disconnect (for example, Home Assistant crashing or losing its network connection).

If a disabled entity is enabled and added after 30 seconds, the MQTT integration will be reloaded and will cause all discovered MQTT entities to be unloaded.
When MQTT starts up, all existing MQTT devices, entities, tags, and device triggers, will be unavailable until a discovery message is received and processed. A device or service that exposes the MQTT discovery should subscribe to the Birth message and use this as a trigger to send the discovery payload. To avoid high IO loads on the MQTT broker, adding some random delay in sending the discovery payload is recommended.

Alternative approaches:

Retaining the discovery payload: This will store the discovery payload at the MQTT broker, and offer it to the MQTT integration as soon as it subscribes for MQTT discovery. When there are a lot of entities, this can cause high IO loads.

Periodically resending the discovery payload: This can cause some delay, or a lot of IO if there are a lot of MQTT discovery messages.

By default, Home Assistant sends online and offline to homeassistant/status.

MQTT Birth and Last Will messages can be customized or disabled from the UI. To do this, select Configure on the integration page in the UI, then Re-configure MQTT, and then Next.

Testing your setup

The mosquitto broker package ships command line tools (often as *-clients package) to send and receive MQTT messages. For sending test messages to a broker running on localhost, you can use mosquitto_pub, check the example below:

mosquitto_pub -h 127.0.0.1 -t homeassistant/switch/1/on -m "Switch is ON"

Another way to send MQTT messages manually is to use the MQTT integration in the frontend. Select Settings on the left menu, select Devices & services, and select Configure in the Mosquitto broker tile. Enter something similar to the example below into the topic field under Publish a packet and select Publish.

Go to Settings > Devices & services.

Select the Mosquitto broker integration, then select Configure.

Enter something similar to the example below into the topic field under Publish a packet. Select Publish.

homeassistant/switch/1/power

and in the Payload field

ON

In the Listen to a topic field, type # to see everything, or homeassistant/switch/# to just follow a published topic, then select Start listening. The messages should appear similar to the text below:

Message 23 received on homeassistant/switch/1/power/stat/POWER at 12:16 PM:
ON
QoS: 0 - Retain: false
Message 22 received on homeassistant/switch/1/power/stat/RESULT at 12:16 PM:
{
"POWER": "ON"
}
QoS: 0 - Retain: false

For reading all messages sent on the topic homeassistant to a broker running on localhost:

mosquitto_sub -h 127.0.0.1 -v -t "homeassistant/#"

Naming of MQTT Entities

For every configured MQTT entity Home Assistant automatically assigns a unique entity_id. If the unique_id option is configured, you can change the entity_id after creation, and the changes are stored in the Entity Registry. The entity_id is generated when an item is loaded the first time.

If the default_entity_id option is set, then this will be used to generate the entity_id.
If, for example, we have configured a sensor, and we have set default_entity_id to sensor.test, then Home Assistant will try to assign sensor.test as entity_id. If sensor.test already exists, Home Assistant will append a suffix to make it unique, for example, sensor.test_2.

This means any MQTT entity which is part of a device will automatically have its friendly_name attribute prefixed with the device name

Unnamed binary_sensor, button, number, and sensor entities will now be named by their device class instead of being named “MQTT binary sensor” and similar.
It’s allowed to set an MQTT entity’s name to None (use null in YAML) to mark it as the main feature of a device.

Note that on each MQTT entity, the has_entity_name attribute will be set to True. More details can be found here.

Grouping entities

If the MQTT entity represents a group of other entities, the member entities can be made visible in the UI by setting the list of unique IDs of the member entities in the group configuration option of the entity group:

group

A list of unique IDs of the member entities. Set this if the entity represents a group entity. Note that the member entities must be already configured before the member entities will become visible in the UI at the moment a group entity is loaded.

MQTT Discovery

The discovery of MQTT devices will enable one to use MQTT devices with only minimal configuration effort on the side of Home Assistant. The configuration is done on the device itself and the topic used by the device. Similar to the HTTP binary sensor and the HTTP sensor. To prevent multiple identical entries if a device reconnects, a unique identifier is necessary. Two parts are required on the device side: The configuration topic, and the device configuration as payload.

MQTT discovery is enabled by default and the subscriptions to perform the discovery are done at Quality of Service level 0 by default. The default prefix for the discovery topic is homeassistant.
To disable discovery, or change the discovery QoS or prefix, see the MQTT Options sections.

Note

Documentation on the MQTT components that support MQTT discovery can be found here.

Discovery messages

MQTT discovery supports two types of discovery messages:

Device discovery, which allows you to include several components in a single discovery message

Single component discovery, where you publish a separate discovery message for each component

If you use a device with multiple components, it is recommended to use MQTT device discovery. It reduces the number of messages sent, and allows you to send the device information only once.

Discovery topic

The discovery topic needs to follow a specific format:

<discovery_prefix>/<component>/[<node_id>/]<object_id>/config

<discovery_prefix>: The Discovery Prefix defaults to homeassistant and this prefix can be changed.

<component>: One of the supported MQTT integrations, for example, binary_sensor, or device in case of device discovery.

<node_id>: (Optional):  ID of the node providing the topic, this is not used by Home Assistant but may be used to structure the MQTT topic. The ID of the node must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).

<object_id>: The ID of the device. This allows for separate topics for each device. The ID of the device must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).

Note: The <object_id> in the topic does not influence the resulting entity_id; use default_entity_id if you need to control the entity_id.
The <node_id> is optional, and can be used by clients to subscribe to their own (command) topics by using one wildcard topic like <discovery_prefix>/+/<node_id>/+/set.

Best practice for entities with a unique_id is to set <object_id> to the unique_id and omit the <node_id>.

Device discovery payload

A device can send a discovery payload to expose all components for a device.
The <component> part in the discovery topic must be set to device.

As an alternative, it is also possible for a device to send a discovery payload for each component it wants to set up.

The shared options at the root level of the JSON message must include:

device mapping (abbreviated as dev)

origin mapping (abbreviated as o)

These mappings are mandatory and cannot be overridden at the entity/component level.

Supported shared options are:

The availability options.

The origin (required) options

command_topic

state_topic

qos

encoding

The component specific options are placed as mappings under the components key (abbreviated as cmps) like:

{
"dev": {
"ids": "ea334450945afc",
"name": "Kitchen",
"mf": "Bla electronics",
"mdl": "xya",
"sw": "1.0",
"sn": "ea334450945afc",
"hw": "1.0rev2"
},
"o": {
"name":"bla2mqtt",
"sw": "2.1",
"url": "https://bla2mqtt.example.com/support"
},
"cmps": {
"some_unique_component_id1": {
"p": "sensor",
"device_class":"temperature",
"unit_of_measurement":"°C",
"value_template":"{{ value_json.temperature}}",
"unique_id":"temp01ae_t"
},
"some_unique_id2": {
"p": "sensor",
"device_class":"humidity",
"unit_of_measurement":"%",
"value_template":"{{ value_json.humidity}}",
"unique_id":"temp01ae_h"
}
},
"state_topic":"sensorBedroom/state",
"qos": 2
}

Note

To see what each abbreviation stands for, refer to the list of supported abbreviations in MQTT discovery messages.

The components id’s under the components (cmps) key, are used as part of the discovery identification. A platform (p) config option is required for each component config that is added to identify the component platform. Also required is a unique_id for entity-based components.

To remove the components, publish an empty (retained) string payload to the discovery topic. This will remove the component and clear the published discovery payload. It will also remove the device entry if there are no further references to it.

An empty config can be published as an update to remove a single component from the device discovery. Note that adding the platform (p) option is still required.

{
"dev": {
"ids": "ea334450945afc",
"name": "Kitchen",
"mf": "Bla electronics",
"mdl": "xya",
"sw": "1.0",
"sn": "ea334450945afc",
"hw": "1.0rev2"
},
"o": {
"name":"bla2mqtt",
"sw": "2.1",
"url": "https://bla2mqtt.example.com/support"
},
"cmps": {
"some_unique_component_id1": {
"p": "sensor",
"device_class":"temperature",
"unit_of_measurement":"°C",
"value_template":"{{ value_json.temperature}}",
"unique_id":"temp01ae_t"
},
"some_unique_id2": {
"p": "sensor"
}
},
"state_topic":"sensorBedroom/state",
"qos": 2
}

This will explicitly remove the humidity sensor and its entry.

After removing a component, you should send another update with the removed component omitted from the configuration. This ensures that Home Assistant has the most up-to-date device configuration. For example:

{
"dev": {
"ids": "ea334450945afc",
"name": "Kitchen",
"mf": "Bla electronics",
"mdl": "xya",
"sw": "1.0",
"sn": "ea334450945afc",
"hw": "1.0rev2"
},
"o": {
"name":"bla2mqtt",
"sw": "2.1",
"url": "https://bla2mqtt.example.com/support"
},
"cmps": {
"some_unique_component_id1": {
"p": "sensor",
"device_class":"temperature",
"unit_of_measurement":"°C",
"value_template":"{{ value_json.temperature}}",
"unique_id":"temp01ae_t"
}
},
"state_topic":"sensorBedroom/state",
"qos": 2
}

A component config part in a device discovery payload must have the platform (p) option set with the name of the component and also must have at least one component specific config option. Entity components must have set the unique_id option and have a device context.

Migration from single component to device discovery

To allow a smooth migration from single component discovery to device discovery:

Ensure all entities have a unique_id and a device context.

Move the object_id inside the discovery payload, if that is available, or use a unique ID or the component.

Consider using the previous node_id as the new object_id of the device discovery topic.

Ensure the unique_id matches and the device context has the correct identifiers.

Send the following payload to all existing single component discovery topics: {"migrate_discovery": true }. This will unload the discovered item, but its settings will be retained.

Switch the discovery topic to the device discovery topic and include all the component configurations.

Clean up the single component discovery messages with an empty payload.

During the migration steps, INFO messages will be logged to inform you about the progress of the migration.

Important

Consider testing the migration process in a non-production environment before applying it to a live system.

Discovery migration example with a device automation and a sensor

Step 1: Original single component discovery configurations:

Discovery topic single: homeassistant/device_automation/0AFFD2/bla1/config
Discovery id: 0AFFD2 bla1 (both 0AFFD2 and bla1 from the discovery topic)
Discovery payload single:

{
"device": {
"identifiers": ["0AFFD2"],
"name": "Test device"
},
"o": {
"name": "foobar"
},
"automation_type": "trigger",
"payload": "short_press",
"topic": "foobar/triggers/button1",
"type": "button_short_press",
"subtype": "button_1"
}

Discovery topic single: homeassistant/sensor/0AFFD2/bla2/config
Discovery id: 0AFFD2 bla2 (both 0AFFD2 and bla2 from the discovery topic)
Discovery payload single:

{
"device": {
"identifiers": ["0AFFD2"],
"name": "Test device"
},
"o": {
"name": "foobar"
},
"state_topic": "foobar/sensor/sensor1",
"unique_id": "bla_sensor001"
}

Step 2: Initiate migration by publishing to both discovery topics:

When these single component discovery payloads are processed, and we want to initiate migration to device discovery, we need to publish …

{"migrate_discovery": true }

… to both discovery topics …

homeassistant/device_automation/0AFFD2/bla1/config

homeassistant/sensor/0AFFD2/bla2/config

Important

Check the logs to ensure this step is executed correctly.

Step 3: Publish the new device discovery configuration:

Discovery topic device: homeassistant/device/0AFFD2/config
Discovery id: 0AFFD2 bla (0AFFD2from discovery topic, bla: The key under cmps in the discovery payload)
Discovery payload device:

{
"device": {
"identifiers": [
"0AFFD2"
]
},
"o": {
"name": "foobar"
},
"cmps": {
"bla1": {
"p": "device_automation",
"automation_type": "trigger",
"payload": "short_press",
"topic": "foobar/triggers/button1",
"type": "button_short_press",
"subtype": "button_1"
},
"bla2": {
"p": "sensor",
"state_topic": "foobar/sensor/sensor1",
"unique_id": "bla_sensor001"
}
}
}

Important

Check the logs to ensure the migration was successful.

Step 4: Clean up after successful migration:

After the logs show a successful migration, the single component discovery topics can be cleaned up safely by publishing an empty payload to them.
The logs should indicate if the discovery migration was successful.

Optional: Rolling back the migration:

To rollback publish …

{"migrate_discovery": true }

To the device discovery topic(s).
After that, re-publish the single component discovery payloads.
At last, clean up the device discovery payloads by publishing an empty payload.

Check the logs for every step.

Single component discovery payload

When using single component discovery messages, the <component> part in the discovery topic must be one of the supported MQTT platforms.

The options in the payload are only used to set up one specific component. If a device contains multiple components, it is recommended to use device discovery instead.

MQTT entities can share device configuration, meaning one entity can include the full device configuration and other entities can link to that device by only setting mandatory fields.

Important

The mandatory fields were previously limited to at least one of connection and identifiers, but have now been extended to at least one of connection and identifiers as well as the name.

Example discovery payload:

{
"dev": {
"ids": "ea334450945afc",
"name": "Kitchen",
"mf": "Bla electronics",
"mdl": "xya",
"sw": "1.0",
"sn": "ea334450945afc",
"hw": "1.0rev2"
},
"o": {
"name":"bla2mqtt",
"sw": "2.1",
"url": "https://bla2mqtt.example.com/support"
},
"device_class":"temperature",
"unit_of_measurement":"°C",
"value_template":"{{ value_json.temperature}}",
"unique_id":"temp01ae_t",
"state_topic":"sensorBedroom/state",
"qos": 2
}

To remove the component, publish an empty string to the discovery topic. This will remove the component and clear the published discovery payload. It will also remove the device entry if there are no further references to it.

For more examples see.

Discovery payload

The payload must be a serialized JSON dictionary and will be checked like an entry in your configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] file if a new device is added, with the exception that unknown configuration keys are allowed but ignored. This means that missing variables will be filled with the integration’s default values. All configuration variables which are required must be present in the payload. The reason for allowing unknown documentation keys is allow some backwards compatibility, software generating MQTT discovery messages can then be used with older Home Assistant versions which will simply ignore new features.

A discovery payload can be sent with a retain flag set. In that case, the discovery message will be stored at the MQTT broker and processed automatically when the MQTT integrations start. This method removes the need for it to be resent. A better approach, though, is for the software generating MQTT discovery messages to send discovery payload(s) when the MQTT integration sends the Birth message.

Subsequent messages on a topic where a valid payload has been received will be handled as a configuration update, and a configuration update with an empty payload will cause a previously discovered device to be deleted.

A base topic ~ may be defined in the payload to conserve memory when the same topic base is used multiple times.
In the value of configuration variables ending with _topic, ~ will be replaced with the base topic, if the ~ occurs at the beginning or end of the value.

Configuration variable names in the discovery payload may be abbreviated to conserve memory when sending a discovery message from memory constrained devices.

It is recommended to add information about the origin of MQTT entities by including the origin option (abbreviated as o) in the discovery payload. For device discovery, this information is required. The origin details will be logged in the core event log when an item is discovered or updated. Adding origin information helps with troubleshooting and provides valuable context about the source of MQTT messages in your Home Assistant setup.

Note: These options also support abbreviations, as shown in the table below.

name

The name of the application that is the origin of the discovered MQTT item. (Required)

sw_version

Software version of the application that supplies the discovered MQTT item.

support_url

Support URL of the application that supplies the discovered MQTT item.

Supported abbreviations in MQTT discovery messages

Supported abbreviations

'act_t':               'action_topic',
'act_tpl':             'action_template',
'atype':               'automation_type',
'aux_cmd_t':           'aux_command_topic',
'aux_stat_t':          'aux_state_topic',
'aux_stat_tpl':        'aux_state_template',
'av_tones':            'available_tones',
'avty':                'availability',
'avty_mode':           'availability_mode',
'avty_t':              'availability_topic',
'avty_tpl':            'availability_template',
'away_mode_cmd_t':     'away_mode_command_topic',
'away_mode_stat_t':    'away_mode_state_topic',
'away_mode_stat_tpl':  'away_mode_state_template',
'b_tpl':               'blue_template',
'bri_cmd_t':           'brightness_command_topic',
'bri_cmd_tpl':         'brightness_command_template',
'bri_scl':             'brightness_scale',
'bri_stat_t':          'brightness_state_topic',
'bri_tpl':             'brightness_template',
'bri_val_tpl':         'brightness_value_template',
'clr_temp_cmd_tpl':    'color_temp_command_template',
'clr_temp_cmd_t':      'color_temp_command_topic',
'clr_temp_k':           'color_temp_kelvin',
'clr_temp_stat_t':     'color_temp_state_topic',
'clr_temp_tpl':        'color_temp_template',
'clr_temp_val_tpl':    'color_temp_value_template',
'clrm_stat_t':         'color_mode_state_topic',
'clrm_val_tpl':        'color_mode_value_template',
'cmd_off_tpl':         'command_off_template',
'cmd_on_tpl':          'command_on_template',
'cmd_t':               'command_topic',
'cmd_tpl':             'command_template',
'cmps':                'components',
'cod_arm_req':         'code_arm_required',
'cod_dis_req':         'code_disarm_required',
'cod_trig_req':        'code_trigger_required',
'cont_type':           'content_type',
'curr_temp_t':         'current_temperature_topic',
'curr_temp_tpl':       'current_temperature_template',
'def_ent_id':          'default_entity_id',
'dev':                 'device',
'dev_cla':             'device_class',
'dir_cmd_t':           'direction_command_topic',
'dir_cmd_tpl':         'direction_command_template',
'dir_stat_t':          'direction_state_topic',
'dir_val_tpl':         'direction_value_template',
'dsp_prc':             'display_precision',
'e':                   'encoding',
'en':                  'enabled_by_default',
'ent_cat':             'entity_category',
'ent_pic':             'entity_picture',
'evt_typ':             'event_types',
'exp_aft':             'expire_after',
'fanspd_lst':          'fan_speed_list',
'flsh':                'flash',
'flsh_tlng':           'flash_time_long',
'flsh_tsht':           'flash_time_short',
'fx_cmd_t':            'effect_command_topic',
'fx_cmd_tpl':          'effect_command_template',
'fx_list':             'effect_list',
'fx_stat_t':           'effect_state_topic',
'fx_tpl':              'effect_template',
'fx_val_tpl':          'effect_value_template',
'fan_mode_cmd_t':      'fan_mode_command_topic',
'fan_mode_cmd_tpl':    'fan_mode_command_template',
'fan_mode_stat_t':     'fan_mode_state_topic',
'fan_mode_stat_tpl':   'fan_mode_state_template',
'frc_upd':             'force_update',
'g_tpl':               'green_template',
'grp':                 'group',
'hs_cmd_t':            'hs_command_topic',
'hs_cmd_tpl':          'hs_command_template',
'hs_stat_t':           'hs_state_topic',
'hs_val_tpl':          'hs_value_template',
'ic':                  'icon',
'img_e':               'image_encoding',
'img_t':               'image_topic',
'init':                'initial',
'hum_cmd_t':           'target_humidity_command_topic',
'hum_cmd_tpl':         'target_humidity_command_template',
'hum_stat_t':          'target_humidity_state_topic',
'hum_state_tpl':       'target_humidity_state_template',
'json_attr':           'json_attributes',
'json_attr_t':         'json_attributes_topic',
'json_attr_tpl':       'json_attributes_template',
'l_ver_t':             'latest_version_topic',
'l_ver_tpl':           'latest_version_template',
'lrst_t':              'last_reset_topic',
'lrst_val_tpl':        'last_reset_value_template',
'max':                 'max',
'max_hum':             'max_humidity',
'max_k':               'max_kelvin',
'max_mirs':            'max_mireds',
'max_temp':            'max_temp',
'migr_discvry':        'migrate_discovery',
'min':                 'min',
'min_hum':             'min_humidity',
'min_k':               'min_kelvin',
'min_mirs':            'min_mireds',
'min_temp':            'min_temp',
'mode':                'mode',
'mode_cmd_t':          'mode_command_topic',
'mode_cmd_tpl':        'mode_command_template',
'mode_stat_t':         'mode_state_topic',
'mode_stat_tpl':       'mode_state_template',
'modes':               'modes',
'msg_exp_int':         'message_expiry_interval',
'name':                'name',
'o':                   'origin',
'off_dly':             'off_delay',
'on_cmd_type':         'on_command_type',
'ops':                 'options',
'opt':                 'optimistic',
'osc_cmd_t':           'oscillation_command_topic',
'osc_cmd_tpl':         'oscillation_command_template',
'osc_stat_t':          'oscillation_state_topic',
'osc_val_tpl':         'oscillation_value_template',
'p':                   'platform',
'pct_cmd_t':           'percentage_command_topic',
'pct_cmd_tpl':         'percentage_command_template',
'pct_stat_t':          'percentage_state_topic',
'pct_val_tpl':         'percentage_value_template',
'pl':                  'payload',
'pl_arm_away':         'payload_arm_away',
'pl_arm_custom_b':     'payload_arm_custom_bypass',
'pl_arm_home':         'payload_arm_home',
'pl_arm_nite':         'payload_arm_night',
'pl_arm_vacation':     'payload_arm_vacation',
'pl_avail':            'payload_available',
'pl_cln_sp':           'payload_clean_spot',
'pl_cls':              'payload_close',
'pl_dir_fwd':          'payload_direction_forward',
'pl_dir_rev':          'payload_direction_reverse',
'pl_disarm':           'payload_disarm',
'pl_home':             'payload_home',
'pl_inst':             'payload_install',
'pl_loc':              'payload_locate',
'pl_lock':             'payload_lock',
'pl_not_avail':        'payload_not_available',
'pl_not_home':         'payload_not_home',
'pl_off':              'payload_off',
'pl_on':               'payload_on',
'pl_open':             'payload_open',
'pl_osc_off':          'payload_oscillation_off',
'pl_osc_on':           'payload_oscillation_on',
'pl_paus':             'payload_pause',
'pl_prs':              'payload_press',
'pl_ret':              'payload_return_to_base',
'pl_rst':              'payload_reset',
'pl_rst_hum':          'payload_reset_humidity',
'pl_rst_mode':         'payload_reset_mode',
'pl_rst_pct':          'payload_reset_percentage',
'pl_rst_pr_mode':      'payload_reset_preset_mode',
'pl_stop':             'payload_stop',
'pl_stop_tilt':        'payload_stop_tilt',
'pl_stpa':             'payload_start_pause',
'pl_strt':             'payload_start',
'pl_toff':             'payload_turn_off',
'pl_ton':              'payload_turn_on',
'pl_trig':             'payload_trigger',
'pl_unlk':             'payload_unlock',
'pos':                 'reports_position',
'pos_clsd':            'position_closed',
'pos_open':            'position_open',
'pr_mode_cmd_t':       'preset_mode_command_topic',
'pr_mode_cmd_tpl':     'preset_mode_command_template',
'pr_mode_stat_t':      'preset_mode_state_topic',
'pr_mode_val_tpl':     'preset_mode_value_template',
'pr_modes':            'preset_modes',
'ptrn':                'pattern',
'r_tpl':               'red_template',
'rel_s':               'release_summary',
'rel_u':               'release_url',
'ret':                 'retain',
'rgb_cmd_t':           'rgb_command_topic',
'rgb_cmd_tpl':         'rgb_command_template',
'rgb_stat_t':          'rgb_state_topic',
'rgb_val_tpl':         'rgb_value_template',
'rgbw_cmd_t':          'rgbw_command_topic',
'rgbw_cmd_tpl':        'rgbw_command_template',
'rgbw_stat_t':         'rgbw_state_topic',
'rgbw_val_tpl':        'rgbw_value_template',
'rgbww_cmd_t':         'rgbww_command_topic',
'rgbww_cmd_tpl':       'rgbww_command_template',
'rgbww_stat_t':        'rgbww_state_topic',
'rgbww_val_tpl':       'rgbww_value_template',
'send_cmd_t':          'send_command_topic',
'send_if_off':         'send_if_off',
'set_fan_spd_t':       'set_fan_speed_topic',
'set_pos_t':           'set_position_topic',
'set_pos_tpl':         'set_position_template',
'pos_t':               'position_topic',
'pos_tpl':             'position_template',
'spd_rng_min':         'speed_range_min',
'spd_rng_max':         'speed_range_max',
'src_type':            'source_type',
'stat_cla':            'state_class',
'stat_closing':        'state_closing',
'stat_clsd':           'state_closed',
'stat_jam':            'state_jammed',
'stat_locked':         'state_locked',
'stat_locking':        'state_locking',
'stat_off':            'state_off',
'stat_on':             'state_on',
'stat_open':           'state_open',
'stat_opening':        'state_opening',
'stat_stopped':        'state_stopped',
'stat_unlocked':       'state_unlocked',
'stat_unlocking':      'state_unlocking',
'stat_t':              'state_topic',
'stat_tpl':            'state_template',
'stat_val_tpl':        'state_value_template',
'step':                'step',
'stype':               'subtype',
'sug_dsp_prc':         'suggested_display_precision',
'sup_clrm':            'supported_color_modes',
'sup_dur':             'support_duration',
'sup_vol':             'support_volume_set',
'sup_feat':            'supported_features',
'swing_mode_cmd_t':    'swing_mode_command_topic',
'swing_mode_cmd_tpl':  'swing_mode_command_template',
'swing_mode_stat_t':   'swing_mode_state_topic',
'swing_mode_stat_tpl': 'swing_mode_state_template',
't':                   'topic',
'temp_cmd_t':          'temperature_command_topic',
'temp_cmd_tpl':        'temperature_command_template',
'temp_hi_cmd_t':       'temperature_high_command_topic',
'temp_hi_cmd_tpl':     'temperature_high_command_template',
'temp_hi_stat_t':      'temperature_high_state_topic',
'temp_hi_stat_tpl':    'temperature_high_state_template',
'temp_lo_cmd_t':       'temperature_low_command_topic',
'temp_lo_cmd_tpl':     'temperature_low_command_template',
'temp_lo_stat_t':      'temperature_low_state_topic',
'temp_lo_stat_tpl':    'temperature_low_state_template',
'temp_stat_t':         'temperature_state_topic',
'temp_stat_tpl':       'temperature_state_template',
'temp_unit':           'temperature_unit',
'tilt_clsd_val':       'tilt_closed_value',
'tilt_cmd_t':          'tilt_command_topic',
'tilt_cmd_tpl':        'tilt_command_template',
'tilt_max':            'tilt_max',
'tilt_min':            'tilt_min',
'tilt_opnd_val':       'tilt_opened_value',
'tilt_opt':            'tilt_optimistic',
'tilt_status_t':       'tilt_status_topic',
'tilt_status_tpl':     'tilt_status_template',
'tit':                 'title',
'trns':                'transition',
'tz':                  'timezone',
'uniq_id':             'unique_id',
'unit_of_meas':        'unit_of_measurement',
'url_t':               'url_topic',
'url_tpl':             'url_template',
'val_tpl':             'value_template',
'vis':                 'visible_by_default',
'whit_cmd_t':          'white_command_topic',
'whit_scl':            'white_scale',
'xy_cmd_t':            'xy_command_topic',
'xy_cmd_tpl':          'xy_command_template',
'xy_stat_t':           'xy_state_topic',
'xy_val_tpl':          'xy_value_template',

Supported abbreviations for device registry configuration

'cu':                  'configuration_url',
'cns':                 'connections',
'ids':                 'identifiers',
'name':                'name',
'mf':                  'manufacturer',
'mdl':                 'model',
'mdl_id':              'model_id',
'hw':                  'hw_version',
'sw':                  'sw_version',
'sa':                  'suggested_area',
'sn':                  'serial_number',

Supported abbreviations for origin info

'name':                'name',
'sw':                  'sw_version',
'url':                 'support_url',

Discovery messages and availability

When MQTT discovery is set up, and a device or service sends a discovery message,
an MQTT entity, tag, or device automation will be set up directly after receiving the message.
When Home Assistant is restarting, discovered MQTT items with a unique ID will be unavailable until a new
discovery message is received. MQTT items without a unique ID will not be added at startup.
So a device or service using MQTT discovery must make sure a configuration message is offered
after the MQTT integration has been (re)started. There are two common approaches to make sure the
discovered items are set up at startup:

Using Birth and Will messages to trigger setup

Using retained messages

Finally, it is a best practice to publish your device or service availability status.

Use the Birth and Will messages to trigger discovery

When the MQTT integration starts, a birth message is published at homeassistant/status by default.
A device or service connected to the shared mqtt broker can subscribe to this topic and use an online message
to trigger discovery messages. See also the birth and last will messages
section. After the configs have been published, the state topics will need an update, so they need to be republished.

Using retained config messages

An alternative method for a device or service is to publish discovery messages with a retain flag. This will make sure
discovery messages are replayed when the MQTT integration connects to the broker.
After the configs have been published, the state topics will need an update.

Using retained state messages

State updates also need to be re-published after a config as been processed.
This can also be done by publishing “retained” messages. As soon as a config is received (or replayed from a retained message),
the setup will subscribe any state topics. If a retained message is available at a state topic,
this message will be replayed so that the state can be restored for this topic.

Warning

A disadvantage of using retained messages is that these messages retain at the broker,
even when the device or service stops working. They are retained even after the system or broker has been restarted.
Retained messages can create ghost entities that keep coming back.

Especially when you have many entities, (unneeded) discovery messages can cause excessive system load. For this reason, use discovery messages with caution.

Using the Message Expiry Interval option

To prevent “retained” messages being kept forever, the publisher can set the Message Expiry Interval option. This will tell the broker to keep messages for a limited time. Home Assistant can set the Message Expiry Interval for an MQTT device via the message_expiry_interval configuration option. This will set the Message Expiry Interval for command payloads published to control the device.

Using Availability topics

A device or service can announce its availability by publishing a Birth message and set a Will message at the broker.
When the device or service loses connection to the broker, the broker will publish the Will message.
This allows the MQTT integration to make an entity unavailable.

Platform specific availability settings are available for mqtt entity platforms only.

Platform specific availability settings

Configuration Variables

Looking for your configuration file?

availability list (Optional)

A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with availability_topic.

payload_available string (Optional, default: online)

The payload that represents the available state.

payload_not_available string (Optional, default: offline)

The payload that represents the unavailable state.

topic string Required

An MQTT topic subscribed to receive availability (online/offline) updates.

value_template template (Optional)

Defines a template to extract a device’s availability from the topic. To determine the device’s availability, the result of this template will be compared to payload_available and payload_not_available.

availability_topic string (Optional)

The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with availability.

availability_mode string (Optional, default: latest)

When availability is configured, this controls the conditions needed to set the entity to available. Valid entries are all, any, and latest. If set to all, payload_available must be received on all configured availability topics before the entity is marked as online. If set to any, payload_available must be received on at least one configured availability topic before the entity is marked as online. If set to latest, the last payload_available or payload_not_available received on any configured availability topic controls the availability.

availability_template template (Optional)

Defines a template to extract device’s availability from the availability_topic. To determine the devices’s availability result of this template will be compared to payload_available and payload_not_available.

payload_available string (Optional, default: online)

The payload that represents the available state.

payload_not_available string (Optional, default: offline)

The payload that represents the unavailable state.

Discovery examples with component discovery

Motion detection (binary sensor)

A motion detection device which can be represented by a binary sensor for your garden would send its configuration as JSON payload to the Configuration topic. After the first message to config, then the MQTT messages sent to the state topic will update the state in Home Assistant.

Configuration topic: homeassistant/binary_sensor/garden/config

State topic: homeassistant/binary_sensor/garden/state

Configuration payload with derived device name:

{
"name":null,
"device_class":"motion",
"state_topic":"homeassistant/binary_sensor/garden/state",
"unique_id":"motion01ad",
"device":{
"identifiers":[
"01ad"
],
"name":"Garden"
}
}

Retain: The -r switch is added to retain the configuration topic in the broker. Without this, the sensor will not be available after Home Assistant restarts.

It is also a good idea to add a unique_id to allow changes to the entity and a device mapping so we can group all sensors of a device together. We can set “name” to null if we want to inherit the device name for the entity. If we set an entity name, the friendly_name will be a combination of the device and entity name. If name is left away and a device_class is set, the entity name part will be derived from the device_class.

Example configuration payload with no name set and derived device_class name:

{
"name":null,
"device_class":"motion",
"state_topic":"homeassistant/binary_sensor/garden/state",
"unique_id":"motion01ad",
"device":{
"identifiers":[
"01ad"
],
"name":"Garden"
}
}

If no name is set, a default name will be set by MQTT (see the MQTT platform documentation).

To create a new sensor manually and with the name set to null to derive the device name “Garden”:

mosquitto_pub -r -h 127.0.0.1 -p 1883 -t "homeassistant/binary_sensor/garden/config" -m '{"name": null, "device_class": "motion", "state_topic": "homeassistant/binary_sensor/garden/state", "unique_id": "motion01ad", "device": {"identifiers": ["01ad"], "name": "Garden" }}'

Update the state:

mosquitto_pub -h 127.0.0.1 -p 1883 -t "homeassistant/binary_sensor/garden/state" -m ON

Delete the sensor by sending an empty message.

mosquitto_pub -h 127.0.0.1 -p 1883 -t "homeassistant/binary_sensor/garden/config" -m ''

For more details, refer to the MQTT testing section.

Sensors

Setting up a sensor with multiple measurement values requires multiple consecutive configuration topic submissions.

Configuration topic no1: homeassistant/sensor/sensorBedroomT/config

Configuration payload no1:

{
"device_class":"temperature",
"state_topic":"homeassistant/sensor/sensorBedroom/state",
"unit_of_measurement":"°C",
"value_template":"{{ value_json.temperature}}",
"unique_id":"temp01ae",
"device":{
"identifiers":[
"bedroom01ae"
],
"name":"Bedroom",
"manufacturer": "Example sensors Ltd.",
"model": "Example Sensor",
"model_id": "K9",
"serial_number": "12AE3010545",
"hw_version": "1.01a",
"sw_version": "2024.1.0",
"configuration_url": "https://example.com/sensor_portal/config"
}
}

Configuration topic no2: homeassistant/sensor/sensorBedroomH/config

Configuration payload no2:

{
"device_class":"humidity",
"state_topic":"homeassistant/sensor/sensorBedroom/state",
"unit_of_measurement":"%",
"value_template":"{{ value_json.humidity}}",
"unique_id":"hum01ae",
"device":{
"identifiers":[
"bedroom01ae"
]
}
}

The sensor identifiers or connections option allows you to set up multiple entities that share the same device.

Note

If a device configuration is shared, then it is not needed to add all device details to the other entity configs. It is enough to add shared identifiers or connections to the device mapping for the other entity config payloads.

A common state payload that can be parsed with the value_template in the sensor configs:

{
"temperature":23.20,
"humidity":43.70
}

Entities with command topics

Setting up a light or switch is similar but requires a command_topic as mentioned in the MQTT switch documentation.

Configuration topic: homeassistant/switch/irrigation/config

State topic: homeassistant/switch/irrigation/state

Command topic: homeassistant/switch/irrigation/set

Payload:

{
"name":"Irrigation",
"command_topic":"homeassistant/switch/irrigation/set",
"state_topic":"homeassistant/switch/irrigation/state",
"unique_id":"irr01ad",
"device":{
"identifiers":[
"garden01ad"
],
"name":"Garden"
}
}

Retain: The -r switch is added to retain the configuration topic in the broker. Without this, the sensor will not be available after Home Assistant restarts.

mosquitto_pub -r -h 127.0.0.1 -p 1883 -t "homeassistant/switch/irrigation/config" \
-m '{"name": "Irrigation", "command_topic": "homeassistant/switch/irrigation/set", "state_topic": "homeassistant/switch/irrigation/state", "unique_id": "irr01ad", "device": {"identifiers": ["garden01ad"], "name": "Garden" }}'

Set the state:

mosquitto_pub -h 127.0.0.1 -p 1883 -t "homeassistant/switch/irrigation/set" -m ON

Using abbreviations and base topic

Setting up a switch using topic prefix and abbreviated configuration variable names to reduce payload length.

Configuration topic: homeassistant/switch/irrigation/config

Command topic: homeassistant/switch/irrigation/set

State topic: homeassistant/switch/irrigation/state

Configuration payload:

{
"~":"homeassistant/switch/irrigation",
"name":"garden",
"cmd_t":"~/set",
"stat_t":"~/state"
}

Note

To see what each abbreviation stands for, refer to the list of supported abbreviations in MQTT discovery messages.

Another example using abbreviations topic name and base topic

Setting up a light that takes JSON payloads, with abbreviated configuration variable names:

Configuration topic: homeassistant/light/kitchen/config

Command topic: homeassistant/light/kitchen/set

State topic: homeassistant/light/kitchen/state

Example state payload: {"state": "ON", "brightness": 255}

Configuration payload:

{
"~": "homeassistant/light/kitchen",
"name": "Kitchen",
"uniq_id": "kitchen_light",
"cmd_t": "~/set",
"stat_t": "~/state",
"schema": "json",
"brightness": true
}

Example with using abbreviated Device and Origin info in discovery messages

{
"~": "homeassistant/light/kitchen",
"name": null,
"uniq_id": "kitchen_light",
"cmd_t": "~/set",
"stat_t": "~/state",
"schema": "json",
"dev": {
"ids": "ea334450945afc",
"name": "Kitchen",
"mf": "Bla electronics",
"mdl": "xya",
"mdl_id": "ABC123",
"sw": "1.0",
"sn": "ea334450945afc",
"hw": "1.0rev2"
},
"o": {
"name":"bla2mqtt",
"sw": "2.1",
"url": "https://bla2mqtt.example.com/support"
}
}

Support by third-party tools

The following software has built-in support for MQTT discovery:

anpr2mqtt

ArduinoHA

Arilux AL-LC0X LED controllers

ble2mqtt

diematic_server

digitalstrom-mqtt

ebusd

ecowitt2mqtt

EMS-ESP32 (and EMS-ESP)

ESPHome

ESPurna

go-iotdevice

HASS.Agent

IOTLink (starting with 2.0.0)

MiFlora MQTT Daemon

MyElectricalData

MqDockerUp

Nuki Hub

Nuki Smart Lock 3.0 Pro, more info

OpenMQTTGateway

OTGateway

rethink

room-assistant (starting with 1.1.0)

SmartHome

SpeedTest-CLI MQTT

SwitchBot-MQTT-BLE-ESP32

Tasmota (starting with 5.11.1e, development halted)

TeddyCloud

Teleinfo MQTT (starting with 3.0.0)

Tydom2MQTT

Updates2MQTT

What’s up Docker? (starting with 3.5.0)

WyzeSense2MQTT

Xiaomi DaFang Hacks

Zehnder Comfoair RS232 MQTT

Zigbee2MQTT

The following software also supports consuming MQTT discovery information that is intended for Home Assistant.
Compatibility and features will vary, and not all devices may work.

Domoticz

openHAB

Manual configured MQTT items

Support to allow adding manual items as a subentry via a config flow, is work in progress. Not all entity platforms are supported yet.

For most integrations, it is also possible to manually set up MQTT items in configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more]. Read more about configuration in YAML.

MQTT supports two styles for configuring items in YAML. All configuration items are placed directly under the mqtt integration key. Note that you cannot mix these styles. Use the YAML configuration listed per item style when in doubt.

YAML configuration listed per item

This method expects all items to be in a YAML list. Each item has a {domain} key and the item config is placed directly under the domain key. This method is considered as best practice. In all the examples we use this format.

mqtt:
- {domain}:
name: ""
...
- {domain}:
name: ""
...

YAML configuration keyed and bundled by {domain}

All items are grouped per {domain} and where all configs are listed.

mqtt:
{domain}:
- name: ""
...
- name: ""
...

If you have many manually configured items, you might want to consider splitting up the configuration.

Note

Documentation on the MQTT components that support YAML can be found here.

Entity state updates

Entities receive state updates via MQTT subscriptions. The payloads received on the state topics are processed to determine whether there is a significant change. If a change is detected, the entity will be updated.

Note that MQTT device payloads often contain information for updating multiple entities that subscribe to the same topics. For example, a light status update might include information about link quality. This data can update a link quality sensor but is not used to update the light itself. MQTT filters out entity state updates when there are no changes.

The last reported state attribute

Because MQTT state updates are often repeated frequently, even when no actual changes exist, it is up to the MQTT subscriber to determine whether a status update was received. If the latest update is missed, it might take some time before the next one arrives. If a retained payload exists at the broker, that value will be replayed first, but it will be an update of a previous last state.

MQTT devices often continuously generate numerous state updates. MQTT does not update last_reported to avoid impacting system stability unless force_update is set. Alternatively, an MQTT sensor can be created to measure the last update.

Using Templates

The MQTT integration supports templating. Read more about using templates with the MQTT integration.

Examples

REST API

Using the REST API to send a message to a given topic.

$ curl -X POST \
-H "Authorization: Bearer ABCDEFGH" \
-H "Content-Type: application/json" \
-d '{"payload": "Test message from HA", "topic": "home/notification"}' \
http://IP_ADDRESS:8123/api/services/mqtt/publish

Automations

Use as script in automations.

automation:
alias: "Send me a message when I get home"
triggers:
- trigger: state
entity_id: device_tracker.me
to: "home"
actions:
- action: script.notify_mqtt
data:
target: "me"
message: "I'm home"

script:
notify_mqtt:
sequence:
- action: mqtt.publish
data:
payload: "{{ message }}"
topic: "home/{{ target }}"
retain: true

List of actions

The MQTT integrationIntegrations connect and integrate Home Assistant with your devices, services, and more. [Learn more] provides the following actions. Each link below opens a dedicated page with examples, parameters, and a step-by-step UI walkthrough.

Export MQTT messages (mqtt.dump)
Writes all messages on a topic to a file for debugging.

Publish an MQTT message (mqtt.publish)
Publishes a message to an MQTT topic.

Reload MQTT entities (mqtt.reload)
Reloads MQTT entities from your YAML configuration.

For an overview of every action across all integrations, see the actions reference.

Logging

The logger integration allows the logging of received MQTT messages.

# Example configuration.yaml entry
logger:
default: warning
logs:
homeassistant.components.mqtt: debug

Event event_mqtt_reloaded

Event event_mqtt_reloaded is fired when Manually configured MQTT entities have been reloaded and entities thus might have changed.

This event has no additional data.

Removing the integration

Go to Settings > Devices & services and select the integration card.

From the list of devices, select the integration instance you want to remove.

Next to the entry, select the three dots  menu. Then, select Delete.

Note: This action does not remove the MQTT broker or its data. If you want to completely remove MQTT:

Check your configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] and other YAML files for MQTT-related configurations and remove them

Review your automations and scripts for any MQTT dependencies

Consider backing up your configuration before making these changes

Help us improve our documentation

Suggest an edit to this page, or provide/view feedback for this page.

Edit

Provide feedback

View pending feedback

The MQTT  service  was introduced in Home Assistant pre 0.7, and it's used by

48.6% of the active installations.

Its IoT class is Local Push.

🏆 Platinum quality

View source on GitHub

View known issues

View feature requests

Integration owners

We are incredibly grateful to the following contributors who currently maintain this integration:

@emontnemery
@jbouwh
@bdraco

Categories

Hub

Update

On this page

Configuration

Setting up a broker

Broker configuration

Other broker configuration settings

Configure MQTT options

Change MQTT discovery options

Discovery options

Birth and last will messages

Testing your setup

Naming of MQTT Entities

Grouping entities

MQTT Discovery

Discovery messages

Discovery messages and availability

Using Availability topics

Discovery examples with component discovery

Support by third-party tools

Manual configured MQTT items

YAML configuration listed per item

YAML configuration keyed and bundled by {domain}

Entity state updates

The last reported state attribute

Using Templates

Examples

List of actions

Logging

Event event_mqtt_reloaded

Removing the integration

Back to top

## Related

- [[mqtt-home-assistant-2]]
