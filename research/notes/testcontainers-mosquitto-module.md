---
title: Testcontainers Mosquitto Module
id: testcontainers-mosquitto-module
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- repo-source
- mqtt-discovery
- primary-source
- birth-message
- testing
- rust
created: '2026-09-02T05:39:32.142575Z'
updated: '2026-09-02T17:37:22.248383Z'
source: https://testcontainers.com/modules/mosquitto/
source_domain: testcontainers.com
fetched_at: '2026-09-02T05:39:25.516547Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'Official Testcontainers community module page for Eclipse Mosquitto: gives
  the exact Rust usage for a throwaway MQTT broker in tests — ''cargo add -F mosquitto
  --dev testcontainers-modules'' then ''testcontainers_modules::mosquitto::Mosquitto::default().start()''
  and building broker_url from get_host() + get_host_port_ipv4(1883). Shows Go/.NET/Python
  equivalents (testcontainers[mqtt], eclipse-mosquitto:2). Directly applicable to
  ha-linux-agent''s reliability problem: integration tests can run against a real
  ephemeral MQTT broker instead of untested live-broker code paths.'
---

Testcontainers Mosquitto Module

Modules

Documentation
Go

.NET

Python

Rust

Community Module
These modules are maintained by the community, outside of the Testcontainers project.

DescriptionEclipse Mosquitto is an open source message broker which implements MQTT version 5, 3.1.1 and 3.1.

Examples

Go

.NET

Python

Rust

Dependency:
go get github.com/testcontainers/testcontainers-go/modules/mosquitto

Usage:
mosquittoContainer, err := mosquitto.Run(context.Background(), "eclipse-mosquitto:2")

Dependency:
dotnet add package Testcontainers.Mosquitto

Usage:
var mosquittoContainer = new MosquittoBuilder("eclipse-mosquitto:2.0")
.Build();
await mosquittoContainer.StartAsync();

Dependency:
pip install testcontainers[mqtt]

Usage:
with MosquittoContainer(image = "eclipse-mosquitto:2.0.20") as mosquitto_broker:
mqtt_client = mosquitto_broker.get_client()

Dependency:
cargo add -F mosquitto --dev testcontainers-modules

Usage:
testcontainers_modules::mosquitto::Mosquitto::default().start()

let broker_url = format!(
"{}:{}",
mosquitto_instance.get_host().unwrap(),
mosquitto_instance.get_host_port_ipv4(1883).unwrap()
);