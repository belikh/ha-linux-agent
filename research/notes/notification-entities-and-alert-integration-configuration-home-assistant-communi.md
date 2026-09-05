---
title: Notification entities and alert integration - Configuration - Home Assistant
  Community
id: notification-entities-and-alert-integration-configuration-home-assistant-communi
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- mqtt-discovery
- community-thread
- ha-issue
- gap-02
created: '2026-09-02T17:05:55.178557Z'
updated: '2026-09-05T10:51:22.455970Z'
source: https://community.home-assistant.io/t/notification-entities-and-alert-integration/741053
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T17:05:55.165266Z'
fetch_provider: builtin
status: evergreen
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'HA community thread (June 2024, koying) ''Notification entities and alert
  integration'': user attempts to use an MQTT notify entity as an alert integration
  notifier; confirms notify ENTITIES (the 2024.5+ entity model, which mqtt notify
  belongs to) could not be wired into the legacy alert integration''s notifier at
  that time — the poster fell back to a command-line notifier + webhook automation.
  Relevant nuance for the adopt-vs-build chapter: MQTT notify entities participate
  in the entity ecosystem (scripts/automations can call notify.send_message targeting
  them) but legacy integrations expecting old-style notify SERVICES (notify.<name>)
  don''t see entity-platform notifiers; a jupiterOS agent exposing MQTT notify must
  be targeted via notify.send_message with entity_id, not via a legacy notify service
  name.'
---

Notification entities and alert integration - Configuration - Home Assistant Community

Notification entities and alert integration

Configuration

koying

(Chris B)

June 19, 2024,  8:49am

1

Banging my head a bit on that one.

I want to use the MQTT Notify platform as a notifier of an alert.

I understand that we are migrating towards entities and that not everything is there, yet, but I’d just want to confirm that you cannot use notify entities with alerts, yet.

Any suggestions for a workaround?

Actually, this is already a workaround (X-Y problem, I know the drill)

I actually want to use a siren as a notify target, so basically having a notification that toggles a switch.

My workaround would be to publish to mqtt, then have an automation subscribing and activating the siren. A tad cumbersome but fine for me.

Any other idea welcome.

I had that idea to use a command-line notifier, which still uses the “old way”, afaict.

But then is there a way to do a MQTT pub from inside the HA container?

Thanks

koying

(Chris B)

June 19, 2024, 10:30am

2

Follow-up:

I imagined a workaround through a command-line notifier executing a curl triggering a webhook automation.

Quite cumbersome, so simpler ideas are welcome
command_line:
- notify:
name: Tuya Siren 01 old
command: >
curl -X POST http://localhost:8123/api/webhook/siren-alert-foobar

Powered by Discourse, best viewed with JavaScript enabled
## Related

- [[mqtt-notify-home-assistant]]
- [[test_notifypy]]
