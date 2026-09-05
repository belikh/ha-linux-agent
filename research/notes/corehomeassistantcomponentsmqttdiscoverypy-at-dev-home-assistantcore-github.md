---
title: core/homeassistant/components/mqtt/discovery.py at dev · home-assistant/core
  · GitHub
id: corehomeassistantcomponentsmqttdiscoverypy-at-dev-home-assistantcore-github
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- mqtt-discovery
- availability
- discovery
- source-code
- primary-source
- official-docs
created: '2026-09-02T06:41:31.054446Z'
updated: '2026-09-02T17:37:22.330154Z'
source: https://github.com/home-assistant/core/blob/dev/homeassistant/components/mqtt/discovery.py
source_domain: github.com
fetched_at: '2026-09-02T06:41:25.760647Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'The actual HA core MQTT discovery engine (discovery.py at dev, 733 lines)
  — the authoritative spec for how HA ingests discovery messages. Key mechanics a
  jupiterOS agent must comply with: (1) TOPIC_MATCHER regex enforces <component>/<node_id>/<object_id>/config
  with [a-zA-Z0-9_-] charset only — illegal topics log a warning and are dropped;
  (2) subscriptions are wildcard per supported component: <prefix>/<component>/+/config
  and <prefix>/<component>/+/+/config, plus device-based <prefix>/device/+/config
  (+2-level) which fans a device payload out to per-component configs; (3) abbreviation
  expansion happens BEFORE validation (avty_t->availability_topic, cmd_t, stat_t,
  uniq_id etc. from abbreviations.py), so abbreviated keys are legal in payloads;
  (4) ''~'' topic-base substitution expands leading/trailing ~ in any *_topic field
  including inside availability configs; (5) an EMPTY payload on a config topic is
  the removal/cleanup semantic — for device discovery it regenerates cleanup messages
  for all previously discovered components, and there is a migr_discvry migration
  path; (6) duplicate discovery messages are deduplicated per (component, discovery_id)
  hash with a pending-update deque — rediscovery while pending is queued, not lost;
  (7) discovery data is NOT persisted by HA: re-discovery after HA restart requires
  retained config messages or re-publish by the device; (8) integration-level discovery
  (e.g. via MQTT for other integrations) is separate, driven by discovery_flow with
  a config-flow lock. Also confirms CONF_DISCOVERY_QOS option default 0.'
---

core/homeassistant/components/mqtt/discovery.py at dev · home-assistant/core · GitHub

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
discovery.pyCopy path

Blame
More file actions

Blame
More file actions

Latest commit

HistoryHistory

History

733 lines (649 loc) · 27.7 KB

dev

/
discovery.pyCopy pathTop

File metadata and controls

Code

Blame

733 lines (649 loc) · 27.7 KB

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
457
458
459
460
461
462
463
464
465
466
467
468
469
470
471
472
473
474
475
476
477
478
479
480
481
482
483
484
485
486
487
488
489
490
491
492
493
494
495
496
497
498
499
500
501
502
503
504
505
506
507
508
509
510
511
512
513
514
515
516
517
518
519
520
521
522
523
524
525
526
527
528
529
530
531
532
533
534
535
536
537
538
539
540
541
542
543
544
545
546
547
548
549
550
551
552
553
554
555
556
557
558
559
560
561
562
563
564
565
566
567
568
569
570
571
572
573
574
575
576
577
578
579
580
581
582
583
584
585
586
587
588
589
590
591
592
593
594
595
596
597
598
599
600
601
602
603
604
605
606
607
608
609
610
611
612
613
614
615
616
617
618
619
620
621
622
623
624
625
626
627
628
629
630
631
632
633
634
635
636
637
638
639
640
641
642
643
644
645
646
647
648
649
650
651
652
653
654
655
656
657
658
659
660
661
662
663
664
665
666
667
668
669
670
671
672
673
674
675
676
677
678
679
680
681
682
683
684
685
686
687
688
689
690
691
692
693
694
695
696
697
698
699
700
701
702
703
704
705
706
707
708
709
710
711
712
713
714
715
716
717
718
719
720
721
722
723
724
725
726
727
728
729
730
731
732
733

"""Support for MQTT discovery."""

import asyncio

from collections import deque

from dataclasses import dataclass

import functools

from itertools import chain

import logging

import re

import time

from typing import TYPE_CHECKING, Any

import voluptuous as vol

from homeassistant.config_entries import (

SOURCE_MQTT,

ConfigEntry,

signal_discovered_config_entry_removed,

)

from homeassistant.const import CONF_DEVICE, CONF_PLATFORM

from homeassistant.core import HassJobType, HomeAssistant, callback

from homeassistant.helpers import (

config_validation as cv,

discovery_flow,

entity_registry as er,

)

from homeassistant.helpers.dispatcher import (

async_dispatcher_connect,

async_dispatcher_send,

)

from homeassistant.helpers.service_info.mqtt import MqttServiceInfo, ReceivePayloadType

from homeassistant.helpers.typing import DiscoveryInfoType

from homeassistant.loader import async_get_mqtt

from homeassistant.util.json import json_loads_object

from homeassistant.util.signal_type import SignalTypeFormat

from .abbreviations import ABBREVIATIONS, DEVICE_ABBREVIATIONS, ORIGIN_ABBREVIATIONS

from .client import async_subscribe_internal

from .const import (

ATTR_DISCOVERY_HASH,

ATTR_DISCOVERY_PAYLOAD,

ATTR_DISCOVERY_TOPIC,

CONF_AVAILABILITY,

CONF_COMPONENTS,

CONF_DISCOVERY_QOS,

CONF_ORIGIN,

CONF_TOPIC,

DOMAIN,

SUPPORTED_COMPONENTS,

)

from .models import DATA_MQTT, MqttComponentConfig, MqttOriginInfo, ReceiveMessage

from .schemas import DEVICE_DISCOVERY_SCHEMA, MQTT_ORIGIN_INFO_SCHEMA, SHARED_OPTIONS

from .util import (

async_cleanup_device_registry,

async_forward_entry_setup_and_setup_discovery,

)

ABBREVIATIONS_SET = set(ABBREVIATIONS)

DEVICE_ABBREVIATIONS_SET = set(DEVICE_ABBREVIATIONS)

ORIGIN_ABBREVIATIONS_SET = set(ORIGIN_ABBREVIATIONS)

_LOGGER = logging.getLogger(__name__)

TOPIC_MATCHER = re.compile(

r"(?P<component>\w+)/(?:(?P<node_id>[a-zA-Z0-9_-]+)/)"

r"?(?P<object_id>[a-zA-Z0-9_-]+)/config"

)

MQTT_DISCOVERY_UPDATED: SignalTypeFormat[MQTTDiscoveryPayload] = SignalTypeFormat(

"mqtt_discovery_updated_{}_{}"

)

MQTT_DISCOVERY_NEW: SignalTypeFormat[MQTTDiscoveryPayload] = SignalTypeFormat(

"mqtt_discovery_new_{}_{}"

)

MQTT_DISCOVERY_DONE: SignalTypeFormat[Any] = SignalTypeFormat(

"mqtt_discovery_done_{}_{}"

)

TOPIC_BASE = "~"

CONF_MIGRATE_DISCOVERY = "migrate_discovery"

MIGRATE_DISCOVERY_SCHEMA = vol.Schema(

{vol.Optional(CONF_MIGRATE_DISCOVERY): True},

)

class MQTTDiscoveryPayload(dict[str, Any]):

"""Class to hold and MQTT discovery payload and discovery data."""

device_discovery: bool = False

migrate_discovery: bool = False

discovery_data: DiscoveryInfoType

@dataclass(frozen=True)

class MQTTIntegrationDiscoveryConfig:

"""Class to hold an integration discovery playload."""

integration: str

msg: ReceiveMessage

@callback

def _async_process_discovery_migration(payload: MQTTDiscoveryPayload) -> bool:

"""Process a discovery migration request in the discovery payload."""

# Allow abbreviation

if migr_discvry := (payload.pop("migr_discvry", None)):

payload[CONF_MIGRATE_DISCOVERY] = migr_discvry

if CONF_MIGRATE_DISCOVERY in payload:

try:

MIGRATE_DISCOVERY_SCHEMA(payload)

except vol.Invalid as exc:

_LOGGER.warning(exc)

return False

payload.migrate_discovery = True

payload.clear()

return True

return False

def clear_discovery_hash(hass: HomeAssistant, discovery_hash: tuple[str, str]) -> None:

"""Clear entry from already discovered list."""

hass.data[DATA_MQTT].discovery_already_discovered.discard(discovery_hash)

def set_discovery_hash(hass: HomeAssistant, discovery_hash: tuple[str, str]) -> None:

"""Add entry to already discovered list."""

hass.data[DATA_MQTT].discovery_already_discovered.add(discovery_hash)

@callback

def get_origin_log_string(

discovery_payload: MQTTDiscoveryPayload, *, include_url: bool

) -> str:

"""Get the origin information from a discovery payload for logging."""

if CONF_ORIGIN not in discovery_payload:

return ""

origin_info: MqttOriginInfo = discovery_payload[CONF_ORIGIN]

sw_version_log = ""

if sw_version := origin_info.get("sw_version"):

sw_version_log = f", version: {sw_version}"

support_url_log = ""

if include_url and (support_url := get_origin_support_url(discovery_payload)):

support_url_log = f", support URL: {support_url}"

return (

" from external application "

f"{origin_info['name']}{sw_version_log}{support_url_log}"

)

@callback

def get_origin_support_url(discovery_payload: MQTTDiscoveryPayload) -> str | None:

"""Get the origin information support URL from a discovery payload."""

if CONF_ORIGIN not in discovery_payload:

return ""

origin_info: MqttOriginInfo = discovery_payload[CONF_ORIGIN]

return origin_info.get("support_url")

@callback

def async_log_discovery_origin_info(

message: str, discovery_payload: MQTTDiscoveryPayload

) -> None:

"""Log information about the discovery and origin."""

if not _LOGGER.isEnabledFor(logging.DEBUG):

# bail out early if debug logging is disabled

return

_LOGGER.debug(

"%s%s", message, get_origin_log_string(discovery_payload, include_url=True)

)

@callback

def _replace_abbreviations(

payload: dict[str, Any] | str,

abbreviations: dict[str, str],

abbreviations_set: set[str],

) -> None:

"""Replace abbreviations in an MQTT discovery payload."""

if not isinstance(payload, dict):

return

for key in abbreviations_set.intersection(payload):

payload[abbreviations[key]] = payload.pop(key)

@callback

def _replace_all_abbreviations(

discovery_payload: dict[str, Any], component_only: bool = False

) -> None:

"""Replace all abbreviations in an MQTT discovery payload."""

_replace_abbreviations(discovery_payload, ABBREVIATIONS, ABBREVIATIONS_SET)

if CONF_AVAILABILITY in discovery_payload:

for availability_conf in cv.ensure_list(discovery_payload[CONF_AVAILABILITY]):

_replace_abbreviations(availability_conf, ABBREVIATIONS, ABBREVIATIONS_SET)

if component_only:

return

if CONF_ORIGIN in discovery_payload:

_replace_abbreviations(

discovery_payload[CONF_ORIGIN],

ORIGIN_ABBREVIATIONS,

ORIGIN_ABBREVIATIONS_SET,

)

if CONF_DEVICE in discovery_payload:

_replace_abbreviations(

discovery_payload[CONF_DEVICE],

DEVICE_ABBREVIATIONS,

DEVICE_ABBREVIATIONS_SET,

)

if CONF_COMPONENTS in discovery_payload:

if not isinstance(discovery_payload[CONF_COMPONENTS], dict):

return

for comp_conf in discovery_payload[CONF_COMPONENTS].values():

_replace_all_abbreviations(comp_conf, component_only=True)

@callback

def _replace_topic_base(discovery_payload: MQTTDiscoveryPayload) -> None:

"""Replace topic base in MQTT discovery data."""

base = discovery_payload.pop(TOPIC_BASE)

for key, value in discovery_payload.items():

if isinstance(value, str) and value:

if value[0] == TOPIC_BASE and key.endswith("topic"):

discovery_payload[key] = f"{base}{value[1:]}"

if value[-1] == TOPIC_BASE and key.endswith("topic"):

discovery_payload[key] = f"{value[:-1]}{base}"

if discovery_payload.get(CONF_AVAILABILITY):

for availability_conf in cv.ensure_list(discovery_payload[CONF_AVAILABILITY]):

if not isinstance(availability_conf, dict):

continue

if topic := str(availability_conf.get(CONF_TOPIC)):

if topic[0] == TOPIC_BASE:

availability_conf[CONF_TOPIC] = f"{base}{topic[1:]}"

if topic[-1] == TOPIC_BASE:

availability_conf[CONF_TOPIC] = f"{topic[:-1]}{base}"

@callback

def _generate_device_config(

hass: HomeAssistant,

object_id: str,

node_id: str | None,

migrate_discovery: bool = False,

) -> MQTTDiscoveryPayload:

"""Generate a cleanup or discovery migration message on device cleanup.

If an empty payload, or a migrate discovery request is received for a device,

we forward an empty payload for all previously discovered components.

"""

mqtt_data = hass.data[DATA_MQTT]

device_node_id: str = f"{node_id} {object_id}" if node_id else object_id

config = MQTTDiscoveryPayload({CONF_DEVICE: {}, CONF_COMPONENTS: {}})

config.migrate_discovery = migrate_discovery

comp_config = config[CONF_COMPONENTS]

for platform, discover_id in mqtt_data.discovery_already_discovered:

ids = discover_id.split(" ")

component_node_id = f"{ids.pop(1)} {ids.pop(0)}" if len(ids) > 2 else ids.pop(0)

component_object_id = " ".join(ids)

if not ids:

continue

if device_node_id == component_node_id:

comp_config[component_object_id] = {CONF_PLATFORM: platform}

return config if comp_config else MQTTDiscoveryPayload({})

@callback

def _parse_device_payload(

hass: HomeAssistant,

payload: ReceivePayloadType,

object_id: str,

node_id: str | None,

) -> MQTTDiscoveryPayload:

"""Parse a device discovery payload.

The device discovery payload is translated info the config payloads for every single

component inside the device based configuration.

An empty payload is translated in a cleanup, which forwards an empty payload to all

removed components.

"""

device_payload = MQTTDiscoveryPayload()

if payload == "":

if not (device_payload := _generate_device_config(hass, object_id, node_id)):

_LOGGER.warning(

"No device components to cleanup for %s, node_id '%s'",

object_id,

node_id,

)

return device_payload

try:

device_payload = MQTTDiscoveryPayload(json_loads_object(payload))

except ValueError:

_LOGGER.warning("Unable to parse JSON %s: '%s'", object_id, payload)

return device_payload

if _async_process_discovery_migration(device_payload):

return _generate_device_config(hass, object_id, node_id, migrate_discovery=True)

_replace_all_abbreviations(device_payload)

try:

DEVICE_DISCOVERY_SCHEMA(device_payload)

except vol.Invalid as exc:

_LOGGER.warning(

"Invalid MQTT device discovery payload for %s, %s: '%s'",

object_id,

exc,

payload,

)

return MQTTDiscoveryPayload({})

return device_payload

@callback

def _valid_origin_info(discovery_payload: MQTTDiscoveryPayload) -> bool:

"""Parse and validate origin info from a single component discovery payload."""

if CONF_ORIGIN not in discovery_payload:

return True

try:

MQTT_ORIGIN_INFO_SCHEMA(discovery_payload[CONF_ORIGIN])

except Exception as exc:  # noqa:BLE001

_LOGGER.warning(

"Unable to parse origin information from discovery message: %s, got %s",

exc,

discovery_payload[CONF_ORIGIN],

)

return False

return True

@callback

def _merge_common_device_options(

component_config: MQTTDiscoveryPayload, device_config: dict[str, Any]

) -> None:

"""Merge common device options with the component config options.

Common options are:

CONF_AVAILABILITY,

CONF_AVAILABILITY_MODE,

CONF_AVAILABILITY_TEMPLATE,

CONF_AVAILABILITY_TOPIC,

CONF_COMMAND_TOPIC,

CONF_ENCODING,

CONF_PAYLOAD_AVAILABLE,

CONF_PAYLOAD_NOT_AVAILABLE,

CONF_STATE_TOPIC,

CONF_QOS

Common options in the body of the device based config are inherited into

the component. Unless the option is explicitly specified at component level,

in that case the option at component level will override the common option.

"""

for option in SHARED_OPTIONS:

if option in device_config and option not in component_config:

component_config[option] = device_config.get(option)

async def async_start(  # noqa: C901

hass: HomeAssistant, discovery_topic: str, config_entry: ConfigEntry

) -> None:

"""Start MQTT Discovery."""

mqtt_data = hass.data[DATA_MQTT]

platform_setup_lock: dict[str, asyncio.Lock] = {}

integration_discovery_messages: dict[str, MQTTIntegrationDiscoveryConfig] = {}

@callback

def _async_add_component(discovery_payload: MQTTDiscoveryPayload) -> None:

"""Add a component from a discovery message."""

discovery_hash = discovery_payload.discovery_data[ATTR_DISCOVERY_HASH]

component, discovery_id = discovery_hash

message = f"Found new component: {component} {discovery_id}"

async_log_discovery_origin_info(message, discovery_payload)

mqtt_data.discovery_already_discovered.add(discovery_hash)

async_dispatcher_send(

hass, MQTT_DISCOVERY_NEW.format(component, "mqtt"), discovery_payload

)

async def _async_component_setup(

component: str, discovery_payload: MQTTDiscoveryPayload

) -> None:

"""Perform component set up."""

async with platform_setup_lock.setdefault(component, asyncio.Lock()):

if component not in mqtt_data.platforms_loaded:

await async_forward_entry_setup_and_setup_discovery(

hass, config_entry, {component}

)

_async_add_component(discovery_payload)

@callback

def async_discovery_message_received(msg: ReceiveMessage) -> None:

"""Process the received message."""

mqtt_data.last_discovery = msg.timestamp

payload = msg.payload

topic = msg.topic

topic_trimmed = topic.replace(f"{discovery_topic}/", "", 1)

if not (match := TOPIC_MATCHER.match(topic_trimmed)):

if topic_trimmed.endswith("config"):

_LOGGER.warning(

(

"Received message on illegal discovery topic '%s'. The topic"

" contains non allowed characters. For more information see "

"https://www.home-assistant.io/integrations/mqtt/#discovery-topic"

),

topic,

)

return

component, node_id, object_id = match.groups()

discovered_components: list[MqttComponentConfig] = []

if component == CONF_DEVICE:

# Process device based discovery message and regenerate

# cleanup config for the all the components that are being removed.

# This is done when a component in the device config is omitted and detected

# as being removed, or when the device config update payload is empty.

# In that case this will regenerate a cleanup message for all every already

# discovered components that were linked to the initial device discovery.

device_discovery_payload = _parse_device_payload(

hass, payload, object_id, node_id

)

if not device_discovery_payload:

return

device_config: dict[str, Any]

origin_config: dict[str, Any] | None

component_configs: dict[str, dict[str, Any]]

device_config = device_discovery_payload[CONF_DEVICE]

origin_config = device_discovery_payload.get(CONF_ORIGIN)

component_configs = device_discovery_payload[CONF_COMPONENTS]

for component_id, config in component_configs.items():

component = config.pop(CONF_PLATFORM)

# The object_id in the device discovery topic is the unique identifier.

# It is used as node_id for the components it contains.

component_node_id = object_id

# The component_id in the discovery playload is used as object_id

# If we have an additional node_id in the discovery topic,

# we extend the component_id with it.

component_object_id = (

f"{node_id} {component_id}" if node_id else component_id

)

# We add wrapper to the discovery payload with the discovery data.

# If the dict is empty after removing the platform, the payload is

# assumed to remove the existing config and we do not want to add

# device or orig or shared availability attributes.

if discovery_payload := MQTTDiscoveryPayload(config):

discovery_payload[CONF_DEVICE] = device_config

discovery_payload[CONF_ORIGIN] = origin_config

# Only assign shared config options

# when they are not set at entity level

_merge_common_device_options(

discovery_payload, device_discovery_payload

)

discovery_payload.device_discovery = True

discovery_payload.migrate_discovery = (

device_discovery_payload.migrate_discovery

)

discovered_components.append(

MqttComponentConfig(

component,

component_object_id,

component_node_id,

discovery_payload,

)

)

_LOGGER.debug(

"Process device discovery payload %s", device_discovery_payload

)

device_discovery_id = f"{node_id} {object_id}" if node_id else object_id

message = f"Processing device discovery for '{device_discovery_id}'"

async_log_discovery_origin_info(

message, MQTTDiscoveryPayload(device_discovery_payload)

)

else:

# Process component based discovery message

try:

discovery_payload = MQTTDiscoveryPayload(

json_loads_object(payload) if payload else {}

)

except ValueError:

_LOGGER.warning("Unable to parse JSON %s: '%s'", object_id, payload)

return

if not _async_process_discovery_migration(discovery_payload):

_replace_all_abbreviations(discovery_payload)

if not _valid_origin_info(discovery_payload):

return

discovered_components.append(

MqttComponentConfig(component, object_id, node_id, discovery_payload)

)

discovery_pending_discovered = mqtt_data.discovery_pending_discovered

for component_config in discovered_components:

component = component_config.component

node_id = component_config.node_id

object_id = component_config.object_id

discovery_payload = component_config.discovery_payload

if TOPIC_BASE in discovery_payload:

_replace_topic_base(discovery_payload)

# If present, the node_id will be included in the discovery_id.

discovery_id = f"{node_id} {object_id}" if node_id else object_id

discovery_hash = (component, discovery_id)

# Attach MQTT topic to the payload, used for debug prints

discovery_payload.discovery_data = {

ATTR_DISCOVERY_HASH: discovery_hash,

ATTR_DISCOVERY_PAYLOAD: discovery_payload,

ATTR_DISCOVERY_TOPIC: topic,

}

if discovery_hash in discovery_pending_discovered:

pending = discovery_pending_discovered[discovery_hash]["pending"]

pending.appendleft(discovery_payload)

_LOGGER.debug(

"Component has already been discovered: %s %s, queuing update",

component,

discovery_id,

)

return

async_process_discovery_payload(component, discovery_id, discovery_payload)

@callback

def async_process_discovery_payload(

component: str, discovery_id: str, payload: MQTTDiscoveryPayload

) -> None:

"""Process the payload of a new discovery."""

_LOGGER.debug("Process component discovery payload %s", payload)

discovery_hash = (component, discovery_id)

already_discovered = discovery_hash in mqtt_data.discovery_already_discovered

if (

already_discovered or payload

) and discovery_hash not in mqtt_data.discovery_pending_discovered:

discovery_pending_discovered = mqtt_data.discovery_pending_discovered

@callback

def discovery_done(_: Any) -> None:

pending = discovery_pending_discovered[discovery_hash]["pending"]

_LOGGER.debug("Pending discovery for %s: %s", discovery_hash, pending)

if not pending:

discovery_pending_discovered[discovery_hash]["unsub"]()

discovery_pending_discovered.pop(discovery_hash)

else:

payload = pending.pop()

async_process_discovery_payload(component, discovery_id, payload)

discovery_pending_discovered[discovery_hash] = {

"unsub": async_dispatcher_connect(

hass,

MQTT_DISCOVERY_DONE.format(*discovery_hash),

discovery_done,

),

"pending": deque(),

}

if component not in mqtt_data.platforms_loaded and payload:

# Load component first

config_entry.async_create_task(

hass, _async_component_setup(component, payload)

)

elif already_discovered:

# Dispatch update

message = (

f"Component has already been discovered:"

f" {component} {discovery_id}, sending update"

)

async_log_discovery_origin_info(message, payload)

async_dispatcher_send(

hass, MQTT_DISCOVERY_UPDATED.format(*discovery_hash), payload

)

elif payload:

_async_add_component(payload)

else:

entity_registry = er.async_get(hass)

if (

(

entity_hash := mqtt_data.discovery_discovered_and_disabled.pop(

discovery_hash, None

)

)

and (entity_id := entity_registry.entities.get_entity_id(entity_hash))

and (entity_entry := entity_registry.async_get(entity_id))

):

# Cleanup discovered disabled entity / device

entity_registry.async_remove(entity_id)

hass.async_create_task(

async_cleanup_device_registry(

hass,

device_id=entity_entry.device_id,

config_entry_id=entity_entry.config_entry_id,

),

name=f"Check for cleanup device registry for {entity_id}",

)

# Finish handling discovery message

async_dispatcher_send(

hass, MQTT_DISCOVERY_DONE.format(*discovery_hash), None

)

discovery_qos: int = config_entry.options.get(CONF_DISCOVERY_QOS, 0)

mqtt_data.discovery_unsubscribe = [

async_subscribe_internal(

hass,

topic,

async_discovery_message_received,

discovery_qos,

job_type=HassJobType.Callback,

)

# Subscribe first for platform discovery wildcard topics first,

# and then subscribe device discovery wildcard topics.

for topic in chain(

(

f"{discovery_topic}/{component}/+/config"

for component in SUPPORTED_COMPONENTS

),

(

f"{discovery_topic}/{component}/+/+/config"

for component in SUPPORTED_COMPONENTS

),

(

f"{discovery_topic}/device/+/config",

f"{discovery_topic}/device/+/+/config",

),

)

]

mqtt_data.last_discovery = time.monotonic()

mqtt_integrations = await async_get_mqtt(hass)

integration_unsubscribe = mqtt_data.integration_unsubscribe

async def _async_handle_config_entry_removed(entry: ConfigEntry) -> None:

"""Handle integration config entry changes."""

for discovery_key in entry.discovery_keys[DOMAIN]:

if (

discovery_key.version != 1

or not isinstance(discovery_key.key, str)

or discovery_key.key not in integration_discovery_messages

):

continue

topic = discovery_key.key

discovery_message = integration_discovery_messages[topic]

del integration_discovery_messages[topic]

_LOGGER.debug("Rediscover service on topic %s", topic)

# Initiate re-discovery

await async_integration_message_received(

discovery_message.integration, discovery_message.msg

)

mqtt_data.discovery_unsubscribe.append(

async_dispatcher_connect(

hass,

signal_discovered_config_entry_removed(DOMAIN),

_async_handle_config_entry_removed,

)

)

async def async_integration_message_received(

integration: str, msg: ReceiveMessage

) -> None:

"""Process the received message."""

if (

msg.topic in integration_discovery_messages

and integration_discovery_messages[msg.topic].msg.payload == msg.payload

):

_LOGGER.debug(

"Ignoring already processed discovery message for '%s' on topic %s: %s",

integration,

msg.topic,

msg.payload,

)

return

if TYPE_CHECKING:

assert mqtt_data.data_config_flow_lock

# Lock to prevent initiating many parallel config flows.

# Note: The lock is not intended to prevent a race, only for performance

async with mqtt_data.data_config_flow_lock:

data = MqttServiceInfo(

topic=msg.topic,

payload=msg.payload,

qos=msg.qos,

retain=msg.retain,

subscribed_topic=msg.subscribed_topic,

timestamp=msg.timestamp,

)

discovery_key = discovery_flow.DiscoveryKey(

domain=DOMAIN, key=msg.topic, version=1

)

discovery_flow.async_create_flow(

hass,

integration,

{"source": SOURCE_MQTT},

data,

discovery_key=discovery_key,

)

if msg.payload:

# Update the last discovered config message

integration_discovery_messages[msg.topic] = (

MQTTIntegrationDiscoveryConfig(integration=integration, msg=msg)

)

elif msg.topic in integration_discovery_messages:

# Cleanup cache if discovery payload is empty

del integration_discovery_messages[msg.topic]

integration_unsubscribe.update(

{

f"{integration}_{topic}": async_subscribe_internal(

hass,

topic,

functools.partial(async_integration_message_received, integration),

discovery_qos,

job_type=HassJobType.Coroutinefunction,

)

for integration, topics in mqtt_integrations.items()

for topic in topics

}

)

async def async_stop(hass: HomeAssistant) -> None:

"""Stop MQTT Discovery."""

mqtt_data = hass.data[DATA_MQTT]

for unsub in mqtt_data.discovery_unsubscribe:

unsub()

mqtt_data.discovery_unsubscribe = []

for key, unsub in list(mqtt_data.integration_unsubscribe.items()):

unsub()

mqtt_data.integration_unsubscribe.pop(key)

You can’t perform that action at this time.