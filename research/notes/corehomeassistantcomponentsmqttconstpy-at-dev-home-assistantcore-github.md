---
title: core/homeassistant/components/mqtt/const.py at dev · home-assistant/core ·
  GitHub
id: corehomeassistantcomponentsmqttconstpy-at-dev-home-assistantcore-github
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- mqtt-discovery
- source-code
- birth-message
- failure-notifications
- gap-02
- version-ground-truth
created: '2026-09-02T17:03:39.574833Z'
updated: '2026-09-05T10:51:22.437773Z'
source: https://github.com/home-assistant/core/blob/dev/homeassistant/components/mqtt/const.py
source_domain: github.com
fetched_at: '2026-09-02T17:03:28.341874Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'HA core mqtt/const.py (dev): the authoritative SUPPORTED_COMPONENTS tuple
  for MQTT discovery — ''notify'' IS in the list (between ''lock'' and ''number''),
  alongside 33 other platforms. This is the exact constant discovery.py iterates to
  build its <prefix>/<component>/+/config wildcard subscriptions, so a retained payload
  on homeassistant/notify/<node>/<object>/config creates a notify entity. Resolves
  the gap-02 evidence hole: the corpus''s earlier discovery.py note had zero ''notify''
  occurrences only because discovery.py imports SUPPORTED_COMPONENTS from const.py
  rather than defining it.'
---

core/homeassistant/components/mqtt/const.py at dev · home-assistant/core · GitHub

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
const.pyCopy path

Blame
More file actions

Blame
More file actions

Latest commit

HistoryHistory

History

456 lines (429 loc) · 16 KB

dev

/
const.pyCopy pathTop

File metadata and controls

Code

Blame

456 lines (429 loc) · 16 KB

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
443
444
445
446
447
448
449
450
451
452
453
454
455
456

"""Constants used by multiple MQTT modules."""

import logging

import jinja2

from homeassistant.components.alarm_control_panel import AlarmControlPanelEntityFeature

from homeassistant.const import CONF_PAYLOAD, Platform

from homeassistant.exceptions import TemplateError

ATTR_DISCOVERY_HASH = "discovery_hash"

ATTR_DISCOVERY_PAYLOAD = "discovery_payload"

ATTR_DISCOVERY_TOPIC = "discovery_topic"

ATTR_MESSAGE_EXPIRY_INTERVAL = "message_expiry_interval"

ATTR_PAYLOAD = "payload"

ATTR_QOS = "qos"

ATTR_RETAIN = "retain"

ATTR_TOPIC = "topic"

AVAILABILITY_ALL = "all"

AVAILABILITY_ANY = "any"

AVAILABILITY_LATEST = "latest"

AVAILABILITY_MODES = [AVAILABILITY_ALL, AVAILABILITY_ANY, AVAILABILITY_LATEST]

CONF_PAYLOAD_AVAILABLE = "payload_available"

CONF_PAYLOAD_NOT_AVAILABLE = "payload_not_available"

CONF_AVAILABILITY = "availability"

CONF_AVAILABILITY_MODE = "availability_mode"

CONF_AVAILABILITY_TEMPLATE = "availability_template"

CONF_AVAILABILITY_TOPIC = "availability_topic"

CONF_AVAILABLE_TONES = "available_tones"

CONF_BROKER = "broker"

CONF_BIRTH_MESSAGE = "birth_message"

CONF_CODE_ARM_REQUIRED = "code_arm_required"

CONF_CODE_DISARM_REQUIRED = "code_disarm_required"

CONF_CODE_FORMAT = "code_format"

CONF_CODE_TRIGGER_REQUIRED = "code_trigger_required"

CONF_COMMAND_TEMPLATE = "command_template"

CONF_COMMAND_TOPIC = "command_topic"

CONF_CONTENT_TYPE = "content_type"

CONF_DEFAULT_ENTITY_ID = "default_entity_id"

CONF_DISCOVERY_PREFIX = "discovery_prefix"

CONF_DISCOVERY_QOS = "discovery_qos"

CONF_ENCODING = "encoding"

CONF_IMAGE_ENCODING = "image_encoding"

CONF_IMAGE_TOPIC = "image_topic"

CONF_JSON_ATTRS_TOPIC = "json_attributes_topic"

CONF_JSON_ATTRS_TEMPLATE = "json_attributes_template"

CONF_KEEPALIVE = "keepalive"

CONF_MESSAGE_EXPIRY_INTERVAL = "message_expiry_interval"

CONF_ORIGIN = "origin"

CONF_QOS = ATTR_QOS

CONF_RETAIN = ATTR_RETAIN

CONF_SCHEMA = "schema"

CONF_STATE_TOPIC = "state_topic"

CONF_STATE_VALUE_TEMPLATE = "state_value_template"

CONF_TIMEZONE = "timezone"

CONF_TOPIC = "topic"

CONF_TRANSPORT = "transport"

CONF_WS_PATH = "ws_path"

CONF_WS_HEADERS = "ws_headers"

CONF_WILL_MESSAGE = "will_message"

CONF_PAYLOAD_RESET = "payload_reset"

CONF_SUPPORTED_FEATURES = "supported_features"

CONF_ACTION_TEMPLATE = "action_template"

CONF_ACTION_TOPIC = "action_topic"

CONF_BLUE_TEMPLATE = "blue_template"

CONF_BRIGHTNESS_COMMAND_TEMPLATE = "brightness_command_template"

CONF_BRIGHTNESS_COMMAND_TOPIC = "brightness_command_topic"

CONF_BRIGHTNESS_SCALE = "brightness_scale"

CONF_BRIGHTNESS_STATE_TOPIC = "brightness_state_topic"

CONF_BRIGHTNESS_TEMPLATE = "brightness_template"

CONF_BRIGHTNESS_VALUE_TEMPLATE = "brightness_value_template"

CONF_COLOR_MODE_STATE_TOPIC = "color_mode_state_topic"

CONF_COLOR_MODE_VALUE_TEMPLATE = "color_mode_value_template"

CONF_COLOR_TEMP_COMMAND_TEMPLATE = "color_temp_command_template"

CONF_COLOR_TEMP_COMMAND_TOPIC = "color_temp_command_topic"

CONF_COLOR_TEMP_KELVIN = "color_temp_kelvin"

CONF_COLOR_TEMP_TEMPLATE = "color_temp_template"

CONF_COLOR_TEMP_STATE_TOPIC = "color_temp_state_topic"

CONF_COLOR_TEMP_VALUE_TEMPLATE = "color_temp_value_template"

CONF_COMMAND_OFF_TEMPLATE = "command_off_template"

CONF_COMMAND_ON_TEMPLATE = "command_on_template"

CONF_CURRENT_HUMIDITY_TEMPLATE = "current_humidity_template"

CONF_CURRENT_HUMIDITY_TOPIC = "current_humidity_topic"

CONF_CURRENT_TEMP_TEMPLATE = "current_temperature_template"

CONF_CURRENT_TEMP_TOPIC = "current_temperature_topic"

CONF_DIRECTION_COMMAND_TEMPLATE = "direction_command_template"

CONF_DIRECTION_COMMAND_TOPIC = "direction_command_topic"

CONF_DIRECTION_STATE_TOPIC = "direction_state_topic"

CONF_DIRECTION_VALUE_TEMPLATE = "direction_value_template"

CONF_ENABLED_BY_DEFAULT = "enabled_by_default"

CONF_EFFECT_COMMAND_TEMPLATE = "effect_command_template"

CONF_EFFECT_COMMAND_TOPIC = "effect_command_topic"

CONF_EFFECT_LIST = "effect_list"

CONF_EFFECT_STATE_TOPIC = "effect_state_topic"

CONF_EFFECT_TEMPLATE = "effect_template"

CONF_EFFECT_VALUE_TEMPLATE = "effect_value_template"

CONF_ENTITY_PICTURE = "entity_picture"

CONF_EXPIRE_AFTER = "expire_after"

CONF_FAN_MODE_COMMAND_TEMPLATE = "fan_mode_command_template"

CONF_FAN_MODE_COMMAND_TOPIC = "fan_mode_command_topic"

CONF_FAN_MODE_LIST = "fan_modes"

CONF_FAN_MODE_STATE_TEMPLATE = "fan_mode_state_template"

CONF_FAN_MODE_STATE_TOPIC = "fan_mode_state_topic"

CONF_FLASH = "flash"

CONF_FLASH_TIME_LONG = "flash_time_long"

CONF_FLASH_TIME_SHORT = "flash_time_short"

CONF_GET_POSITION_TEMPLATE = "position_template"

CONF_GET_POSITION_TOPIC = "position_topic"

CONF_GREEN_TEMPLATE = "green_template"

CONF_GROUP = "group"

CONF_HS_COMMAND_TEMPLATE = "hs_command_template"

CONF_HS_COMMAND_TOPIC = "hs_command_topic"

CONF_HS_STATE_TOPIC = "hs_state_topic"

CONF_HS_VALUE_TEMPLATE = "hs_value_template"

CONF_HUMIDITY_COMMAND_TEMPLATE = "target_humidity_command_template"

CONF_HUMIDITY_COMMAND_TOPIC = "target_humidity_command_topic"

CONF_HUMIDITY_STATE_TEMPLATE = "target_humidity_state_template"

CONF_HUMIDITY_STATE_TOPIC = "target_humidity_state_topic"

CONF_HUMIDITY_MAX = "max_humidity"

CONF_HUMIDITY_MIN = "min_humidity"

CONF_LAST_RESET_VALUE_TEMPLATE = "last_reset_value_template"

CONF_MAX = "max"

CONF_MAX_KELVIN = "max_kelvin"

CONF_MAX_MIREDS = "max_mireds"

CONF_MIN = "min"

CONF_MIN_KELVIN = "min_kelvin"

CONF_MIN_MIREDS = "min_mireds"

CONF_MODE_COMMAND_TEMPLATE = "mode_command_template"

CONF_MODE_COMMAND_TOPIC = "mode_command_topic"

CONF_MODE_LIST = "modes"

CONF_MODE_STATE_TEMPLATE = "mode_state_template"

CONF_MODE_STATE_TOPIC = "mode_state_topic"

CONF_OFF_DELAY = "off_delay"

CONF_ON_COMMAND_TYPE = "on_command_type"

CONF_OSCILLATION_COMMAND_TOPIC = "oscillation_command_topic"

CONF_OSCILLATION_COMMAND_TEMPLATE = "oscillation_command_template"

CONF_OSCILLATION_STATE_TOPIC = "oscillation_state_topic"

CONF_OSCILLATION_VALUE_TEMPLATE = "oscillation_value_template"

CONF_PATTERN = "pattern"

CONF_PAYLOAD_ARM_AWAY = "payload_arm_away"

CONF_PAYLOAD_ARM_CUSTOM_BYPASS = "payload_arm_custom_bypass"

CONF_PAYLOAD_ARM_HOME = "payload_arm_home"

CONF_PAYLOAD_ARM_NIGHT = "payload_arm_night"

CONF_PAYLOAD_ARM_VACATION = "payload_arm_vacation"

CONF_PAYLOAD_CLOSE = "payload_close"

CONF_PAYLOAD_DISARM = "payload_disarm"

CONF_PAYLOAD_LOCK = "payload_lock"

CONF_PAYLOAD_OPEN = "payload_open"

CONF_PAYLOAD_OSCILLATION_OFF = "payload_oscillation_off"

CONF_PAYLOAD_OSCILLATION_ON = "payload_oscillation_on"

CONF_PAYLOAD_PRESS = "payload_press"

CONF_PAYLOAD_RESET_PERCENTAGE = "payload_reset_percentage"

CONF_PAYLOAD_RESET_PRESET_MODE = "payload_reset_preset_mode"

CONF_PAYLOAD_STOP = "payload_stop"

CONF_PAYLOAD_STOP_TILT = "payload_stop_tilt"

CONF_PAYLOAD_TRIGGER = "payload_trigger"

CONF_PAYLOAD_UNLOCK = "payload_unlock"

CONF_PERCENTAGE_COMMAND_TEMPLATE = "percentage_command_template"

CONF_PERCENTAGE_COMMAND_TOPIC = "percentage_command_topic"

CONF_PERCENTAGE_STATE_TOPIC = "percentage_state_topic"

CONF_PERCENTAGE_VALUE_TEMPLATE = "percentage_value_template"

CONF_POSITION_CLOSED = "position_closed"

CONF_POSITION_OPEN = "position_open"

CONF_POWER_COMMAND_TOPIC = "power_command_topic"

CONF_POWER_COMMAND_TEMPLATE = "power_command_template"

CONF_PRECISION = "precision"

CONF_PRESET_MODE_COMMAND_TEMPLATE = "preset_mode_command_template"

CONF_PRESET_MODE_COMMAND_TOPIC = "preset_mode_command_topic"

CONF_PRESET_MODES_LIST = "preset_modes"

CONF_PRESET_MODE_STATE_TOPIC = "preset_mode_state_topic"

CONF_PRESET_MODE_VALUE_TEMPLATE = "preset_mode_value_template"

CONF_RED_TEMPLATE = "red_template"

CONF_REPORTS_POSITION = "reports_position"

CONF_RGB_COMMAND_TEMPLATE = "rgb_command_template"

CONF_RGB_COMMAND_TOPIC = "rgb_command_topic"

CONF_RGB_STATE_TOPIC = "rgb_state_topic"

CONF_RGB_VALUE_TEMPLATE = "rgb_value_template"

CONF_RGBW_COMMAND_TEMPLATE = "rgbw_command_template"

CONF_RGBW_COMMAND_TOPIC = "rgbw_command_topic"

CONF_RGBW_STATE_TOPIC = "rgbw_state_topic"

CONF_RGBW_VALUE_TEMPLATE = "rgbw_value_template"

CONF_RGBWW_COMMAND_TEMPLATE = "rgbww_command_template"

CONF_RGBWW_COMMAND_TOPIC = "rgbww_command_topic"

CONF_RGBWW_STATE_TOPIC = "rgbww_state_topic"

CONF_RGBWW_VALUE_TEMPLATE = "rgbww_value_template"

CONF_SET_POSITION_TEMPLATE = "set_position_template"

CONF_SET_POSITION_TOPIC = "set_position_topic"

CONF_SPEED_RANGE_MAX = "speed_range_max"

CONF_SPEED_RANGE_MIN = "speed_range_min"

CONF_STATE_CLOSED = "state_closed"

CONF_STATE_CLOSING = "state_closing"

CONF_STATE_JAMMED = "state_jammed"

CONF_STATE_LOCKED = "state_locked"

CONF_STATE_LOCKING = "state_locking"

CONF_STATE_OFF = "state_off"

CONF_STATE_ON = "state_on"

CONF_STATE_OPEN = "state_open"

CONF_STATE_OPENING = "state_opening"

CONF_STATE_STOPPED = "state_stopped"

CONF_STATE_UNLOCKED = "state_unlocked"

CONF_STATE_UNLOCKING = "state_unlocking"

CONF_STEP = "step"

CONF_SUGGESTED_DISPLAY_PRECISION = "suggested_display_precision"

CONF_SUPPORT_DURATION = "support_duration"

CONF_SUPPORT_VOLUME_SET = "support_volume_set"

CONF_SUPPORTED_COLOR_MODES = "supported_color_modes"

CONF_SWING_HORIZONTAL_MODE_COMMAND_TEMPLATE = "swing_horizontal_mode_command_template"

CONF_SWING_HORIZONTAL_MODE_COMMAND_TOPIC = "swing_horizontal_mode_command_topic"

CONF_SWING_HORIZONTAL_MODE_LIST = "swing_horizontal_modes"

CONF_SWING_HORIZONTAL_MODE_STATE_TEMPLATE = "swing_horizontal_mode_state_template"

CONF_SWING_HORIZONTAL_MODE_STATE_TOPIC = "swing_horizontal_mode_state_topic"

CONF_SWING_MODE_COMMAND_TEMPLATE = "swing_mode_command_template"

CONF_SWING_MODE_COMMAND_TOPIC = "swing_mode_command_topic"

CONF_SWING_MODE_LIST = "swing_modes"

CONF_SWING_MODE_STATE_TEMPLATE = "swing_mode_state_template"

CONF_SWING_MODE_STATE_TOPIC = "swing_mode_state_topic"

CONF_TEMP_COMMAND_TEMPLATE = "temperature_command_template"

CONF_TEMP_COMMAND_TOPIC = "temperature_command_topic"

CONF_TEMP_HIGH_COMMAND_TEMPLATE = "temperature_high_command_template"

CONF_TEMP_HIGH_COMMAND_TOPIC = "temperature_high_command_topic"

CONF_TEMP_HIGH_STATE_TEMPLATE = "temperature_high_state_template"

CONF_TEMP_HIGH_STATE_TOPIC = "temperature_high_state_topic"

CONF_TEMP_INITIAL = "initial"

CONF_TEMP_LOW_COMMAND_TEMPLATE = "temperature_low_command_template"

CONF_TEMP_LOW_COMMAND_TOPIC = "temperature_low_command_topic"

CONF_TEMP_LOW_STATE_TEMPLATE = "temperature_low_state_template"

CONF_TEMP_LOW_STATE_TOPIC = "temperature_low_state_topic"

CONF_TEMP_MAX = "max_temp"

CONF_TEMP_MIN = "min_temp"

CONF_TEMP_STATE_TEMPLATE = "temperature_state_template"

CONF_TEMP_STATE_TOPIC = "temperature_state_topic"

CONF_TEMP_STEP = "temp_step"

CONF_TILT_COMMAND_TEMPLATE = "tilt_command_template"

CONF_TILT_COMMAND_TOPIC = "tilt_command_topic"

CONF_TILT_STATUS_TOPIC = "tilt_status_topic"

CONF_TILT_STATUS_TEMPLATE = "tilt_status_template"

CONF_TILT_CLOSED_POSITION = "tilt_closed_value"

CONF_TILT_MAX = "tilt_max"

CONF_TILT_MIN = "tilt_min"

CONF_TILT_OPEN_POSITION = "tilt_opened_value"

CONF_TILT_STATE_OPTIMISTIC = "tilt_optimistic"

CONF_TRANSITION = "transition"

CONF_URL_TEMPLATE = "url_template"

CONF_URL_TOPIC = "url_topic"

CONF_VISIBLE_BY_DEFAULT = "visible_by_default"

CONF_XY_COMMAND_TEMPLATE = "xy_command_template"

CONF_XY_COMMAND_TOPIC = "xy_command_topic"

CONF_XY_STATE_TOPIC = "xy_state_topic"

CONF_XY_VALUE_TEMPLATE = "xy_value_template"

CONF_WHITE_COMMAND_TOPIC = "white_command_topic"

CONF_WHITE_SCALE = "white_scale"

# Config flow constants

CONF_CERTIFICATE = "certificate"

CONF_CLIENT_KEY = "client_key"

CONF_CLIENT_CERT = "client_cert"

CONF_COMPONENTS = "components"

CONF_TLS_INSECURE = "tls_insecure"

# Device and integration info options

CONF_IDENTIFIERS = "identifiers"

CONF_CONNECTIONS = "connections"

CONF_MANUFACTURER = "manufacturer"

CONF_HW_VERSION = "hw_version"

CONF_SW_VERSION = "sw_version"

CONF_SERIAL_NUMBER = "serial_number"

CONF_VIA_DEVICE = "via_device"

CONF_DEPRECATED_VIA_HUB = "via_hub"

CONF_SUGGESTED_AREA = "suggested_area"

CONF_CONFIGURATION_URL = "configuration_url"

CONF_SUPPORT_URL = "support_url"

DEFAULT_ALARM_CONTROL_PANEL_COMMAND_TEMPLATE = "{{action}}"

DEFAULT_BRIGHTNESS = False

DEFAULT_BRIGHTNESS_SCALE = 255

DEFAULT_CLIMATE_INITIAL_TEMPERATURE = 21.0

DEFAULT_PREFIX = "homeassistant"

DEFAULT_BIRTH_WILL_TOPIC = DEFAULT_PREFIX + "/status"

DEFAULT_DISCOVERY = True

DEFAULT_EFFECT = False

DEFAULT_ENCODING = "utf-8"

DEFAULT_FLASH_TIME_LONG = 10

DEFAULT_FLASH_TIME_SHORT = 2

DEFAULT_OPTIMISTIC = False

DEFAULT_ON_COMMAND_TYPE = "last"

DEFAULT_QOS = 0

DEFAULT_PAYLOAD_ARM_AWAY = "ARM_AWAY"

DEFAULT_PAYLOAD_ARM_CUSTOM_BYPASS = "ARM_CUSTOM_BYPASS"

DEFAULT_PAYLOAD_ARM_HOME = "ARM_HOME"

DEFAULT_PAYLOAD_ARM_NIGHT = "ARM_NIGHT"

DEFAULT_PAYLOAD_ARM_VACATION = "ARM_VACATION"

DEFAULT_PAYLOAD_AVAILABLE = "online"

DEFAULT_PAYLOAD_CLOSE = "CLOSE"

DEFAULT_PAYLOAD_DISARM = "DISARM"

DEFAULT_PAYLOAD_LOCK = "LOCK"

DEFAULT_PAYLOAD_NOT_AVAILABLE = "offline"

DEFAULT_PAYLOAD_OFF = "OFF"

DEFAULT_PAYLOAD_ON = "ON"

DEFAULT_PAYLOAD_OPEN = "OPEN"

DEFAULT_PAYLOAD_OSCILLATE_OFF = "oscillate_off"

DEFAULT_PAYLOAD_OSCILLATE_ON = "oscillate_on"

DEFAULT_PAYLOAD_PRESS = "PRESS"

DEFAULT_PAYLOAD_RESET = "None"

DEFAULT_PAYLOAD_STOP = "STOP"

DEFAULT_PAYLOAD_TRIGGER = "TRIGGER"

DEFAULT_PAYLOAD_UNLOCK = "UNLOCK"

DEFAULT_RETAIN = False

DEFAULT_TILT_CLOSED_POSITION = 0

DEFAULT_TILT_MAX = 100

DEFAULT_TILT_MIN = 0

DEFAULT_TILT_OPEN_POSITION = 100

DEFAULT_TILT_OPTIMISTIC = False

DEFAULT_WS_PATH = "/"

DEFAULT_POSITION_CLOSED = 0

DEFAULT_POSITION_OPEN = 100

DEFAULT_SPEED_RANGE_MAX = 100

DEFAULT_SPEED_RANGE_MIN = 1

DEFAULT_STATE_LOCKED = "LOCKED"

DEFAULT_STATE_LOCKING = "LOCKING"

DEFAULT_STATE_OPEN = "OPEN"

DEFAULT_STATE_OPENING = "OPENING"

DEFAULT_STATE_STOPPED = "stopped"

DEFAULT_STATE_UNLOCKED = "UNLOCKED"

DEFAULT_STATE_UNLOCKING = "UNLOCKING"

DEFAULT_STATE_JAMMED = "JAMMED"

DEFAULT_WHITE_SCALE = 255

COVER_PAYLOAD = "cover"

TILT_PAYLOAD = "tilt"

VALUES_ON_COMMAND_TYPE = ["first", "last", "brightness"]

ALARM_CONTROL_PANEL_SUPPORTED_FEATURES = {

"arm_home": AlarmControlPanelEntityFeature.ARM_HOME,

"arm_away": AlarmControlPanelEntityFeature.ARM_AWAY,

"arm_night": AlarmControlPanelEntityFeature.ARM_NIGHT,

"arm_vacation": AlarmControlPanelEntityFeature.ARM_VACATION,

"arm_custom_bypass": AlarmControlPanelEntityFeature.ARM_CUSTOM_BYPASS,

"trigger": AlarmControlPanelEntityFeature.TRIGGER,

}

REMOTE_CODE = "REMOTE_CODE"

REMOTE_CODE_TEXT = "REMOTE_CODE_TEXT"

PROTOCOL_31 = "3.1"

PROTOCOL_311 = "3.1.1"

PROTOCOL_5 = "5"

SUPPORTED_PROTOCOLS = [PROTOCOL_5, PROTOCOL_311, PROTOCOL_31]

TRANSPORT_TCP = "tcp"

TRANSPORT_WEBSOCKETS = "websockets"

DEFAULT_PORT = 1883

DEFAULT_KEEPALIVE = 60

DEFAULT_PROTOCOL = PROTOCOL_5

DEFAULT_TRANSPORT = TRANSPORT_TCP

DEFAULT_BIRTH = {

ATTR_TOPIC: DEFAULT_BIRTH_WILL_TOPIC,

CONF_PAYLOAD: DEFAULT_PAYLOAD_AVAILABLE,

ATTR_QOS: DEFAULT_QOS,

ATTR_RETAIN: DEFAULT_RETAIN,

}

DEFAULT_WILL = {

ATTR_TOPIC: DEFAULT_BIRTH_WILL_TOPIC,

CONF_PAYLOAD: DEFAULT_PAYLOAD_NOT_AVAILABLE,

ATTR_QOS: DEFAULT_QOS,

ATTR_RETAIN: DEFAULT_RETAIN,

}

DOMAIN = "mqtt"

LOGGER = logging.getLogger(__package__)

MQTT_CONNECTION_STATE = "mqtt_connection_state"

MQTT_PROCESSED_SUBSCRIPTIONS = "mqtt_processed_subscriptions"

PAYLOAD_EMPTY_JSON = "{}"

PAYLOAD_NONE = "None"

CONFIG_ENTRY_VERSION = 2

CONFIG_ENTRY_MINOR_VERSION = 1

ENTITY_PLATFORMS = [

Platform.ALARM_CONTROL_PANEL,

Platform.BINARY_SENSOR,

Platform.BUTTON,

Platform.CAMERA,

Platform.CLIMATE,

Platform.COVER,

Platform.DATE,

Platform.DATETIME,

Platform.DEVICE_TRACKER,

Platform.EVENT,

Platform.FAN,

Platform.HUMIDIFIER,

Platform.IMAGE,

Platform.INFRARED,

Platform.LIGHT,

Platform.LAWN_MOWER,

Platform.LOCK,

Platform.NOTIFY,

Platform.NUMBER,

Platform.SCENE,

Platform.SELECT,

Platform.SENSOR,

Platform.SIREN,

Platform.SWITCH,

Platform.TEXT,

Platform.TIME,

Platform.UPDATE,

Platform.VACUUM,

Platform.VALVE,

Platform.WATER_HEATER,

]

TEMPLATE_ERRORS = (jinja2.TemplateError, TemplateError, TypeError, ValueError)

SUPPORTED_COMPONENTS = (

"alarm_control_panel",

"binary_sensor",

"button",

"camera",

"climate",

"cover",

"date",

"datetime",

"device_automation",

"device_tracker",

"event",

"fan",

"humidifier",

"image",

"infrared",

"lawn_mower",

"light",

"lock",

"notify",

"number",

"scene",

"siren",

"select",

"sensor",

"switch",

"tag",

"text",

"time",

"update",

"vacuum",

"valve",

"water_heater",

)

You can’t perform that action at this time.
## Related

- [[corehomeassistantcomponentsmqttdiscoverypy-at-dev-home-assistantcore-github]]
- [[test_notifypy]]
- [[mqtt-notify-home-assistant]]
