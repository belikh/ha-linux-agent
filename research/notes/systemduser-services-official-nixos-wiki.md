---
title: Systemd/User Services - Official NixOS Wiki
id: systemduser-services-official-nixos-wiki
tags:
- linux-agent-jupiteros-fleet-15537b
- official-docs
created: '2026-09-02T04:04:41.154716Z'
updated: '2026-09-05T10:51:21.777076Z'
source: https://wiki.nixos.org/wiki/Systemd/User_Services/en
source_domain: wiki.nixos.org
fetched_at: '2026-09-02T04:04:33.147827Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Official NixOS wiki reference for systemd.user.services.<name>: user services
  are managed by the user''s own systemd instance, letting users start/stop/restart
  their services without sudo. Key mechanics: units default to stopping at logout
  unless lingering is enabled via users.users.<name>.linger = true, and lingered services
  should switch wantedBy to multi-user.target to start at boot. Per-user scoping uses
  unitConfig.ConditionUser (e.g. ''UserA|UserB'' to enable for specific users, ''!root''
  to exclude); by default a user service is installed for every user. Interaction
  is via systemctl --user and journalctl --user-unit. Directly relevant to deploying
  an agent fleet-wide on headless NixOS hosts: a system service (or lingered user
  service) is required for agents to run with no interactive login.'
---

Systemd/User Services - Official NixOS Wiki

Jump to content

Official NixOS Wiki

Search

Search

Create account

Log in

Personal tools

Create account
Log in

Systemd/User Services

From Official NixOS Wiki

Translate this page

← Back to systemd

Systemd supports running a separate instance of systemd for a given user, allowing the user to control their own services. See here for more information: https://wiki.archlinux.org/title/Systemd/User

In NixOS, a user service can be expressed with systemd.user.services.<name>, as documented here: https://search.nixos.org/options?query=systemd.user.services

This may be useful if you want a user to be able to start, stop, and restart their own instance of a service without needing to make the user a sudoer.

Here is an example:

systemd.user.services.my-cool-user-service = {
enable = true;
after = [ "network.target" ];
wantedBy = [ "default.target" ];
description = "My Cool User Service";
serviceConfig = {
Type = "simple";
ExecStart = ''/my/cool/user/service'';
};
};

By default, user services will be stopped when the user logs out and will start again when the user logs back in due to us setting wantedBy = [ "default.target" ] in the example.

Keeping user services running after logout

If you need a user service to stay running after a user logs out, you need to enable "lingering" by setting users.users.<username>.linger = true;

You'll also likely want to change to wantedBy = [ "multi-user.target" ]; so the service starts at boot time.

Enabling a service for specific users

By default, enabling a user service enables it for every user for which systemd spawns a service manager. If you wish for the service to be run only for specific users (say, UserA and UserB), use ConditionUser (man 5 systemd.unit):

systemd.user.services.my-cool-user-service = {
unitConfig.ConditionUser = "UserA|UserB";
};

Likewise, you can also disable a service for a specific user:

systemd.user.services.my-cool-user-service = {
unitConfig.ConditionUser = "!root";
};

Usage

To interact with user-specific systemd services, use the --user flag with the systemctl command. For example, to check the status of a user service:

$ systemctl --user status my-cool-user-service

To view logs for a specific user service, use journalctl with the --user-unit option:

$ journalctl --user-unit my-cool-user-service

To list all active user units:

$ systemctl --user list-units

Retrieved from "https://wiki.nixos.org/w/index.php?title=Systemd/User_Services/en&oldid=27204"

Category:
Systemd

Search

Search

Systemd/User Services

Add languages

Add topic