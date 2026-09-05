---
title: What is the difference between `systemd.services` and `systemd.user.services`?
  - Help - NixOS Discourse
id: what-is-the-difference-between-systemdservices-and-systemduserservices-help-nixo
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-fleet-service-model-by-host-class
created: '2026-09-02T04:04:41.170506Z'
updated: '2026-09-05T10:51:21.764698Z'
source: https://discourse.nixos.org/t/what-is-the-difference-between-systemd-services-and-systemd-user-services/25222
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T04:04:35.073989Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'NixOS Discourse thread (Feb 2023) clarifying that systemd.user.services
  in NixOS creates REAL systemd user units (started by the user''s own systemd --user
  instance) but installs the unit file for EVERY user via /etc/systemd/user (vs home-manager
  which installs per-user). Load-bearing caveats from tejing: (1) NixOS-installed
  systemd user services do NOT start/restart on ''nixos-rebuild switch'' like system
  services do — you must relog so the user instance restarts from scratch, or handle
  it manually; (2) for a single user''s service, home-manager is the recommended mechanism;
  (3) alternative pattern is a system service with serviceConfig.User. Critical for
  fleet deployment: a system-level systemd service avoids the relog/restart gap that
  user services have under NixOS rebuilds, while user services require lingering for
  headless persistence.'
---

What is the difference between `systemd.services` and `systemd.user.services`? - Help - NixOS Discourse

What is the difference between `systemd.services` and `systemd.user.services`?

Help

busti

February 5, 2023,  4:20am

1

What is the difference between systemd.services and systemd.user.services?

Both are used quite regularly throughout nixos modules, but I never see the latter being used in conjunction with a configured user. Is it the same as an actual user service, or is it more similar to DynamicUser=yes or is there something entirely different going on?

Can I work with a systemd service without adding it the global system configuration?

tejing

February 5, 2023,  5:49am

2

It’s an actual user service, that is, it’s started by the user’s systemd instance. It’s installed for every user though, by putting the unit file in /etc/systemd/user. If you want a user service for a single user, home-manager is probably the best way to do it.

1 Like

tejing

February 5, 2023,  8:30pm

3

One other thing worth noting about nixos-installed systemd user services:

They don’t start/restart on nixos-rebuild switch like system ones do. You need to relog, causing your systemd user instance to restart from scratch, or manually handle them.

Yet another reason home-manager is often a better way to handle user services.

1 Like

claes

February 5, 2023,  9:04pm

4

So for a service that is to run as a specific user,  it can be run as system service for that user, user service for that user, and home management service for that user? When would it be the right call to let it run as a user service and not using home manager?

tejing

February 5, 2023,  9:14pm

5

claes:

When would it be the right call to let it run as a user service and not using home manager?

When you don’t have and don’t want to use home-manager, I suppose.

And to be clear, home-manager services are systemd user services, too. Just installed a different way.

busti

February 8, 2023,  8:58pm

6

tejing:

And to be clear, home-manager services are systemd user services, too. Just installed a different way.

I suppose they only run for one user rather than all users though, right?

(this implies that I expect services under systemd.user.services to run for all users once logged in, please correct me if I am wrong)

There is also the third option of setting a user to run the system service as: systemd.services.<name>.serviceConfig.User.

tejing

February 9, 2023,  2:58am

7

busti:

I suppose they only run for one user rather than all users though, right?

(this implies that I expect services under systemd.user.services to run for all users once logged in, please correct me if I am wrong)

Yes. Though it’s a bit confusing because systemd.user.services is also the name of the home-manager option for creating services for just that user. The nixos option by that name affects all users, though.

busti:

There is also the third option of setting a user to run the system service as: systemd.services.<name>.serviceConfig.User .

Yes, that’s also an option. In particular, this is how home-manager activates itself when used as a nixos module.

1 Like

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.