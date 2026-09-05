---
title: Adding NixOS option for systemd user lingering - Development - NixOS Discourse
id: adding-nixos-option-for-systemd-user-lingering-development-nixos-discourse
tags:
- linux-agent-jupiteros-fleet-15537b
created: '2026-09-02T04:04:41.174703Z'
updated: '2026-09-05T10:51:21.755411Z'
source: https://discourse.nixos.org/t/adding-nixos-option-for-systemd-user-lingering/28762
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T04:04:36.095792Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'NixOS Discourse thread (June 2023) on adding a first-class NixOS option
  for systemd user lingering. Quotes loginctl(1): enable-linger spawns a user manager
  at boot and keeps it around after logouts, letting users who are not logged in run
  long-running services; mechanically it just creates an empty file at /var/lib/systemd/linger/USER.
  rnhmjoj''s interim workaround (6 likes): systemd.tmpfiles.rules = [ "f /var/lib/systemd/linger/alice"
  ... ] — declaratively creating the linger file without a dedicated option. Use-cases
  named: headless servers running Syncthing via home-manager when no one is logged
  in, and rootless Docker/podman containers that must stay up without a login session.
  Counter-position from peterhoeg: for headless servers without interactive users,
  run the daemon as a system service (nixos module, or virtualisation.oci-containers
  with DynamicUser=yes) instead of lingering user services. Links nixpkgs issue #3702
  (Enabling persistent user instance systemd). Relevant to fleet agent deployment:
  the linger-file-via-tmpfiles trick is the declarative NixOS way to keep user-scoped
  agents alive headless.'
---

Adding NixOS option for systemd user lingering - Development - NixOS Discourse

Adding NixOS option for systemd user lingering

Development

aidalgol

June 5, 2023,  2:55am

1

I have started using the systemd “user lingering” feature on one of my NixOS systems, and I would like to add a NixOS option for controlling this functionality.  From the loginctl(1) man page:

enable-linger [USER...], disable-linger [USER...]

Enable/disable user lingering for one or more users. If enabled for a specific user, a user manager is spawned for the user at boot and kept around after logouts. This allows users who are not logged in to run long-running services. Takes one or more user names or numeric UIDs as argument. If no argument is specified, enables/disables lingering for the user of the session of the caller.

Enabling this this simply creates an empty file at /var/lib/systemd/linger/USER.

I think the option belongs either in users.users.<name> as a boolean attribute, or in systemd.users as a list of usernames.  As far as I can tell, there are no systemd-related options under users.users.<name>, so systemd.users seems the more appropriate place.  I would like to get some feedback before opening a pull request to implement the option.

3 Likes

peterhoeg

June 5, 2023,  3:10am

2

Out of curiosity, what is the use-case here?

aidalgol

June 5, 2023,  3:18am

3

The system is a headless server, and this allows users to have Syncthing (installed via home-manager) running when not logged in.

peterhoeg

June 5, 2023,  3:39am

4

If it’s a headless server without any interactive users, can’t you run syncthing using the nixos module instead?

aidalgol

June 5, 2023,  3:54am

5

There are interactive users, and Syncthing is designed to be run as a normal user if you want multi-user support.

mightyiam

June 5, 2023,  4:27am

6

If you don’t have enough feedback, consider pinging the people listed as maintainers of the package you intend to contribute to. The maintainers are in the package files somewhere.

1 Like

rnhmjoj

June 5, 2023,  7:08am

7

It’s not super elegant, but you can simply do
# Enable linger for some user
systemd.tmpfiles.rules = [
"f /var/lib/systemd/linger/alice"
"f /var/lib/systemd/linger/bob"
];

6 Likes

linj

June 9, 2023,  4:07pm

8

related Enabling persistent user instance systemd · Issue #3702 · NixOS/nixpkgs · GitHub

2 Likes

j0sh

October 2, 2023, 12:11am

9

Linger is important for running Docker services in rootless mode. Otherwise you need a logged-in user session for the containers to stay up, which is not ideal for network services that are supposed to be persistent.

2 Likes

peterhoeg

October 3, 2023,  8:16pm

10

I run zigbee2mqtt in a container, but no lingering is needed for that. podman runs in a regular systemd system unit with DynamicUser = yes set.

j0sh

October 3, 2023,  9:10pm

11

Possibly, I haven’t figured out the incantations needed to get docker compose into a NixOS systemd unit. But linger worked for my purposes.

peterhoeg

October 4, 2023,  1:29pm

12

I haven’t had a need for docker/podman-compose, so I can’t really say anything about that, but you can configure containers through virtualisation.oci-containers which is what I do for zigbee2mqtt. Very straight-forward.

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.