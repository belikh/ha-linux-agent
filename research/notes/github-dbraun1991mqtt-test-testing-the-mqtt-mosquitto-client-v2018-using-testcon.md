---
title: 'GitHub - dbraun1991/mqtt-test: Testing the MQTT mosquitto-client v2.0.18 using
  Testcontainers · GitHub'
id: github-dbraun1991mqtt-test-testing-the-mqtt-mosquitto-client-v2018-using-testcon
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- repo-source
- home-assistant
- mqtt-discovery
- repo-map
- testing
- known-issue
- practitioner-forum
created: '2026-09-02T05:39:32.177805Z'
updated: '2026-09-02T17:37:22.265159Z'
source: https://github.com/dbraun1991/mqtt-test
source_domain: github.com
fetched_at: '2026-09-02T05:39:30.803558Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'dbraun1991/mqtt-test (Java/Spring Boot, 7 commits): reproducible Mosquitto
  2.0.18 test scenario with Testcontainers, motivated by eclipse-paho/paho.mqtt.java#673.
  README documents the exact failure mode that makes MQTT integration tests flaky:
  the app connects fine to a local broker at tcp://localhost:1883, but when the test
  runs under Testcontainers ''The test fails on client.connect'' — connecting to a
  containerised broker whose address/port differs from the hardcoded/env config. Low-star
  repo, practitioner evidence only, but the failure signature (hardcoded broker URL
  vs ephemeral mapped port) is the exact class of bug a ha-linux-agent MQTT test harness
  must avoid by always deriving broker_url from the container''s get_host()/get_host_port_ipv4()
  rather than fixed localhost:1883.'
---

GitHub - dbraun1991/mqtt-test: Testing the MQTT mosquitto-client v2.0.18 using Testcontainers · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

dbraun1991

/

mqtt-test

Public

Notifications
You must be signed in to change notification settings

Fork
0

Star
0

main

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History7 Commits

7 Commits
Folders and filesNameName
Last commit message
Last commit date

endpoint

endpoint

.gitignore

.gitignore

LICENSE

LICENSE

README.md

README.md

pom.xml

pom.xml

techstack.md

techstack.md

View all files

Repository files navigation

MQTT Mosquitto-Test

This REPO offers a testing scenario, which connects to a MQTT Mosquitto Client using testcontainers.

Its main goal is reproducability.

Why is this REPO needed?

On 'Apr 3, 2020' requested in:

eclipse-paho/paho.mqtt.java#673

Setup

Tested on:

Hardware

MacBook Pro

Processor: Apple M2 Pro

RAM: 32 GB

macOS: 14.2.1

Desktop PC (only Test with Testcontainers)

Prozessor:	AMD Ryzen 5 2600X Six-Core Processor with 3.60 GHz

RAM: 16 GB

Windows: 11 Home

Systemtype: 64-Bit OS/Prozessor

Software

Java OpenJDK v21

Spring Boot v3.2.x

Maven v3.9.x

IntelliJ: 2023.3.3

Docker Desktop: 4.27.0

Mosquitto:

brew info mosquitto   ==>   mosquitto: stable 2.0.18 (bottled)

testcontainers   ==>   DockerImageName.parse("eclipse-mosquitto:2.0.18"))

Installation for testing:

1) Local instance of mosquitto

brew install mosquitto
brew services start mosquitto

2) Check your mosquitto using bash:

mosquitto_sub -h localhost -p 1883 -t default

mosquitto_pub -h localhost -p 1883 -t default -m "Test message"

3) Verify assertion:

The MQTT Mosquitto client is available at tcp://localhost:1883.

Given in the src/main/resources/application.env

4) Check out the POM.mxl and run the maven lifecycle steps

clean

validate

compile

5) Run the Java-application (without TEST-Environment)

Do not forget to give the Mosquitto-URL parameter (.env-File).

Sample application-LOG:

2024-01-26T15:16:41.178+01:00  INFO 2318 --- [application] [-192.168.128.24] o.a.c.c.C.[Tomcat].[localhost].[/]       : Initializing Spring DispatcherServlet 'dispatcherServlet'
2024-01-26T15:16:41.179+01:00  INFO 2318 --- [application] [-192.168.128.24] o.s.web.servlet.DispatcherServlet        : Initializing Servlet 'dispatcherServlet'
2024-01-26T15:16:41.181+01:00  INFO 2318 --- [application] [-192.168.128.24] o.s.web.servlet.DispatcherServlet        : Completed initialization in 1 ms
2024-01-26T15:16:41.839+01:00  INFO 2318 --- [application] [   scheduling-1] de.application.domain.cron.MessageCron   : Scheduled CRON - publishing a new random UUID to topic: 'defaultTopic'
2024-01-26T15:16:51.822+01:00  INFO 2318 --- [application] [   scheduling-1] de.application.domain.cron.MessageCron   : Scheduled CRON - publishing a new random UUID to topic: 'defaultTopic'
2024-01-26T15:17:01.820+01:00  INFO 2318 --- [application] [   scheduling-1] de.application.domain.cron.MessageCron   : Scheduled CRON - publishing a new random UUID to topic: 'defaultTopic'

Sample mosquitto-LOG:

mosquitto_sub -h localhost -p 1883 -t defaultTopic
7d6de8e0-b825-4a4d-8daf-6af0e27ee421
6d9aad95-cc10-44c3-a48f-035cec099f41
fd4754ad-86f4-4c96-a9c9-0df7a80b2845

That far so good

Hopefully you faced no problems until here.
Everything runs fine and 'as expected'.

Using the test-environment including Testcontainers

1) Start your docker-environment for the testcontainers

2) Start the test ... .. .

Either run the maven lifecycle step test
.. OR ..
go to the test failing at my side at:

src/test/java/de/slr/kbaas/authendpoint/AuthEndpointApplicationTest.java

3) . .. ... and fail

The test fails on 'client.connect',:

2024-01-26T15:15:41.829+01:00  INFO 2281 --- [application] [           main] d.a.i.m.mqtt.MqttServiceImpl             : Startup - ENV - mqttUrl - About to connect to    ==>   tcp://localhost:50318
2024-01-26T15:15:41.871+01:00 ERROR 2281 --- [application] [           main] d.a.i.m.mqtt.MqttServiceImpl             : Exception in 'connectToBroker' in 'MqttServiceImpl'

org.eclipse.paho.client.mqttv3.MqttException: Verbindung wurde getrennt
at org.eclipse.paho.client.mqttv3.internal.CommsReceiver.run(CommsReceiver.java:197) ~[org.eclipse.paho.client.mqttv3-1.2.5.jar:na]
at java.base/java.lang.Thread.run(Thread.java:1583) ~[na:na]
Caused by: java.io.EOFException: null
at java.base/java.io.DataInputStream.readUnsignedByte(DataInputStream.java:297) ~[na:na]
at java.base/java.io.DataInputStream.readByte(DataInputStream.java:275) ~[na:na]
at org.eclipse.paho.client.mqttv3.internal.wire.MqttInputStream.readMqttWireMessage(MqttInputStream.java:92) ~[org.eclipse.paho.client.mqttv3-1.2.5.jar:na]
at org.eclipse.paho.client.mqttv3.internal.CommsReceiver.run(CommsReceiver.java:137) ~[org.eclipse.paho.client.mqttv3-1.2.5.jar:na]
... 1 common frames omitted

2024-01-26T15:15:42.488+01:00  INFO 2281 --- [application] [           main] o.s.b.a.e.web.EndpointLinksResolver      : Exposing 1 endpoint(s) beneath base path '/actuator'

Logs are given in:

logs/infrastructure/mqtt/foo

Fixing the MQTT mosquitto in this Testcontainers-setup

Change the DockerImageName.parse("eclipse-mosquitto:2.0.18"))
in the applicationTest-class according to your local or published new mosquitto container.

Authors and acknowledgment

Technical & Maintenance:

dbraun1991 (Maintainer)

About
Testing the MQTT mosquitto-client v2.0.18 using Testcontainers
Resources
Readme
MIT license
Activity
Stars
0 stars
Watchers
1 watching
Forks
0 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.