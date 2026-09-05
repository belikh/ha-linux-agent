---
title: 'switch-to-configuration-ng: harden user-unit migration second pass by r-vdp
  · Pull Request #517768 · NixOS/nixpkgs · GitHub'
id: switch-to-configuration-ng-harden-user-unit-migration-second-pass-by-r-vdp-pull
tags:
- linux-agent-jupiteros-fleet-15537b
- systemd
- nixos
- source-code
- birth-message
- community-thread
- gap-03
- stc-ng
- user-unit-restart
created: '2026-09-02T16:29:19.127722Z'
updated: '2026-09-02T17:37:22.626675Z'
source: https://github.com/NixOS/nixpkgs/pull/517768
source_domain: github.com
fetched_at: '2026-09-02T16:29:17.436637Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'PR #517768 (r-vdp, opened 2026-05-07, merged to staging-nixos 2026-05-26
  via 907350b, backported to release-26.05/staging-26.05): hardens switch-to-configuration-ng''s
  post-activation user-unit migration second pass. Establishes the mechanism: the
  pass runs ''for units migrating from a per-user manager (home-manager) to NixOS''
  and ''unconditionally restarts or starts any candidate'' (bug fixed: dbus-broker.service
  opting out via reloadIfChanged was restarted anyway, killing session clients). Reworked
  candidate selection: active unit + new generation defines it in /etc/systemd/user
  + (FragmentPath under $XDG_CONFIG_HOME/systemd/user OR anywhere outside /etc not
  previously in /etc). Honours X-ReloadIfChanged/X-RestartIfChanged/RefuseManualStop/RefuseManualStart/X-OnlyManualStart.
  Also proves nixos-activation.service is ''explicitly restarted by stc-ng'' (via
  RemainAfterExit=yes + restartIfChanged=false to run exactly once per switch, commit
  663a59e, with test asserting single run). Commit 6ced06a1b''s broadened candidate
  selection is the trigger of the 2026-05-26 session-crash regression per Discourse
  79578.'
---

switch-to-configuration-ng: harden user-unit migration second pass by r-vdp · Pull Request #517768 · NixOS/nixpkgs · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

Uh oh!

There was an error while loading. Please reload this page.

NixOS

/

nixpkgs

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
20k

Star
26k

Merged

switch-to-configuration-ng: harden user-unit migration second pass#517768

r-vdp merged 4 commits into
NixOS:staging-nixosNixOS/nixpkgs:staging-nixosfrom
r-vdp:stc-user-migration-directivesr-vdp/nixpkgs:stc-user-migration-directivesCopy head branch name to clipboard

Conversation

r-vdp

commented

May 7, 2026

Copy link

Copy Markdown

Contributor

The post-activation pass added in 5cc82c4 to handle units migrating from a per-user manager (home-manager) to NixOS unconditionally restarts or starts any candidate. dbus-broker.service explicitly opts out of restarts via reloadIfChanged because restarting the session bus kills running clients; the second pass ignored that and restarted it anyway.

Honour the same X-* directives handle_modified_unit checks in the second pass.

Also tighten the candidate set: only consider units whose pre-activation FragmentPath is exactly under $XDG_CONFIG_HOME/systemd/user.

Things done

Built on platform:

x86_64-linux

aarch64-linux

x86_64-darwin

aarch64-darwin

Tested, as applicable:

NixOS tests in nixos/tests: nixosTests.switchTest (added userServiceMigratedToNixosReloadOnly and userServiceMigratedToNixosNoRestart specialisations).

Package tests at passthru.tests.

Tests in lib/tests or pkgs/test for functions and "core" functionality.

Ran nixpkgs-review on this PR. See nixpkgs-review usage.

Tested basic functionality of all binary files, usually in ./result/bin/.

Nixpkgs Release Notes

Package update: when the change is major or breaking.

NixOS Release Notes

Module addition: when adding a new NixOS module.

Module update: when the change is significant.

Fits CONTRIBUTING.md, pkgs/README.md, maintainers/README.md and other READMEs.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

r-vdp

requested review from
ElvishJerricco,
jmbaur and
phaer

May 7, 2026 17:43

nixpkgs-ci
Bot

added

10.rebuild-linux: 1-10

This PR causes between 1 and 10 packages to rebuild on Linux.

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

6.topic: nixos

Issues or PRs affecting NixOS modules, or package usability issues specific to NixOS

labels

May 7, 2026

r-vdp

force-pushed
the

stc-user-migration-directives

branch
from
42e2d6e    to
b146aa5
Compare

May 8, 2026 09:38

r-vdp

mentioned this pull request

May 17, 2026

nixos/activation: fix handling of user activation
#521094

Closed

13 tasks

r-vdp

commented

May 17, 2026

View reviewed changes

Comment thread

pkgs/by-name/sw/switch-to-configuration-ng/src/main.rs

Show resolved

Hide resolved

Uh oh!

There was an error while loading. Please reload this page.

r-vdp

force-pushed
the

stc-user-migration-directives

branch
from
b146aa5    to
230aec8
Compare

May 20, 2026 22:46

r-vdp

requested a review
from Ma27

May 20, 2026 22:47

r-vdp

commented

May 20, 2026

Copy link

Copy Markdown

Contributor

Author

@Ma27 I cherry-picked your two commits here. Would you be able to review this? Ideally we would merge this before branch-off.

In this state, I think we have the most correct implementation, combining your and my fixes.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

This comment was marked as outdated.

Sign in to view

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

nixpkgs-ci
Bot

added

10.rebuild-nixos-tests

This PR causes rebuilds for all NixOS tests and should normally target the staging branches.

8.has: module (update)

This PR changes an existing module in `nixos/`

labels

May 20, 2026

r-vdp

commented

May 20, 2026

Copy link

Copy Markdown

Contributor

Author

The PR's base branch is set to master, but this PR rebuilds all NixOS tests.

Please change the base branch to the right base branch for your changes (probably staging-nixos).

Will rebase tomorrow.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

Ma27

reviewed

May 21, 2026

View reviewed changes

Ma27

left a comment

Copy link

Copy Markdown

Member

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

I agree, looks reasonable to me 👍

No explicit approval: I do understand what's going on and I think it makes sense, however I haven't spent enough though with stc-ng itself, sop I might be missing some issues with this.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

r-vdp

added 2 commits
May 22, 2026 12:42

switch-to-configuration-ng: honour X-* directives in user-unit migrat…

…

76c8d45

…ion pass

The post-activation pass added in 5cc82c4 to handle units migrating
from a per-user manager (home-manager) to NixOS unconditionally restarts
or starts any candidate. dbus-broker.service explicitly opts out of
restarts via reloadIfChanged because restarting the session bus kills
running clients; the second pass ignored that and restarted it anyway.

Apply the same X-ReloadIfChanged / X-RestartIfChanged / RefuseManualStop /
RefuseManualStart / X-OnlyManualStart checks that handle_modified_unit
performs, so a migrated unit is reloaded, skipped, restarted or started
as its directives require.

Covered by new switch-test specialisations for reloadIfChanged and
restartIfChanged = false.

switch-to-configuration-ng: rework user-unit migration candidate sele…

…

6ced06a

…ction

The previous "FragmentPath not under /etc" deny-list swept up units that
were never managed by a per-user manager (e.g. dbus-broker, whose
FragmentPath systemd reports under /run/current-system/sw/share via
systemd.packages) and missed nothing it should have caught, but for the
wrong reason.

Make the intent explicit. A unit is a migration candidate iff it is
active, the new generation defines it in /etc/systemd/user, and either
* its FragmentPath is under $XDG_CONFIG_HOME/systemd/user (the
home-manager case; ~/.config shadows /etc, so we must wait for
sd-switch to remove the copy), or
* its FragmentPath is anywhere else outside /etc and the previous
generation did not have it in /etc (package-shipped units found via
$XDG_DATA_HOME / $XDG_DATA_DIRS, e.g. ~/.nix-profile/share; /etc
outranks these so it wins on daemon-reload).

The "previous generation did not have it" guard keeps units that have
always been in /etc, but whose FragmentPath systemd reports elsewhere, out
of the candidate set, and the existing now_etc check verifies /etc
actually won before acting. Compare FragmentPath by parent directory
instead of string prefix while here.

Covered by a new switch-test case that seeds a unit in
~/.local/share/systemd/user.

r-vdp

force-pushed
the

stc-user-migration-directives

branch
from
230aec8    to
06bc7f2
Compare

May 22, 2026 10:51

r-vdp

changed the base branch from

master

to

staging-nixos

May 22, 2026 10:51

nixpkgs-ci
Bot

closed this

May 22, 2026

nixpkgs-ci
Bot

reopened this

May 22, 2026

nixpkgs-branch-check
Bot

dismissed
their stale review

May 22, 2026 10:56

Review dismissed automatically

ElvishJerricco

reviewed

May 22, 2026

View reviewed changes

ElvishJerricco

left a comment

Copy link

Copy Markdown

Contributor

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

Other than one thing, this LGTM. I think I'm understanding this well enough now.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Comment thread

nixos/modules/system/activation/activation-script.nix

Outdated

description = "Run user-specific NixOS activation";

script = config.system.userActivationScripts.script;

unitConfig.ConditionUser = "!@system";

unitConfig.DefaultDependencies = false;

ElvishJerricco

May 22, 2026

Copy link

Copy Markdown

Contributor

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

I don't understand this. How does this prevent "being restarted twice"?

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Ma27

May 22, 2026

Copy link

Copy Markdown

Member

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

nixos-activation.service is explicitly restarted by stc-ng. Without this, it'd be restarted a second time because stc-ng always restarts default.target.

This fix is analogous to what's already being done for the system-wide nixos-activation.service.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

ElvishJerricco

May 22, 2026

Copy link

Copy Markdown

Contributor

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

Without this, it'd be restarted a second time because stc-ng always restarts default.target.

I understand the problem the commit message describes. What I don't understand is how this fixes it. I don't see how it has any effect on that behavior whatsoever.

This fix is analogous to what's already being done for the system-wide nixos-activation.service.

Uh, there isn't a system-wide nixos-activation.service?

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Ma27

May 22, 2026

Copy link

Copy Markdown

Member

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

I understand the problem the commit message describes. What I don't understand is how this fixes it. I don't see how it has any effect on that behavior whatsoever.

DefaultDependencies removes the dependency of nixos-activation.service on default.target, no?

As a result, when stc-ng restarts default.target (as it restarts all active units) it no longer starts nixos-activation.service.

Reversing this change breaks nixosTests.user-activation-scripts fwiw.

Uh, there isn't a system-wide nixos-activation.service?

Apologies, I confused this one with initrd-nixos-activation upon reading the code 🫠 🤡

Yeah, scratch the part with it being analogous. But in my understanding this still fixes the problem though, no?

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

r-vdp

May 22, 2026

Copy link

Copy Markdown

Contributor

Author

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

Now I'm wondering, does it still get pulled in during boot then? I was afk all day and too tired to look into it now, I'll try to do so tomorrow.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

ElvishJerricco

May 22, 2026

Copy link

Copy Markdown

Contributor

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

DefaultDependencies removes the dependency of nixos-activation.service on default.target, no?

As a result, when stc-ng restarts default.target (as it restarts all active units) it no longer starts nixos-activation.service.

No, there is no such default dependency. The Wants dependency that default.target has on nixos-activation.service is created a couple lines down with wantedBy = [ "default.target" ];, which is obviously not eliminated by DefaultDependencies = false;. And there is no Wants dependency from nixos-activation.service on default.target. There is an ordering dependency though; a target unit  has an After dependency on any unit that it has a Wants dependency on, and that ordering is eliminated by DefaultDependencies = false; on either unit. But that ordering has nothing to do with how stc decides which units to start.

Reversing this change breaks nixosTests.user-activation-scripts fwiw.

Interesting. That is likely for a very different reason, such as maybe an ordering cycle introduced by that default ordering w.r.t. target units that I mentioned.

Now I'm wondering, does it still get pulled in during boot then?

That is what wantedBy = [ "default.target" ]; does, assuming you mean "when the user logs in" when you say "during boot" , since this is the user manager not the system manager.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

r-vdp

May 23, 2026

•

edited

Loading

Uh oh!

There was an error while loading. Please reload this page.

Copy link

Copy Markdown

Contributor

Author

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

I pushed a different approach. The test was succeeding with DefaultDependencies=False only because the activation service was started earlier, and when stc then explicitly restarted it again, the first run got killed. So we had a race condition that in the test led to running the service 1.5 times.

I think the proper fix is to set RemainAfterExit=True so that activating default.target doesn't start it, and only the explicit restart that we already had starts it.

@ElvishJerricco does this look better now?

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Ma27

May 23, 2026

Copy link

Copy Markdown

Member

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

Makes sense to me, thanks 👍

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Ma27

May 23, 2026

Copy link

Copy Markdown

Member

There was a problem hiding this comment.

Choose a reason for hiding this comment

The reason will be displayed to describe this comment to others. Learn more.

Choose a reason

Spam
Abuse
Off Topic
Outdated
Duplicate
Resolved
Low Quality

Hide comment

And apologies, I'm very sorry for the confusion on my end here!

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

r-vdp

and others
added 2 commits
May 23, 2026 12:50

nixos/activation: run user nixos-activation.service exactly once per …

…

663a59e

…switch

stc-ng starts every active target (including default.target) and then
explicitly restarts nixos-activation.service. As a Type=oneshot without
RemainAfterExit the unit is inactive after login, so the default.target
start job re-runs it via Wants=, and the explicit restart runs it again
(or, depending on ordering, SIGTERMs the currently running script and re-runs it).

Set RemainAfterExit=yes so target starts are a no-op for an already-run
activation, and restartIfChanged=false so the unit-diff pass leaves it
alone when the script changes. The explicit restart in stc-ng remains
the single trigger per switch. Print that restart so it is visible in
the switch output, and drop it from the "NOT restarting" list.

Extend the user-activation-scripts test to assert the activation is
only run once, and never killed.

nixos/user-activation-scripts: refactor assert

…

b1a881e

By using unittest's assertEqual you actually see how often the
activation script was being run, i.e. the expected value.

(cherry picked from commit 864a84d)

r-vdp

force-pushed
the

stc-user-migration-directives

branch
from
06bc7f2    to
b1a881e
Compare

May 23, 2026 10:54

r-vdp

added
the
backport release-26.05

Backport PR automatically
label

May 26, 2026

ElvishJerricco

approved these changes

May 26, 2026

View reviewed changes

r-vdp

added this pull request to the merge queue
May 26, 2026

Hide details
View details

Merged
via the queue into

NixOS:staging-nixos

with commit 907350b
May 26, 2026

34 of 36 checks passed

Uh oh!

There was an error while loading. Please reload this page.

r-vdp

deleted the

stc-user-migration-directives

branch

May 26, 2026 20:27

nixpkgs-ci
Bot

mentioned this pull request

May 26, 2026

[Backport release-26.05] switch-to-configuration-ng: harden user-unit migration second pass
#524575

Closed

nixpkgs-ci
Bot

commented

May 26, 2026

Copy link

Copy Markdown

Contributor

Successfully created backport PR for release-26.05:

[Backport release-26.05] switch-to-configuration-ng: harden user-unit migration second pass #524575

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

github-actions
Bot

added
the
8.has: port to stable

This PR already has a backport to the stable release.
label

May 26, 2026

r-vdp

added

backport staging-26.05

Backport PR automatically

and removed

backport release-26.05

Backport PR automatically

labels

May 26, 2026

nixpkgs-ci
Bot

mentioned this pull request

May 26, 2026

[Backport staging-26.05] switch-to-configuration-ng: harden user-unit migration second pass
#524583

Merged

nixpkgs-ci
Bot

commented

May 26, 2026

Copy link

Copy Markdown

Contributor

Successfully created backport PR for staging-26.05:

[Backport staging-26.05] switch-to-configuration-ng: harden user-unit migration second pass #524583

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

zowoq

added

backport staging-nixos-26.05

Backport PR automatically

and removed

backport staging-26.05

Backport PR automatically

labels

May 29, 2026

nixpkgs-ci
Bot

commented

May 29, 2026

Copy link

Copy Markdown

Contributor

Successfully created backport PR for staging-nixos-26.05:

[Backport staging-nixos-26.05] switch-to-configuration-ng: harden user-unit migration second pass #525745

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

nixpkgs-ci
Bot

mentioned this pull request

May 29, 2026

[Backport staging-nixos-26.05] switch-to-configuration-ng: harden user-unit migration second pass
#525745

Merged

This file contains hidden or bidirectional Unicode text that may be interpreted or compiled differently than what appears below. To review, open the file in an editor that reveals hidden Unicode characters.
Learn more about bidirectional Unicode characters

Show hidden characters

Sign up for free
to join this conversation on GitHub.
Already have an account?
Sign in to comment

Reviewers

Ma27

Ma27 left review comments

ElvishJerricco

ElvishJerricco approved these changes

nixpkgs-branch-check[bot]

nixpkgs-branch-check[bot] left review comments

phaer

Awaiting requested review from phaer

jmbaur

Awaiting requested review from jmbaur

Assignees

No one assigned

Labels

6.topic: nixos

Issues or PRs affecting NixOS modules, or package usability issues specific to NixOS

8.has: module (update)

This PR changes an existing module in `nixos/`

8.has: port to stable

This PR already has a backport to the stable release.

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

10.rebuild-linux: 1-10

This PR causes between 1 and 10 packages to rebuild on Linux.

10.rebuild-nixos-tests

This PR causes rebuilds for all NixOS tests and should normally target the staging branches.

backport staging-nixos-26.05

Backport PR automatically

Projects

None yet

Milestone

No milestone

Development

Successfully merging this pull request may close these issues.

Uh oh!

There was an error while loading. Please reload this page.

4 participants

Add this suggestion to a batch that can be applied as a single commit.This suggestion is invalid because no changes were made to the code.Suggestions cannot be applied while the pull request is closed.Suggestions cannot be applied while viewing a subset of changes.Only one suggestion per line can be applied in a batch.Add this suggestion to a batch that can be applied as a single commit.Applying suggestions on deleted lines is not supported.You must change the existing code in this line in order to create a valid suggestion.Outdated suggestions cannot be applied.This suggestion has been applied or marked resolved.Suggestions cannot be applied from pending reviews.Suggestions cannot be applied on multi-line comments.Suggestions cannot be applied while the pull request is queued to merge.Suggestion cannot be applied right now. Please check back later.

You can’t perform that action at this time.

## Related

- [[comments]]
