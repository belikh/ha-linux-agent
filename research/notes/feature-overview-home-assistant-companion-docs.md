---
title: Feature overview | Home Assistant Companion Docs
id: feature-overview-home-assistant-companion-docs
tags:
- linux-agent-jupiteros-fleet-15537b
- home-assistant-companion
- official-docs
- comparative-benchmark
created: '2026-09-02T04:02:37.780895Z'
updated: '2026-09-05T10:51:21.665551Z'
source: https://companion.home-assistant.io/docs/core/
source_domain: companion.home-assistant.io
fetched_at: '2026-09-02T04:02:37.551684Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: Official companion-app feature matrix comparing Full vs Minimal Android and
  iOS/macOS across integrations, location updates, notifications, and sensors. Confirms
  by absence (no Linux column) that the official companion programme has no Linux
  story; also demonstrates the feature taxonomy a mature companion app is expected
  to cover — actionable/channelled/persistent notifications, text-to-speech, iBeacon
  and significant-location-change, per-app shortcut surfaces, and ~60 sensor types.
  Thin page (224 words of prose plus the matrix); use the sensors doc for the sensor
  contract.
---

Feature overview | Home Assistant Companion Docs

Skip to main content

On this page

The Home Assistant Companion App provides a convenient way to view and control your Home Assistant instance however it also extends the power of your instance by allowing your device to act as a data source. The Home Assistant Companion App adds numerous sensors (such as battery and network status among others) and creates a device_tracker entity to allow location updates to be sent from the device.

Not all features are supported by Android at the moment but eventually most features will be supported.  Look for the  Android logo to see what is currently supported.

Feature Comparison:​
Integrations Full MinimalAndroid Device Controls✅✅Android Quick Settings✅✅Android Shortcuts✅✅Android WebView✅✅Android Widgets✅✅App Events✅✅✅✅Haptic Feedback✅Siri Shortcuts✅Sharing✅✅✅✅Theming✅✅✅✅URL Handler✅✅✅✅Universal Links✅✅X-Callback-URL✅✅Location Updates Full MinimalApp Opened✅✅✅App Refreshed✅✅Background✅✅Enter/Exit Zone✅✅iBeacon✅✅Intent✅Notification✅✅✅Significant Location Change✅✅✅URL Handler✅✅X-Callback-URL✅✅Notifications Full MinimalActionable✅✅✅Alert Once✅Badge✅✅Channels✅Cleared✅✅✅Color✅Commands✅✅✅Critical Alerts✅✅Dynamic Attachments✅Grouping✅✅✅HTML Formatting✅Icon✅Image✅✅✅Importance✅LED Color✅Local Push✅✅Message✅✅✅Opening a URL✅✅✅Persistent✅Presentation Options✅✅Replaceable Notifications✅✅✅Request Location Updates✅✅✅Sound✅✅Status Bar Icon✅Sticky✅Subject / Subtitle✅✅✅Text to Speech✅Timeout✅Title✅✅✅Vibration Pattern✅Video✅✅✅✅Sensors Full MinimalActive Sensor✅Active Camera✅Active Microphone✅Activity Sensors✅✅Android Auto Sensor✅✅Android OS Sensors✅✅App Data Sensors✅✅App Importance Sensor✅✅App Memory Sensor✅✅App Usage Sensors✅✅Audio Sensors✅✅Average Active Pace✅Battery Level✅✅✅✅Battery State✅✅✅✅Bluetooth Sensors✅✅BSSID✅✅✅✅Camera in Use✅Connection Type✅✅✅✅Current Time Zone✅✅Current Version✅✅Displays✅✅✅Distance✅Do Not Disturb✅✅Doze✅✅Dynamic Color✅✅Floors Ascended✅Floors Descended✅Frontmost App✅Geocoded Location✅✅Health Connect✅High Accuracy Mode✅High Accuracy Update Interval✅Interactive✅✅Keyguard Sensors✅✅Last Reboot✅✅Last Update Trigger✅✅✅✅Last Used App✅✅Light✅✅Microphone in Use✅Mobile Data Sensors✅✅Notification Sensors✅✅Phone Sensors✅✅Power Save✅✅Pressure✅✅Primary Display ID & Name✅Proximity✅✅Public IP✅✅Next Alarm✅✅Sim 1✅✅✅Sim 2✅✅✅SSID✅✅Steps✅✅✅Storage✅✅✅✅Traffic Stats✅✅Work Profile✅✅

Feature Comparison: