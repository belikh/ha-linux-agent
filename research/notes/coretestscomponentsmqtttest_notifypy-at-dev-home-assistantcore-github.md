---
title: core/tests/components/mqtt/test_notify.py at dev · home-assistant/core · GitHub
id: coretestscomponentsmqtttest_notifypy-at-dev-home-assistantcore-github
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- mqtt-discovery
- availability
- discovery
- source-code
created: '2026-09-02T17:03:39.587291Z'
updated: '2026-09-02T17:40:10.894603Z'
source: https://github.com/home-assistant/core/blob/dev/tests/components/mqtt/test_notify.py
source_domain: github.com
fetched_at: '2026-09-02T17:03:30.876571Z'
fetch_provider: builtin
status: deprecated
type: note
tier: practitioner
content_type: code
deprecated: false
summary: core/tests/components/mqtt/testnotify.py at dev · home-assistant/core · GitHub
---

core/tests/components/mqtt/test_notify.py at dev · home-assistant/core · GitHub

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
test_notify.pyCopy path

Blame
More file actions

Blame
More file actions

Latest commit

HistoryHistory

History

442 lines (374 loc) · 13.5 KB

dev

/
test_notify.pyCopy pathTop

File metadata and controls

Code

Blame

442 lines (374 loc) · 13.5 KB

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
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
104
105
106
107
108
109
110
111
112
113
114
115
116
117
118
119
120
121
122
123
124
125
126
127
128
129
130
131
132
133
134
135
136
137
138
139
140
141
142
143
144
145
146
147
148
149
150
151
152
153
154
155
156
157
158
159
160
161
162
163
164
165
166
167
168
169
170
171
172
173
174
175
176
177
178
179
180
181
182
183
184
185
186
187
188
189
190
191
192
193
194
195
196
197
198
199
200
201
202
203
204
205
206
207
208
209
210
211
212
213
214
215
216
217
218
219
220
221
222
223
224
225
226
227
228
229
230
231
232
233
234
235
236
237
238
239
240
241
242
243
244
245
246
247
248
249
250
251
252
253
254
255
256
257
258
259
260
261
262
263
264
265
266
267
268
269
270
271
272
273
274
275
276
277
278
279
280
281
282
283
284
285
286
287
288
289
290
291
292
293
294
295
296
297
298
299
300
301
302
303
304
305
306
307
308
309
310
311
312
313
314
315
316
317
318
319
320
321
322
323
324
325
326
327
328
329
330
331
332
333
334
335
336
337
338
339
340
341
342
343
344
345
346
347
348
349
350
351
352
353
354
355
356
357
358
359
360
361
362
363
364
365
366
367
368
369
370
371
372
373
374
375
376
377
378
379
380
381
382
383
384
385
386
387
388
389
390
391
392
393
394
395
396
397
398
399
400
401
402
403
404
405
406
407
408
409
410
411
412
413
414
415
416
417
418
419
420
421
422
423
424
425
426
427
428
429
430
431
432
433
434
435
436
437
438
439
440
441
442

"""The tests for the MQTT notify platform."""

import copy

from typing import Any

from unittest.mock import patch

import pytest

from homeassistant.components import notify

from homeassistant.components.mqtt.const import DOMAIN

from homeassistant.components.notify import ATTR_MESSAGE

from homeassistant.const import ATTR_ENTITY_ID, ATTR_FRIENDLY_NAME, STATE_UNKNOWN

from homeassistant.core import HomeAssistant

from .common import (

help_test_availability_when_connection_lost,

help_test_availability_without_topic,

help_test_custom_availability_payload,

help_test_default_availability_payload,

help_test_discovery_broken,

help_test_discovery_removal,

help_test_discovery_update,

help_test_discovery_update_attr,

help_test_discovery_update_unchanged,

help_test_entity_debug_info_message,

help_test_entity_device_info_remove,

help_test_entity_device_info_update,

help_test_entity_device_info_with_connection,

help_test_entity_device_info_with_identifier,

help_test_entity_id_update_discovery_update,

help_test_publishing_with_custom_encoding,

help_test_reloadable,

help_test_setting_attribute_via_mqtt_json_message,

help_test_setting_attribute_with_template,

help_test_setting_blocked_attribute_via_mqtt_json_message,

help_test_unique_id,

help_test_unload_config_entry_with_platform,

help_test_update_with_json_attrs_bad_json,

help_test_update_with_json_attrs_not_dict,

)

from tests.typing import MqttMockHAClientGenerator, MqttMockPahoClient

DEFAULT_CONFIG = {

DOMAIN: {notify.DOMAIN: {"name": "test", "command_topic": "test-topic"}}

}

@pytest.mark.freeze_time("2021-11-08 13:31:44+00:00")

@pytest.mark.parametrize(

"hass_config",

[

{

DOMAIN: {

notify.DOMAIN: {

"command_topic": "command-topic",

"name": "test",

"default_entity_id": "notify.test_notify",

"qos": "2",

}

}

}

],

)

async def test_sending_mqtt_commands(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test the sending MQTT commands."""

mqtt_mock = await mqtt_mock_entry()

state = hass.states.get("notify.test_notify")

assert state.state == STATE_UNKNOWN

assert state.attributes.get(ATTR_FRIENDLY_NAME) == "test"

await hass.services.async_call(

notify.DOMAIN,

notify.SERVICE_SEND_MESSAGE,

{ATTR_MESSAGE: "Beer message", ATTR_ENTITY_ID: "notify.test_notify"},

blocking=True,

)

mqtt_mock.async_publish.assert_called_once_with(

"command-topic", "Beer message", 2, False, message_expiry_interval=None

)

mqtt_mock.async_publish.reset_mock()

state = hass.states.get("notify.test_notify")

assert state.state == "2021-11-08T13:31:44+00:00"

@pytest.mark.parametrize(

"hass_config",

[

{

DOMAIN: {

notify.DOMAIN: {

"command_topic": "command-topic",

"command_template": '{ "{{ entity_id }}": "{{ value }}" }',

"name": "test",

}

}

}

],

)

async def test_command_template(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test the sending of MQTT commands through a command template."""

mqtt_mock = await mqtt_mock_entry()

state = hass.states.get("notify.test")

assert state.state == STATE_UNKNOWN

assert state.attributes.get(ATTR_FRIENDLY_NAME) == "test"

await hass.services.async_call(

notify.DOMAIN,

notify.SERVICE_SEND_MESSAGE,

{ATTR_MESSAGE: "Beer message", ATTR_ENTITY_ID: "notify.test"},

blocking=True,

)

mqtt_mock.async_publish.assert_called_once_with(

"command-topic",

'{ "notify.test": "Beer message" }',

0,

False,

message_expiry_interval=None,

)

mqtt_mock.async_publish.reset_mock()

@pytest.mark.parametrize("hass_config", [DEFAULT_CONFIG])

async def test_availability_when_connection_lost(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test availability after MQTT disconnection."""

await help_test_availability_when_connection_lost(

hass, mqtt_mock_entry, notify.DOMAIN

)

@pytest.mark.parametrize("hass_config", [DEFAULT_CONFIG])

async def test_availability_without_topic(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test availability without defined availability topic."""

await help_test_availability_without_topic(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_default_availability_payload(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test availability by default payload with defined topic."""

await help_test_default_availability_payload(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG, True

)

async def test_custom_availability_payload(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test availability by custom payload with defined topic."""

await help_test_custom_availability_payload(

hass,

mqtt_mock_entry,

notify.DOMAIN,

DEFAULT_CONFIG,

True,

)

async def test_setting_attribute_via_mqtt_json_message(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test the setting of attribute via MQTT with JSON payload."""

await help_test_setting_attribute_via_mqtt_json_message(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_setting_blocked_attribute_via_mqtt_json_message(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test the setting of attribute via MQTT with JSON payload."""

await help_test_setting_blocked_attribute_via_mqtt_json_message(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG, None

)

async def test_setting_attribute_with_template(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test the setting of attribute via MQTT with JSON payload."""

await help_test_setting_attribute_with_template(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_update_with_json_attrs_not_dict(

hass: HomeAssistant,

mqtt_mock_entry: MqttMockHAClientGenerator,

caplog: pytest.LogCaptureFixture,

) -> None:

"""Test attributes get extracted from a JSON result."""

await help_test_update_with_json_attrs_not_dict(

hass, mqtt_mock_entry, caplog, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_update_with_json_attrs_bad_json(

hass: HomeAssistant,

mqtt_mock_entry: MqttMockHAClientGenerator,

caplog: pytest.LogCaptureFixture,

) -> None:

"""Test attributes get extracted from a JSON result."""

await help_test_update_with_json_attrs_bad_json(

hass, mqtt_mock_entry, caplog, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_discovery_update_attr(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test update of discovered MQTTAttributes."""

await help_test_discovery_update_attr(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

@pytest.mark.parametrize(

"hass_config",

[

{

DOMAIN: {

notify.DOMAIN: [

{

"name": "Test 1",

"command_topic": "command-topic",

"unique_id": "TOTALLY_UNIQUE",

},

{

"name": "Test 2",

"command_topic": "command-topic",

"unique_id": "TOTALLY_UNIQUE",

},

]

}

}

],

)

async def test_unique_id(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test unique id option only creates one notify entity per unique_id."""

await help_test_unique_id(hass, mqtt_mock_entry, notify.DOMAIN)

async def test_discovery_removal_notify(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test removal of discovered notify."""

data = '{ "name": "test", "command_topic": "test_topic" }'

await help_test_discovery_removal(hass, mqtt_mock_entry, notify.DOMAIN, data)

async def test_discovery_update_notify(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test update of discovered notify."""

config1 = copy.deepcopy(DEFAULT_CONFIG[DOMAIN][notify.DOMAIN])

config2 = copy.deepcopy(DEFAULT_CONFIG[DOMAIN][notify.DOMAIN])

config1["name"] = "Beer"

config2["name"] = "Milk"

await help_test_discovery_update(

hass, mqtt_mock_entry, notify.DOMAIN, config1, config2

)

async def test_discovery_update_unchanged_notify(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test update of discovered notify."""

data1 = (

'{ "name": "Beer",'

'  "state_topic": "test_topic",'

'  "command_topic": "test_topic" }'

)

with patch(

"homeassistant.components.mqtt.notify.MqttNotify.discovery_update"

) as discovery_update:

await help_test_discovery_update_unchanged(

hass, mqtt_mock_entry, notify.DOMAIN, data1, discovery_update

)

@pytest.mark.no_fail_on_log_exception

async def test_discovery_broken(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test handling of bad discovery message."""

data1 = '{ "name": "Beer" }'

data2 = '{ "name": "Milk", "command_topic": "test_topic" }'

await help_test_discovery_broken(hass, mqtt_mock_entry, notify.DOMAIN, data1, data2)

async def test_entity_device_info_with_connection(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test MQTT notify device registry integration."""

await help_test_entity_device_info_with_connection(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_entity_device_info_with_identifier(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test MQTT notify device registry integration."""

await help_test_entity_device_info_with_identifier(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_entity_device_info_update(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test device registry update."""

await help_test_entity_device_info_update(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_entity_device_info_remove(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test device registry remove."""

await help_test_entity_device_info_remove(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_entity_id_update_discovery_update(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test MQTT discovery update when entity_id is updated."""

await help_test_entity_id_update_discovery_update(

hass, mqtt_mock_entry, notify.DOMAIN, DEFAULT_CONFIG

)

async def test_entity_debug_info_message(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test MQTT debug info."""

await help_test_entity_debug_info_message(

hass,

mqtt_mock_entry,

notify.DOMAIN,

DEFAULT_CONFIG,

notify.SERVICE_SEND_MESSAGE,

command_topic="test-topic",

command_payload="Milk",

state_topic=None,

service_parameters={"message": "Milk"},

)

@pytest.mark.parametrize(

("service", "topic", "parameters", "payload", "template"),

[

(

notify.SERVICE_SEND_MESSAGE,

"command_topic",

{"message": "Beer test"},

"Beer test",

"command_template",

),

],

)

async def test_publishing_with_custom_encoding(

hass: HomeAssistant,

mqtt_mock_entry: MqttMockHAClientGenerator,

caplog: pytest.LogCaptureFixture,

service: str,

topic: str,

parameters: dict[str, Any],

payload: str,

template: str | None,

) -> None:

"""Test publishing MQTT payload with different encoding."""

domain = notify.DOMAIN

config = DEFAULT_CONFIG

await help_test_publishing_with_custom_encoding(

hass,

mqtt_mock_entry,

caplog,

domain,

config,

service,

topic,

parameters,

payload,

template,

)

async def test_reloadable(

hass: HomeAssistant,

mqtt_client_mock: MqttMockPahoClient,

) -> None:

"""Test reloading the MQTT platform."""

domain = notify.DOMAIN

config = DEFAULT_CONFIG

await help_test_reloadable(hass, mqtt_client_mock, domain, config)

@pytest.mark.parametrize(

"hass_config",

[DEFAULT_CONFIG, {"mqtt": [DEFAULT_CONFIG["mqtt"]]}],

ids=["platform_key", "listed"],

)

async def test_setup_manual_entity_from_yaml(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test setup manual configured MQTT entity."""

await mqtt_mock_entry()

platform = notify.DOMAIN

assert hass.states.get(f"{platform}.test")

async def test_unload_entry(

hass: HomeAssistant, mqtt_mock_entry: MqttMockHAClientGenerator

) -> None:

"""Test unloading the config entry."""

domain = notify.DOMAIN

config = DEFAULT_CONFIG

await help_test_unload_config_entry_with_platform(

hass, mqtt_mock_entry, domain, config

)

You can’t perform that action at this time.