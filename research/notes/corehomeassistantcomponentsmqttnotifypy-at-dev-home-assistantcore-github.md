---
title: core/homeassistant/components/mqtt/notify.py at dev · home-assistant/core ·
  GitHub
id: corehomeassistantcomponentsmqttnotifypy-at-dev-home-assistantcore-github
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- mqtt-discovery
- source-code
- birth-message
- retained-messages
- gap-02
created: '2026-09-02T17:03:39.592126Z'
updated: '2026-09-02T17:39:22.157890Z'
source: https://github.com/home-assistant/core/blob/dev/homeassistant/components/mqtt/notify.py
source_domain: github.com
fetched_at: '2026-09-02T17:03:31.693279Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'HA core mqtt/notify.py platform implementation (dev, 88 lines): MqttEntity
  subclass of NotifyEntity wired via async_setup_entity_entry_helper; command_topic
  required, command_template supported to generate the published payload, retain/encoding
  configurable. Confirms the notify platform is a first-class MQTT entity platform
  inside the mqtt integration (not YAML-only), consistent with SUPPORTED_COMPONENTS
  containing ''notify''.'
---

core/homeassistant/components/mqtt/notify.py at dev · home-assistant/core · GitHub

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

home-assistant

/

core

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
38.5k

Star
90.2k

FilesExpand file tree

dev

/
notify.pyCopy path

Blame
More file actions

Blame
More file actions

Latest commit

HistoryHistory

History

88 lines (70 loc) · 2.77 KB

dev

/
notify.pyCopy pathTop

File metadata and controls

Code

Blame

88 lines (70 loc) · 2.77 KB

Raw
Copy raw file
Download raw file
Open symbols panel
Edit and raw actions

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88

"""Support for MQTT notify."""

from typing import override

import voluptuous as vol

from homeassistant.components import notify

from homeassistant.components.notify import NotifyEntity

from homeassistant.config_entries import ConfigEntry

from homeassistant.const import CONF_NAME

from homeassistant.core import HomeAssistant, callback

from homeassistant.helpers import config_validation as cv

from homeassistant.helpers.entity_platform import AddConfigEntryEntitiesCallback

from homeassistant.helpers.typing import ConfigType

from .config import DEFAULT_RETAIN, MQTT_BASE_SCHEMA

from .const import CONF_COMMAND_TEMPLATE, CONF_COMMAND_TOPIC, CONF_RETAIN

from .entity import MqttEntity, async_setup_entity_entry_helper

from .models import MqttCommandTemplate

from .schemas import MQTT_ENTITY_COMMON_SCHEMA

from .util import valid_publish_topic

PARALLEL_UPDATES = 0

DEFAULT_NAME = "MQTT notify"

PLATFORM_SCHEMA_MODERN = MQTT_BASE_SCHEMA.extend(

{

vol.Optional(CONF_COMMAND_TEMPLATE): cv.template,

vol.Required(CONF_COMMAND_TOPIC): valid_publish_topic,

vol.Optional(CONF_NAME): vol.Any(cv.string, None),

vol.Optional(CONF_RETAIN, default=DEFAULT_RETAIN): cv.boolean,

}

).extend(MQTT_ENTITY_COMMON_SCHEMA.schema)

DISCOVERY_SCHEMA = PLATFORM_SCHEMA_MODERN.extend({}, extra=vol.REMOVE_EXTRA)

async def async_setup_entry(

hass: HomeAssistant,

config_entry: ConfigEntry,

async_add_entities: AddConfigEntryEntitiesCallback,

) -> None:

"""Set up MQTT notify through YAML and through MQTT discovery."""

async_setup_entity_entry_helper(

hass,

config_entry,

MqttNotify,

notify.DOMAIN,

async_add_entities,

DISCOVERY_SCHEMA,

PLATFORM_SCHEMA_MODERN,

)

class MqttNotify(MqttEntity, NotifyEntity):

"""Notification entity that can send messages using MQTT."""

_default_name = DEFAULT_NAME

_entity_id_format = notify.ENTITY_ID_FORMAT

@staticmethod

@override

def config_schema() -> vol.Schema:

"""Return the config schema."""

return DISCOVERY_SCHEMA

@override

def _setup_from_config(self, config: ConfigType) -> None:

"""(Re)Setup the entity."""

self._command_template = MqttCommandTemplate(

config.get(CONF_COMMAND_TEMPLATE), entity=self

).async_render

@callback

@override

def _prepare_subscribe_topics(self) -> None:

"""(Re)Subscribe to topics."""

@override

async def _subscribe_topics(self) -> None:

"""(Re)Subscribe to topics."""

@override

async def async_send_message(self, message: str, title: str | None = None) -> None:

"""Send a message."""

payload = self._command_template(message)

await self.async_publish_with_config(self._config[CONF_COMMAND_TOPIC], payload)

You can’t perform that action at this time.
## Related

- [[test_notifypy]]
- [[corehomeassistantcomponentsmqttconstpy-at-dev-home-assistantcore-github]]
