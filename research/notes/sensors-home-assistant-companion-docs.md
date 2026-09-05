---
title: Sensors | Home Assistant Companion Docs
id: sensors-home-assistant-companion-docs
tags:
- linux-agent-jupiteros-fleet-15537b
- home-assistant-companion
- comparative-benchmark
- sensor-catalogue
- official-docs
created: '2026-09-02T04:02:37.774406Z'
updated: '2026-09-05T10:51:21.690187Z'
source: https://companion.home-assistant.io/docs/core/sensors/
source_domain: companion.home-assistant.io
fetched_at: '2026-09-02T04:02:36.035120Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Official HA Companion docs sensor reference (8,199 words). Covers ONLY iOS,
  macOS, and Android — AUDITED ABSENCE: the page contains zero Linux content, confirming
  no official Linux companion agent exists, which is the gap ha-linux-agent/jupiterOS
  agents fill. Benchmark value for the sensor contract: macOS companion (closest desktop
  analogue) exposes active/active_camera/active_audio_input/active_audio_output/frontmost_app/camera_in_use/audio_input_in_use/audio_output_in_use/displays/primary_display_id+name,
  battery_level+state, bssid/connection_type/ssid, geocoded_location, last_update_trigger,
  storage, watch battery, plus the sensor.active activity sensor with attributes idle/screensaver/locked/screen_off/fast_user_switched/sleeping
  — a per-attribute pattern for user-presence detection on a desktop. Android documents
  the update model (periodic 15-min SensorWorker with foreground-notification anti-halt,
  or 1-min ''Fast Always'', foreground-service for SensorWorker, enable-per-sensor
  permission gating, 2025.5+ most sensors disabled by default) and update triggers
  (location change, pull-to-refresh, Update Sensors shortcut/notification). HA-side
  enable/disable of individual sensors is supported per-server. These are the de-facto
  UX expectations any Linux companion agent will be measured against.'
---

Sensors | Home Assistant Companion Docs

Skip to main content

On this page

Along with providing location services, the companion app also adds several additional sensors to Home Assistant. If you don't want the device_tracker entity but still want sensors to update then just disable the entity in the entity registry to stop location updates and keep sensor updates.

The sensors provided by the companion app depend on which app you're using, see the lists below.

Multi-server support​

If multiple servers are connected to the companion app, you can configure whether sensors are sent on a per-server basis. Currently the sensor settings will be common for all connected servers.

In Settings > Companion App, open the server's settings and change Sensors Sent setting under Privacy. Options available:

All sends all enabled sensors.

None does not send any sensors.

In Settings > Companion App, go to Manage Sensors and select the sensor you'd like to manage. Tap on the expand/collapse icon at the top of the screen to change settings for a specific server.

iOS & macOS sensors​

When sensors update​

On iOS, sensors update in limited situations: when your location changes, periodically when the app is running in the foreground, when you pull-to-refresh the web view, in the background at a rate determined by iOS, and when performing an "Update Sensors" or via "Send Location" shortcut or push notification. When Local Push is enabled and available in  2022.6 or later, periodic updates will also be performed.

On macOS, sensors update in the same situations as above as well as immediately when some sensors change.

From 2025.5​

Not all  sensors are enabled by default. If you don't see your sensor in Home Assistant, go to Companion App Settings > Sensors, and enable it manually.

Sensor list​
SensorAttributesDescriptionsensor.battery_levelNoneThe current battery level of the device.sensor.battery_stateLow Power ModeThe current charging state (either Charging, Not Charging, or Full) of the device.sensor.bssidNoneThe MAC address of the wireless access point your phone is connected to. When off Wi-Fi, this sensor will report Not Connected.sensor.connection_typeiOS: Cellular Technology
macOS: Name, Hardware AddressThe current data connection being used by the device. On macOS, this requires app version 2021.2 or later.binary_sensor.focusNoneWhether focus is currently enabled. Requires iOS-2021.10 or later, macOS 12 update later this year. Will not work if Home Assistant is in the "Allowed Notifications" list, see interruption level for more.sensor.geocoded_locationSee BelowCalculated address based on GPS data.sensor.last_update_triggerNoneThe cause of the last update of location and sensor data from the device to Home Assistantsensor.ssidNoneThe human-readable name of the Wi-Fi network the device is currently connected to. When off Wi-Fi, this sensor will report Not Connected.sensor.storageSee BelowThe amount of total and available storage on your device.

specific sensors
SensorAttributesDescriptionsensor.activityconfidence, typesThe current activity type as computed by iOS. Requires motion permissions to be enabled.sensor.app_versionNoneThe current Home Assistant companion App for iOS app version.sensor.average_active_paceNoneThe averaged pace calculated by iOS from pedometer data. Units: meters per second, m/sbinary_sensor.camera_motionSee BelowMotion detected by the device's front camera.sensor.camera_streamSee BelowStatus of the camera stream, which serves the front camera as an MJPEG camera on your local network.sensor.distanceNoneThe estimated distance walked by the user since midnight local time. Units: meters, msensor.floors_ascendedNoneThe approximate number of floors ascended by walking since midnight local time.sensor.floors_descendedNoneThe approximate number of floors descended by walking. Sincesensor.location_permissionNoneThe current location permission that was selected by the user. The permission can be set via the location permission popup or modified in the iOS settings.sensor.sim_1See BelowName of your cellular provider.sensor.sim_2See BelowName of your cellular provider.sensor.stepsNoneThe number of steps taken by the user.sensor.watch_battery_levelNoneThe battery level of 1 paired Apple Watch. Requires any Home Assistant complication installed on your watch face.sensor.watch_battery_stateNoneThe current charging state (either Charging, Not Charging, or Full) of 1 paired Apple Watch. Requires any Home Assistant complication installed on your watch face.

specific sensors
SensorAttributesDescriptionbinary_sensor.activeSee BelowWhether the device is actively being used.sensor.active_cameraAll, ActiveThe name of the active camera, or Inactive if not in use.sensor.active_audio_inputAll, ActiveThe name of the active audio input (microphone), or Inactive if not in use.sensor.active_audio_outputAll, ActiveRequires app version 2021.12 or later. The name of the active audio output (speaker), or Inactive if not in use.sensor.frontmost_appSee BelowRequires app version 2021.2 or later. The name of the current frontmost app.binary_sensor.camera_in_useNoneWhether a camera on the system is currently in use.binary_sensor.audio_input_in_useNoneWhether an audio output (microphone) on the system is currently in use.binary_sensor.audio_output_in_useNoneRequires app version 2021.12 or later. Whether an audio output (speaker) on the system is currently in use.sensor.displaysDisplay IDs, Display NamesRequires app version 2021.2 or later. Number of displays connected to the device.sensor.primary_display_idNoneRequires app version 2021.2 or later. ID of the current primary display, which is the display with the menu bar. In the form of a UUID, for example BE82E2E6-EA40-4963-93AD-A0BDC9D2F18F.sensor.primary_display_nameNoneRequires app version 2021.2 or later. Name of the current primary display, which is the display with the menu bar.

Attributes such as Cellular Technology can be accessed with a template such as:

{{ states.sensor.connection_type.attributes['Cellular Technology'] }}

Android sensors​

Each  sensor below can be enabled by navigating to Settings > Companion App > Manage Sensors. By default, most are disabled with the exception of the battery_level, battery_state, charger_type and any that were given permission during onboarding. Once enabled the sensor will begin to send data to your Home Assistant server, if you chose to disable it later on the sensor will stop updating. Upon enabling a sensor the app will request for permissions, if required. If you do not see a sensor listed below then your device does not support it. Some of the sensors below offer custom settings for each of their own needs, read about each one to see what it offers. These settings can be found in the same location where you enable the sensor.

How sensors update​

All sensors update during a periodic 15-minute interval and they will also update if other certain conditions are met. Read about each sensor below to understand how often to expect updates. During the 15-minute update interval a low priority foreground notification is temporarily created to prevent the Android system from halting the worker. This notification does not make a sound unless the user has installed a third-party app that intercepts notifications and decides to make a sound. If you are on Android 8.0+ you are free to minimize and/or turn off the notification channel for the SensorWorker.

You can change the frequency of sensor updates by navigating to Settings > Companion App > Sensor Update Frequency. You can select between Normal, Fast While Charging or Fast Always. Normal is the default mentioned in the previous paragraph. When set to Fast Always updates will come in every minute. When set to Fast While Charging updates will only come in every minute only while the device is charging, otherwise the default interval will be used. After changing this option you will need to restart the app.

Sensor list​
SensorAttributesDescriptionbinary_sensor.dozeSee BelowWhether or not the device is in doze mode.binary_sensor.interactiveNoneWhether or not the device is in an interactive state.binary_sensor.nfc_stateNoneWhether or not the device has its NFC sensor enabled.binary_sensor.power_saveNoneWhether or not the device is in power saving mode.Activity SensorsSee BelowThe current activity type, sleep confidence and sleep segment as computed by Google. Requires activity recognition permissions on supported devices.binary_sensor.android_autoSee BelowA binary sensor to indicate if the device is connected to Android Auto.Android OS SensorsNoneSeveral different sensors around the Android OS.App Data SensorsNoneSensors that show how much data was sent or received by the app.App Importance SensorNoneThe current importance of the app to determine if its in the foreground or cached.sensor.app_memorySee BelowInformation about the memory that is available for the app.App Usage SensorsNoneSensors that represent how the app is treated based on its usage.Audio SensorsNoneSeveral different sensors around different types of audio detection from the device.Battery SensorsNoneSeveral different sensors around the state of the device's battery.Bluetooth SensorsSee BelowSeveral different sensors about the state of bluetooth on the device. Sensors are also available for beacon transmitting and monitoring.Car SensorsSee BelowSeveral different sensors about the state of the car.sensor.current_time_zoneSee BelowThe current time zone the device is in.sensor.current_versionNoneThe current installed version of the application.Display SensorsSee BelowSeveral sensors about the state of the device's display.Dynamic ColorRGB ColorThe hexadecimal color value for the accent color used in the current device theme.sensor.do_not_disturbNoneThe state of do not disturb on the device.sensor.geocoded_locationSee BelowCalculated address based on GPS data.Health Connect SensorsVariesHealth and fitness data stored by other apps on your device in Health Connect.binary_sensor.high_accuracy_modeNoneThe state of high accuracy mode on the device.sensor.high_accuracy_update_intervalNoneThe update interval for high accuracy mode on the device.Keyguard SensorsNoneSensors that represent various states about the device being locked or secured.sensor.last_rebootSee BelowThe timestamp of the device's last reboot.sensor.last_updateNoneThe state will reflect the intent that caused the last update to get sent.sensor.last_used_appNoneThe last used application on the device.sensor.lightNoneThe current level of illuminance the device detects.Mobile Data SensorsNoneSeveral different sensors around the state of mobile data.sensor.next_alarmSee BelowDate of the next scheduled alarm.Notification SensorsSee BelowDetails about the notifications on the device.Phone SensorsNoneSensors that represent different states of the phone modem.sensor.pressureNoneThe pressure reading from the device.sensor.proximityNoneThe current proximity reading from the device, certain devices will only show boolean value of near or far.sensor.public_ipNoneThe public IP address of the device as generated by ipify API.sensor.sim_1See BelowName of your cellular provider.sensor.sim_2See BelowName of your cellular provider.sensor.stepsNoneThe number of steps taken from the user since the last device reboot. Requires activity recognition permissions on supported devices.Storage SensorsSee BelowThe amount of total and available internal & external storage on your Android device.Traffic Stats SensorNoneAmount of data transmitted and received from mobile and total device usage since last reboot.WiFi SensorsNoneSeveral different sensors around the state of WiFi.Work ProfileNoneWhether or not the work profile is currently active on the device.

Active sensor​

sensor.active provides whether the device is currently being used, based on a few different inputs which are provided as attributes to be informative.
AttributeDescriptionIdletrue when the machine is not any of the following attributes, but input devices haven't been used in a number of minutes.Screensavertrue when the screensaver began playing to turn inactiveLockedtrue when the device is showing the login screenScreen Offtrue when the screens have been turned offFast User Switchedtrue when switched to another userSleepingtrue when the device is sleepingTerminatingtrue when the app was quit available. Requires app version 2021.2 or later.

This sensor has a setting to decide the duration that is considered 'idle'.

Activity sensors​

sensor.activity provides the current motion activity as calculated by iOS along with the confidence of the calculations. Activities known by iOS and given by sensor.activity are:

Stationary

Walking

Running

Automotive

Cycling

If iOS is unable to calculate an activity from motion data, Unknown will be given.

It is possible for multiple activities to be returned, such as Cycling and Stationary (if you are cycling but at a stop light), the state of the sensor is simply the first of these return by iOS (not necessarily the most likely). A complete list of calculated activities is given by the types attribute. See this post by @Mattt over at nshipster for a description of how different scenarios yield multiple activities.

The confidence attribute corresponds how accurate iOS believes the report of the current activity is. Possible values are:

Low

Medium

High

This sensor is only available on the full flavor of the Android app that is found in the Google Play Store, it is not available for the minimal flavor. For android the user will have a different set of states to go by:

in_vehicle

on_bicycle

on_foot

running

still

tilting

walking

unknown

The attribute for the state will reflect the confidence rating from the Activity Recognition API. This sensor requires the Activity Recognition permission.

The Sleep Confidence and Sleep Segment sensors utilize the new Sleep API from Google services. Sleep Segment updates about once a day and Sleep Confidence will update about every 10 minutes. All data is provided by Google.

Android Auto​

This sensor is used to determine if the device is connected to Android Auto.  The attributes will return the specific type of connection.

Android OS sensors​

Several different sensors around the Android OS build. These sensors make use of android.os.Build.
SensorDescriptionandroid_os_versionAndroid OS release (e.g. 13).android_os_security_patchAndroid OS security patch (e.g. 2023-03-05).

App data sensors​

These sensors will represent how much data was transmitted and received by the Home Assistant Android app, since the last device reboot. These sensors make use of the Traffic Stats API.

App importance sensor​

This sensor will represent the state of the app to reflect if its in the foreground or service or any other state it can be. This sensor will update any time any other sensor has an update. See all of the Importance variables in ActivityManager to see what they mean.

Possible states are:

cached

cant_save_state

foreground

foreground_service

gone

not_running

perceptible

service

top_sleeping

visible

App memory sensor​

This sensor will represent how much memory is being used by the application. The attributes will include how much memory is free and available for the application. This sensor makes use of the Runtime API.

App usage sensors​

These sensors will represent how the Android system is treating the app based on its usage. There is one binary sensor app_inactive which will report whether or not the system currently considers the app to be inactive. The other sensor app_standby_bucket will reflect the current standby bucket that the Android system considers for the app. Standby buckets determine how much an app will be restricted from running background tasks such as jobs and alarms. Both of these sensors make use of the UsageStatsManager API.

Possible states for app_standby_bucket sensor (please refer to the API linked above for their definitions):

active

frequent

rare

restricted

working_set

never

Audio sensors​

These sensors use the AudioManager API to retrieve their state. Look at the table below to find out more about each sensor including how often they update.
SensorAttributesDescriptionaudio_modeNoneThe current audio mode of the device can be either: normal, ringing (identical to phone sensor), call_redirect, communication_redirect, in_call, in_communication, assistant_conversation (BETA), or unknown. This sensor will update during the normal interval.is_headphonesNoneBoolean value if headsets or headphones are plugged in, will update as soon as the device detects the change.is_mic_mutedNoneBoolean value if the microphone is currently muted, Android 10+ will update as this value changes.is_music_activeNoneBoolean value if the device is actively playing music, this sensor will update during the normal interval.is_speakerphone_onNoneBoolean value if the device speakerphone is enabled, Android 10+ will update as this value changes.ringer_modeNoneThe ringer mode on the device, possible values are normal, vibrate or silent. This sensor will update as soon as the ringer mode changes.
On Android 9 and newer, the ringer mode will always be silent when do not disturb is turned on.volume_level_*See belowThe current device volume level for the given volume streams.

Volume levels​

The current device volume level for the given volume streams (volume_level_*). These sensors will update during the normal interval, or as soon as a change is detected.

Possible volume streams are:

accessibility

alarm

assistant BETA (Android 17+)

call

dtmf

music

notification

ring

system

Attributes indicate the minimum and maximum value.
AttributeDescriptionminProvides the minimum value of the volume level. Usually this is 0, but it can also be different based on the Android version and device manufacturer.maxProvides the maximum value of the volume level. This value differs between Android versions and device manufacturers.

Battery sensors​

The Battery State sensor (sensor.battery_state) provides information on the current status of the device's battery. The three possible values are Charging, Not Charging, or Full when the device is 100 % charged. The Battery Level sensor (sensor.battery_level) reports the current battery level of the device from 0–100 %. The charge level is reflected in the sensor icon. Additionally there is a "Low Power Mode" attribute that reports true or false depending on whether your iOS device is in Low Power Mode or not.

The battery sensors listed below describe the state of the battery for a few different data points. The sensor's icon reflects the charging status, and type of charging being used. The battery_state, charger_type and is_charging sensor will be updated when the device has a charger connected or disconnected. The battery_health, battery_level, battery_power and battery_temperature sensors will be updated any time any of the other sensors get an update as well as when the device reports low battery or when it has recovered from the low battery alert. All of these sensors make use of BatteryManager.
SensorDescriptionbattery_cycle_countThe number of charge cycles completed by the battery. Requires Android 14 or newer. Note: not all devices will report or update the cycle count.battery_healthThe health of the batterybattery_levelThe percentage of battery remainingbattery_powerThe current wattage on the devicebattery_stateThe state of charging on the devicebattery_temperatureThe current battery temperaturecharger_typeThe type of charger being used on the deviceis_chargingWhether or not the device is actively chargingremaining_charge_timeComputed remaining charge time in minutes. Returns unavailable if no time can be computed: either there is not enough current data to make a decision or the battery is currently discharging. Returns 0 if calculation is not complete but device is currently charging. Requires Android 9 or newer.

info

The battery_power sensor converts the values returned by the device to amperes and volts. However, some devices do not follow Android documentation and may return values in a different unit, which results in the sensor being incorrect. For these devices you may need to adjust the sensor setting for 'Battery current divisor' to properly convert the current to amperes or 'Battery voltage divisor' to properly convert the voltage to volts.
Common values for the Battery current divisor: 1000000 (default, microamperes), 1000 (milliamperes), 1000000000 (nanoamperes)
Common values for the Battery voltage divisor: 1000 (default, millivolts), 1 (no conversion required, volts)

Bluetooth sensors​

This Bluetooth Connection state will be the total number of connected bluetooth devices. The sensor will update as soon as the bluetooth state of the device changes. This sensor makes use of Android's Bluetooth package.
AttributeDescriptionConnected Paired DevicesThe list of paired devices that are currently connected.Connected Not Paired DevicesThe list of devices that are connected but not paired.Paired DevicesThe list of devices that are paired.

There will also be a binary sensor for the bluetooth_state that will represent whether or not bluetooth is turned on for the device. This sensor will update anytime the state of bluetooth changes.

A BLE Transmitter sensor allows your device to transmit a BLE iBeacon. The iBeacon is capable of being detected by the iBeacon integration if your device sends the device name (see why here). If your device does not send its device name, it can still be detected if you explicitly allow the UUID via the iBeacon integration options. This sensor can also be useful in conjunction with projects like roomassistant and esp32-mqtt-room  to allow room level tracking.  The current transmitting ID (UUID-Major-Minor) is reported as an attribute that can be copied for use with these systems.

caution

This sensor can impact battery life, particularly if used wih Transmit Power set to High. The iBeacon is transmitted every second (low latency to save battery, but sufficient for room presence).

Settings are available to change the UUID, Major and Minor masks. These can be used to change the overall identifier, as well as to allow groups, e.g. family phone devices can have particular Major value which can be whitelisted in apps like roomassistant. These settings are validated: UUID should be the standard format, Major and Minor need to be within 0 and 65535.

There are also settings to alter:

the Transmit power (between Ultra Low, Low, Medium and High)

the Advertise mode (between Low Power (1Hz), Balanced (3Hz) and Low latency (10Hz))

the Measured power at 1 meter (must be a negative number)

whether Transmit is only enabled on Home Wifi Network SSIDs

A Transmit setting toggle will start or stop the BLE transmissions. This setting as well as most of the above settings can be changed via the notification command.

The Beacon Monitor shows scans for BLE iBeacons. The state of the sensor shows if the app is monitoring or not. All beacons in range and their distance are listed in the attributes. This sensor will update when there is a new distance measurement available.

Settings are available to change scan period and interval which can be useful to preserve battery life. The setting Filter Iterations and Filter RSSI Multiplier can be adjusted to archive more stable measurements. All of these settings will affect the responsiveness of the sensor. A UUID filter is also available, to limit the reported beacons to those matching (or not matching) a list of UUIDs.

A Monitor setting toggle will start or stop the scans - this setting can also be adjusted via the notification command.

When the app is actively scanning for beacons a notification will be shown to make background scanning more reliable. If you are on Android 8.0+ you are free to minimize and/or turn off the notification channel for the Beacon Monitor Scanning.

Camera motion sensor​

binary_sensor.camera_motion reports motion detected by the device's front camera, using a lightweight comparison between consecutive camera images. It is designed for wall-mounted tablets, for example to trigger automations when someone walks up to the device, and works together with the kiosk mode screensaver.

This sensor is disabled by default so the camera never turns on without your explicit choice. When you enable it, iOS asks for camera permission and shows the camera indicator (a green dot) in the status bar while the camera runs. Like all camera use, the sensor only works while the app is open and in the foreground.

You can adjust the detection in the sensor's settings, under Settings > Companion app > Sensors > Camera Motion:

Frame rate: How many images per second are analyzed, from 1 to 30 (default 8). Higher frame rates may impact performance and make the device run hotter.

Changed area threshold: How much of the image must change for it to count as motion, from 1% to 100% (default 40%). Lower values make the sensor more sensitive, but also more likely to react to lighting changes, like a TV or a light turning on.

Clear delay: How long without motion before the sensor turns off again, from 2 seconds to 5 minutes (default 15 seconds).

AttributeDescriptionFrame RateThe configured detection frame rate.Area Threshold (%)The configured changed area threshold.Clear Delay (s)The configured clear delay.Last Changed Ratio (%)How much of the image changed in the last analyzed frame. Useful to tune the threshold.Last MotionWhen motion was last detected.

Camera stream sensor​

sensor.camera_stream reports the status of the camera stream: streaming while at least one client is watching, idle otherwise. The sensor is only available while the camera stream is enabled.

The camera stream turns the device's front camera into a camera you can view in Home Assistant: while it is enabled, the app runs a small MJPEG server on the device. To enable it, turn on the Camera Stream sensor in the app, under Settings > Companion app > Sensors. It is disabled by default, so the camera never turns on without your explicit choice. While the stream is enabled, the camera runs continuously (foreground only) and iOS shows the camera indicator (a green dot) in the status bar.

To view the camera in Home Assistant, add the MJPEG camera integration and use the address from the sensor's Stream URL attribute, for example http://192.168.1.20:8090/camera. The device and your Home Assistant server must be on the same network, and giving the device a fixed IP address (a DHCP reservation) is recommended.

You can adjust the stream in the sensor's settings, under Settings > Companion app > Sensors > Camera Stream:

Frame rate: The stream frame rate, from 1 to 30 (default 15). Higher frame rates may impact performance and make the device run hotter.

Stream port: The port the server listens on (default 8090).

Username and Password: Optional credentials. When set, clients must authenticate to view the stream; enter the same credentials in the MJPEG camera integration. When left empty, anyone on your network can view the stream.

AttributeDescriptionStream URLThe address to use in the MJPEG camera integration.PortThe configured port.ClientsThe number of clients currently watching the stream.

Car sensors​

The sensors listed below describe the state of the car for a few different data points. Currently this is only available for Android Auto. These sensors may not provide data depending on your phone and/or car software. If you see a state of unknown check the status attribute to see the reason for why it has no data.

caution

Note that you need to start the Home-Assistant app on your Android Auto screen each time that you connect the phone to the car to allow these sensors to work (once started, you can dismiss the app). If the app is not started the state will be unavailable.
To make things easier, you may want to use the car_ui parameter to show a notification on your car when you connect your phone.
SensorDescriptioncar_batteryThe percentage of battery remainingcar_charging_statusThe charging status of the car (only for EVs). The state of the charging port is in the attributescar_ev_connectorList of available EV connectors for the carcar_fuelThe percentage of fuel remainingcar_fuel_typeList of available fuel types for the car.car_nameThe name of the car. The manufacturer name and manufactured year are in the attributescar_odometerThe value of the car odometer in meterscar_range_remainingThe range remaining for the car in meterscar_speedThe speed of the car in meters per second

Cellular provider sensor​

The cellular provider sensor displays information about the user’s cellular service provider, such as its unique identifier and whether it allows VoIP calls on its network. sensor.sim_1 corresponds to the physical SIM card installed and sensor.sim_2 corresponds to the eSIM (this is only shown if the eSIM is enabled).

Android users will see these sensors update anytime the network has changed, which makes use of SubscriptionManager. These sensors require the Read Phone State permission.
AttributeDescriptionCarrier NameThe name of the user’s home cellular service provider.Current Radio Technology only.ISO Country CodeThe ISO country code for the user’s cellular service provider.Mobile Country CodeThe mobile country code (MCC) for the user’s cellular service provider.Mobile Network CodeThe mobile network code for the user’s cellular service provider.Carrier IDAllows VoIPIndicates if the carrier allows making VoIP calls on its network. Is OpportunisticAn opportunistic subscription connects to a network that is limited in functionality and / or coverage. Data RoamingIs data roaming enabled for the device.

Connection type sensor​

The following connection types are known by the companion app:

Wi-Fi

Cellular

No Connection

A more specific description of the data connection can be found in the Cellular Technology attribute of the sensor (which only appears when on cellular). Possible values for this attribute are:

4G

3G

2G

Cellular

No Connection

If the connection type is not recognized, either Unknown or Unknown Technology will be returned.

For Android several different types of connection sensors are available and they will update when a network state change has been detected:
SensorDescriptionwifi_connectionThe name of the current connected networkbssidThe mac address of the current connected networkfrequencyThe frequency band of the connected networkwifi_ip_addressThe current IP address of the device on the networklink_speedThe current link speed of the device to the connected networksignal_strengthThe signal strength of the device to the WiFi networkwifi_stateWhether or not WiFi is turned on for the devicetransport_typeThe transport type for the current network connection. An attribute will reflect if the current network is metered.hotspot_stateWhether or not the device is currently broadcasting a WiFi hotspot. (Not available on Wear OS)ip6_addressesThe ip6_addresses bound to the currently active network

The bssid sensor offers settings to let you rename the current mac address to help avoid the need for templates and secret usage in automations and the front end. This is generally useful if you have multiple access points and want an easy way to differentiate between them. These settings are turned off by default. These sensors require either Background Location or Fine Location permissions, depending on what version of Android you run.

Current time zone sensor​

This sensor will represent the current time zone the device is in. There are also a few attributes to help describe this time zone. Data is provided by the TimeZone API.
AttributeDescriptionin_daylight_timeIf the  time zone is currently observing daylight time.time_zone_idThe display name of the time zone.time_zone_shortThe short name of the time zone.uses_daylight_timeIf the current time zone observes daylight time.

Current version sensor​

This sensor will represent the current installed version of the Android app.

Display sensors​

Screen brightness sensor​

This sensor will report the screen brightness value as its state. An attribute also exists if the screen is currently using automatic brightness mode or not. This sensor makes use of the Settings.System API.

Screen off timeout sensor​

This sensor will report the screen off timeout value as its state, in milliseconds. This sensor makes use of the Settings.System API.

Screen orientation sensor​

This sensor reports the screen's orientation, when the screen is on and the orientation changes this sensor will update immediately. This sensor uses the Orientation API.

Screen rotation sensor​

This sensor reports the degrees of rotation relative to the device's "natural" orientation. This sensor will only update for following rotation angles: 0, 90, 180 & 270. This sensor uses the Rotation API.

Do Not Disturb sensor​

6+ only

This sensor will represent the state of Do Not Disturb (DND) on the device. The functionality of DND depends on the version of Android. Possible state values are off, priority_only, total_silence, alarms_only, unavailable or unknown. Not all states will show up on all versions of Android, for example a Pixel 4 XL will only show off or priority_only. If you never used DND you may see unavailable until you change the setting on your device. This sensor will update as soon as the state of DND changes. This sensor uses the NotificationManager API.

Dynamic color sensor​

Only available on devices with support for Material 3 Dynamic color.

This sensors state will be a hexadecimal color value for the accent color used in the current device theme. Dynamic color can either be derived from the wallpaper or chosen by the user. An attribute also exists for rgb_color in case you wanted to use this color in an automation for the light.turn_on service call. This sensor uses the Dynamic Colors API.

Doze sensor​

The state will reflect whether or not the device is in doze mode. The state will update immediately upon a state change and data is provided by PowerManager. There is one attribute ignoring_battery_optimizations which will show true or false if the Companion app is ignoring battery optimizations. If you are curious about how the state actually changes you may test it by following these outlined steps.

Frontmost app sensor​

This sensor updates immediately when the frontmost app changes.
AttributeDescriptionBundle IdentifierThe bundle identifier of the app. For example, io.home-assistant.example.Is HiddenWhether the application is hidden.Launch DateThe date (in ISO 8601, RFC 3339 format) the app was launched. For example, 2021-01-06T22:17:30-08:00.Owns Menu BarWhether the application "owns" the menu bar. For example, a menu-bar-only app will not change the contents of the menu bar, even when it is frontmost it is not necessarily as primary.

Geocoded location sensor​

The geocoded location sensor provides a user-friendly description of a users current location coordinates, often containing the name of the place, its address, and other relevant information. This sensor reports many detailed attributes allowing you to create useful template sensors.

Geocoding is handled directly by iOS's MapKit and Core Location services. In Android geocoding is handled by the internal Geocoder.
AttributeDescriptionLocationThe latitude and longitude coordinates of the placemark.NameThe name of the placemark. only and CountryThe name of the country associated with the placemark.ISOCountryCodeThe abbreviated country name.TimeZoneThe time zone associated with the placemark. onlyAdministrativeAreaThe state or province associated with the placemark.SubAdministrativeAreaAdditional administrative area information for the placemark.PostalCodeThe postal code associated with the placemark.LocalityThe city associated with the placemark.SubLocalityAdditional city-level information for the placemark.ThoroughfareThe street address associated with the placemark.SubThoroughfareAdditional street-level information for the placemark.AreasOfInterestThe relevant areas of interest associated with the placemark. onlyOceanThe name of the ocean associated with the placemark. onlyInlandWaterThe name of the inland water body associated with the placemark. onlyphoneThe phone number for the placemark, if available. premisesThe premises for the placemark, if available. urlThe URL for the placemark, if available.

Android users will have a sensor setting for the minimum required accuracy, that defaults to 200m. Users may adjust this to fit their own needs if they find inaccurate reports or not enough reports. This sensor requires either Background Location or Fine Location permissions, depending on what version of Android you run. All attributes will be lowercase and all spaces are replaced with an underscore. The sensor will only send an update if it is accurate and recent. In the full flavor, the sensor also updates with location changes if location tracking is enabled. A setting is available to keep the sensor synchronized with location updates. By default, this setting is turned off.

and  users will have a sensor setting for whether to use the name of an active Zone if present instead of the geocoded state, defaulting to not using it.

Health Connect sensors​

9+ with Play Store version • Android 14+ with all versions

note

On Android 13 or older, you need to install and set up the Health Connect app to use these sensors.

These sensors will reflect health and fitness data stored by other apps on your device in Health Connect. Unless otherwise noted, only the last 30 days of data is used.
SensorUnitDescriptionhealth_connect_active_calories_burnedkilocaloriesThe last estimate for number of active calories burned, excluding basal metabolic rate (BMR).health_connect_basal_body_temperaturecelsusThe last recorded basal body temperature.health_connect_basal_metabolic_ratekilocalories per dayThe last recorded basal metabolic rate.health_connect_blood_glucosemilligrams per deciliterThe last recorded blood glucose reading.health_connect_body_fatpercentThe last recorded body fat percentage.health_connect_body_temperaturecelsiusThe last recorded body temperature.health_connect_body_water_massgramsThe last recorded body water mass.health_connect_bone_massgramsThe last recorded bone mass.health_connect_daily_distancemetersTotal distance traveled since midnight.health_connect_daily_elevation_gainedmetersTotal elevation gained since midnight.health_connect_daily_floorsfloorsTotal floors climbed since midnight.health_connect_daily_hydrationmillilitersTotal hydration since midnight.health_connect_daily_stepsstepsTotal steps taken since midnight.health_connect_diastolic_blood_pressuremillimeters of MercuryThe last recorded diastolic blood pressure.health_connect_heart_ratebeats per minuteThe last recorded heart rate.health_connect_heart_rate_variabilitymillisecondsThe last recorded heart rate variability.health_connect_heightmetersThe last recorded height.health_connect_lean_body_massgramsThe last recorded lean body mass.health_connect_oxygen_saturationpercentThe last recorded oxygen saturation percentage.health_connect_respiratory_ratebreaths per minuteThe last recorded respiratory rate.health_connect_resting_heart_ratebeats per minuteThe last recorded resting heart rate.health_connect_sleep_durationminutesThe last recorded sleep duration.health_connect_systolic_blood_pressuremillimeters of MercuryThe last recorded systolic blood pressure.health_connect_total_calories_burnedkilocaloriesTotal amount of calories burned since midnight, including active & basal energy burned (BMR).health_connect_vo2_maxmilliliters per minute per kilogramThe last recorded VO2 max score.health_connect_weightgramsThe last recorded weight.

High accuracy mode​

This sensors state will reflect if the device has high accuracy mode currently enabled or not. This sensor will update as soon as the state of high accuracy mode changes, the sensor will not appear until high accuracy mode is enabled for the first time.

High accuracy update interval​

This sensors state will reflect the update interval for the device in seconds for high accuracy mode. This sensor will update as soon as the value changes either manually or by the notification command.

Interactive sensor​

This sensors state will reflect if the device is in an interactive state. This is typically when the screen comes on and off but may vary from device to device. This sensor will update as soon state changes are detected, data is provided by PowerManager.

Using the History Stats Integration, it is possible to monitor both the daily screen time type: time as well as the amount of times the screen has been turned on that day type: count.

Keyguard sensors​

These sensors will reflect various states from the Keyguard Manager. You will be able to determine if the device is actively locked, has a password setup or even if the device requires a password to unlock. These sensors will update with the periodic sensor interval.

Last reboot sensor​

This sensors state will be the date and time of the last reboot from the device in UTC format. The sensor will update during the normal sensor update interval. The state will be unavailable if the timestamp cannot be determined. This sensor uses the SystemClock and current System time to calculate the timestamp. This sensor offers a deadband setting, that defaults to 1 minute, to account for time calculation issues seen over certain carriers.
AttributeDescriptionLocal TimeThe date and time of the last reboot in local time.Time in MillisecondsThe time date and time of the last reboot in milliseconds.

Last update trigger sensor​

For Android this sensors state will reflect the intent of the most recent update sent. Additionally the sensor offers settings to allow the user to receive app events from other Android apps that broadcast an intent. Users can register for as many intents as they like, an event will be sent to Home Assistant once the intent has been received. Once you save an intent be sure to restart the application to register for the intent.

If you notice an intent you registered for in settings is no longer being triggered by the app then you will need to add categories that the intent is expecting. You can add categories after the intent by editing the setting for the intent and adding a , followed by the category. If more than 1 category is required then you will need to add each category followed by a , until there are no more categories to add. For example if your intent requires 2 categories the format will be: intent,category1,category2. After saving the intent and category make sure to restart the application.

This sensor displays exactly what caused the last update of location and sensor data from the device to Home Assistant.
StateDescriptionManualA manual update is triggered when the user pulls to refresh.LaunchSensors are updated upon initial app launch.PeriodicUpdates periodically according to your settings in Configuration -> Companion App -> Sensors.Significant Location ChangeTriggers when there has been a significant change in the device’s location, such as 500 meters or more. See location for additional details.Geographic Region EnteredTriggered when entering any user-specified Home Assistant zone (also known as geofencing).Geographic Region ExitedTriggered when exiting any user-specified Home Assistant zone (also known as geofencing).Push NotificationRequesting location updates via push notification.Background FetchWhen the app refreshes sensor information in the background.SiriLocation updates triggered via the Siri Shortcuts "Send Location" shortcut.iBeacon Region EnteredTriggered when an iBeacon is seen that corresponds to a known zone.RegistrationTriggered once when the app is first connected to your Home Assistant instance.SignaledTriggered when the app detects a change, such as battery state changes, while running.

Last used app sensor​

6+

The state of the sensor will always be the package name of the last used application to ensure it is always a unique value. The label of the application will be an attribute of the sensor, if it is known. This sensor updates during the normal sensor update interval and makes use of UsageStatsManager API.

Light sensor​

This sensor will reflect the current level of illuminance the device detects. The sensor updates during the normal sensor update interval or with the other sensor updates and makes use of Environment Sensors.

Mobile data sensors​

Several different sensors around the state of mobile data. These sensors make use of Settings.Global and TelephonyManager to get the mobile data states.
SensorDescriptionmobile_dataWhether or not mobile data is turned on for the device.mobile_data_roamingWhether or not mobile data roaming is turned on for the device.

Next alarm sensor​

This sensors state will be the date and time of the next alarm in UTC format. The sensor will update as soon as the next alarm is scheduled. The state will be unavailable when there is no next alarm. This sensor makes use of AlarmManager to get the next scheduled alarm which can be set by any app at any time. This sensor has settings that will let you create an allow list by selecting the packages you want to get alarm events from, just keep in mind the API is only able to get the next scheduled alarm. This setting is turned off by default.
AttributeDescriptionLocal TimeThe date and time of the next alarm in local time.PackageThe package that scheduled the next alarm.Time in MillisecondsThe time date and time of the next alarm in milliseconds.

NFC state sensor​

This sensors state will reflect if the device has its NFC sensor currently enabled or not. This sensor will update as soon state changes are detected. Data is provided by NfcAdapter.

Notification sensors​

Note: Sensors with allow lists will not appear as new entities in Home Assistant until one of the allowed apps receives a new notification.

Last notification​

This sensor will reflect the last notification posted on the device. This sensor requires a special permission that the app will take the user to so they can grant access to notifications. This sensors state will default to the text of the notification or if not available the posting package name. This sensor offers a setting for an Allow List to let the user select which packages they wish to get notification data from, notifications sent by Home Assistant are always ignored. You need to either create an allow list or enable the setting to "Disable Allow List Requirement". Keep in mind without an allow list this sensor has the potential to drain a lot of battery. We highly recommend creating an allow list over disabling this requirement. This can be very useful to integrate any app that sends a notification but does not offer direct integration (ex: food delivery apps or 2FA SMS codes). There are several attributes a user can expect to see, although not all attributes will contain data. This sensor makes use of the NotificationListenerService API.  More details on each attribute can be found in the Notification Extras.

Last removed notification​

This sensor is similar to Last Notification except that it will update when a notification has been removed from the device, either by the user or an application. You can expect to see similar attributes for this sensor, some of which are outlined below. This sensor requires the same permission as mentioned up above. This sensor also has an allow list that functions similar to Last Notification.

Active notification count​

This sensor will reflect the total active notifications on the device. This count will include notifications that are persistent and/or silent. At times it may even include the Sensor Worker notification. This sensor will update whenever any of the other sensors have an update. This sensor requires the same permissions as mentioned in Last Notification. There is no allow list for this sensor.

Below you can find some details that can be given with some notifications. These will be provided as attributes unless you disable the corresponding sensor setting.
AttributeDescriptionandroid.appInfoApp Info that contains the package name.android.infoTextText that is informative to the notification.android.largeIconThe large icon of the notification.android.progressThe progress of the notification, if it has a progress bar.android.progressIndeterminateWhether or not the progress can be determined.android.progressMaxThe max position of the progress (ex: 100 for 100%).android.reduced.imagesIf images on the notification were reduced.android.remoteInputHistoryThe most recent input for the notification.android.showChronometerIf the chronometer is shown.android.showWhenIf the notification should be shown at a specific time.android.subTextThe subtitle of the notification.android.textThe text of the notification.android.titleThe title of the notification.is_clearableIf the notification can be cleared.is_ongoingIf the notification is persistent on the device.packageThe package that posted the notification.post_timeThe time the notification was posted on the device.channel_idThe ID of the channel the notification posted to. This attribute is only available on Android 8+.group_idThe ID of the group the notification posted to.categoryThe category that the notification defined.

Media session sensor​

This sensor requires notification permissions in order to be enabled and send data. The state will be the playback state of the primary media session. If no media sessions are active then the state will be unavailable. Attributes will include a total count of active sessions and media data from all active sessions separated by package name. This sensor will update during the normal sensor update interval. To get the most out of this sensor we recommend using Last Notification to hook into your media apps to send faster updates. This sensor uses the MediaController and MediaSessionManager APIs to get the data.

Pedometer sensors​

The pedometer sensors provide step-counting data from the device's built-in motion processor. They keep a tally of your daily on-foot activity, and reset at midnight. These sensors require motion permissions to be enabled.
SensorDescriptionsensor.stepsThe number of steps taken by the user.sensor.distanceThe estimated distance (in meters) traveled by the user.sensor.average_active_paceThe average pace of the user, measured in seconds per meter.sensor.floors_ascendedThe approximate number of floors ascended by walking.sensor.floors_descendedThe approximate number of floors descended by walking.

Android users will only have a sensor.steps entity which will represent the total number of steps taken since the last device reboot. A recommended approach to getting your daily step count is to use the Utility Meter integration with cycle: daily. This sensor will update during the normal sensor update interval and makes use of the Motion Sensor. This sensor requires the Activity Recognition permission.

Phone sensors​

Data network type sensor​

The radio technology (network type) provided by the SIM slot for data transmission. Data is provided by TelephonyManager.

info

Not all 5G networks are created equally, therefore some networks may identify as LTE

Phone state sensor​

This sensor will only show up if a user explicitly grants the Phone permission for the app in your device's App Info screen. The only data tracked for this sensor are the following states: idle, ringing, offhook. This sensor will update anytime a phone state change is detected and makes use of TelephonyManager. This sensor requires the Read Phone State permission.

Signal strengh sensor​

A sensor that represents the cellular signal strength in dBm provided by the SIM slot. Attributes will exist for signal quality and also Arbritary Strength Units. Due to power saving this data may not always be current. Data is provided by TelephonyManager.

Power save sensor​

This sensor will show the state of power save mode on the device. Depending on the device this is usually a user configurable option to indicate when the device should enter a special power saving mode. The state will update as soon as a state change is detected and the sensor makes use of PowerManager.

Pressure sensor​

This sensor will show the current barometric pressure reading from the device.

On Android, this sensor will update during the normal sensor update interval and makes use of Environment Sensors.

On iOS, this sensor reads barometric pressure from the device's built-in barometer using CMAltimeter. The value is reported in hectopascals (hPa). This sensor requires motion permissions to be enabled.

Proximity sensor​

This sensor will show the current proximity reading from the device. This sensor will update during the normal sensor update interval. Not all devices report an actual reading so those devices will show either near or far depending if the sensors maximum range is 5. This sensor makes use of Position Sensors.

Public IP sensor​

This sensor uses the ipify API in order to determine the device's public IP address. This sensor will update during the normal sensor update interval.

Storage sensor​

This sensor displays information on the device storage. The file sizes reported are in Base-10.
AttributeDescriptionAvailableThe amount of available storage remaining on your device.Available (Important)The volume’s available capacity in bytes for storing important resources.Available (Opportunistic)The volume’s available capacity in bytes for storing nonessential resources.TotalThe total storage capacity of your device.

For Android the behavior is slightly different due to the differences in the 2 operating systems. The state will be the same as iOS where we show the percentage of free space, the attributes will not be identical. These sensors will update during the normal sensor update interval, calculations are done with the help of StatFs.

sensor.internal_storage
AttributeDescriptionFree internal storageThe amount of free internal storage space remaining on your device.Total internal storageThe total internal storage capacity of your device.

sensor.external_storage
AttributeDescriptionFree external storageThe amount of free external storage remaining on your SD card, for devices without a SD card it will reflect No SD Card.Total external storageThe total external storage of your SD card, for devices without a SD card it will reflect No SD Card.

Traffic stats sensors​

These sensors will show the total data transmitted and received by the device. There are both total and mobile sensors to use and the statistics reset on device reboot. These sensors use the Traffic Stats API.

Work profile sensor​

This sensor will be on if the device's work profile has been enabled, otherwise it will be off. This sensor makes use of the Device Policy Manager API.

Multi-server support
iOS & macOS sensors
When sensors update
iOS From 2025.5
Sensor list
Android sensors
How sensors update
Sensor list
Active sensor
Activity sensors
Android Auto
Android OS sensors
App data sensors
App importance sensor
App memory sensor
App usage sensors
Audio sensors
Volume levels
Battery sensors
Bluetooth sensors
Camera motion sensor
Camera stream sensor
Car sensors
Cellular provider sensor
Connection type sensor
Current time zone sensor
Current version sensor
Display sensors
Screen brightness sensor
Screen off timeout sensor
Screen orientation sensor
Screen rotation sensor
Do Not Disturb sensor
Dynamic color sensor
Doze sensor
Frontmost app sensor
Geocoded location sensor
Health Connect sensors
High accuracy mode
High accuracy update interval
Interactive sensor
Keyguard sensors
Last reboot sensor
Last update trigger sensor
Last used app sensor
Light sensor
Mobile data sensors
Next alarm sensor
NFC state sensor
Notification sensors
Last notification
Last removed notification
Active notification count
Media session sensor
Pedometer sensors
Phone sensors
Data network type sensor
Phone state sensor
Signal strengh sensor
Power save sensor
Pressure sensor
Proximity sensor
Public IP sensor
Storage sensor
Traffic stats sensors
Work profile sensor