# Evidence digest — linux-agent-jupiteros-fleet-15537b

115 claims ranked from 627 extracted; grouped by atomic item. Verbatim quotes are the load-bearing evidence; source note ids in brackets.


## Sub-Q1 verified defect audit (17 claims)

- paho-mqtt 0.14.0 AsyncClient exposes an explicit callback-based reconnect API: reconnect(), reconnect_with_callbacks(), set_connected_callback, set_connection_lost_callback, set_disconnected_callback, and set_message_callback — connection lifecycle events are delivered to application callbacks rather than a poll loop. — (empirical) [high]
  > "pub fn reconnect(&self) -> ConnectToken — Attempts to reconnect to the broker. This can only be called after a connection was initially made or attempted. It will retry with the same connect options. ... pub fn set_connected_callback<F>(&self, cb: F) ... pub fn set_connection_lost_callback<F>(&self, cb: F) — Sets the callback for when the connectio"
  [asyncclient-in-paho_mqttasync_client-rust]
- Z2M's availability supervisor schedules per-device pings with bounded random jitter: delay = (timeout + 1s + random*max_jitter) * backoff, and MAX_TIMEOUT=2147483647 caps the setTimeout delay because Node stores delays as 32-bit signed ints. — (empirical) [high]
  > "const jitter = Math.random() * this.getMaxJitter(device); ... setTimeout(this.addToPingQueue.bind(this, device), Math.min((this.getTimeout(device) + utils.seconds(1) + jitter) * backoff, MAX_TIMEOUT)); ... const MAX_TIMEOUT = 2147483647;"
  [availabilityts]
- Failed pings multiply the next backoff by 1.5 for a device previously marked available and by 2 when already unavailable, producing the documented sequence ×1.5, ×3, ×6, ×12 (with default timeout: 10, 15, 30, 60, 120 minutes), and pause_on_backoff_gt parks devices in availability sleep to stop unbounded growth. — (empirical) [high]
  > "this.pingBackoffs.set(device.ieeeAddr, currentBackoff * (available ? 1.5 : 2)); // results in backoffs: *1.5, *3, *6, *12... (with default timeout: 10, 15, 30, 60, 120)"
  [availabilityts]
- go-hass-agent's complete changelog from v4.3.1 through v14.15.1 (2026-08-09) contains zero mentions of select entities, MQTT control state synchronisation, or NixOS support — the select-with-state-sync control gap and NixOS integration gap persist through the current v14.x era. — (empirical) [high]
  > "States are not kept in sync. This is most important for all controls besides buttons. For example, if you configure a switch, any changes to the state you make outside of Home Assistant will not be reflected in Home Assistant automatically. (README, corroborating the changelog absence)"
  [changelogmd]
- rumqttc 0.25.1's EventLoop::poll() automatically reconnects to the broker if polling continues after a disconnection, making the poll loop itself the reconnect supervisor. — (empirical) [high]
  > "Yields Next notification or outgoing request and periodically pings the broker. Continuing to poll will reconnect to the broker if there is a disconnection. NOTE Don't block this while iterating"
  [eventloop-in-rumqttc-rust]
- mqtt_blackbox_exporter probes an MQTT broker by performing a publish/subscribe round-trip on the same topic: start subscriber, start publisher, publish N messages, count received messages — its definition of a completed probe. — (empirical) [high]
  > "Tests MQTT messaging roundtrips (publish/subscribe on same topic).  Definition of roundtrip:  start subscriber on $topic  start publisher on $topic  publish $messages on $topic  receive $message on $topic"
  [github-inovexmqtt_blackbox_exporter-prometheus-exporter-for-mqtt-monitoring-gith]
- go-hass-agent v14.15.1 is packaged in nixpkgs proper, is not marked broken, insecure, or unfree, and builds for all 24 Linux platforms nixpkgs supports — (empirical) [high]
  > "Install Version 14.15.1 ... Status Broken No Insecure No Unfree No Unsupported No ... Platforms 24 (24) Linux"
  [go-hass-agent-mynixos]
- The mobile_app integration page is a hub only: it states the integration lets HA mobile apps integrate with HA and is enabled by default, but it contains no notification schema or action-event details, deferring to companion and developer documentation. — (empirical) [high]
  > "The Mobile App integration lets Home Assistant mobile apps integrate with Home Assistant. If you are planning to use a mobile application that integrates with Home Assistant, we recommend that you keep this integration enabled. If you are a mobile app developer, see the developer documentation for instructions on how to build your app on top of the"
  [mobile-app-home-assistant]
- nixpkgs ships an in-tree Mosquitto NixOS VM test at nixos/tests/mosquitto.nix with one server node and two client nodes, structured as { name; nodes; testScript; }. — (empirical) [high]
  > "{ name = "mosquitto"; ... nodes = { server = { pkgs, ... }: { networking.firewall.allowedTCPPorts = [ port tlsPort anonPort ]; networking.useNetworkd = true; services.mosquitto = { enable = true; ... listeners = [ ... ]; }; }; client1 = client; client2 = client; }; testScript = '' ... ''; }"
  [mosquittonix]
- Zigbee2MQTT's availability feature uses retained availability messages with asymmetric check-in windows — active devices 10 minutes, passive (battery) devices 25 hours (1500 minutes) — plus ping backoff (x1.5/x3/x6/x12), 30s max jitter, per-device overrides, and persisted timeout state across restarts. — (empirical) [high]
  > "Active devices (non battery-powered): by default they have to check-in every 10 minutes. If they don't, they will be pinged, if that fails the device will be marked as offline. Passive devices (battery-powered): by default they have to check-in every 25 hours."
  [mqtt-availability-community]
- Zigbee2MQTT persists availability timeout state between restarts — if Z2M is stopped longer than the timeout, all active devices are initially marked offline until they check in again. — (empirical) [high]
  > "Note that this timeout is persisted between Zigbee2MQTT restarts. So if you for example stop Zigbee2MQTT for longer than 10 minutes, all your active devices will be marked as offline initially until they check-in again."
  [mqtt-availability-community]
- HA's discovery processing log ('Found new component: switch dev1' followed by 'Subscribing with mid: 24 to topics with qos: [...]') is the verifiable debug signature that a discovery payload was accepted and its subscriptions registered. — (empirical) [high]
  > "2024-08-09 14:21:48.706 INFO (MainThread) [homeassistant.components.mqtt.discovery] Found new component: switch dev1 ... Subscribing with mid: 24 to topics with qos: [('b/office/dev1/available', 0), ('b/office/dev1/state', 0)]"
  [mqtt-discovery-availability-topics-not-receiving-configuration-home-assistant-co]
- The Home Assistant MQTT notify entity publishes the notify.send_message payload to a configured command_topic, with an optional command_template to generate the payload, but the platform defines no title/message/actions JSON schema and no event round-trip for actionable notifications. — (empirical) [high]
  > "The MQTT notify integration lets you send an MQTT message when the send_message action is called. This can be used to expose a action of a remote device that allows processing a message, such as showing it on a screen. ... command_template template (Optional): Defines a template to generate the payload to send to command_topic. ... command_topic st"
  [mqtt-notify-home-assistant]
- The mosquitto issue URL cited by both official Telegraf MQTT-output sources (github.com/eclipse/mosquitto/issues/2117) returns 404 because the GitHub org was renamed to eclipse-mosquitto — the issue is reachable only at github.com/eclipse-mosquitto/mosquitto/issues/2117 — (empirical) [high]
  > "Client error '404 Not Found' for url 'https://github.com/eclipse/mosquitto/issues/2117' (fetch result) — the issue lives at eclipse-mosquitto/mosquitto#2117: 'No keepalive (0) still allowed with max_keepalive set'"
  [no-keepalive-0-still-allowed-with-max_keepalive-set-issue-2117-eclipse-mosquitto]
- Home Assistant's built-in Prometheus integration exposes only HA entity-level metrics at /api/prometheus for an external Prometheus server to scrape (pull model with long-lived access token); it does not export host/OS metrics. — (empirical) [high]
  > "The Prometheus integration exposes metrics in a format which Prometheus can read. ... Metrics are exported only for the following domains: alarm_control_panel, automation, binary_sensor, climate, cover, counter, device_tracker, fan, geo_location, humidifier, input_boolean, input_number, light, lock, number, person, sensor, switch, update, water_hea"
  [prometheus-home-assistant]
- lnxlink's Python dependency surface is heavy and GUI/audio/X11-coupled (pygobject3, ewmh, xlib, pulsectl, pyalsaaudio, opencv-python, flask, waitress, docker, speechrecognition alongside paho-mqtt/psutil/jeepney), which complicates headless NixOS server packaging and explains why no one has upstreamed a module. — (empirical) [high]
  > "propagatedBuildInputs = [ distro pyyaml paho-mqtt requests psutil inotify jeepney aiohttp setuptools wheel pygobject3 speechrecognition docker ewmh flask mss numpy opencv-python pulsectl pyalsaaudio xlib vdf waitress ]"
  [python-packaging-error-needs-specific-setuptools-version-nixos-discourse]
- go-hass-agent v14.15.1 fixed a concurrent map-read data race in the CPU usage worker — a defect class that produces intermittent, unreproducible agent misbehaviour — (empirical) [high]
  > "14.15.1 (2026-08-09) Bug Fixes cpu: guard the usage worker's reading maps (4ad1c07)"
  [releases-joshuargo-hass-agent-github]

## Sub-Q2 MQTT lifecycle (32 claims)

- A second independent real-world witness of the silent-drop mechanism (mosquitto 2.0.16, unrelated to Home Assistant): the broker logged 'Outgoing messages are being dropped for client auto-BBB3F4DC-...' when the queue limit was hit, with no error delivered to the subscriber. — (empirical) [high]
  > "1692604552: Outgoing messages are being dropped for client auto-BBB3F4DC-A2B8-A350-CAE6-B3B99F9DAC9B."
  [all-messages-dropped-when-max_queued_messages-0-issue-2879-eclipse-mosquittomosq]
- Availability publishes are retained QoS 1 JSON payloads {state: online|offline} on <friendlyName>/availability, and identical consecutive states are deduped via a lastPublishedAvailabilities map. — (empirical) [high]
  > "const topic = `${entity.name}/availability`; const payload: Zigbee2MQTTAPI["{friendlyName}/availability"] = {state: available ? "online" : "offline"}; ... await this.mqtt.publish(topic, JSON.stringify(payload), {clientOptions: {retain: true, qos: 1}});"
  [availabilityts]
- On device reconnect (announce or offline→online transition) Z2M retrieves device state via a 2-second-debounced retrieveState that reads state/brightness/colour per RETRIEVE_ON_RECONNECT, and on rename it clears the old availability topic by publishing an empty retained payload. — (empirical) [high]
  > "await this.mqtt.publish(`${data.from}/availability`, "", {clientOptions: {retain: true, qos: 1}}); ... if (entity.isDevice() && available && this.lastPublishedAvailabilities.get(entity.ieeeAddr) === false) { logger.debug(`Device '${entity.name}' reconnected`); this.retrieveState(entity); }"
  [availabilityts]
- Root cause of HA #135266: the MQTT broker's allowed maximum number of queued messages was too low — at HA startup the replay of retained discovery/state messages for thousands of entities overflowed the broker's per-client queue, which silently dropped retained messages; raising the limit was the instant cure. — (empirical) [high]
  > "I have raised the allowed maximum number of queued messages on my broker, and that seems to be the instant cure for the problem."
  [comments-135266]
- The broker dropped messages silently — MQTT Explorer connected to the same broker at the same time showed the retained messages present, while HA's debug logs and device diagnostics showed empty message lists for those topics. — (empirical) [high]
  > "in MQTT Explorer I can see messages on them. I have the discovery prefix differ from the MQTT prefix in zwavejs. But I can see both the discovery and the usual state topics on the broker from MQTT Explorer, HA still does not have any of these messages either in the debug logs, or in the MQTT Device diagnostics."
  [comments-135266]
- HA's MQTT debug log at startup shows a single wildcard subscription ('Subscribing with mid: 11 to topics with qos: [("#", 0)]') which triggers the retained-message replay; a manual listen on '#' later in the session re-triggered delivery of exactly the dropped messages. — (empirical) [high]
  > "It is really strange, that the retained messages are only arriving after I have issued the listen on the '#' topic, below this line in the log: 2025-01-10 22:35:14.695 DEBUG (MainThread) [homeassistant.components.mqtt.client] Subscribing with mid: 11 to topics with qos: [('#', 0)]"
  [comments-135266]
- HA core abbreviations.py defines the complete legal set of abbreviated MQTT discovery keys, expanded by discovery.py before validation — keys such as avty_t (availability_topic), cmd_t (command_topic), stat_t (state_topic), uniq_id (unique_id), val_tpl (value_template), dev_cla (device_class), ent_cat (entity_category), en (enabled_by_default), pl_on/pl_off (payload_on/off), ret (retain) and '~' topic-base. — (empirical) [high]
  > """"Abbreviations supported by MQTT discovery.""" ABBREVIATIONS = { "act_t": "action_topic", ... "avty": "availability", "avty_mode": "availability_mode", "avty_t": "availability_topic", "avty_tpl": "availability_template", ... "cmd_t": "command_topic", ...}"
  [corehomeassistantcomponentsmqttabbreviationspy-at-dev-home-assistantcore-github]
- The mqtt notify platform implementation (mqtt/notify.py) is an entity platform inside the mqtt integration wired through async_setup_entity_entry_helper and MqttEntity, requiring command_topic and supporting command_template/retain/encoding — i.e., discovery-registered notify entities are first-class MQTT entities. — (empirical) [high]
  > """"Support for MQTT notify.""" ... from .const import CONF_COMMAND_TEMPLATE, CONF_COMMAND_TOPIC, CONF_RETAIN ... from .entity import MqttEntity, async_setup_entity_entry_helper"
  [corehomeassistantcomponentsmqttnotifypy-at-dev-home-assistantcore-github]
- Queue admission is PER-CLIENT and counts queued messages against max_queued_messages (default 1000): db__ready_for_queue computes valid_count = source_count - adjust_count < db.config->max_queued_messages on the client's own msg_data, and for a disconnected client the inflight adjustment zeroes out (adjust_count = 0 when !net__is_connected), making the full queue depth available to the offline replay. — (empirical) [high]
  > "valid_count = source_count - adjust_count < db.config->max_queued_messages; /* nothing in flight for offline clients */ if(!net__is_connected(context)){ adjust_bytes = 0; adjust_count = 0; }"
  [databasec]
- mosquitto(8)'s $SYS/broker/mqtt/* topic listing (e.g. $SYS/broker/mqtt/publish/dropped, $SYS/broker/mqtt/connect/received) is wrong as of 2.1.2: sys_tree.c never publishes those topics ('Not published in $SYS, may be made available for plugins'), so a $SYS monitoring agent must use $SYS/broker/publish/messages/dropped and the load tree, not the mqtt/* variants. — (empirical) [high]
  > "These are never published, as of version 2.1.2. In sys_tree.c, these topics are excluded explicitly from being published. The associated commit states 'Not published in $SYS, may be made available for plugins.'"
  [documentation-on-sysbrokermqtt-topics-is-wrong-issue-3726-eclipse-mosquittomosqu]
- HASS.Agent's release history documents recurring MQTT-discovery and sensor-pipeline reliability failures of exactly the class a fleet-wide Linux agent must defend against: autodiscovery messages not republished after broker reconnect (#230), MQTT entity IDs changing on device rename (#388), discovery messages not removed on rename (#376), and all WMI sensors stopping when the WMI service restarts (#205) — (statistical) [high]
  > "MQTT autodiscovery messages were not republished after connection was lost and recovered (thanks to @whc2001 for reporting) #230 ... Fixed MQTT entities changing ID after device rename #388 ... Restarting "Windows Management Instrumentation (WMI)" while HASS.Agent is running causes all WMI based sensors to stop functioning #205"
  [github-hass-agenthassagent-unofficial-development-project-for-the-hassagent-plat]
- IoPC's MQTT design is a two-prefix service bus: inbound commands on {PREFIX}/{CLIENT_ID}/command/# with JSON {service, payload} envelopes, outbound sensor values on {PREFIX}/{CLIENT_ID}/#, configured via web UI (127.0.0.1:60555) or IOPC_MQTT_* env vars — (empirical) [high]
  > "By default, inbound MQTT prefix for IoPC instance is {MQTT_PREFIX}/{MQTT_CLIENT_ID}/command/#. ... For example, we want to set PC volume from HomeAssistant. We should send following message to topic iopc/myComputer/command/homeassistant {"service": "volume.set", "payload": {"volume": 54}}"
  [github-maksimkurbiopc-iopc-aka-internet-of-pc-helps-you-to-integrate-pc-into-you]
- pkg.go.dev's record for go-hass-agent is stale at v1.4.3 (published Jul 2023, flagged as not latest) versus the v14.x current releases, with 0 external importers — (empirical) [high]
  > "Version: v1.4.3 ... This package is not in the latest version of its module. ... Published: Jul 8, 2023 ... Imported by: 0"
  [go-hass-agent-command-githubcomjoshuargo-hass-agent-go-packages]
- rumqttc's MqttOptions defaults to clean_session=true, keep_alive=60s, inflight=100 — meaning a default-configured client discards its pending queue and loses all subscriptions on every reconnect. — (empirical) [high]
  > "pub fn new(id: S, host: T, port: u16) -> MqttOptions { MqttOptions { ... keep_alive: Duration::from_secs(60), clean_session: true, ... inflight: 100, last_will: None, ..."
  [librs]
- Mosquitto's max_queued_messages defaults to 1000 QoS 1 or 2 messages held in the per-client queue above in-flight messages, and 0 means no maximum (explicitly not recommended). — (empirical) [high]
  > "max_queued_messages count — The maximum number of QoS 1 or 2 messages to hold in the queue (per client) above those messages that are currently in flight. Defaults to 1000. Set to 0 for no maximum (not recommended)."
  [mosquittoconf-man-page-eclipse-mosquitto]
- max_queued_bytes defaults to 0 (no byte limit); when both it and max_queued_messages are set, packets queue until the FIRST limit is reached, and subsequent messages are then silently dropped. — (empirical) [high]
  > "The number of outgoing QoS 1 and 2 messages above those currently in-flight will be queued (per client) by the broker. Once this limit has been reached, subsequent messages will be silently dropped. ... Defaults to 0. (No maximum). ... If both max_queued_messages and max_queued_bytes are specified, packets will be queued until the first limit is re"
  [mosquittoconf-man-page-eclipse-mosquitto]
- queue_qos0_messages defaults to false — QoS 0 messages are NOT queued for disconnected persistent clients (queuing them is a documented non-standard deviation from MQTT v3.1.1), and if enabled they count toward max_queued_messages. — (empirical) [high]
  > "Set to true to queue messages with QoS 0 when a persistent client is disconnected. ... These messages are included in the limit imposed by max_queued_messages. Defaults to false. Note that the MQTT v3.1.1 spec states that only QoS 1 and 2 messages should be saved in this situation so this is a non-standard option."
  [mosquittoconf-man-page-eclipse-mosquitto]
- Mosquitto persistence defaults to false: connection, subscription and message data is only written to mosquitto.db when persistence is enabled, at shutdown and at autosave_interval (default 1800 seconds / 30 minutes), or on SIGUSR1. — (empirical) [high]
  > "If true, then built-in persistence is enabled. ... If enabled, connection, subscription and message data will be written to disk in mosquitto.db at the location dictated by persistence_location. ... The data will be written to disk when mosquitto closes and also at periodic intervals as defined by autosave_interval. ... If false, the data will be s"
  [mosquittoconf-man-page-eclipse-mosquitto]
- autosave_interval counts seconds between in-memory-database saves (default 1800 s = 30 minutes), only effective when built-in persistence is enabled, and 0 restricts saves to exit/SIGUSR1. — (empirical) [high]
  > "The number of seconds that mosquitto will wait between each time it saves the in-memory database to disk. If set to 0, the in-memory database will only be saved when mosquitto exits or when receiving the SIGUSR1 signal. Note that this setting only has an effect if the built-in persistence is enabled. Defaults to 1800 seconds (30 minutes)."
  [mosquittoconf-man-page-eclipse-mosquitto]
- retain_available defaults to true and retain_expiry_interval governs when retained messages carrying a MQTT5 message-expiry-interval are removed; allow_duplicate_messages is deprecated with behaviour defaulting to true. — (empirical) [high]
  > "retain_available [ true | false ] — If set to false, then retained messages are not supported. Clients that send a message with the retain bit will be disconnected if this option is set to false. Defaults to true. ... allow_duplicate_messages [ true | false ] — This option is deprecated and will be removed in a future version. The behaviour will de"
  [mosquittoconf-man-page-eclipse-mosquitto]
- The test also covers TLS listeners using snakeoil certificates generated inside the test with pkgs.runCommand + gnutls certtool, and an allow_anonymous listener gated by omitPasswordAuth. — (empirical) [high]
  > "snakeOil = pkgs.runCommand "snakeoil-certs" { buildInputs = [ pkgs.gnutls.bin ]; ... } ... { port = tlsPort; ... settings = { cafile = "${snakeOil}/ca.crt"; certfile = "${snakeOil}/server.crt"; keyfile = "${snakeOil}/server.key"; require_certificate = true; use_identity_as_username = true; }; }"
  [mosquittonix]
- Home Assistant's MQTT integration publishes a birth message 'online' to homeassistant/status when the MQTT integration starts, and devices using MQTT discovery must subscribe to this topic and re-send their discovery payload in response to restore entities after HA restarts. — (empirical) [high]
  > "When MQTT starts up, all existing MQTT devices, entities, tags, and device triggers, will be unavailable until a discovery message is received and processed. A device or service that exposes the MQTT discovery should subscribe to the Birth message and use this as a trigger to send the discovery payload. To avoid high IO loads on the MQTT broker, ad"
  [mqtt-home-assistant]
- MQTT entity availability is configured per-entity via availability topic(s) with payload_available ('online' default) / payload_not_available ('offline' default), and availability_mode (all/any/latest) controls when an entity is marked online. — (empirical) [high]
  > "A device or service can announce its availability by publishing a Birth message and set a Will message at the broker. When the device or service loses connection to the broker, the broker will publish the Will message. This allows the MQTT integration to make an entity unavailable."
  [mqtt-home-assistant]
- Enabling a previously disabled entity more than 30 seconds after startup causes the MQTT integration to reload and unload all discovered MQTT entities. — (empirical) [high]
  > "If a disabled entity is enabled and added after 30 seconds, the MQTT integration will be reloaded and will cause all discovered MQTT entities to be unloaded."
  [mqtt-home-assistant]
- The MQTT notify platform supports availability topics, qos, retain, encoding, and message_expiry_interval for queued/retained messages to offline subscribers, but none of these provide interactivity — the payload is one-way outbound from HA to the remote device. — (empirical) [high]
  > "message_expiry_interval map (Optional): Controls how long queued or retained messages sent from Home Assistant persist at the broker for offline subscribers."
  [mqtt-notify-home-assistant]
- After an HA restart, MQTT sensors discovered by a non-retained discovery message became unavailable and stayed broken until the user deleted the entity and resent the discovery message — HA did not itself re-request or persist discovery. — (empirical) [high]
  > "A lot of my MQTT sensors are unavailable after a restart. The solution to get them back is to delete the entity, and resend the discovery message."
  [mqtt-sensors-unavailable-in-01140bx-others-stay-unknown-issue-38661-home-assista]
- Z2M's MQTT config defaults keepalive to 60 s, protocol version 4 (version 5 opt-in), and its publisher state machinery uses cache_state + cache_state_persistent + cache_state_send_on_startup — full state cached to disk and republished on startup. — (empirical) [high]
  > "# Optional: MQTT keepalive in seconds (default: 60) keepalive: 60 ... # Optional: state caching, MQTT message payload will contain all attributes, not only changed ones. # Has to be true when integrating via Home Assistant (default: true) cache_state: true ... # Optional: persist cached state, only used when cache_state: true (default: true) cache_"
  [mqtt-zigbee2mqtt]
- NixOS provides a services.mosquitto module whose default listener ACL is empty and therefore disallows all client access until ACLs are explicitly configured (e.g. acl = [ "pattern readwrite #" ] for a fully open broker). — (empirical) [high]
  > "The default ACL for a listener is empty, disallowing all accesses from all clients. To configure a completely open ACL, set acl = [ "pattern readwrite #" ] in the listener."
  [nixos-manual]
- Live $SYS dump from a loaded broker (v2.0.15/2.0.18) confirms the drop counters are actually published: '$SYS/broker/publish/messages/dropped 0' alongside '$SYS/broker/publish/messages/received 1000000' and '$SYS/broker/store/messages/count 51' = '$SYS/broker/retained messages/count 51' (no queue divergence). — (empirical) [high]
  > "$SYS/broker/publish/messages/dropped 0 $SYS/broker/publish/messages/received 1000000 ... $SYS/broker/retained messages/count 51"
  [no-data-in-sysbrokermessagesinflight-issue-3021-eclipse-mosquittomosq]
- Mosquitto issue #2117 ('No keepalive (0) still allowed with max_keepalive set', opened 2021-03-07, closed, milestone 2.0.9): even with max_keepalive set in mosquitto's config, clients could CONNECT with keepalive=0 and receive no Server Keep Alive in the connack — the root cause behind Telegraf's documented 'identifier rejected' failures with keep_alive=0 — (empirical) [high]
  > "Even if max_keepalive is set in the config file, clients are still allowed to connect using 0 (i.e. no keepalive) without a Server Keep Alive being sent in the connect ack. I'm not sure whether this is a bug, but it was certainly unexpected behaviour to me after reading the v5 spec and mosquitto documentation."
  [no-keepalive-0-still-allowed-with-max_keepalive-set-issue-2117-eclipse-mosquitto]
- rumqttc v0.25.0 added MQTT v5 session_expiry_interval support, v5 Auth packet support, public DisconnectProperties, TCP no_delay, set_client_id in MqttOptions, and fixed subscribe_many EmptySubscription errors and poll() event ordering — but no auto re-subscribe feature. — (empirical) [high]
  > "Enhanced MQTT v5 Support — Session Management: Added support for session_expiry_interval in MQTT v5 connections, giving you better control over session persistence ... Subscribe Many Fix: Resolved critical issue where subscribe_many always caused EmptySubscription errors ... Improved event ordering in poll() method for more predictable behavior"
  [releases-bytebeamiorumqtt-github]
- rumqttc v0.24.0 was released 21 May by de-sh and its headline features were EventLoop::clean for graceful shutdown, TLS key-format expansion (removal of the Key enum), and a websocket request modifier — plus a change where set_clean_session now panics on empty client_id. — (empirical) [high]
  > "MqttOptions::new now accepts empty client_id and MqttOptions::set_clean_session panics if client_id is empty and clean_session flag is set to false. ... Expose EventLoop::clean to allow triggering shutdown and subsequent storage of pending requests by @de-sh in #741"
  [releases-bytebeamiorumqtt-github]

## Sub-Q3 proper Linux agent comparison (21 claims)

- HASS.Agent (the Windows agent) implements actionable notifications that 'work the same way as the companion apps do': the notify service call carries data.actions with action/title pairs, buttons appear on the notification, and clicking a button fires an HA event of type 'hass_agent_notifications' (bound to a device via device_name) that triggers automations — a complete action-button round-trip. — (empirical) [high]
  > "HASS.Agent's actionable notifications work the same way as the companion apps do. ... As soon as you click one of the buttons, the action name will get triggered. Every automation that has that trigger will activate. ... Make sure you enter hass_agent_notifications as the Event type."
  [actionable-notifications-hassagent]
- The companion-app actionable-notification contract: a notify action includes data.actions (each with required action key 'passed back in events' and title), and when a button is tapped the mobile_app_notification_action event fires in HA with the action identifier plus optional reply_text, action_data and tag — this is the platform-native round-trip that MQTT notify lacks. — (empirical) [high]
  > "Actionable notifications are a unique type of notification as they allow the user to add buttons to the notification which can then send an event to Home Assistant once clicked. ... action Required. The identifier passed back in events ... When the notification action is performed, the mobile_app_notification_action event fires"
  [actionable-notifications-home-assistant-companion-docs]
- The MQTT notify platform was added to HA core by PR #115653 (jbouwh, Apr 15 2024), labelled new-feature/new-platform/integration:mqtt/has-tests/Quality Scale: gold, merged for the 2024.5 release together with docs PR home-assistant.io#32327. — (empirical) [high]
  > "Add mqtt notify platform ... labels core has-tests integration: mqtt new-feature new-platform"
  [add-mqtt-notify-platform-by-jbouwh-pull-request-115653-home-assistantcore-github]
- HA 2024.5 shipped the notify entity model and the MQTT notify platform in the same release ('Add notify entity component' #110950; 'Add mqtt notify platform' #115653), so any HA >= 2024.5 instance (including callisto's connected HA) can host MQTT-notify-via-discovery entities. — (empirical) [high]
  > "Add notify entity component ([@jbouwh] - #110950) ... Add mqtt notify platform ([@jbouwh] - #115653)"
  [add-mqtt-notify-platform-by-jbouwh-pull-request-115653-home-assistantcore-github]
- go-hass-agent maintained a roughly monthly release cadence through 2026 (v14.10.x in March 2026 through v14.15.1 on 2026-08-09) with substantive fixes — concurrent-map-read race in the CPU usage worker (v14.15.1), NVIDIA/AMD GPU memory sensors (v14.15.0), ddcutil backlight detection (v14.10.3), PipeWire volume-control fixes (v14.11.0) — confirming the project is actively maintained but investing in sensors/hardware support rather than control state sync or NixOS. — (empirical) [high]
  > "14.15.1 (2026-08-09) Bug Fixes: cpu: guard the usage worker's reading maps"
  [changelogmd]
- lnxlink is a Linux-only HA companion app built on MQTT Autodiscovery (Python, 480 stars, 790 commits), requiring no sudo except in server environments, installable via pipx/Docker/Flathub/AUR, with an auto-importing modular architecture for custom modules — (empirical) [high]
  > "LNXlink is a Linux companion app that seamlessly integrates your system with external applications like Home Assistant. It uses MQTT ... Utilizes MQTT Autodiscovery to create entities in Home Assistant dashboard. ... Easy Installation: No sudo privileges required for installation and operation, except for server environments. ... Expandable Archite"
  [github-bkbillylnxlink-effortlessly-manage-your-linux-machine-using-mqtt-github]
- HA_Desktop_Companion (GamerClassN7, Windows, C#) is a Home Assistant desktop companion built specifically to avoid MQTT, using the native HA API with a long-lived token, inspired by ESPHome's native communication protocol. — (empirical) [high]
  > "Cause I don't like existing implementations using MQTT and I took inspiration from awesome ESphome and its native communication protocol to HA and implemented it my own way :)"
  [github-gamerclassn7ha_desktop_companion-app-which-is-using-native-ha-api-to-comu]
- HASS.Agent 2.2.0 removed the LibreHardwareMonitor library (WinRing0 kernel-driver security issue) and as a result the GPU Temperature Sensor is permanently non-functioning (always returns 0) — (empirical) [high]
  > "Due to security concerns, we were forced to remove Libre Hardware Monitor library that allowed HASS.Agent to access hardware information. The breaking change in this is that starting with this release, the GPU Temperature Sensor is non-functioning. It has been left present for backward compatibility reasons, however it will always return 0."
  [github-hass-agenthassagent-unofficial-development-project-for-the-hassagent-plat]
- HASS.Agent 2.2.0 added the ability to test the MQTT connection during onboarding and an experimental WebSocket option for MQTT connections — (empirical) [high]
  > "Added ability to test MQTT connection during onboarding process (thanks to @jgstew for suggestion) #379 ... Added WebSocket support for MQTT connections (experimental :)) #253"
  [github-hass-agenthassagent-unofficial-development-project-for-the-hassagent-plat]
- go-hass-agent is a dual-transport agent: it registers with Home Assistant as a mobile_app device via the Native App Integration API (long-lived token) and cannot run in MQTT-only mode, while an optional MQTT v5 side adds controls and additional sensors — (empirical) [high]
  > "Unfortunately no, Go Hass Agent cannot run in an MQTT-only credentials. It makes use of the Native App Integration API that is not MQTT only. ... Go Hass Agent requires MQTT v5 support on your MQTT broker."
  [github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device]
- go-hass-agent's Linux sensor surface is sourced from D-Bus (logind sessions, power state, screen lock, NetworkManager connections/Wi-Fi, XDG portal active-app/desktop settings, UPower battery, MPRIS, fwupd), ProcFS (memory, disk usage, load, CPU usage/frequency, uptime, kernel, distro, CPU vulns), SysFS (disk IO rates, hwmon hardware sensors), Netlink (link states/rates), and geoclue (location), each with its own update cadence from ~5s (disk/network rates) to ~15min (uptime, ABRT) — (empirical) [high]
  > "Active App and Total Running Apps ... Via D-Bus (requires XDG Desktop Portal Support support). ... Memory Stats ... Sourced via ProcFS. Updated ~every minute. ... Both sourced via SysFS. Updated ~every 5 seconds. ... Device/Link State Via Netlink."
  [github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device]
- Disk SMART monitoring in go-hass-agent requires file capabilities on the binary (cap_sys_rawio, cap_sys_admin, cap_mknod, cap_dac_override=+ep) which the rpm/deb/arch packages and containers apply automatically, and user-activity detection requires cap_setgid/cap_setuid plus membership of the input group — (empirical) [high]
  > "Requires the following capabilities on the Go Hass Agent binary (already applied for containers and rpm/deb/arch packages): cap_sys_rawio,cap_sys_admin,cap_mknod,cap_dac_override=+ep. ... Requires user running Go Hass Agent is in the input group."
  [github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device]
- go-hass-agent's MQTT controls include arbitrary D-Bus method invocation via JSON payloads on the topic gohassagent/HOSTNAME/dbuscommand, plus a commands.toml file defining button, switch, and number controls that run executables without a shell and WITHOUT state synchronisation back to HA — (empirical) [high]
  > "The agent will subscribe to the MQTT topic gohassagent/HOSTNAME/dbuscommand ... States are not kept in sync. This is most important for all controls besides buttons. For example, if you configure a switch, any changes to the state you make outside of Home Assistant will not be reflected in Home Assistant automatically."
  [github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device]
- go-hass-agent script sensors accept any shebang script (bash, python, etc.) emitting JSON/YAML/TOML with a schedule field using Quartz cron expressions or @every <duration> intervals, and each script can emit multiple sensors with icon, state, units, device_class, state_class, and attributes — (empirical) [high]
  > "Each script run by the agent can create one or more sensors and each script can run on its own schedule, specified using a Cron syntax. ... Go Hass Agent makes no attempt to do any analysis or sanitization of script output, other than ensuring the output is a supported format."
  [github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device]
- IoPC sensors are NOT updated automatically — a service call (e.g. volume.get) must be invoked to refresh sensor values; the sensor set is availability state, volume, RAM free/total, CPU usage %, per-drive info, lastUserInput, and an activity state machine (active/idle at 30s/away at 5min) — (empirical) [high]
  > "For now, sensors are not updated automatically, so you should run a special service to update sensor value. ... activity/state: User state. Can be active, idle (if not active for 30sec), away (if not active for 5min), unknown"
  [github-maksimkurbiopc-iopc-aka-internet-of-pc-helps-you-to-integrate-pc-into-you]
- halinuxcompanion receives actionable HA notifications by running a LOCAL aiohttp HTTP server that Home Assistant POSTs notification service calls to, with D-Bus used to display notifications and listen for action/clear events, plus D-Bus sleep/shutdown signals to update a Status sensor immediately before power events — (empirical) [high]
  > "HTTP Server (aiohttp): Listen to POST notification service call from Home Assistant ... Dbus interface (dbus_next): Sending notifications and listening to notification actions from the desktop, also listens to sleep, shutdown to update the status sensor"
  [github-muniterhalinuxcompanion-homeassistant-linux-companion-github]
- halinuxcompanion registers as a mobile_app device in Home Assistant and exposes declarative notification-attached commands (suspend/poweroff/reboot/hibernate via systemctl, xdg-open, flatpak run) configured in config.json, run as a systemd --user service without sudo — (empirical) [high]
  > "Now in your Home Assistant you will see a new device in the "mobile_app" integration, and there will be a new service to notify your Linux desktop. Notification actions work and the expected events will be fired in Home Assistant. ... "command_suspend": {"name": "Suspend", "command": ["systemctl", "suspend"]}"
  [github-muniterhalinuxcompanion-homeassistant-linux-companion-github]
- The companion-app clear_notification contract is documented verbatim: 'You can clear an existing notification which has a tag by sending clear_notification' — message: "clear_notification" plus the original tag in data (iOS requires app 2021.5+); notifications are replaced, not cleared, by reusing a tag. — (empirical) [high]
  > "Replacing: Replace an existing notification by using a tag for the notification. All subsequent notifications will take the place of a notification with the same tag. ... Clearing: You can clear an existing notification which has a tag by sending clear_notification. ... message: "clear_notification" ... data: tag: "backyard-motion-detected""
  [introduction-home-assistant-companion-docs]
- lnxlink (2023.6.0 README on PyPI) is a Linux MQTT companion app for HA that uses MQTT autodiscovery and explicitly supports headless server installation, installed as root with a system service when there is no graphical interface. — (empirical) [high]
  > "The headless installation is used for linux environments that don't use a Graphical Interface like servers. ... sudo pip3 install -U lnxlink ... When asked, it's recommended to answer false on install as a user service. ... sudo systemctl restart lnxlink.service"
  [lnxlink-pypi]
- lnxlink's module catalogue (~50 modules) defines the fullest feature envelope of any Linux HA agent, including rich desktop notifications, URL/file opening, keep-alive (prevent monitor sleep), idle-time sensing, media control with metadata, brightness control, screen on/off, fullscreen detection, desktop screenshot streaming to an HA image entity, audio device selection, keyboard hotkey capture (X11-only, not Wayland), mouse simulation, Steam game launcher, active window monitoring, clipboard view/update, unlocked-graphical-user monitoring, boot-select, power-profile toggling, per-unit systemd service management, NVIDIA/AMD GPU usage, restart-required and system-update tracking, disk IO/usage/mounts, Bluetooth control with device battery, WiFi strength, WOL toggle, BeaconDB WiFi-triangulation location, mic/speaker/camera/gamepad in-use sensors, webcam switch+feed, fingerprint, speech, GPIO, IR remote, bash custom entities, docker management, in-HA self-update, dynamic log level, and a sensor-collection-latency probe — (empirical) [high]
  > "Notify: Send rich desktop notifications via notify.send_message ... Audio Select: Switch between available speaker or microphone input devices. ... Keyboard Hotkeys: Capture specific keypresses for automation triggers (Not for Wayland) ... SystemD: Manage Linux services; check status, start, or stop specific units. ... Screenshot: Stream your deskt"
  [lnxlink]
- go-hass-agent's current release is v14.15.1 (2026-08-09) with an active ~monthly cadence through 2026, resolving the nixpkgs v14.15.1 vs pkg.go.dev v1.4.3 discrepancy in favour of nixpkgs being current — (empirical) [high]
  > "v14.15.1 ... Latest ... released this 09 Aug 01:21 ... 14.15.1 (2026-08-09) Bug Fixes cpu: guard the usage worker's reading maps"
  [releases-joshuargo-hass-agent-github]

## Sub-Q4 NixOS fleet architecture (6 claims)

- As of mid-2026 NixOS unstable, nixos-rebuild switch DOES restart systemd user units other than nixos-activation.service — a behaviour change introduced via the switch-to-configuration-ng Rust rewrite that replaced the old reload-only handling. — (empirical) [high]
  > "Namely, the change was that `nixos-rebuild switch` now restarts user units other than `nixos-activation.service`. Since that change, some user services (`niri.service`, `gnome-session-monitor.service`, there might be more) started restarting on `nixos-rebuild switch` and causing instability"
  [anyone-seeing-nixos-rebuild-switch-kick-you-out-to-gdm-andor-log-gpf-errors-spec]
- The user-unit restart regression fired first on 2026-05-26, isolating nixpkgs 26.05.20260515.d233902 → 26.05.20260523.64c08a7 with systemd constant at 260.1, implicating PR #517768 commits 6ced06a1b and 76c8d4509 as the trigger. — (empirical) [high]
  > "The switch that first did it moved nixpkgs `26.05.20260515.d233902` → `26.05.20260523.64c08a7`. systemd is 260.1 on both sides, so it isn't a systemd bump. The only commits in that range touching the relevant code are from #517768"
  [anyone-seeing-nixos-rebuild-switch-kick-you-out-to-gdm-andor-log-gpf-errors-spec]
- systemd_mon subscribes to systemd unit state changes via DBus with no polling or busy loops, firing notifications only on failed-state entry/exit, and queues rapid intermediate state transitions to notify once with full history. — (empirical) [high]
  > "The command line tool runs as a daemon, using dbus to get notifications of changes to systemd services. ... It works by subscribing to DBus notifications from Systemd. This means that there is no polling, and no busy-loops. ... SystemdMon queues up states until it comes across one that you think you should know about."
  [github-joontysystemd_mon-monitor-for-systemd-to-alert-failed-services-github]
- go-hass-agent supports containerised deployment with alternative system mount points via PROCFS_ROOT, DEVFS_ROOT, SYSFS_ROOT environment variables, with host /proc, /sys, /run/dbus and the user session bus mounted read-only, plus SYS_ADMIN/SYS_RAWIO/MKNOD capabilities added — (empirical) [high]
  > "PROCFS_ROOT: alternative mount point for /proc. DEVFS_ROOT: alternative mount point for /dev. SYSFS_ROOT: alternative mount point for /sys. When these are set, any sensors that would normally source their data from a file in one of the canonical system mount points, will use the alternative mount point location specified."
  [github-joshuargo-hass-agent-a-home-assistant-native-app-for-desktoplaptop-device]
- github.com/dbus2/zbus serves the same 'z-galaxy/zbus' page content (org rename/redirect), confirming z-galaxy/dbus2 as the current home of the canonical zbus repo. — (empirical) [high]
  > "GitHub - z-galaxy/zbus: Rust D-Bus crate. (fetched at URL https://github.com/dbus2/zbus)"
  [github-z-galaxyzbus-rust-d-bus-crate-github-2]
- Neither go-hass-agent nor lnxlink appears in nixpkgs' nixos/modules/module-list.nix (master, 2026) — there is no services.go-hass-agent or services.lnxlink NixOS module; the services/home-automation/ directory hosts modules for home-assistant, esphome, zigbee2mqtt, zwave-js, matter-server, govee2mqtt, evcc, ebusd, homebridge and others, but neither Linux-agent incumbent. — (empirical) [high]
  > "module-list.nix contains zero occurrences of 'go-hass', 'lnxlink', or 'hass-agent'; services/home-automation/ enumerates home-assistant.nix, esphome.nix, zigbee2mqtt.nix, zwave-js.nix, matter-server.nix, govee2mqtt.nix, evcc.nix, ebusd.nix, homebridge.nix — no Linux-agent module"
  [module-listnix]

## Sub-Q5 build-vs-buy dialectic (18 claims)

- Flakes are still officially classified as an experimental Nix feature (introduced in Nix 2.4), providing uniform project structure, input pinning via flake.lock, and a registry-backed URL syntax (github:NixOS/nixpkgs). — (empirical) [high]
  > "Nix flakes are an experimental feature first introduced in the 2.4 Nix release, aiming to address a number of areas of improvement for the Nix ecosystem: they provide a uniform structure for Nix projects, allow for pinning specific versions of each dependencies, and sharing these dependencies via lock files, and overall make it more convenient to w"
  [flakes-official-nixos-wiki]
- Paho Rust is a safe wrapper around the Paho C library (v1.3.16), requiring a C compiler and CMake for the default 'bundled' build; pre-generated bindings ship for x86_64/aarch64-linux-gnu, armv7, Windows MSVC, and Apple Darwin. — (empirical) [high]
  > "The Rust crate is a safe wrapper around the Paho C Library. ... Requires Paho C v1.3.16, or possibly later. ... The default behaviour can be altered by enabling or disabling the features: "default" - [bundled, ssl]"
  [github-eclipse-pahopahomqttrust-pahomqttrust-github]
- The exporter emits probe_mqtt_duration_seconds histograms (round-trip latency), probe_mqtt_messages_published_total and probe_mqtt_messages_received_total counters, and probe_mqtt_completed_total/started_total — so message loss is directly observable as a published-minus-received delta and probe hangs as started-minus-completed. — (empirical) [high]
  > "probe_mqtt_messages_published_total Number of published messages. probe_mqtt_messages_received_total Number of received messages."
  [github-inovexmqtt_blackbox_exporter-prometheus-exporter-for-mqtt-monitoring-gith]
- All exporter metrics are labelled by broker URL (including TLS brokers like ssl://mqtt.example.net:8883) and probe name, so a single exporter instance can supervise multiple brokers simultaneously. — (empirical) [high]
  > "probe_mqtt_completed_total{broker="ssl://mqtt.example.net:8883",name="mqtt broker SSL"} 64"
  [github-inovexmqtt_blackbox_exporter-prometheus-exporter-for-mqtt-monitoring-gith]
- prometheus-community/smartctl_exporter requires smartmontools >= 7.0 (for JSON output), polls smartctl on a default 60s interval with a 10-minute device rescan (rescan disabled if devices are pinned via --smartctl.device or if interval < 1s), and exposes metrics on port 9633. — (empirical) [high]
  > "Requirements: smartmontools >= 7.0, because export to json released in 7.0. ... --smartctl.interval=60s The interval between smartctl polls. --smartctl.rescan=10m The interval between rescanning for new/disappeared devices."
  [github-prometheus-communitysmartctl_exporter-export-smartctl-statistics-to-prome]
- HA's Glances integration is a Local Polling integration that scrapes a Glances REST webserver (Web Server Mode, default port 61208) running on each monitored host, requiring Glances >= 2.3 with API v3 (v2 deprecated) — (empirical) [high]
  > "These sensors needs a running instance of glances in Web Server Mode on the host. The minimal supported version of glances is 2.3. ... Its IoT class is Local Polling."
  [glances-home-assistant]
- The HA Glances integration's sensor catalogue spans disk (per-mount use percent/absolute/size), per-physical-disk diskio read/write MB/s, memory and swap (percent + absolute), processor load, process counts (running/total/thread/sleeping), CPU percent, temperature (lm-sensors), Docker container counts and CPU/memory, RAID device counts, per-NIC rx/tx Mbps, per-GPU VRAM/processor/temperature/fan (py3nvml), and uptime — the de-facto scope of host monitoring in the HA ecosystem — (empirical) [high]
  > "For each detected disk (or mount point) the following sensors will be created: disk_use_percent ... diskio_read: Average rate of data read from the device in megabytes per second. ... For each detected GPU (video card) the following sensors will be created: memory_use ... Not all platforms can provide all metrics."
  [glances-home-assistant]
- The Glances integration is maintained by a single integration owner (@engrbm87) and used by only 1.3% of active HA installations — (statistical) [high]
  > "The Glances service was introduced in Home Assistant 0.7.3, and it's used by 1.3% of the active installations."
  [glances-home-assistant]
- Telegraf2Hassio's implementation is a hub-side HA add-on (Dockerfile + run.sh + config.yaml) that subscribes to Telegraf's MQTT topic wildcard (default 'telegraf/#') with broker credentials, and re-publishes measurements as HA self-discoverable MQTT sensors — all discovery logic lives on the HA side, not the monitored host — (empirical) [high]
  > "options: mqtt_broker: localhost, mqtt_port: 1883, mqtt_user: mqtt_user_here, mqtt_pass: mqtt_pass_here, telegraf_topic: telegraf/#, calc_rate: host_sensor_measurement_1,host_sensor_measurement_2, log_level: info"
  [hassio-addonstelegraf2hassio-at-master-joaoflhassio-addons-github]
- dbengine v2 (merged January 2023, shipped in v1.37) reduced a default-config Netdata agent on 64-bit machines to about 100 MB RAM with 3 storage tiers and about 1 year retention, and the journal-file PR #13885 reduced dbengine memory footprint by 80-90% at a cost of 15% speed and 10% disk. — (statistical) [high]
  > "A Netdata Agent running with default settings on a 64 bit machine should now use about 100MB of RAM, using 3 storage tiers, providing about 1 year of data retention ... this reduces the memory footprint of dbengine by 80 - 90%, while sacrificing just 15% of its speed and 10% of disk space."
  [insane-netdata-memory-usage-help-netdata-community-forums]
- Netdata's Mosquitto collector is implemented via the generic go.d prometheus module scraping a separate mosquitto_exporter over HTTP (default 10s interval) rather than speaking MQTT natively, with auto-detection of local exporter ports and support for remote instances. — (empirical) [high]
  > "Metrics are gathered by periodically sending HTTP requests to mosquitto exporter. ... By default, it detects instances running on the local host by trying to connect to known ports that are allocated to exporters."
  [mosquitto-databases-learn-netdata]
- Netdata's 'MQTT Blackbox' synthetic-testing page actually documents a prometheus collector job that scrapes an MQTT Blackbox Exporter over HTTP (not MQTT itself), configured via go.d/prometheus.conf YAML with url, timeout (default 10s), update_every (default 10s), selector allow/deny filters, fallback_type patterns, metric_relabel_configs, and profiles. — (empirical) [high]
  > "Metrics are gathered by periodically sending HTTP requests to MQTT Blackbox Exporter. ... UI configuration requires paid Netdata Cloud plan."
  [mqtt-blackbox-synthetic-testing-learn-netdata]
- Netdata's prometheus collector treats disappearing metrics specially: when a chart or dimension expires after metrics vanish from a successful scrape, its alerts become REMOVED rather than CLEAR and send no recovery notification — so reliable alert clearing requires exporting an explicit normal value (e.g. 0). — (empirical) [high]
  > "An expired chart or dimension makes its alerts REMOVED; this is not a normal CLEAR transition and does not send a recovery notification. Export an explicit normal value (for example 0) whenever an alert needs a reliable recovery transition."
  [mqtt-blackbox-synthetic-testing-learn-netdata]
- Netdata's default-database-tier agent needs about 16 KiB of RAM per unique metric collected, independent of collection frequency; a child agent defaults to 100-200 MB depending on metric count. — (statistical) [high]
  > "Using the default Database Tier configuration, Netdata needs about 16KiB per unique metric collected, independently of the data collection frequency. ... Netdata by default should need 100MB to 200MB of RAM, depending on the number of metrics being collected."
  [ram-utilization-resource-utilization-learn-netdata]
- Netdata Parent memory cost model: each actively-collected metric costs 26 KiB (1 KiB index + 20 KiB collection structures + 5 KiB ML model), each archived metric 1 KiB, and each active node 1034 KiB (10 KiB index + 512 KiB reception + 512 KiB dispatch buffers); a 2-Parent cluster with 1M active metrics from 500 nodes needs 35.7 GiB. — (statistical) [high]
  > "Each metric currently being collected needs (1 index + 20 collection + 5 ml) = 26 KiB. When it stops being collected, it needs 1 KiB (index). Each node currently being collected needs (10 index + 512 reception + 512 dispatch) = 1034 KiB. ... Memory required per node 35.7 GiB."
  [ram-utilization-resource-utilization-learn-netdata]
- Netdata 2.1 added dbengine out-of-memory protection (default 10% of total system RAM, capped at 5 GiB) that automatically releases cache memory when free memory drops below the threshold, plus a 'use all ram for caches' mode. — (empirical) [high]
  > "[db].dbengine out of memory protection is by default 10% of total system RAM, but not more than 5GiB. When the amount of free memory is less than this, Netdata automatically starts releasing memory from its caches to avoid getting out of memory."
  [ram-utilization-resource-utilization-learn-netdata]
- go-hass-agent v14.15.0 added NVIDIA and AMD GPU memory-usage sensors and a series of fixes making disabled-sensor preferences respected before privileged capability checks, plus PipeWire pw-dump robustness fixes for non-string node.nick values — (empirical) [high]
  > "Features: gpu memory usage (a0fbdd7), linux: (AMD) GPU memory usage (521ec76) ... Bug Fixes: linux/disk: respect disabled preference before SMART capability check (0d27760) ... linux: improve capability error clarity and respect disabled pref for activity worker (7efe012) ... pipewire: tolerate non-string node.nick values from pw-dump (0e285f2)"
  [releases-joshuargo-hass-agent-github]
- A standalone Netdata Agent has a measured footprint of 1-5% of a single CPU core with default settings (up to 5-20% in production) and 100-200 MB RAM on an empty system (250-350 MB in typical production), with ~4 GiB disk by default (3 GiB metrics plus metadata). — (statistical) [high]
  > "CPU 1%-5% of a single core with default settings; up to 5%-20% in production. RAM 100-200 MB on an empty system; 250-350 MB in typical production. Disk ~4 GiB by default (3 GiB metrics plus metadata), configurable per tier."
  [resource-utilization-netdata-agent-learn-netdata]

## Sub-Q6 jupiterOS feature gaps (11 claims)

- Backlight power state is configurable from user space via /sys/class/backlight/<backlight>/bl_power with values 0 (full on) and 4 (full off). — (empirical) [high]
  > "User space can configure the power mode using the sysfs attribute: /sys/class/backlight/<backlight>/bl_power ... The possible values are: (0: full on, 4: full off)"
  [backlight-support-the-linux-kernel-documentation]
- The NVMe admin ioctl NVME_IOCTL_ADMIN_CMD requires CAP_SYS_ADMIN at the kernel level (checked in nvme_user_cmd in drivers/nvme/host/pci.c) — CAP_SYS_RAWIO is insufficient — so containerised or capability-restricted SMART collectors must be granted CAP_SYS_ADMIN to read NVMe health; SATA/SCSI drives need only CAP_SYS_RAWIO. — (empirical) [high]
  > "if you read the source code for NVME_IOCTL_ADMIN_CMD it requires CAP_SYS_ADMIN, so there is nothing you can do except grant that. ... So it looks like --cap-add SYS_ADMIN is required for NVMe drives, but --cap-add SYS_RAWIO is enough for SATA/SCSI drives. If you have a mix of both SATA/NVMe, you must include both flags"
  [clarification-of-volume-permissions-issue-26-analogjscrutiny-github]
- The global CPU boost knob lives at /sys/devices/system/cpu/cpufreq/boost (values 0/1) and is the recommended interface; the legacy AMD cpb knob appears per-policy but actually acts system-wide and may be removed. — (empirical) [high]
  > "This file is located under /sys/devices/system/cpu/cpufreq/ and controls the “boost” setting for the whole system. ... The only values that can be written to this file are 0 and 1. ... it is always possible use the boost knob instead of the cpb one which is highly recommended"
  [cpu-performance-scaling-the-linux-kernel-documentation]
- A worked example in the thermal sysfs doc shows acpitz zone values of temp 37000 (37°C), trip points at 100000 critical / 80000 passive / 70000 active0 / 60000 active1, and an hwmon mirror (temp1_input 37000, temp1_crit 100000). — (empirical) [high]
  > "|---type: acpitz |---temp: 37000 ... |---trip_point_0_temp: 100000 |---trip_point_0_type: critical ... /sys/class/hwmon: |hwmon0: |---name: acpitz |---temp1_input: 37000 |---temp1_crit: 100000"
  [generic-thermal-sysfs-driver-how-to-the-linux-kernel-documentation]
- Scrutiny's hub/spoke deployment model runs one collector container per server (ghcr.io/analogj/scrutiny:latest-collector with COLLECTOR_API_ENDPOINT pointing at a central web container backed by InfluxDB 2) — a collector-per-host, central-aggregation architecture. — (empirical) [high]
  > "you can deploy in Hub/Spoke mode ... ghcr.io/analogj/scrutiny:latest-collector - Contains the Scrutiny data collector, smartctl binary and cron-like scheduler. You can run one collector on each server. ... -e COLLECTOR_API_ENDPOINT=http://SCRUTINY_WEB_IPADDRESS:8080"
  [github-analogjscrutiny-hard-drive-smart-monitoring-historical-trends-real-world]
- smartctl_exporter must run privileged (Docker: privileged: true, user: root) to access block devices, and maintains an explicit no-data-mutation policy: it does not fix or patch smartctl data in flight - incorrect data is reported upstream to smartmontools. — (empirical) [high]
  > "services: smartctl-exporter: image: prometheuscommunity/smartctl-exporter privileged: true user: root. ... In general, the smartctl_exporter should not modify the data in flight. If the data is missing from smartctl, it should not be in smartctl_exporter."
  [github-prometheus-communitysmartctl_exporter-export-smartctl-statistics-to-prome]
- The niri wiki (hosted on GitHub, generated from docs/wiki/ in the repo) is a documentation hub whose TOC lists the pages relevant to kiosk-agent integration: 'IPC, niri msg', 'Example systemd Setup', 'Integrating niri', and 'Security Model'; the wiki content has moved to a website at niri-wm.github.io/niri/. — (empirical) [high]
  > "If you're not already here, check out our new wiki website! https://niri-wm.github.io/niri/"
  [home-niri-wmniri-wiki-github]
- niri's event-stream IPC mode (since 0.1.9) continuously streams events over the connection and is designed to deliver complete current state up-front followed by incremental updates, guaranteeing the consumer's state cannot desync and eliminating polling. — (empirical) [high]
  > "This is useful for implementing various bars and indicators that update as soon as something happens, without continuous polling. The event stream IPC is designed to give you the complete current state up-front, then follow up with updates to that state. This way, your state can never "desync" from niri, and you don't need to make any other IPC inf"
  [ipc-niri-msg-niri]
- niri is a Rust Wayland compositor (niri-wm/niri, 27.4k stars, GPL-3.0) that is stable for daily use, supports multi-monitor + mixed DPI from the start, floating windows since 25.01, and Xwayland via xwayland-satellite since 25.08, but is not a complete desktop environment (needs a shell like DankMaterialShell or Noctalia). — (empirical) [high]
  > "Niri is stable for day-to-day use and does most things expected of a Wayland compositor. ... Grab a desktop shell like DankMaterialShell or Noctalia (or build a more traditional setup): niri by itself is not a complete desktop environment."
  [ipc-niri-msg-niri]
- HA 2021.12+ lets MQTT devices control generated entity names via object_id, and ESPHome's discovery_object_id_generator: device_name prefixes entity IDs with the device name (sensor.uptime -> sensor.<device name>_uptime) to disambiguate fleets of similar devices. — (empirical) [high]
  > "Home Assistant 2021.12 allows MQTT devices to change this behaviour by specifying the object_id discovery attribute which replaces the sensor name part of the generated entity name."
  [mqtt-client-component-esphome-smart-home-made-simple]
- ESPHome's keepalive default is 15 seconds and shorter keepalive increases ping traffic; wait_for_connection blocks other components from starting until MQTT connects; publish_nan_as_none publishes None instead of NaN for HA Unknown/Unavailable states. — (empirical) [high]
  > "keepalive (Optional, Time): The time to keep the MQTT socket alive... Defaults to 15 seconds. ... wait_for_connection (Optional, bool): Blocks other components from starting until the MQTT connection is established. Defaults to false. ... publish_nan_as_none (Optional, bool): Publish None instead of NaN to handle Unknown/Unavailable sensor states i"
  [mqtt-client-component-esphome-smart-home-made-simple]

## Ungrouped (10 claims)

- paho-mqtt 0.14.0 lists tokio ^1.49 as an OPTIONAL normal dependency (runtime-agnostic core; async-channel, futures, futures-timer, crossbeam-channel are the core async deps), consistent with the README's claim of being tested with tokio and smol. — (empirical) [high]
  > "Dependencies: async-channel ^2.5 ... crossbeam-channel ^0.5 ... futures ^0.3 ... futures-timer ^3.0 ... tokio ^1.49 normal optional"
  [asyncclient-in-paho_mqttasync_client-rust]
- The ping queue is strictly serial: one ping executes at a time, a previously-available device gets 2 attempts with a 3-second retry gap, and each queue item sleeps 2 seconds before the next ping runs. — (empirical) [high]
  > "const attempts = available ? 2 : 1; ... await device.zh.ping(!available || i !== 2); ... if (i !== attempts) { await utils.sleep(3); } ... // Sleep 2 seconds before executing next ping await utils.sleep(2);"
  [availabilityts]
- HA core's MQTT discovery topic matcher only accepts component/node_id/object_id segments matching [a-zA-Z0-9_-]; any other character makes the topic illegal and HA logs a warning and drops the message. — (empirical) [high]
  > "TOPIC_MATCHER = re.compile(r"(?P<component>\w+)/(?:(?P<node_id>[a-zA-Z0-9_-]+)/)?(?P<object_id>[a-zA-Z0-9_-]+)/config")"
  [corehomeassistantcomponentsmqttdiscoverypy-at-dev-home-assistantcore-github]
- Eclipse Paho MQTT Rust Client supports MQTT v5, 3.1.1, and 3.1, with automatic reconnect and offline buffering built in — 'Supports MQTT v5, 3.1.1, and 3.1 ... Automatic Reconnect ... Offline Buffering'. — (empirical) [high]
  > "Supports MQTT v5, 3.1.1, and 3.1 ... Automatic Reconnect ... Offline Buffering ... High Availability"
  [github-eclipse-pahopahomqttrust-pahomqttrust-github]
- paho.mqtt.rust v0.14.0 (released 26 Mar by fpagliughi) added synchronous and async event streams where 'All events from the client flow through the stream: Connect, Connection Lost, Disconnected, Incoming Message' — giving applications explicit connection-lost signalling that rumqttc 0.24.0 lacks. — (empirical) [high]
  > "Added synchronous (blocking) and async event streams. All events from the client flow through the stream: Connect, Connection Lost, Disconnected, Incoming Message"
  [github-eclipse-pahopahomqttrust-pahomqttrust-github]
- Paho Rust's release cadence shows active maintenance through 2025-2026: v0.13.0 (21 Jan), v0.13.1 (19 Feb), v0.13.2 (27 Mar), v0.13.3 (28 Apr), v0.14.0 (26 Mar), sys-v0.10.3 (14 May); v0.12.3 specifically fixed 'numerous issues with reconnecting to the broker' including 'crashes on reconnect callbacks'. — (empirical) [high]
  > "Upgrade to Paho C v1.3.13 to fix a number of bugs, including numerous issues with reconnecting to the broker. The -sys crate now wraps Paho C v1.3.13, fixing several issues, including crashes on reconnect callbacks."
  [github-eclipse-pahopahomqttrust-pahomqttrust-github]
- Paho Rust v0.14.0 carries breaking changes in MQTT v5 error handling (Reason Code Error variant now contains Properties from ACK packet; ACKs with a single reason code error generate an Error result) and bumped MSRV to Rust v1.75. — (empirical) [high]
  > "[Breaking] Reason Code Error variant now contains Properties from ACK packet ... [Breaking] ACKs with a single reason code error generate an Error result, instead of an Ok() with a possible error code in it. ... Bumped MSRV to Rust v1.75"
  [github-eclipse-pahopahomqttrust-pahomqttrust-github]
- flake-utils provides eachDefaultSystem (over defaultSystems = x86_64-linux, aarch64-linux, x86_64-darwin, aarch64-darwin) plus eachSystemPassThrough for outputs that must sit outside the per-system attrset, such as nixosConfigurations and homeConfigurations. — (empirical) [high]
  > "inputs.flake-utils.lib.eachDefaultSystem (system: { checks.../packages.../devShells... }) // inputs.flake-utils.lib.eachDefaultSystemPassThrough (system: { homeConfigurations."<HOME_CONFIGURATION>" = ...; nixosConfigurations."<NIXOS_CONFIGURATION>" = ...; })"
  [github-numtideflake-utils-pure-nix-flake-utility-functions-maintainerzimbatm-git]
- rumqttc v0.25.1 (latest release, tagged 21 Nov) is a patch release focused on fixing the WebSocket build breakage and dependency updates, with no re-subscription or session-handling changes. — (empirical) [high]
  > "This patch release focuses on fixing the broken WebSocket feature and includes dependency updates. ... Fixed: Build failure when compiling with the websocket feature enabled (#999) ... Added: New use-rustls-no-provider feature for more flexible TLS configuration (#988)"
  [releases-bytebeamiorumqtt-github]
- The rumqtt release cadence from 0.23.0 through 0.25.1 spans roughly 12 months with active maintenance: 0.23.0 (10 Oct), 0.24.0 (21 May), 0.25.0 (09 Sep), 0.25.1 (21 Nov), interleaved with rumqttd releases 0.18.0-0.20.0. — (empirical) [high]
  > "rumqttc 0.22.0 ... rumqttc 0.23.0 ... rumqttc-0.24.0 ... rumqttc-0.25.0 ... rumqttc-0.25.1 Latest"
  [releases-bytebeamiorumqtt-github]
### Post-critic gap fill

- jupiterOS ships NO notification daemon anywhere (audited absence: recursive grep for mako|dunst|swaync|fnott|notify-send across modules/ and hosts/*/ of the live fleet checkout returns zero matches) — org.freedesktop.Notifications delivery is dead code on every host today
  > "anything published to the daemon's MQTT topic overlays as a notification for a few seconds... the customer display is both a live smart-home notifier (which the proprietary driver can't do at all) AND a cool screensaver."
  [jupiteros-notification-ground-truth]
