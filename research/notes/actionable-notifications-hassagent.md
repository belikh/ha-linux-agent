---
title: Actionable Notifications - HASS.Agent
id: actionable-notifications-hassagent
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-adopt-vs-build-honest-verdict
- adopt-vs-build
created: '2026-09-02T12:06:21.720538Z'
updated: '2026-09-02T17:37:22.511321Z'
source: https://hassagent.readthedocs.io/en/latest/notifications/new/notification-actionable/
source_domain: hassagent.readthedocs.io
fetched_at: '2026-09-02T12:06:21.719362Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: docs
deprecated: false
summary: 'HASS.Agent (Windows agent) docs for actionable notifications (requires HASS.Agent
  2022.13.0): ''HASS.Agent''s actionable notifications work the same way as the companion
  apps do'' — a notify service call includes data.actions with action/title pairs
  (e.g. action: ''yes'', title: ''Yes''); the notification shows buttons, and clicking
  one fires an HA event (event type ''hass_agent_notifications'', with device_name
  binding to a specific device) that triggers any automation listening for that action
  name — a full action-button round-trip implemented by the legacy Windows agent but
  not present in MQTT-based agents like lnxlink.'
---

*Suggested by [[github-hassagenthassagent-unofficial-development-project-for-the-hassagent-plat]]*

Actionable Notifications - HASS.Agent

»

Notifications »

Actionable Notifications

Actionable Notifications

Important: this feature requires at least HASS.Agent 2022.13.0.

Introduction

HASS.Agent's actionable notifications work the same way as the companion apps do. Read HA's docs for more info, found here.

It basically works as follows:

You send a notification, combined with a few actions

Each of those actions have a unique name, eg. yes or no

It's even better to give them more descriptive names, eg. turn_garden_light_on or activate_alarm

Don't worry: that's not what you see in the notification, you can set any title you want

When the notification shows, it'll have buttons containing the actions

As soon as you click one of the buttons, the action name will get triggered

Every automation that has that trigger will activate

If that still sounds a bit complex, just follow the examples below and it'll start making sense (hopefully).

Preparation

You need a notifier entity ready. If you use the legacy integration, check the examples for info on how to do that. Otherwise, you should have it autodetected after installing the new integration.

Actionable Notification

Let's prepare the notification. For our test, we'll create a script to handle that. In the GUI editor, configure it like this:

Or if you're into yaml:
alias: Notification - Actionable Test
sequence:
- service: notify.hass_agent_staging
data:
title: Test
message: This is an actionable test message.
data:
actions:
- action: "yes"
title: "Yes"
- action: "no"
title: "No"
mode: single
icon: mdi:bell

We added two actions: yes and no, which will show as Yes and No in the notification. The action value should be descriptive, the title value is what the user gets to see.

Action Automations

HA needs to know what to do with the triggered actions, so let's make automations to handle that.

Create a new automation in the GUI for our yes action, and configure the following trigger:

The device_name value helps us bind it to a specific device. Make sure you enter hass_agent_notifications as the Event type.

Then the action, it just sends another notification:

Save your automation.

Now create another automation for our no action, and configure the following trigger:

And the action:

Save your automation.

Testing

Restart HA (or reload scripts & automations). When you run the test script, a notification like this should show on your PC:

Click Yes, and another popup should show:

Done, you've now got actionable notifications!

Thanks @fillefilip8 for implementing this!