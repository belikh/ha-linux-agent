---
title: '[New Add-on] Prometheus Node Exporter - Share your Projects! - Home Assistant
  Community'
id: new-add-on-prometheus-node-exporter-share-your-projects-home-assistant-community
tags:
- linux-agent-jupiteros-fleet-15537b
- repo-source
- ha-linux-agent
- home-assistant
- practitioner-forum
- node-exporter
- known-issue
created: '2026-09-02T04:02:40.506684Z'
updated: '2026-09-05T10:51:21.745830Z'
source: https://community.home-assistant.io/t/new-add-on-prometheus-node-exporter/354629
source_domain: community.home-assistant.io
fetched_at: '2026-09-02T04:02:36.656213Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'HA community thread (2021-2026, still active) for loganmarchione''s Prometheus
  Node Exporter add-on (forked by racksync) - the standard workaround for HA''s lack
  of HOST-level metrics: run prometheus node_exporter as a HAOS add-on on port 9100.
  Key operational facts: requires ''Protection mode'' DISABLED because node_exporter
  needs host-level access to read host CPU/memory/disk from inside its container (a
  security tradeoff users accept on internal networks); only amd64 tested initially,
  aarch64 (RPi4) build added later; reverse-proxy breaks the Open Web UI button; SSL
  deliberately deferred to the user''s proxy layer. Author released v3.0.0 (April
  2026) re-basing the image per HA dev blog post. Confirms the gap ha-linux-agent
  fills natively: a metrics agent that does NOT need HAOS add-on machinery, container
  privilege relaxation, or an external Prometheus server.'
---

[New Add-on] Prometheus Node Exporter - Share your Projects! - Home Assistant Community

[New Add-on] Prometheus Node Exporter

Share your Projects!

lmm7425

November 8, 2021,  7:28pm

1

prometheus_node_exporter

The Prometheus Node Exporter for hardware and OS metrics exposed by *NIX kernels.

Installation

Add my repository. The URL is https://github.com/loganmarchione/hassos-addons.

Search for the "Prometheus Node Exporter" add-on in the Supervisor add-on store and install it.

Disable "Protection mode" in the add-on panel.

Start the add-on.

Check the logs of the add-on to see if everything went well.

To verify the metrics are available, visit http://your_home_assistant_ip_address:9100/metrics in your browser, or use curl curl -X GET http://your_home_assistant_ip_address:9100/metrics.

Configuration

N/A

Usage

Add the following to the /etc/prometheus/prometheus.yml config file on your Prometheus server:
scrape_configs:
...
...
...
- job_name: 'homeassistant'
static_configs:
- targets: ['your_home_assistant_ip_address:9100']

The following Prometheus query should return data:
node_uname_info{job="homeassistant"}

Support

WIP

Authors & contributors

Logan Marchione

License

WIP

Known issues

The "Open Web UI" button doesn't work when Home Assistant is behind a reverse proxy.

Only tested on amd64 builds.

FAQ

Doesn't Home Assistant already have Prometheus integration?

Yes, but the official integration only exposes entity-related metrics, not host-related metrics.

Isn't there already an Prometheus add-on?

Yes, but that add-on is for Prometheus server, not the node exporter.

Why does this add-on require so many permissions?

The add-on needs to access to host-level metrics (CPU, memory, disk, etc...). As such, I have requested all possible permissions. Please inspect the code of this add-on before you run it.

Node exporter on raspberry pi

Latest base image with s6-overlay v3 breaks add-on

BangerTech

(BangerTech)

November 9, 2021,  7:03pm

2

thanks for the addon. it´s a bummer that this doesn´t work on a rpi 4  maybe someone could do that in the future

lmm7425

November 16, 2021, 10:25pm

3

@BangerTech - I got my hands on a RPi 4B and created an aarch64 built. It seems to work for me, test it and let me know.

hassos-addons/prometheus_node_exporter at main · loganmarchione/hassos-addons

main/prometheus_node_exporter

Home Assistant Add-ons. Contribute to loganmarchione/hassos-addons development by creating an account on GitHub.

taintedkernel

January 26, 2022, 11:31pm

4

I was just looking to set up node_exporter myself via SSH to HassOS and manual installation, when I stumbled across this.  Thank you!  I installed and seems to work fine so far on my RPi 4.

norman.rasmussen

(Norman Rasmussen)

April 10, 2022,  4:31pm

5

@lmm7425 - great job! I just installed it and it is looking great. It’s nice to have another node in my grafana boards to better understand conntrack and networking metrics.

If I can offer some feedback: I admit that I don’t know exactly what Protection Mode does with Add-ons, but I don’t really like that I have to disable it. I see that SSL is on your to-do list. Is there any help you need with either SSL or being able to use the add-on with protection mode enabled?

lmm7425

April 11, 2022,  1:43pm

6

@norman.rasmussen  Thanks, always open to feedback and help!

Node exporter requires host-level access (since it’s running in a container) so that it can read CPU, memory, disk, etc… of the host. This is why protection mode needs to be disabled. I tried to enable it, but wasn’t getting all the metrics I needed. I’m running my Home Assistant instance in my homelab and it’s not exposed to the internet, so I’m comfortable disabling protection mode internally.

As far as SSL, I’m going back and forth on that. I do want to expose metrics via HTTPS, but it shouldn’t be my add-on’s job to do that. I’m leaning more towards letting the user expose their Home Assistant instance via HTTPS (reverse proxy, directly, whatever the user wants, etc…) and then having my add-on “play nice” with that. I guess my job is to figure out the best way to do that. I’m honestly not sure what APIs I can call to get the Home Assistant URL or how to make port 9100 work if Home Assistant is behind a reverse proxy. There are a lot of scenarios that could be out there, so that’s kind of where I’m stuck…

norman.rasmussen

(Norman Rasmussen)

April 22, 2022, 10:21pm

7

Thanks for your reply! Makes sense on all fronts. I’ll look into the SSL thing and what you are considering and let you know if I come up with anything useful.

Paddi

(Paddi)

May 3, 2022,  7:05am

8

Hi.

I have some troubles making this add-on work.

I can see the metrics page on port 9100 but I am unable to get the data in my prometheus database.

Can someone help me ?

what config should I do in Grafana/prometheus database ? Should I use port 9090 or 9100 ?

Thanks a lot…

lmm7425

May 3, 2022,  3:12pm

9

@Paddi  - this is a 2-step process

You need to setup a prometheus scrape config. Below is a snippet from mine. Note that port 9100 is used here. Authentication is optional.

- job_name: 'homeassistant'
static_configs:
- targets: ['hass02.internal.mydomain.com:9100']
basic_auth:
username: user
password: asldfjaslkdfjasdflkasjdlfjasdlfknalsdfsdf

Once the scrape config is setup, you should be able to query the job from prometheus’ web interface. Note how the job name in the query below is the same as the scrape config from above.
node_uname_info{job="homeassistant"}

In Grafana, you’ll need to setup a Prometheus data source with the URL of http://prometheus_server:9090.  Then, you can setup Grafana queries using regular PromQL. You won’t be able to do anything in Grafana if you can’t get the stuff in step 1 working.

Paddi

(Paddi)

May 4, 2022, 12:14pm

10

lmm7425:

node_uname_info{job="homeassistant"}

Thank you for the reply. However as I am a total newbie in that domain, I can’t make it work.

I installer HA on a Raspi 4 with influxdb and grafana plugins.

I then added a prometheus database using port 9090.

I installed your add-on and I can see the metrics page on port 9100.

For the prometheus scraping config, I created a prometheus folder (as it was not existing) and created the prometheus.yml file. The only thing I have in it is your snippet.

I am then missing the prometheus server ? Based on what I read, adding ‘prometheus:’ in HA configuration file is enough.

So, sorry… I am a bit lost.

lmm7425

May 9, 2022,  1:24am

11

I don’t use Prometheus server installed on HA (I run my own instance on a separate Docker container), so I can’t speak to how Prometheus on HA is setup 100%.

Setting prometheus: in the HA config file exports HA data to a format a separate Prometheus server can read. That integration is documented here. Doing this does not setup a Prometheus server.

If you want to setup an actual Prometheus server on your HA instance, you want to first add the Community Beta Repo, then follow these instructions to install Prometheus server on your HA instance.

Then, you can use my snippet in the config file.

burnsba

(Ben Burns)

August 4, 2022,  1:47am

12

Thank you, I followed the directions and everything is working perfectly first try.

ddaniel

(Daniel Dekovic)

November 1, 2022,  5:36pm

13

I done all that except using your snippet in config file.

Maybe I’m asking stupid question but where the heck is config file for prometheus addon.?

ddaniel

(Daniel Dekovic)

November 6, 2022, 11:57am

14

Let me replay to my self. After some digging about docker I found out how to manually edit config file.

I done this:
sudo docker exec -u 0 -it container_id /bin/bash

I edited prometheus.yml file and save it. When I restated container or addon it reverts its configuration to default values.

Is this a bug or a feature in home assistant ?

lmm7425

November 8, 2022,  2:08am

15

@ddaniel the fact that the config is reverting is part of how containers work. Containers are ephemeral, so any changes made to them are lost upon restart. You should read about Docker volumes, it’s a way to mount an external storage volume to a container, so that changes are not lost on restart.

mgbarton

(Mgbarton)

March 6, 2026,  5:30pm

16

Thanks for your contribution.  I have an existing prometheus-node-exporter → prometheus → grafana setup in place for my other linux-based services, and HAOS was the missing piece. Dropped it in - didn’t even change the configuration - and I’m happily monitoring Home Assistant.

If you ever rev the software, it would be nice to be able to configure the nodename, so I can make it meaningful along side my other nodes, but no biggie.

Cheers!

Edit: Just saw the comments in the FAQ about changing the nodename - tweaking the prometheus configuration is a quick and easy fix.

lmm7425

April 20, 2026,  2:49am

17

Add-on dev here

Anyone here still running this add-on? I just released v3.0.0, which re-does the base image and GitHub Actions setup as per the Home Assistant dev’s blog post.

The problem is, I did this migration in multiple steps, so I wasn’t able to actually  perform an upgrade from v2–>v3 on my running instance (and I don’t run a test instance ).

Just curious if anyone is getting prompted for an upgrade from v2–>v3?

Thanks!

Powered by Discourse, best viewed with JavaScript enabled