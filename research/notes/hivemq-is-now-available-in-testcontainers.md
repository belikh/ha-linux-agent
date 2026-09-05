---
title: HiveMQ is now available in Testcontainers
id: hivemq-is-now-available-in-testcontainers
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- source-code
- repo-source
- official-docs
- known-issue
- testing
- vendor-blog
created: '2026-09-02T05:39:32.183005Z'
updated: '2026-09-02T17:37:22.240557Z'
source: https://www.hivemq.com/blog/hivemq-is-now-available-in-testcontainers/
source_domain: www.hivemq.com
fetched_at: '2026-09-02T05:39:32.141524Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'HiveMQ blog (27 Apr 2022, vendor announcement) on the HiveMQ Testcontainers
  module: ''Automated integration testing of MQTT applications is a challenging task
  since it requires you to simulate or set up an MQTT deployment of some kind... sharing
  the deployment across multiple tests can cause unwanted interference between them
  and produce unexpected results and flaky tests.'' Key practices: Testcontainers
  auto-creates per-container port mappings to mitigate port collisions, auto-deletes
  containers after tests, supports programmatic container networks for multi-service
  setups; HiveMQ module adds lifecycle management ensuring the broker and extensions
  are running before tests execute, and pre-configures in-memory persistence since
  disk persistence is pointless for ephemeral test setups. Vendor-backed framing,
  but the flaky-shared-broker and ephemeral-persistence rationales transfer directly
  to Mosquitto-based testing of ha-linux-agent.'
---

HiveMQ is now available in Testcontainers

Skip to content

Search the website   Search

See more results / press return for more results

HiveMQMQTT Client
HiveMQ is now available in Testcontainers

by
HiveMQ Team Apr 27, 2022 10 min read

Table of Contents

HiveMQ is now available in Testcontainers
Testcontainers + HiveMQ
Test your MQTT application
Example
Test your custom HiveMQ extension
Example
Start Testing Now!

Automated integration testing of MQTT applications is a challenging task since it requires you to simulate or set up an MQTT deployment of some kind. When testing MQTT client applications, you need a deployed MQTT broker. But sharing the deployment across multiple tests can cause unwanted interference between them and produce unexpected results and flaky tests. Integration-testing custom HiveMQ extensions can be even more difficult since the extension must be packaged, deployed, and started automatically. We frequently encountered this issue while developing our Enterprise Security Extension and the HiveMQ Extension for Kafka. So, to eliminate this pain point, we developed the HiveMQ Testcontainer in 2020. Testcontainers is a Java library that gives you the right tools for automated JUnit 4 and JUnit 5 integration-testing with throwaway instances of third party services.
Presenting the HiveMQ Testcontainers Module
Today, we proudly announce that HiveMQ Testcontainer is now available through the Testcontainers Project and will be called HiveMQ Testcontainers module. You can find the source code on GitHub and the official documentation can be found on the Testcontainers website.
Testcontainers + HiveMQ
Thanks to its large community and adoption, the Testcontainers library combines many best practices for integration testing with Docker. The library makes it easy to interface with the Docker api and provides an intuitive abstraction that is great for creating integration tests. It’s not necessary to build the setup manually via docker run etc. prior to a test as it can be created completely automatically. Testcontainers also automatically creates port-mapping for each container to mitigate the risk of port-collisions and other interference. After the tests are executed, Testcontainers deletes all containers, to conserve resources. One feature we particularly enjoy at HiveMQ is that Testcontainers allows you to programmatically create networks of docker containers. This is extremely useful when you need to test the integration of multiple software services. For example, we use this extensively when testing the HiveMQ Extension for Kafka, where a setup requires both HiveMQ and Kafka The HiveMQ Testcontainers Module builds on top of all the benefits of Testcontainers and adds more advantages that are specially adapted to HiveMQ:

Support for HiveMQ Community Edition and HiveMQ Enterprise Edition

HiveMQ specific lifecycle management to make sure that HiveMQ and all HiveMQ extensions are up and running before tests are executed.

As HiveMQ fully supports MQTT 3 and MQTT 5 every MQTT application can be tested end-to-end. See Test Your MQTT Application

The HiveMQ Testcontainers Module eases the configuration of the HiveMQ instance, because it provides a well-defined API to:

Add a custom HiveMQ config

Add a HiveMQ extension

Enable or disable a HiveMQ extension during runtime

Add a license file

Enable the HiveMQ Control Center

And much more

Full support for the HiveMQ Extension system, allows you to fully test HiveMQ Extensions in containerized setup. See Test Your Custom HiveMQ Extension

HiveMQ configuration optimizes a testing environment and speeds up test execution. For example, HiveMQ is pre-configured with in-memory persistence as disk-persistence is not required for ephemeral test-setups.
Test your MQTT application
When fully testing an MQTT application you want to simulate an actual setup as closely as possible. Therefore, a full MQTT broker that your application can connect to and receive and publish data to, is required. In order to have a successful test, the broker must be available even when you are offline and it needs to be exclusive to your test, so that unwanted interference between tests and other applications is ruled out. The HiveMQ Testcontainers Module gives you exactly that. Before a test starts, the container boots up and makes sure that it is completely set up before the test code runs. Right after the test execution has finished, the container is automatically destroyed.
Example
Testing your MQTT application can look something like this:

Create a new HiveMQContainer.

Register the Rule with JUnit 4

Connect your MQTT clients to the HiveMQ instance that is running inside the container. Use the host and port that you retrieve from the rule with the getMqttPort() and getHost() methods.

Assert the expected behavior.
public class TestcontainerExample {

@Rule // 2
public HiveMQContainer hivemq
= new HiveMQContainer(DockerImageName.parse("hivemq/hivemq-ce:latest")); // 1

@Test
public void test_mqtt() throws Exception {
var publisher = Mqtt5Client.builder()
.serverPort(hivemq.getMqttPort()) // 3
.serverHost(hivemq.getHost())
.identifier("publisher")
.buildBlocking();

publisher.connect();

var subscriber = Mqtt5Client.builder()
.serverPort(hivemq.getMqttPort()) // 3
.serverHost(hivemq.getHost())
.identifier("subscriber")
.buildBlocking();

var publishes = subscriber.publishes(MqttGlobalPublishFilter.ALL);
subscriber.connect();
subscriber.subscribeWith().topicFilter("topic/test").send();

publisher.publishWith()
.topic("topic/test")
.payload("Hello World!".getBytes()).send();

var receive = publishes.receive();

assertNotNull(receive); // 4
assertEquals("Hello World!", new String(receive.getPayloadAsBytes())); // 4
}
}

Test your custom HiveMQ extension
One of HiveMQ’s key features is that you can highly customize its behaviour with HiveMQ Extensions. The open HiveMQ extension framework adds many possibilities to tailor HiveMQ to your specific use-case:

Intercept and manipulate MQTT messages

Integrate other services

Collect statistics

Add fine-grained security

And much more
HiveMQ extensions can be very sophisticated pieces of software that also require a fully fledged automatic testing environment. The HiveMQ Testcontainers Module allows you to load your HiveMQ Extension into the HiveMQ instance that is running in the container. With that, you can test the behaviour of HiveMQ that is altered by your custom extension logic. This gives you a real end-to-end test strategy for your own HiveMQ extension.
Example
Testing your HiveMQ extension can look something like this: The extension to test provides a custom PublishInboundInterceptor which replaces every incoming publish payload with the string “modified”. It looks like this:  public class MyExtension implements ExtensionMain {

@Override
public void extensionStart(@NotNull ExtensionStartInput extensionStartInput, @NotNull ExtensionStartOutput extensionStartOutput) {

final PublishInboundInterceptor publishInboundInterceptor = (publishInboundInput, publishInboundOutput) -> {
publishInboundOutput.getPublishPacket().setPayload(ByteBuffer.wrap("modified".getBytes(StandardCharsets.UTF_8)));
};

final ClientInitializer clientInitializer = (initializerInput, clientContext) -> {
clientContext.addPublishInboundInterceptor(publishInboundInterceptor);
};

Services.initializerRegistry().setClientInitializer(clientInitializer);
}

...

}

To actually test the HiveMQ extension, we do the following:

Create a new HiveMQContainer

Register the Rule with JUnit 4

Add the HiveMQ Extension to the HiveMQContainer by providing its id, name, version, priority , start-priority and main class

Connect your MQTT clients to the HiveMQ instance that is running inside the container. Use the host and port that you retrieve from the rule with the getMqttPort() and getHost() methods.

Assert the expected behavior.
public class TestMqttIT {

public static final HiveMQExtension HIVE_MQ_EXTENSION = HiveMQExtension.builder()
.id("extension-1")
.name("my-extension")
.version("1.0")
.mainClass(MyExtension.class).build();

@Rule // 2
public final HiveMQContainer hivemq
= new HiveMQContainer(DockerImageName.parse("hivemq/hivemq-ce:latest")) // 1
.withExtension(HIVE_MQ_EXTENSION); // 3

@Test
public void test_mqtt() throws Exception {
var publisher = Mqtt5Client.builder()
.serverPort(hivemq.getMqttPort()) // 4
.identifier("publisher")
.buildBlocking();

publisher.connect();

var subscriber = Mqtt5Client.builder()
.serverPort(hivemq.getMqttPort()) // 4
.identifier("subscriber")
.buildBlocking();

var publishes = subscriber.publishes(MqttGlobalPublishFilter.ALL);
subscriber.connect();
subscriber.subscribeWith().topicFilter("topic/test").send();

publisher.publishWith()
.topic("topic/test")
.payload("Hello World!".getBytes()).send();

var receive = publishes.receive();

assertNotNull(receive); // 5
assertEquals("modified", new String(receive.getPayloadAsBytes())); // 5
}
}

Include in your project
To use the HiveMQ Testcontainers Module, add the following dependencies to your build.gradle:  testImplementation("org.testcontainers:testcontainers:1.17.1")
testImplementation("org.testcontainers:hivemq:1.17.1")

Start Testing Now!
Using the HiveMQ Testcontainers Module makes the process of automatic testing of MQTT client applications and custom HiveMQ extensions easier and more robust. The HiveMQ Testcontainers Module combines the long-proven best practices and established abstractions that the testcontainers project provides with HiveMQ specific API. To start testing now, head over to the Testcontainers Project and get set up with HiveMQ.

HiveMQ Team

Team HiveMQ brings together deep expertise in MQTT, Industrial AI, IoT data streaming, UNS, and Industrial IoT protocols. Follow us for practical deployment guidance, best practices for building a secure, reliable data backbone, and insights into how we are shaping the future of connected industries.
Our mission is to transform industrial data into real-time intelligence, actionable insights, and measurable business outcomes.
Have questions or need support? Contact us. Our experts are ready to help.

Related content:

What Has HiveMQ Shipped Over the Last Six Months?
HiveMQ's last six months: unified governance, enterprise security updates, improved diagnostics, and steady Edge and Kubernetes operator progress.

Blog
HiveMQ Now Transactable on Microsoft Marketplace
HiveMQ is now available on Microsoft Azure Marketplace. Buy through public plans or private offers and apply eligible purchases toward your MACC commitment.

Blog
Build, Integrate, and Grow: Announcing the New HiveMQ Partner Network
HiveMQ Partner Network (HPN) empowers SIs, VARs, and ISVs to scale enterprise IoT and industrial AI solutions with enablement, margin protection, and co-sell support.

Blog

HiveMQ Reviews

Newsletter sign up
By clicking the subscribe button you give your consent to the use of your data according to our
Privacy Policy. You can withdraw your consent at any time with future effect.

Opens in a new window/tab