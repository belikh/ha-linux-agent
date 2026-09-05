---
title: 'GitHub - joonty/systemd_mon: Monitor for systemd to alert failed services
  · GitHub'
id: github-joontysystemd_mon-monitor-for-systemd-to-alert-failed-services-github
tags:
- linux-agent-jupiteros-fleet-15537b
- ha-linux-agent
- systemd
- dbus
- practitioner-guide
created: '2026-09-02T04:02:40.527268Z'
updated: '2026-09-05T10:51:21.710061Z'
source: https://github.com/joonty/systemd_mon
source_domain: github.com
fetched_at: '2026-09-02T04:02:39.749699Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'GitHub README for systemd_mon (joonty, 108 stars, MIT, Ruby): daemon that
  subscribes to systemd unit state changes over DBus - explicitly NO polling, no busy
  loops, near-zero idle cost - and fires notifications (email/Slack/HipChat, extensible
  via Ruby API) when a unit enters/leaves failed state. Key design detail: it queues
  rapid intermediate state transitions (activating -> activating(start) -> active)
  and notifies once with full history, and classifies outcomes as ''recovered''/''automatically
  restarted''/''still failed''. Ships self-watch: attempts a final notification on
  its own shutdown, acknowledges SIGKILL makes that impossible (''who watches the
  watcher''). Docker pattern requires mounting host /var/run/dbus. Useful design reference
  for ha-linux-agent: event-driven DBus subscription (mirrors zbus) beats polling
  for systemd unit monitoring on NixOS fleet hosts; the state-history summarisation
  pattern is directly portable to MQTT-published unit-health messages.'
---

GitHub - joonty/systemd_mon: Monitor for systemd to alert failed services · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

joonty

/

systemd_mon

Public

Notifications
You must be signed in to change notification settings

Fork
28

Star
108

master

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History23 Commits

23 Commits
Folders and filesNameName
Last commit message
Last commit date

bin

bin

lib

lib

.gitignore

.gitignore

Gemfile

Gemfile

LICENSE.txt

LICENSE.txt

README.md

README.md

Rakefile

Rakefile

systemd_mon.gemspec

systemd_mon.gemspec

View all files

Repository files navigation

SystemdMon

Monitor systemd units and trigger alerts for failed states. The command line tool runs as a daemon, using dbus to get notifications of changes to systemd services. If a service enters a failed state, or returns from a failed state to an active state, notifications will be triggered.

Built-in notifications include email, slack, and hipchat, but more can be added via the ruby API.

It works by subscribing to DBus notifications from Systemd. This means that there is no polling, and no busy-loops. SystemdMon will sit in the background, happily waiting and using minimal processes.

Requirements

A linux server

Ruby > 1.9.3

Systemd (v204 was used in development)

mail gem (if email notifier is used)

slack-notifier gem > 1.0 (if slack notifier is used)

hipchat (if hipchat notifier is used)

Installation

Install the gem using:

gem install systemd_mon

Usage

To run the command line tool, you will first need to create a YAML configuration file to specify which systemd units you want to monitor, and which notifications you want to trigger. A full example looks like this:

---
verbose: true # Default is off
notifiers:
email:
to: "team@mydomain.com"
from: "systemdmon@mydomain.com"
# These are options passed to the 'mail' gem
smtp:
address: smtp.gmail.com
port: 587
domain: mydomain.com
user_name: "user@mydomain.com"
password: "supersecr3t"
authentication: "plain"
enable_starttls_auto: true
slack:
webhook_url: https://hooks.slack.com/services/super/secret/tokenthings
channel: mychannel
username: doge
icon_emoji: ":computer"
icon_url: "http://example.com/icon"
hipchat:
token: bigsecrettokenhere
room: myroom
username: doge
units:
- unicorn.service
- nginx.service
- sidekiq.service

Save that somewhere appropriate (e.g. /etc/systemd_mon.yml), then start the command line tool with:

$ systemd_mon /etc/systemd_mon.yml

You'll probably want to run it via systemd, which you can do with this example service file (change file paths as appropriate):

[Unit]
Description=SystemdMon
After=network.target

[Service]
Type=simple
User=deploy
StandardInput=null
StandardOutput=syslog
StandardError=syslog
ExecStart=/usr/local/bin/systemd_mon /etc/systemd_mon.yml

[Install]
WantedBy=multi-user.target

Behaviour

Systemd provides information about state changes in very fine detail. For example, if you start a service, it may go through the following states: activating (start-pre), activiating (start) and finally active (running). This will likely happen in less than a second, and you probably don't want 3 notifications. Therefore, SystemdMon queues up states until it comes across one that you think you should know about. In this case, it will notify you when the state reaches active (running), but the notification can show the history of how the state changed so you get the full picture.

SystemdMon does simple analysis on the history of state changes, so it can summarise with statuses like "recovered", "automatically restarted", "still failed", etc. It will also report with the host name of the server.

You'll also want to know if SystemdMon itself falls over, and when it starts back up again. It will attempt to send a final notification before it exits, and one to say it's starting. However, be aware that it might not send a notification in some conditions (e.g. in the case of a SIGKILL), or a network failure. The age-old question: who will watch the watcher?

Docker integration

There is a public Docker image available which bundles all requirements (Ruby + Gems). Since systemd_mon relies on dbus, you need to mount the host dbus directory into your container. Besides that, the configuration filename is currently hardcoded to systemd_mon.yml. You have to mount the directory where the systemd_mon.yml file is located on your host system into your container as well. Below is a working example:

docker run --name "systemd_mon" -v /var/run/dbus:/var/run/dbus -v /path/to/systemd_mon/config/:/systemd_mon/ kromit/systemd_mon

If you want to run this image with systemd (very handy on CoreOS for example) you can use it as follows:

[Unit]
Description=systemd_mon
After=docker.service
Requires=docker.service

[Service]
Restart=always
RestartSec=60
ExecStartPre=-/usr/bin/docker kill systemd_mon
ExecStartPre=-/usr/bin/docker rm systemd_mon
ExecStart=/usr/bin/docker run --name "systemd_mon" -v /var/run/dbus:/var/run/dbus -v /path/to/systemd_mon/config/:/systemd_mon/ kromit/systemd_mon

[Install]
WantedBy=multi-user.target

Contributing

I'd love more contributions, particulary new notifiers. Follow the example of the slack and email notifiers and either package as a new gem or submit a pull request if you think it should be part of the main project.

Fork it ( https://github.com/joonty/systemd_mon/fork )

Create your feature branch (git checkout -b my-new-feature)

Commit your changes (git commit -am 'Add some feature')

Push to the branch (git push origin my-new-feature)

Create a new Pull Request

About
Monitor for systemd to alert failed services
Resources
Readme
MIT license
Activity
Stars
108 stars
Watchers
8 watching
Forks
28 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.