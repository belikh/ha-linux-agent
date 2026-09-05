---
title: 'What is MQTT Last Will and Testament (LWT)? – MQTT Essentials: Part 9'
id: what-is-mqtt-last-will-and-testament-lwt-mqtt-essentials-part-9
tags:
- linux-agent-jupiteros-fleet-15537b
- vendor-blog
- primary-source
- birth-message
- availability-semantics
- locus-mqtt-lifecycle-supervisor-spec
created: '2026-09-02T07:20:56.294071Z'
updated: '2026-09-02T17:37:22.391381Z'
source: https://www.hivemq.com/blog/mqtt-essentials-part-9-last-will-and-testament/
source_domain: www.hivemq.com
fetched_at: '2026-09-02T07:20:56.292357Z'
fetch_provider: builtin
status: review
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'HiveMQ MQTT Essentials Part 9 (updated Feb 2026, vendor-authoritative protocol
  explainer) on Last Will and Testament semantics — the primary cited by HA community
  threads on birth/LWT. Core contract: the LWT message (topic, payload, QoS, retain
  flag) is specified in the MQTT CONNECT packet at connection initiation; the broker
  stores it and publishes it to the will topic''s subscribers when it detects an ungraceful
  disconnect (keepalive timeout, network break); the broker DISCARDS the stored LWT
  if the client disconnects gracefully with a DISCONNECT packet. Use cases listed
  include notifying others of client unavailability — exactly the fleet-agent availability
  pattern. Confirms why a systemd-graceful agent stop never triggers LWT: the agent
  must either publish its own offline status before DISCONNECT (shutdown-message pattern,
  cf. ESPHome shutdown_message) or let HA-side expire_after catch staleness. Part
  of a 10-part series (Part 7 persistent sessions, Part 8 retained messages, Part
  10 keep alive and client take-over are adjacent context for fleet design).'
---

*Suggested by [[setting-mqtt-birth-and-last-will-with-the-new-mqtt-integration-configuration-hom]] — tom_l cites HiveMQ MQTT Essentials Part 9 as the authority on LWT semantics referenced in the thread*

What is MQTT Last Will and Testament (LWT)? – MQTT Essentials: Part 9

Skip to content

Search the website   Search

See more results / press return for more results

MQTT
What is MQTT Last Will and Testament (LWT)? – MQTT Essentials: Part 9

by
HiveMQ Team Feb 9, 2026 10 min read

Table of Contents

What is MQTT Last Will and Testament (LWT)? – MQTT Essentials: Part 9
What is the Purpose of Last Will and Testament (LWT) in MQTT?
How to Configure a Last Will and Testament (LWT) Message for an MQTT Client?
When does the MQTT Broker Send the LWT Message?
When to Use Last Will and Testament (LWT) in MQTT?
The Importance of Last Will and Testament in MQTT: A Summary

The MQTT Essentials Series
View other content in this series: View other content in this series
MQTT Tutorial: An Easy Guide to Getting Started with MQTTIntroducing the MQTT Protocol – MQTT Essentials: Part 1MQTT Publish/Subscribe Architecture (Pub/Sub) – MQTT Essentials: Part 2MQTT Client, MQTT Broker, and MQTT Server Connection Establishment Explained – MQTT Essentials: Part 3MQTT Publish, MQTT Subscribe & Unsubscribe – MQTT Essentials: Part 4MQTT Topics, Wildcards, & Best Practices – MQTT Essentials: Part 5What is MQTT Quality of Service (QoS) 0,1, & 2? – MQTT Essentials: Part 6Understanding Persistent Sessions and Clean Sessions – MQTT Essentials: Part 7What are Retained Messages in MQTT? – MQTT Essentials: Part 8What is MQTT Last Will and Testament (LWT)? – MQTT Essentials: Part 9What Is MQTT Keep Alive and Client Take-Over? – MQTT Essentials Part 10A Beginner's Guide to MQTT BrokersIntroduction to MQTT 5 Protocol - MQTT 5 Essentials Part 1MQTT 5 Subscription Options: A Quick Guide MQTT 5 Vs. MQTT 3 – MQTT 5 Essentials Part 2MQTT 5: Seven Reasons to Upgrade to it from MQTT 3.1.1 – MQTT 5 Essentials Part 3MQTT Session Expiry and Message Expiry Intervals – MQTT 5 Essentials Part 4MQTT 5’s Improved Client Feedback & Negative ACKs – MQTT 5 Essentials Part 5What are MQTT User Properties? – MQTT 5 Essentials Part 6MQTT Shared Subscriptions – MQTT 5 Essentials Part 7MQTT Payload Format Description and Content Type – MQTT 5 Essentials Part 8MQTT Request-Response Pattern – MQTT 5 Essentials Part 9MQTT Topic Alias – MQTT 5 Essentials Part 10Enhanced Authentication - MQTT 5 Essentials Part 11MQTT Flow Control – MQTT 5 Essentials Part 12MQTT 5 - Why You Need It and Potential Pitfalls Go

TL;DR

This blog explains MQTT Last Will and Testament (LWT), showing how clients can notify others when they disconnect unexpectedly. It covers configuration, use cases, and why LWT is critical for reliable IoT systems.

Who is this blog for: IoT Developers, MQTT Enthusiasts.

Last Will and Testament (LWT) is a powerful feature in MQTT that allows clients to specify a message that will be automatically published by the broker on their behalf, if or when an unexpected disconnection occurs. It provides a reliable means of communication and ensures that clients can gracefully handle disconnections without leaving topics in an inconsistent state. This feature is particularly valuable when clients must notify others of their unavailability or convey important information upon an unexpected disconnection.
Here’s Part 9 of MQTT Essentials, a ten-part blog series on the core features and concepts of the MQTT protocol, where we we will dive into the concept of Last Will and Testament (LWT) in detail. If you want to understand what are Retained Messages in MQTT?, check out Part 8 of this series. Else, let’s dive in to LWT.
What is the Purpose of Last Will and Testament (LWT) in MQTT?
In scenarios where unreliable networks are prevalent, it is common for MQTT clients to experience occasional unintended breaks, which can happen due to loss of connection or depleted batteries. Understanding the type of disconnection (graceful - with a disconnect message, or ungraceful - without a disconnect message) is crucial for taking appropriate actions.

The Last Will and Testament feature in MQTT offers a solution for clients to respond effectively to ungraceful disconnects and ensure proper handling of such events.

By clicking on the image, you interact with a video on YouTube.

Please read our
privacy policy page to understand how we process data.

The LWT allows clients to notify others about their unexpected disconnections. When a client connects to a broker, it can specify a last-will message. This message follows the structure of a regular MQTT message structure, including a topic, retained message flag, Quality of Service (QoS), and payload. The broker stores this message until it detects an ungraceful disconnect from the client. Upon detecting the disconnection, the broker broadcasts the last will message to all subscribed clients of the corresponding topic. The broker discards the stored LWT message if the client disconnects gracefully using the DISCONNECT message.
DISCONNECT MQTT Packet
By utilizing LWT, you can implement various strategies to handle client disconnections and inform other clients about the offline status.
How to Configure a Last Will and Testament (LWT) Message for an MQTT Client?
To specify an LWT message for an MQTT client, you include it in the CONNECT message, which is used to initiate the connection between the client and the broker.
CONNECT MQTT Packet
For detailed information on establishing the connection between the client and broker, read our article MQTT Client, MQTT Broker, and MQTT Server Connection Establishment Explained.
When does the MQTT Broker Send the LWT Message?
According to the MQTT 3.1.1 specification, the broker sends a client’s Last Will and Testament (LWT) message in the following situations:

I/O error or network failure: If the broker detects any issues with the input/output or network connection, it will distribute the LWT message.

Failed communication within Keep Alive period: If the client fails to communicate with the broker within the specified Keep Alive period, the LWT message is sent. In Part-10 of our MQTT Essentials, we will explore the concept of MQTT Keep Alive time and delve into its significance it.

Client closes connection without DISCONNECT: When the client terminates the network connection without sending a DISCONNECT packet, the broker ensures the LWT message is distributed.

Broker closes connection due to protocol error: If the broker closes the network connection due to a protocol error, it will send the LWT message.
Understanding when and why the broker sends the Last Will and Testament (LWT) messages lays the groundwork for implementing best practices in leveraging this feature, which we will delve into in the next section.
When to Use Last Will and Testament (LWT) in MQTT?
LWT proves invaluable for alerting subscribed clients about an abrupt disconnection of a client. It becomes a powerful tool for storing and communicating client state on specific topics when combined with retained messages.
For instance, by setting a lastWillMessage with Offline payload, enabling the lastWillRetain flag, and specifying the lastWillTopic as client1/status, followed by publishing an Online retained message to the same topic, client1 can keep newly-subscribed clients informed about its online status. Should client1 disconnect unexpectedly, the broker publishes the LWT message with Offline payload as the new retained message, ensuring that clients subscribing to the topic while client1 is offline receive the LWT message and stay up to date on its current status.
LWT not only notifies subscribed clients about unexpected disconnections but also assists in maintaining the system’s integrity by providing valuable information on client states. Combining LWT with retained messages allows you to create a robust solution that stores and communicates the latest client state on specific topics, ensuring reliable updates for all subscribers. This approach enables seamless integration and synchronization between clients, enhancing the overall resilience and functionality of the MQTT network.
The Importance of Last Will and Testament in MQTT: A Summary
To summarize, the Last Will and Testament (LWT) feature in MQTT is crucial in ensuring efficient communication and maintaining system integrity in the event of unexpected client disconnections. By combining LWT with retained messages, developers can store and communicate client state on specific topics, providing valuable information to subscribed clients. LWT empowers MQTT networks with enhanced resilience, seamless integration, and reliable updates, making it a powerful tool for various applications. By understanding the benefits and best practices of LWT, you can leverage this feature to create robust and effective MQTT solutions.
That brings us to the end of Part 9 of our MQTT Essentials series. In the next and the final part of this series, we’ll cover the MQTT heartbeat mechanism and how the broker knows a client is online or offline.
Are you enjoying our content? Then sign up for our newsletter below. Subscribe to our RSS feed here to stay updated. Do check out MQTT FAQs and MQTT Glossary to know all the key MQTT terminologies. Watch the video below that complements the concepts discussed in this article.

FAQs on MQTT Last Will and Testament (LWT)

Can I customize the LWT settings for different clients or topics?

Can I combine MQTT Last Will and Testament (LWT) with QoS?

How does MQTT handle multiple LWT messages for the same client?

What happens if a client connects with the same client identifier as a previously disconnected client with a Last Will and Testament message?

How does MQTT manage multiple LWT messages for a client subscribed to various topics?

How does MQTT handle Last Will and Testament messages in scenarios with intermittent network connectivity?

What happens to LWT messages when a client with a persistent session reconnects to the broker?

How does MQTT handle Last Will and Testament messages in a high-availability or clustered broker setup?

Are LWT messages replicated and synchronized between multiple MQTT brokers?

Navigate this series:

#9 What are Retained Messages in MQTT? – MQTT Essentials: Part 8
#11 What Is MQTT Keep Alive and Client Take-Over? – MQTT Essentials Part 10

HiveMQ Team

Team HiveMQ brings together deep expertise in MQTT, Industrial AI, IoT data streaming, UNS, and Industrial IoT protocols. Follow us for practical deployment guidance, best practices for building a secure, reliable data backbone, and insights into how we are shaping the future of connected industries.
Our mission is to transform industrial data into real-time intelligence, actionable insights, and measurable business outcomes.
Have questions or need support? Contact us. Our experts are ready to help.

Related content:

How to win an Industrial Data Innovation Award
Enter the Industrial Data Innovation Awards - recognizing excellence in MQTT, industrial AI and industrial data innovation.

Blog
Why strong MQTT adoption sets the stage for manufacturing data intelligence
Why strong MQTT adoption is the foundation for manufacturing data intelligence. Learn how semantic context and governance accelerate operational insights.

Blog
OPC UA and MQTT: How to Bridge OT Protocols for Scalable Industrial Data
Bridge OPC UA and MQTT to scale your industrial data. Learn architectures, translation methods, and best practices for seamless OT-to-IT connectivity.

Blog

HiveMQ Reviews

Newsletter sign up
By clicking the subscribe button you give your consent to the use of your data according to our
Privacy Policy. You can withdraw your consent at any time with future effect.

Opens in a new window/tab