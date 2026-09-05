---
title: 'nixos/users-groups: add user option to enable lingering by ToxicFrog · Pull
  Request #260248 · NixOS/nixpkgs · GitHub'
id: nixosusers-groups-add-user-option-to-enable-lingering-by-toxicfrog-pull-request
tags:
- linux-agent-jupiteros-fleet-15537b
created: '2026-09-02T05:11:15.437207Z'
updated: '2026-09-02T17:37:22.177250Z'
source: https://github.com/NixOS/nixpkgs/pull/260248
source_domain: github.com
fetched_at: '2026-09-02T05:11:15.435605Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'nixpkgs PR #260248 (merged into master Oct 10 2023 by amaxine, authored
  by ToxicFrog, approved by ambroisie): adds the boolean option users.users.<name>.linger
  to NixOS, fixing issue #3702 (open since 2014). Semantics from the PR body: distinct
  from DontKillUserProcesses=true because (a) it is per-user rather than per-system,
  and (b) lingering users get their slice and services started ON BOOT (or on nixos-rebuild)
  rather than on first login. Mutable coexistence: root can still manage lingering
  with loginctl, but the declarative setting takes precedence whenever nixos-rebuild
  runs; the merge implementation diffs /var/lib/systemd/linger against the configured
  user list and loginctl disable-linger''s anyone not configured (stringAfter [ "users"
  ] activation snippet). Adapted from graham33''s gist. Merged for the 23.11 release
  (23.11 release-notes checklist ticked). Design discussion: proposal to enable linger
  by default for isNormalUser users was rejected with 6 thumbs-down (Ralith''s suggestion;
  aanderse, tilpner, michalrus, armeenm, ambroisie, SuperSandro2000 against). Load-bearing
  for jupiterOS: fleet configs should set users.users.<agent>.linger = true declaratively
  (available since NixOS 23.11) rather than tmpfiles hacks or imperative loginctl.'
---

*Suggested by [[enabling-persistent-user-instance-systemd-issue-3702-nixosnixpkgs-github]] — issue #3702 was closed by PR #260248; the PR body should confirm what linger option landed in NixOS*

nixos/users-groups: add user option to enable lingering by ToxicFrog · Pull Request #260248 · NixOS/nixpkgs · GitHub

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

nixos/users-groups: add user option to enable lingering#260248

amaxine merged 1 commit into
NixOS:masterNixOS/nixpkgs:masterfrom
ToxicFrog:lingerToxicFrog/nixpkgs:lingerCopy head branch name to clipboard

Conversation

ToxicFrog

commented

Oct 10, 2023

Copy link

Copy Markdown

Contributor

Description of changes

Adds a new boolean user option, linger, which can be used to enable systemd lingering for individual users.

This is not the same as DontKillUserProcesses=true; (a) it allows configuration per-user rather than per-system and (b) lingering users will have their slice and services start on boot (or on nixos-rebuild) rather than on first login.

Adapted from

https://gist.github.com/graham33/fdbdcc18317a621d9dd54beb36be6683

Fixes #3702

Lingering users can still be managed mutably by root with loginctl, but the settings here will take precedence when nixos-rebuild is run.

Things done

Built on platform(s)

x86_64-linux

aarch64-linux

x86_64-darwin

aarch64-darwin

For non-Linux: Is sandbox = true set in nix.conf? (See Nix manual)

Tested, as applicable:

NixOS test(s) (look inside nixos/tests)

and/or package tests

or, for functions and "core" functionality, tests in lib/tests or pkgs/test

made sure NixOS tests are linked to the relevant packages

Tested compilation of all packages that depend on this change using nix-shell -p nixpkgs-review --run "nixpkgs-review rev HEAD". Note: all changes have to be committed, also see nixpkgs-review usage

Tested basic functionality of all binary files (usually in ./result/bin/)

23.11 Release Notes (or backporting 23.05 Release notes)

(Package updates) Added a release notes entry if the change is major or breaking

(Module updates) Added a release notes entry if the change is significant

(Module addition) Added a release notes entry if adding a new NixOS module

Fits CONTRIBUTING.md.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

nixos/users-groups: add user option to enable lingering

…

e648d46

Adapted from
https://gist.github.com/graham33/fdbdcc18317a621d9dd54beb36be6683

Fixes NixOS#3702

Lingering users can still be managed mutably by root with `loginctl`,
but the settings here will take precedence when `nixos-rebuild` is run.

github-actions
Bot

added

6.topic: nixos

Issues or PRs affecting NixOS modules, or package usability issues specific to NixOS

8.has: module (update)

This PR changes an existing module in `nixos/`

labels

Oct 10, 2023

ToxicFrog

requested a review
from colemickens

October 10, 2023 15:44

ambroisie

approved these changes

Oct 10, 2023

View reviewed changes

delroth

added
the
12.approvals: 1

This PR was reviewed and approved by one person.
label

Oct 10, 2023

ofborg
Bot

added

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

10.rebuild-darwin: 1

This PR causes 1 package to rebuild on Darwin.

10.rebuild-linux: 1-10

This PR causes between 1 and 10 packages to rebuild on Linux.

labels

Oct 10, 2023

amaxine

commented

Oct 10, 2023

Copy link

Copy Markdown

Contributor

Oh, this is great, thank you!

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

amaxine

merged commit f3d84b9
into

NixOS:master

Oct 10, 2023

Ralith

commented

Oct 10, 2023

Copy link

Copy Markdown

Contributor

Should this be enabled by default when isNormalUser is set?

👎
6
aanderse, tilpner, michalrus, armeenm, ambroisie, and SuperSandro2000 reacted with thumbs down emoji

All reactions

👎
6 reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

SuperSandro2000

reviewed

Oct 14, 2023

View reviewed changes

Comment thread

nixos/modules/config/users-groups.nix

in stringAfter [ "users" ] ''

if [ -e ${lingerDir} ] ; then

cd ${lingerDir}

ls ${lingerDir} | sort | comm -3 -1 ${lingeringUsersFile} - | xargs -r ${pkgs.systemd}/bin/loginctl disable-linger

SuperSandro2000

Oct 14, 2023

•

edited

Loading

Uh oh!

There was an error while loading. Please reload this page.

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

This wouldn't deactivate lingering for a user after it is was enabled and disabled again, right? It would just flip lingering for all users that have it enabled.

Edit: I didn't immediately catch the comm usage here which reveals another problem: Now we deactivate any lingering for any user that has it manually configured and there is no way to deactivate this. This breaks at least one of my systems which I would need to switch to this setting. Since there is no release not for this change, normally it would went unnoticed.

I think we can only enforce this when users cannot be manually changed, so we would need to rely on users.mutableUsers

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

doodead

Nov 16, 2023

Copy link

Copy Markdown

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

Sorry, I'm just a random person that followed the issue where this commit was first introduced, and I may be missing a bunch of things here, but wouldn't relying on mutableUsers in turn break the ability of setting this through configuration.nix for users that aren't mutable?

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

aanderse

Dec 15, 2023

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

does this "break" (i'm using this term very lightly) my system if i remove a user? i'm thinking yes because the update-lingering activation script keeps failing on me - consider

$ sudo loginctl disable-linger someone-who-no-longer-exists                         nix-shell-env
Failed to look up user someone-who-no-longer-exists: No such process

my system works fine if i simply rm the offending file out of /var/lib/systemd/linger

cc @ToxicFrog @SuperSandro2000 @ambroisie

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

aanderse

Feb 26, 2024

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

as mentioned in #283769 (comment) i'm considering reverting this PR - would anyone care to comment before i do that?

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

tomeon

mentioned this pull request

Oct 16, 2023

loginctl user lingering configuration revisited
#261319

Open

12 tasks

Yarny0

mentioned this pull request

Oct 29, 2023

NixOS 23.11 — Feature Freeze & Release Blockers
#259040

Closed

38 tasks

aanderse

mentioned this pull request

Jan 25, 2024

update-lingering activation script is broken
#283769

Closed

This file contains hidden or bidirectional Unicode text that may be interpreted or compiled differently than what appears below. To review, open the file in an editor that reveals hidden Unicode characters.
Learn more about bidirectional Unicode characters

Show hidden characters

Sign up for free
to join this conversation on GitHub.
Already have an account?
Sign in to comment

Reviewers

SuperSandro2000

SuperSandro2000 left review comments

aanderse

aanderse left review comments

ambroisie

ambroisie approved these changes

colemickens

Awaiting requested review from colemickens

+1 more reviewer

doodead

doodead left review comments

Reviewers whose approvals may not affect merge requirements

Assignees

No one assigned

Labels

6.topic: nixos

Issues or PRs affecting NixOS modules, or package usability issues specific to NixOS

8.has: module (update)

This PR changes an existing module in `nixos/`

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

10.rebuild-darwin: 1

This PR causes 1 package to rebuild on Darwin.

10.rebuild-linux: 1-10

This PR causes between 1 and 10 packages to rebuild on Linux.

12.approvals: 1

This PR was reviewed and approved by one person.

Projects

None yet

Milestone

No milestone

Development

Successfully merging this pull request may close these issues.

Enabling persistent user instance systemd

Uh oh!

There was an error while loading. Please reload this page.

8 participants

Add this suggestion to a batch that can be applied as a single commit.This suggestion is invalid because no changes were made to the code.Suggestions cannot be applied while the pull request is closed.Suggestions cannot be applied while viewing a subset of changes.Only one suggestion per line can be applied in a batch.Add this suggestion to a batch that can be applied as a single commit.Applying suggestions on deleted lines is not supported.You must change the existing code in this line in order to create a valid suggestion.Outdated suggestions cannot be applied.This suggestion has been applied or marked resolved.Suggestions cannot be applied from pending reviews.Suggestions cannot be applied on multi-line comments.Suggestions cannot be applied while the pull request is queued to merge.Suggestion cannot be applied right now. Please check back later.

You can’t perform that action at this time.