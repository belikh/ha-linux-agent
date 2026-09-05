---
title: MQTT notify - Home Assistant
id: mqtt-notify-home-assistant
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-adopt-vs-build-honest-verdict
- adopt-vs-build
created: '2026-09-02T12:04:17.019874Z'
updated: '2026-09-02T17:37:22.487764Z'
source: https://www.home-assistant.io/integrations/notify.mqtt/
source_domain: www.home-assistant.io
fetched_at: '2026-09-02T12:04:17.018228Z'
fetch_provider: builtin
status: review
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'HA MQTT notify entity platform (introduced HA 2024.5): an MQTT device exposes
  a notify entity via MQTT discovery or YAML with a required command_topic; the notify.send_message
  action publishes the message payload to that topic, with optional command_template
  to generate the payload; config supports availability, qos, retain, encoding and
  message_expiry_interval — but the page defines NO title/message/actions payload
  schema and NO event round-trip, so there is no native actionable-notification (action-button)
  mechanism for MQTT notify entities.'
---

MQTT notify - Home Assistant

On this page

Configuration

Examples

Full configuration

MQTT notify

The MQTT notify integrationIntegrations connect and integrate Home Assistant with your devices, services, and more. [Learn more] lets you send an MQTT message when the send_message action is called. This can be used to expose a action of a remote device that allows processing a message, such as showing it on a screen.

Configuration

To use an MQTT notify entity in your installation, add an MQTT device as a subentry, or add the following to your configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] file.
After changing the configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] file, restart Home Assistant to apply the changes.

To use an MQTT notify entity in your installation, add the following to your configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] file.
After changing the configuration.yamlThe configuration.yaml file is the main configuration file for Home Assistant. It lists the integrations to be loaded and their specific configurations. In some cases, the configuration needs to be edited manually directly in the configuration.yaml file. Most integrations can be configured in the UI. [Learn more] file, restart Home Assistant to apply the changes.

# Example configuration.yaml entry
mqtt:
- notify:
command_topic: "home/living_room/status_screen/notifications"

Alternatively, you can set it up via MQTT discovery.

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

Defines a template to extract the device’s availability from the topic. To determine the device’s availability, the result of this template will be compared to payload_available and payload_not_available.

availability_mode string (Optional, default: latest)

When availability is configured, this controls the conditions needed to set the entity to available. Valid entries are all, any, and latest. If set to all, payload_available must be received on all configured availability topics before the entity is marked as online. If set to any, payload_available must be received on at least one configured availability topic before the entity is marked as online. If set to latest, the last payload_available or payload_not_available received on any configured availability topic controls the availability.

availability_template template (Optional)

Defines a template to extract the device’s availability from the availability_topic. To determine the device’s availability result, the template will be compared to payload_available and payload_not_available.

availability_topic string (Optional)

The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with availability.

command_template template (Optional)

Defines a template to generate the payload to send to command_topic.

command_topic string (Optional)

The MQTT topic to publish send message commands at.

default_entity_id string (Optional)

Use default_entity_id instead of name for automatic generation of the entity ID. For example, notify.foobar. When used without a unique_id, the entity ID will update during restart or reload if the entity ID is available. If the entity ID already exists, the entity ID will be created with a number at the end. When used with a unique_id, the default_entity_id is only used when the entity is added for the first time. When set, this overrides a user-customized entity ID if the entity was deleted and added again.

device map (Optional)

Information about the device this notify entity is a part of to tie it into the device registry. Only works when unique_id is set. At least one of the identifiers or connections must be present to identify the device.

configuration_url string (Optional)

A link to the webpage that can manage the configuration of this device. Can be either an http://, https:// or an internal homeassistant:// URL.

connections list (Optional)

A list of connections of the device to the outside world as a list of tuples [connection_type, connection_identifier]. For example, the MAC address of a network interface: "connections": [["mac", "02:5b:26:a8:dc:12"]].

hw_version string (Optional)

The hardware version of the device.

identifiers string | list (Optional)

A list of IDs that uniquely identify the device. For example, a serial number.

manufacturer string (Optional)

The manufacturer of the device.

model string (Optional)

The model of the device.

model_id string (Optional)

The model identifier of the device.

name string (Optional)

The name of the device.

serial_number string (Optional)

The serial number of the device.

suggested_area string (Optional)

Suggest an area if the device isn’t in one yet.

sw_version string (Optional)

The firmware version of the device.

via_device string (Optional)

Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.

enabled_by_default boolean (Optional, default: true)

Controls whether this entity is enabled by default. When set to true, the entity is enabled and usable immediately. Disabled entities are hidden by default until you enable them from the device page.

encoding string (Optional, default: utf-8)

The encoding of the published messages.

entity_category string (Optional)

The category of the entity.

entity_picture string (Optional)

Picture URL for the entity.

icon icon (Optional)

Icon for the entity.

json_attributes_template template (Optional)

Defines a template to extract the JSON dictionary from messages received on the json_attributes_topic. Usage example can be found in MQTT sensor documentation.

json_attributes_topic string (Optional)

The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in MQTT sensor documentation.

message_expiry_interval map (Optional)

Controls how long queued or retained messages sent from Home Assistant persist at the broker for offline subscribers. This option prevents the broker from retaining stale messages. The expected value for this option is a JSON mapping, for example, {"days": 1, "hours": 2, "minutes": 20, "seconds": 30} or {"seconds": 3600}.

days integer (Optional)

Number of days published messages are queued or retained for offline subscribers.

hours integer (Optional)

Number of hours published messages are queued or retained for offline subscribers.

minutes integer (Optional)

Number of minutes published messages are queued or retained for offline subscribers.

seconds integer (Optional)

Number of seconds published messages are queued or retained for offline subscribers.

name string (Optional, default: MQTT notify)

The name to use when displaying this notify entity. Can be set to null if only the device name is relevant.

payload_available string (Optional, default: online)

The payload that represents the available state.

payload_not_available string (Optional, default: offline)

The payload that represents the unavailable state.

qos integer (Optional, default: 0)

The maximum QoS level to be used when receiving and publishing messages.

retain boolean (Optional, default: false)

If the published message should have the retain flag on or not.

unique_id string (Optional)

An ID that uniquely identifies this notify entity. If two notify entities have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.

visible_by_default boolean (Optional, default: true)

Control whether this entity is visible by default. When set to false, the entity is hidden and does not appear on dashboards until you manually make it visible in its settings.

Important

Make sure that your topic matches exactly. some-topic/ and some-topic are different topics.

Examples

In this section, you will find some real-life examples of how to use this feature.

Full configuration

The example below shows a full configuration for a notify entity.

# Example configuration.yaml entry
mqtt:
- notify:
unique_id: living_room_stat_scr01
name: "Living room status screen"
command_topic: "home/living_room/status_screen/notifications"
availability:
- topic: "home/living_room/status_screen/available"
qos: 0
retain: false

Help us improve our documentation

Suggest an edit to this page, or provide/view feedback for this page.

Edit

Provide feedback

View pending feedback

The MQTT notify  integration  was introduced in Home Assistant 2024.5, and it's used by

48.6% of the active installations.

Its IoT class is Configurable.

View source on GitHub

View known issues

View feature requests

Integration owners

This integration is community maintained.

If you are a developer and would like to help, feel free to contribute!

Categories

Notifications

On this page

Configuration

Examples

Full configuration

Back to top