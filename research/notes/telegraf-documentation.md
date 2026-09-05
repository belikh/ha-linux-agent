---
title: Telegraf Documentation
id: telegraf-documentation
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- repo-source
- mqtt
- official-docs
created: '2026-09-02T04:02:33.491061Z'
updated: '2026-09-02T17:37:21.938958Z'
source: https://docs.influxdata.com/telegraf/v1/output-plugins/mqtt/
source_domain: docs.influxdata.com
fetched_at: '2026-09-02T04:02:26.991796Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'InfluxData official docs page for the Telegraf MQTT Producer output plugin
  (v1 docs, current as of 2026) — near-duplicate of the GitHub README but adds context:
  plugin introduced in Telegraf v0.2.0, OS support ''all'', secret-store support for
  username/password. Confirms the full config surface: brokers in ''[{scheme}://]{host}:{port}''
  format (tcp/mqtt/tls/mqtts, non-TLS and TLS not mixable), protocol 3.1.1|5, Go-template
  topics with Sprig functions, QoS default 2, keep_alive default 0 (must be non-zero
  for mosquitto v2.0.12+ per eclipse/mosquitto#2117), timeout 5s, retain flag, layouts
  non-batch/batch/field/homie-v4, MQTT v5 publish properties table ([outputs.mqtt.v5]
  content_type, response_topic, message_expiry, topic_alias, user_properties — must
  sit at END of plugin block due to TOML parsing). Also carries the same homie-v4
  limitations: / churn, no dynamic ''will'' (devices only marked lost on normal Telegraf
  exit), Homie ID sanitisation risking collisions. Relevant to ha-linux-agent as the
  incumbent-alternative design reference: what a mature fleet metrics agent publishes
  over MQTT and where its discovery/lifecycle model falls short.'
---

Telegraf Documentation

Documentation

Telegraf

InfluxDB 3
InfluxDB 3 Core New
InfluxDB 3 Enterprise New
InfluxDB Clustered
InfluxDB 3 Cloud New
InfluxDB Cloud Serverless
InfluxDB Cloud Dedicated
InfluxDB 3 Explorer New

InfluxDB 2
InfluxDB OSS v2
InfluxDB Cloud (TSM)
Flux

InfluxDB 1
InfluxDB OSS v1
InfluxDB Enterprise
InfluxDB Cloud 1

Telegraf
Telegraf
Telegraf Controller New
Telegraf Enterprise New

Other products
Chronograf
Kapacitor

Telegraf
Install Telegraf
Get started
How Telegraf works
Telegraf metrics
Data pipeline
Configure Telegraf
Configuration file
TOML syntax
Agent settings
Common plugin options
Filter metrics
Environment variables
Secrets
TLS
Labels and selectors
Use plugins
Input plugins
Parse incoming data
Output plugins
Serialize outgoing data
Processors and aggregators
External plugins
Use the execd shim
Write an external plugin
Configuration examples
Monitor system metrics
Monitor Docker containers
Collect JSON from an HTTP API
Parse CSV files
Parse log files
Collect metrics from Kafka
Collect data from MQTT
Collect industrial data
Monitor SNMP devices
Scrape Prometheus endpoints
Downsample metrics
Route metrics to outputs
Administer Telegraf
Run Telegraf as a service
Monitor Telegraf
Configure agent statuses
Manage agents at scale
Troubleshoot Telegraf
Telegraf Enterprise
Reference
Release notes
Plugin directory
Aggregator plugins
Basic Statistics
Derivative
Final
Histogram
Merge
Minimum-Maximum
Quantile
Starlark
Value Counter
Input plugins
ActiveMQ
Aerospike
Alibaba Cloud Monitor Service (Aliyun)
Amazon CloudWatch Metric Streams
Amazon CloudWatch Statistics
Amazon Elastic Container Service
AMD ROCm System Management Interface (SMI)
AMQP Consumer
Apache
Apache Aurora
Apache CouchDB
Apache Kafka Consumer
Apache Mesos
Apache Solr
Apache Tomcat
Apache Zookeeper
APC UPSD
Arista LANZ Consumer
AWS Data Firehose
Azure Event Hub Consumer
Azure Monitor
Azure Queue Storage
Bcache
Beanstalkd
Beat
BIND 9 Nameserver
Bond
Bosch Rexroth ctrlX Data Layer
Burrow
Ceph Storage
chrony
Cisco Model-Driven Telemetry (MDT)
ClickHouse
Control Group
Couchbase
Counter-Strike Global Offensive (CSGO)
CPU
Data Plane Development Kit (DPDK)
Dell EMC XtremIO
Device Mapper Cache
Directory Monitor
Disk
DiskIO
Disque
DNS Query
Docker
Docker Log
Dovecot
Elasticsearch
Elasticsearch Query
Ethtool
Exec
Execd
Fail2ban
Fibaro
File
File statistics
Filecount
Fireboard
Fluentd
Fritzbox
GitHub
gNMI (gRPC Network Management Interface)
gNMI (gRPC Network Management Interface) dial-out
Google Cloud PubSub
Google Cloud PubSub Push
Google Cloud Storage
GrayLog
HAProxy
Hashicorp Consul
Hashicorp Consul Agent
Hashicorp Nomad
Hashicorp Vault
HDDtemp
HTTP
HTTP Listener v2
HTTP Response
HueBridge
Hugepages
Icinga2
InfiniBand
InfluxDB
InfluxDB Listener
InfluxDB V2 Listener
Intel Baseband Accelerator
Intel Performance Monitoring Unit
Intel PowerStat
Intel RDT
Intel® Dynamic Load Balancer
Intel® Platform Monitoring Technology
Internet Speed Monitor
Interrupts
IPMI Sensor
Ipset
Iptables
IPVS
Jenkins
Jolokia2 Agent
Jolokia2 Proxy
Juniper Telemetry
Kapacitor
Kernel
Kernel Network Statistics
Kernel VM Statistics
Kibana
Kinesis Consumer
KNX
Kubernetes
Kubernetes Inventory
LDAP
LeoFS
Libvirt
Linux CPU
Linux Sysctl Filesystem
LM Sensors
Logical Volume Manager
LogQL
Logstash
Lustre
Mailchimp
MarkLogic
MavLink
Mcrouter
MD RAID Statistics
Memcached
Memory
Mesosphere Distributed Cloud OS
Microsoft SQL Server
Minecraft
Mock Data
Modbus
MongoDB
Monit
MQTT Consumer
Multifile
MySQL
NATS Consumer
NATS Server Monitoring
Neoom Beaam
Neptune Apex
Netfilter Conntrack
Netflow
Netgear Switch Discovery Protocol
Network
Network Connection Statistics
Network Filesystem
Network Response
Network Time Protocol Query
Nftables
Nginx
Nginx Plus
Nginx Plus API
Nginx Stream Server Traffic
Nginx Upstream Check
Nginx Virtual Host Traffic
NLnet Labs Name Server Daemon
NSQ
NSQ Consumer
Nvidia System Management Interface (SMI)
OPC UA Client Listener
OPC UA Client Reader
OpenLDAP
OpenNTPD
OpenSearch Query
OpenSMTPD
OpenStack
OpenTelemetry
OpenWeatherMap
P4 Runtime
Passenger
PF
PgBouncer
PHP-FPM
Ping
Postfix
PostgreSQL
PostgreSQL Extensible
PowerDNS
PowerDNS Recursor
Processes
Procstat
Prometheus
PromQL
Proxmox
Puppet Agent
RabbitMQ
Radius
Raindrops Middleware
RAS Daemon
RavenDB
Redfish
Redis
Redis Sentinel
RethinkDB
Riak
Riemann Listener
S.M.A.R.T.
Salesforce
SFlow
Siemens S7
SIP
Slab
SLURM
smartctl JSON
SNMP
SNMP Trap
Socket Listener
Socket Statistics
SQL
Stackdriver Google Cloud Monitoring
StatsD
Supervisor
Suricata
Swap
Synproxy
Syslog
System
System Performance Statistics
Systemd-Units
Tacacs
Tail
Teamspeak
Telegraf Internal
Temperature
Tengine Web Server
Timex
Trig
Turbostat
Twemproxy
Unbound
UPSD
uWSGI
Varnish
VMware vSphere
Webhooks
WHOIS
Windows Eventlog
Windows Management Instrumentation
Windows Performance Counters
Windows Services
Wireguard
Wireless
x509 Certificate
ZFS
Zipkin
Processor plugins
AWS EC2 Metadata
Batch
Clone
Converter
Cumulative Sum
Date
Dedup
Defaults
Enum
Execd
Filepath
Filter
Lookup
Network Interface Name
Noise
Override
Parser
Pivot
Port Name Lookup
Printer
Regex
Rename
Reverse DNS
Round
S2 Geo
Scale
SNMP Lookup
Split
Starlark
Strings
Tag Limit
Template
Timestamp
TopK
Unpivot
Secret store plugins
Docker
GoogleCloud Credentials
HashiCorp Vault
HTTP Secret store
Javascript Object Signing and Encryption
OAuth2
OS
Systemd
Output plugins
ActiveMQ STOMP
Amazon CloudWatch
Amazon CloudWatch Logs
Amazon Kinesis
Amazon Timestream
Amon
AMQP
Apache IoTDB
Arc
Azure Application Insights
Azure Data Explorer
Azure Event Hubs
Azure Monitor
Clarify
CrateDB
Datadog
Discard
Dynatrace
Elasticsearch
Executable
Executable Daemon
File
Google BigQuery
Google Cloud Monitoring
Google Cloud PubSub
Grafana Loki
Graphite
Graylog
GroundWork
Health
Heartbeat
HTTP
InfluxDB v1.x
InfluxDB v2.x
InfluxDB v3.x
Inlong
Instrumental
Kafka
Librato
Logz.io
Microsoft Fabric
MongoDB
MQTT Producer
NATS
Nebius Cloud Monitoring
New Relic
NSQ
OpenSearch
OpenTelemetry
OpenTSDB
Parquet
PostgreSQL
Prometheus
Quix
Redis Time Series
Remote File
Riemann
Sensu Go
SignalFx
Socket Writer
SQL
Sumo Logic
Syslog
Warp10
Wavefront
Websocket
Yandex Cloud Monitoring
Zabbix
Telegraf commands
telegraf config
telegraf config create
telegraf config migrate
telegraf plugins
telegraf plugins inputs
telegraf plugins outputs
telegraf plugins parsers
telegraf plugins serializers
telegraf plugins processors
telegraf plugins aggregators
telegraf plugins secretstores
telegraf secrets
telegraf secrets get
telegraf secrets list
telegraf secrets set
telegraf service
telegraf version
Data formats
Input data formats
Avro
Binary
collectd
CSV
Dropwizard
Form URL-encoded
Graphite
Grok
InfluxDB line protocol
JSON
JSON v2
logfmt
Nagios
OpenMetrics
OpenTSDB
Parquet
Prometheus
Prometheus Remote Write
Value
Wavefront
XML
XPath CBOR
XPath JSON
XPath MessagePack
XPath Protocol Buffers
Output data formats
Binary
Carbon2
CloudEvents
CSV
Graphite
InfluxDB line protocol
JSON
MessagePack
Prometheus
Prometheus Remote Write
ServiceNow metrics
Splunk metric
Template
Wavefront
Template patterns
Agent status evaluation
CEL variables
CEL functions and operators
CEL expression examples
Supported platforms
Glossary
Contribute to Telegraf
Documentation MCP server

Telegraf v0.2.0+

Copy page for AI

Plugin source
Download configuration
MQTT Producer Output Plugin
This plugin writes metrics to a MQTT broker acting as a MQTT producer.
The plugin supports the MQTT protocols 3.1.1 and 5.

In v2.0.12+ of the mosquitto MQTT server, there is a bug
requiring the keep_alive value to be set non-zero in Telegraf. Otherwise,
the server will return with identifier rejected.
As a reference eclipse/paho.golang sets the keep_alive to 30.
Introduced in: Telegraf v0.2.0
Tags: messaging
OS support: all
Global configuration options
Plugins support additional global and plugin configuration settings for tasks
such as modifying metrics, tags, and fields, creating aliases, and configuring
plugin ordering. See CONFIGURATION.md for more details.
Secret store support
This plugin supports secrets from secret stores for the username and
password option.
See the secret store documentation for more details on how
to use them.
Configuration
# Configuration for MQTT server to send metrics to
[[outputs.mqtt]]
## MQTT Brokers
## The list of brokers should only include the hostname or IP address and the
## port to the broker. This should follow the format `[{scheme}://]{host}:{port}`. For
## example, `localhost:1883` or `mqtt://localhost:1883`.
## Scheme can be any of the following: tcp://, mqtt://, tls://, mqtts://
## non-TLS and TLS servers can not be mix-and-matched.
servers = ["localhost:1883", ] # or ["mqtts://tls.example.com:1883"]

## Protocol can be `3.1.1` or `5`. Default is `3.1.1`
# protocol = "3.1.1"

## MQTT Topic for Producer Messages
## MQTT outputs send metrics to this topic format:
## prefix/{{ .Tag "host" }}/{{ .Name }}/{{ .Tag "tag_key" }}
## (e.g. prefix/web01.example.com/mem/some_tag_value)
## Each path segment accepts either a template placeholder, an environment variable, or a tag key
## of the form `{{.Tag "tag_key_name"}}`. All the functions provided by the Sprig library
## (http://masterminds.github.io/sprig/) are available. Empty path elements as well as special MQTT
## characters (such as `+` or `#`) are invalid to form the topic name and will lead to an error.
## In case a tag is missing in the metric, that path segment omitted for the final topic.
topic = 'telegraf/{{ .Tag "host" }}/{{ .Name }}'

## QoS policy for messages
## The mqtt QoS policy for sending messages.
## See https://www.ibm.com/support/knowledgecenter/en/SSFKSJ_9.0.0/com.ibm.mq.dev.doc/q029090_.htm
##   0 = at most once
##   1 = at least once
##   2 = exactly once
# qos = 2

## Keep Alive
## Defines the maximum length of time that the broker and client may not
## communicate. Defaults to 0 which turns the feature off.
##
## For version v2.0.12 and later mosquitto there is a bug
## (see https://github.com/eclipse/mosquitto/issues/2117), which requires
## this to be non-zero. As a reference eclipse/paho.mqtt.golang defaults to 30.
# keep_alive = 0

## username and password to connect MQTT server.
# username = "telegraf"
# password = "metricsmetricsmetricsmetrics"

## client ID
## The unique client id to connect MQTT server. If this parameter is not set
## then a random ID is generated.
# client_id = ""

## Timeout for write operations. default: 5s
# timeout = "5s"

## Optional TLS Config
# tls_ca = "/etc/telegraf/ca.pem"
# tls_cert = "/etc/telegraf/cert.pem"
# tls_key = "/etc/telegraf/key.pem"

## Use TLS but skip chain & host verification
# insecure_skip_verify = false

## When true, metric will have RETAIN flag set, making broker cache entries until someone
## actually reads it
# retain = false

## Layout of the topics published.
## The following choices are available:
##   non-batch -- send individual messages, one for each metric
##   batch     -- send all metric as a single message per MQTT topic
## NOTE: The following options will ignore the 'data_format' option and send single values
##   field     -- send individual messages for each field, appending its name to the metric topic
##   homie-v4  -- send metrics with fields and tags according to the 4.0.0 specs
##                see https://homieiot.github.io/specification/
# layout = "non-batch"

## HOMIE specific settings
## The following options provide templates for setting the device name
## and the node-ID for the topics. Both options are MANDATORY and can contain
## {{ .Name }} (metric name), {{ .Tag "key"}} (tag reference to 'key') or
## constant strings. The templates MAY NOT contain slashes!
# homie_device_name = ""
# homie_node_id = ""

## Each data format has its own unique set of configuration options, read
## more about them here:
## https://github.com/influxdata/telegraf/blob/master/docs/DATA_FORMATS_OUTPUT.md
data_format = "influx"

## NOTE: Due to the way TOML is parsed, tables must be at the END of the
## plugin definition, otherwise additional config options are read as part of
## the table

## Optional MQTT 5 publish properties
## These setting only apply if the "protocol" property is set to 5. This must
## be defined at the end of the plugin settings, otherwise TOML will assume
## anything else is part of this table. For more details on publish properties
## see the spec:
## https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901109
# [outputs.mqtt.v5]
#   content_type = ""
#   response_topic = ""
#   message_expiry = "0s"
#   topic_alias = 0
# [outputs.mqtt.v5.user_properties]
#   "key1" = "value 1"
#   "key2" = "value 2"
field layout
This layout will publish one topic per metric field, only containing the
value as string. This means that the data_format option will be ignored.
For example writing the metrics
modbus,location=main\ building,source=device\ 1,status=ok,type=Machine\ A temperature=21.4,serial\ number="324nlk234r5u9834t",working\ hours=123i,supplied=true 1676522982000000000
modbus,location=main\ building,source=device\ 2,status=offline,type=Machine\ B temperature=25.0,supplied=true 1676522982000000000
with configuration
[[outputs.mqtt]]
topic = 'telegraf/{{ .Name }}/{{ .Tag "source" }}'
layout = "field"
...
will result in the following topics and values
telegraf/modbus/device 1/temperature    21.4
telegraf/modbus/device 1/serial number  324nlk234r5u9834t
telegraf/modbus/device 1/supplied       true
telegraf/modbus/device 1/working hours  123
telegraf/modbus/device 2/temperature    25
telegraf/modbus/device 2/supplied       false
NOTE: Only fields will be output, tags and the timestamp are omitted. To
also output those, please convert them to fields first.
homie-v4 layout
This layout will publish metrics according to the
Homie v4.0 specification. Here, the topic template will be
used to specify the device-id path. The mandatory options
homie_device_name will specify the content of the $name topic of the device,
while homie_node_id will provide a template for the node-id part of the
topic. Both options can contain Go templates similar to topic
with {{ .Name }} referencing the metric name and {{ .Tag "key"}} referencing
the tag with the name key.
Sprig helper functions are available.
For example writing the metrics
modbus,source=device\ 1,location=main\ building,type=Machine\ A,status=ok temperature=21.4,serial\ number="324nlk234r5u9834t",working\ hours=123i,supplied=true 1676522982000000000
modbus,source=device\ 2,location=main\ building,type=Machine\ B,status=offline supplied=false 1676522982000000000
modbus,source=device\ 2,location=main\ building,type=Machine\ B,status=online supplied=true,Throughput=12345i,Load\ [%]=81.2,account\ no="T3L3GrAf",Temperature=25.38,Voltage=24.1,Current=100 1676542982000000000
with configuration
[[outputs.mqtt]]
topic = 'telegraf/{{ .Name }}'
layout = "homie-v4"

homie_device_name ='{{ .Name }} plugin'
homie_node_id = '{{ .Tag "source" }}'
...
will result in the following topics and values
telegraf/modbus/$homie                            4.0
telegraf/modbus/$name                             modbus plugin
telegraf/modbus/$state                            ready
telegraf/modbus/$nodes                            device-1

telegraf/modbus/device-1/$name                    device 1
telegraf/modbus/device-1/$properties              location,serial-number,source,status,supplied,temperature,type,working-hours

telegraf/modbus/device-1/location                 main building
telegraf/modbus/device-1/location/$name           location
telegraf/modbus/device-1/location/$datatype       string
telegraf/modbus/device-1/status                   ok
telegraf/modbus/device-1/status/$name             status
telegraf/modbus/device-1/status/$datatype         string
telegraf/modbus/device-1/type                     Machine A
telegraf/modbus/device-1/type/$name               type
telegraf/modbus/device-1/type/$datatype           string
telegraf/modbus/device-1/source                   device 1
telegraf/modbus/device-1/source/$name             source
telegraf/modbus/device-1/source/$datatype         string
telegraf/modbus/device-1/temperature              21.4
telegraf/modbus/device-1/temperature/$name        temperature
telegraf/modbus/device-1/temperature/$datatype    float
telegraf/modbus/device-1/serial-number            324nlk234r5u9834t
telegraf/modbus/device-1/serial-number/$name      serial number
telegraf/modbus/device-1/serial-number/$datatype  string
telegraf/modbus/device-1/working-hours            123
telegraf/modbus/device-1/working-hours/$name      working hours
telegraf/modbus/device-1/working-hours/$datatype  integer
telegraf/modbus/device-1/supplied                 true
telegraf/modbus/device-1/supplied/$name           supplied
telegraf/modbus/device-1/supplied/$datatype       boolean

telegraf/modbus/$nodes                            device-1,device-2

telegraf/modbus/device-2/$name                    device 2
telegraf/modbus/device-2/$properties              location,source,status,supplied,type

telegraf/modbus/device-2/location                 main building
telegraf/modbus/device-2/location/$name           location
telegraf/modbus/device-2/location/$datatype       string
telegraf/modbus/device-2/status                   offline
telegraf/modbus/device-2/status/$name             status
telegraf/modbus/device-2/status/$datatype         string
telegraf/modbus/device-2/type                     Machine B
telegraf/modbus/device-2/type/$name               type
telegraf/modbus/device-2/type/$datatype           string
telegraf/modbus/device-2/source                   device 2
telegraf/modbus/device-2/source/$name             source
telegraf/modbus/device-2/source/$datatype         string
telegraf/modbus/device-2/supplied                 false
telegraf/modbus/device-2/supplied/$name           supplied
telegraf/modbus/device-2/supplied/$datatype       boolean

telegraf/modbus/device-2/$properties              account-no,current,load,location,source,status,supplied,temperature,throughput,type,voltage

telegraf/modbus/device-2/location                 main building
telegraf/modbus/device-2/location/$name           location
telegraf/modbus/device-2/location/$datatype       string
telegraf/modbus/device-2/status                   online
telegraf/modbus/device-2/status/$name             status
telegraf/modbus/device-2/status/$datatype         string
telegraf/modbus/device-2/type                     Machine B
telegraf/modbus/device-2/type/$name               type
telegraf/modbus/device-2/type/$datatype           string
telegraf/modbus/device-2/source                   device 2
telegraf/modbus/device-2/source/$name             source
telegraf/modbus/device-2/source/$datatype         string
telegraf/modbus/device-2/temperature              25.38
telegraf/modbus/device-2/temperature/$name        Temperature
telegraf/modbus/device-2/temperature/$datatype    float
telegraf/modbus/device-2/voltage                  24.1
telegraf/modbus/device-2/voltage/$name            Voltage
telegraf/modbus/device-2/voltage/$datatype        float
telegraf/modbus/device-2/current                  100
telegraf/modbus/device-2/current/$name            Current
telegraf/modbus/device-2/current/$datatype        float
telegraf/modbus/device-2/throughput               12345
telegraf/modbus/device-2/throughput/$name         Throughput
telegraf/modbus/device-2/throughput/$datatype     integer
telegraf/modbus/device-2/load                     81.2
telegraf/modbus/device-2/load/$name               Load [%]
telegraf/modbus/device-2/load/$datatype           float
telegraf/modbus/device-2/account-no               T3L3GrAf
telegraf/modbus/device-2/account-no/$name         account no
telegraf/modbus/device-2/account-no/$datatype     string
telegraf/modbus/device-2/supplied                 true
telegraf/modbus/device-2/supplied/$name           supplied
telegraf/modbus/device-2/supplied/$datatype       boolean
Important notes and limitations
It is important to notice that the “devices” and “nodes” are dynamically
changing in Telegraf as the metrics and their structure is not known a-priori.
As a consequence, the content of both $nodes and $properties topics are
changing as new device-ids, node-ids, and properties (tags and fields)
appear. Best effort is made to limit the number of changes by keeping a
superset of all devices and nodes seen, however especially during startup those
topics will change more often. Both topic and homie_node_id should be chosen
in a way to group metrics with identical structure!
Furthermore, lifecycle management of devices is very limited! Devices will
only be in ready state due to the dynamic nature of Telegraf. Due to
limitations in the MQTT client library, it is not possible to set a “will”
dynamically. In consequence, devices are only marked lost when exiting
Telegraf normally and might not change in abnormal aborts.
Note that all field- and tag-names are automatically converted to adhere to
the Homie topic ID specification. In that process, the
names are converted to lower-case and forbidden character sequences (everything
not being a lower-case character, digit or hyphen) will be replaces by a hyphen.
Finally, leading and trailing hyphens are removed.
This is important as there is a risk of name collisions between fields and
tags of the same node especially after the conversion to ID. Please make sure
to avoid those collisions as otherwise property topics will be sent multiple
times for the colliding items.

Was this page helpful?
Yes
No

Thank you for your feedback!

Support and feedback
Thank you for being part of our community!
We welcome and encourage your feedback and bug reports for Telegraf and this documentation.
To find support, use the following resources:
InfluxDB Community Slack (Preferred)
InfluxData Community
InfluxDB Subreddit
Customers with an annual or support contract can contact InfluxData Support.
Edit this page
Submit docs issue
Submit Telegraf issue
© 2026 InfluxData, Inc.

Where are you running InfluxDB?
Select your InfluxDB Cloud region and cluster or your InfluxDB OSS URL and we’ll customize code examples for you. Identify your InfluxDB Cloud cluster.

InfluxDB Cloud
InfluxDB OSS or Enterprise

AWS

US West (Oregon)

us-west-2-1

us-west-2-2

US East (Virginia)

EU Frankfurt

GCP

US Central (Iowa)

Azure

West Europe (Amsterdam)

East US (Virginia)

Default

localhost:8086

Custom

For more information, see InfluxDB Cloud regions or InfluxDB OSS URLs.

Thank you for your feedback!
Let us know what we can do better:

Thank you!

No thanks

InfluxDB OSS 2.9.0: API tokens are hashed by default

Stronger token security in InfluxDB OSS 2.9.0 — tokens are
hashed on disk by default. Existing tokens are hashed on first
startup and can’t be recovered afterward. Capture any plaintext
tokens you still need before you upgrade.
View InfluxDB OSS 2.9.0 release notes

Hashed tokens authenticate exactly like unhashed tokens — clients
and integrations keep working.
Also new in 2.9.0:
Configurable backup compression
Restore support for backups containing hashed tokens
Tighter Edge Data Replication queue validation
Flux upgrade
Compaction reliability improvements

Key enhancements in Explorer 1.9

Explorer 1.9 is now available with InfluxQL support, an AI-assisted
Flux to SQL converter (beta), and new live sample data simulators.
View Explorer 1.9 release notes

Explorer 1.9 includes new features and improvements that make it easier to
query, visualize, and manage data.
Highlights:
Flux to SQL converter (beta): Convert Flux queries to SQL with an AI-assisted converter.
InfluxQL support: Query data with InfluxQL in the Data Explorer and dashboards, and save and load InfluxQL queries.
InfluxQL visualizations: Render line and bar charts from InfluxQL results with per-tag series grouping.
Query error history: Review a history of query errors in the query tool.
Live sample data simulators: Generate continuous live sample data with new bird data and signal generator simulators.
For more details, see Explorer 1.9 release notes

InfluxDB 3.11 is now available
InfluxDB 3 Core 3.11 improves processing engine trigger reliability and
adds a configurable graceful shutdown.

Key updates in InfluxDB 3 Core 3.11:

Processing engine trigger reliability improvements—capped async
trigger concurrency, bounded retries for failed triggers, and WAL
triggers that skip empty flushes.

Configurable graceful shutdown with the new --shutdown-timeout option.

Read the announcement

InfluxDB 3 Core release notes

InfluxDB 3 Enterprise 3.11: A significant performance upgrade for complex time series workloads
Performance and flexibility for heavier, more complex workloads.

Key updates in InfluxDB 3 Enterprise 3.11:

Upgraded storage engine reaches general availability. New
clusters default to Parquet; upgrade with --upgrade-pacha-tree.

In-place and incremental backup and restore.

Integrated Explorer UI—run with --mode all,webui.

Bulk import: import from remote object store sources (for
example, S3) with configurable concurrency.

Compaction and query performance improvements.

Catalog migration—back up your catalog before upgrading.

Read the announcement

InfluxDB 3 Enterprise release notes

Telegraf Controller 1.1 now available
Telegraf Controller 1.1 adds high availability (Telegraf Enterprise),
global constants, configuration groups, configuration aliases, and
configuration versioning.

Telegraf Controller 1.1 lets teams make fleet-wide configuration changes
with a single edit using
global constants,
configuration groups, and
configuration aliases.
Configuration versioning makes
every change traceable, comparable, and reversible.
High availability, available with
Telegraf Enterprise, automatically fails over between Controller instances
so agents can continue pulling configurations and reporting health if an
instance goes down.
Read the announcement
See the release notes
Download and install Telegraf Controller 1.1

InfluxDB OSS 1.13.0 is now available

InfluxDB OSS 1.13.0 adds mTLS support, adaptive TSI cache sizing, and
a hardening option to mitigate SSRF.
View InfluxDB OSS 1.13.0 release notes

Key updates in InfluxDB OSS 1.13.0:
mTLS support for the HTTP, OpenTSDB, and subscriber services.
Adaptive TSI cache sizing—the series ID set cache can grow and
shrink based on measured query hit rate.
hardening-enabled option to mitigate server-side request
forgery (SSRF) in Flux HTTP requests.
For details, see the
InfluxDB OSS 1.13.0 release notes.

InfluxDB Enterprise 1.13.0 is now available

InfluxDB Enterprise 1.13.0 adds mTLS support and changes the default
replication factor from 3 to 2.
View InfluxDB Enterprise 1.13.0 release notes

Key updates in InfluxDB Enterprise 1.13.0:
Default replication factor changed to 2 (previously 3) for new
databases and retention policies. Existing ones aren’t affected.
mTLS support for Enterprise data and meta nodes.
All InfluxDB OSS 1.13.0 updates apply to Enterprise too.
For details, see the
InfluxDB Enterprise 1.13.0 release notes.

InfluxDB Docker latest tag changing to InfluxDB 3 Core
On September 15, 2026, the latest tag for InfluxDB Docker images will
point to InfluxDB 3 Core. To avoid unexpected upgrades, use specific version
tags in your Docker deployments.

If using Docker to install and run InfluxDB, the latest tag will point to
InfluxDB 3 Core. To avoid unexpected upgrades, use specific version tags in
your Docker deployments. For example, if using Docker to run InfluxDB v2,
replace the latest version tag with a specific version tag in your Docker
pull command–for example:
docker pull influxdb:2

Ask AI