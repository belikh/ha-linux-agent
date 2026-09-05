---
title: Restart oneshot systemd service on every rebuild - Help - NixOS Discourse
id: restart-oneshot-systemd-service-on-every-rebuild-help-nixos-discourse
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-fleet-service-model-by-host-class
- nixos
- discourse
- user-services
created: '2026-09-02T09:57:01.898642Z'
updated: '2026-09-05T10:51:22.192738Z'
source: https://discourse.nixos.org/t/restart-oneshot-systemd-service-on-every-rebuild/70853
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T09:57:01.896960Z'
fetch_provider: builtin
status: evergreen
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'NixOS Discourse thread (Oct 2025, syvlorg + ElvishJerricco): how to restart
  systemd.user.services units on every nixos-rebuild switch. Key authoritative statements
  from ElvishJerricco: (1) ''nixos-rebuild switch only restarts nixos-activation.service
  in the user level systemd manager for users that are currently logged in'' — there
  is no user-service reactivation mechanism beyond the user nixos-activation.service
  running system.userActivationScripts; (2) there is no equivalent of the system sysinit-reactivation.target
  for user services (the workaround pattern is requiredBy = [sysinit-reactivation.target]
  + restartTriggers, but that target is a system-side construct); (3) if using the
  home-manager NixOS module, home-manager activation runs as a SYSTEM service on nixos-rebuild,
  not a user service, though it may set up user-level services. Implication for jupiterOS
  fleet: declarative restart-on-switch for user services does not exist for non-logged-in
  users; a fleet agent as a system service gets restartIfChanged semantics, while
  user services only restart if the owning user is logged in at switch time — favouring
  the system-service model for headless hosts.'
---

Restart oneshot systemd service on every rebuild - Help - NixOS Discourse

Restart oneshot systemd service on every rebuild

Help

syvlorg

October 13, 2025,  9:31pm

1

I’d like to run a script on every configuration activation, but since activation scripts are now advised against, I’m trying to figure out how to do it using a oneshot systemd service instead. serviceConfig.RemainAfterExit = false doesn’t seem to work.

eblechschmidt

October 13, 2025, 10:06pm

2

How about using systemd.services.<name>.restartTriggers

1 Like

syvlorg

October 13, 2025, 10:21pm

3

Hmm… I could restart the service based on whether a file exists or not, but that file isn’t a part of the repo, so it wouldn’t work in pure mode…

delliott

October 13, 2025, 11:59pm

4

Perhaps a system activation script that runs systemctl restart <service>?

https://search.nixos.org/options?channel=unstable&show=system.activationScripts&query=system.activationScripts

I would consider that activation scripts are discouraged as they tie things to the idea of updates and state (IE having just activated). This seems to be your goal, so doing anything else seems like activations scripts with extra steps, not solving at least my understanding of why they are discouraged.

I think there might be some systemd service that is restarted every activation, so that could be a trigger but I may be imagining its existence.

ElvishJerricco

October 14, 2025, 12:07am

5

see the section on sysinit-reactivation.target: NixOS Manual though do note that it mentions restartTriggers when that’s only necessary if you’ve set RemainAfterExit=true

delliott:

I would consider that activation scripts are discouraged as they tie things to the idea of updates and state (IE having just activated).

No. Activation scripts are discouraged because they’re a sequential, imperative hack with bad dependency management and which runs at an overly-sensitive phase of bootup. systemd services are very much preferred when at all possible because they are a much more robust model of system organization.

3 Likes

delliott

October 14, 2025, 12:09am

6

Well, fair enough.

I stand corrected, thank you.

1 Like

syvlorg

October 14, 2025,  1:06am

7

So would something similar to the following work?
{
systemd.user.services = {
a = {
...
requiredBy = [ "sysinit-reactivation.target" ];
before = [ "sysinit-reactivation.target" ];
restartTriggers = [ config.environment.etc."a.d".source ];
...
};
b = {
...
requiredBy = [ "sysinit-reactivation.target" ];
before = [ "sysinit-reactivation.target" ];
restartTriggers = [ config.environment.etc."b.d".source ];
...
};
c = {
...
before = [ "d.service" ];
after = [ "a.service" "b.service" ];
...
};
d = { ... };
};
}

I’m trying to create some files before a home-manager service is run, and would like a.service and b.service to run on every rebuild.

ElvishJerricco

October 14, 2025,  1:25am

8

Oh, user services are a whole different thing from system services. I don’t think we have an equivalent reactivation thing for user services, other than the user service nixos-activation.service, which just runs system.userActivationScripts (user activation scripts are much more reasonable than system activation scripts, though I still tend to prefer properly structured systemd services). Note that nixos-rebuild switch only restarts nixos-activation.service in the user level systemd manager for users that are currently logged in. And also note that if you’re using the home-manager NixOS module, that gets run on nixos-rebuild by a system service, not a user service, though of course that home-manager activation might setup user-level services and stuff.

syvlorg

October 14, 2025,  1:48am

9

Hmm… I don’t really need these to be user services, I think, or at least, not the ones that need to be restarted. Would switching just those two to systemd.services work with the setup above, then? Or do they all need to be systemd user services?

waffle8946

October 14, 2025,  1:54am

10

Within systemd, any relations between services (or any units) must be among services under the same service manager. There’s one service manager instance for the system, and one per user. So you cannot have user services depend on system ones or vice versa, by design.

1 Like

syvlorg

October 14, 2025,  2:11am

11

Okay, so I think I can change all of them to system services, since home-manager runs as a system service anyway. I’d wanted to move away from multiple users controlling files and directories in the system config anyway. If the services above were all system services, then, would the setup work?

waffle8946

October 14, 2025,  2:21am

12

Yes, that’s basically what we do in Hjem, though we created an intermediate target.

modules/nixos/default.nix

31f969f69

systemd.targets.hjem = {

description = "Hjem File Management";

after = ["local-fs.target"];

wantedBy = ["sysinit-reactivation.target" "multi-user.target"];

before = ["sysinit-reactivation.target"];

requires = let

requiredUserServices = name: [

"hjem-activate@${name}.service"

"hjem-copy@${name}.service"

];

We also weakened the dependency on the reactivation target from requiredBy to wantedBy to avoid brittleness - i.e. failure (for our usecase) should not disrupt activation. See the upstream docs for more context:

Often, it is a better choice to use Wants= instead of Requires= in order to achieve a system that is more robust when dealing with failing services.

I suggest testing to see what works best for your usecase.

1 Like

syvlorg

October 14, 2025,  2:45am

13

I think I want it to fail pretty loudly, so requiredBy might be better for me… By the way, can I still use User in the serviceConfig, or is that limited just to user scripts?

waffle8946

October 14, 2025,  2:50am

14

https://www.freedesktop.org/software/systemd/man/latest/systemd.directives.html#User=

https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#User=

syvlorg

October 14, 2025,  2:57am

15

Got it. I figured out a workaround to avoid different user shenanigans, so I might not need it anyway. I’ll report back with the result.

syvlorg

October 14, 2025,  7:46am

16

Everything seems to be running fine after a few permission tweaks and applying the requirements, but it’s still saying that sysinit-reactivation.target failed to restart when rebuilding, even though the target seems to be reactivating properly.

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.