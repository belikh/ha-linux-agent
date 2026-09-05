---
title: 'Add mqtt notify platform by jbouwh · Pull Request #115653 · home-assistant/core
  · GitHub'
id: add-mqtt-notify-platform-by-jbouwh-pull-request-115653-home-assistantcore-github
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- mqtt-discovery
- source-code
- birth-message
- community-thread
- gap-02
- version-ground-truth
created: '2026-09-02T17:03:39.581573Z'
updated: '2026-09-05T10:51:22.418945Z'
source: https://github.com/home-assistant/core/pull/115653
source_domain: github.com
fetched_at: '2026-09-02T17:03:30.080697Z'
fetch_provider: builtin
status: evergreen
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'HA core PR #115653 ''Add mqtt notify platform'' (jbouwh, opened Apr 15 2024,
  merged for 2024.5): adds the MQTT notify entity platform. Labels: new-feature, new-platform,
  integration: mqtt, has-tests, Quality Scale: gold. Paired with documentation PR
  home-assistant.io#32327 ''Add mqtt notify entity''. Changelog cross-reference: HA
  2024.5 changelog lists both ''Add notify entity component'' (#110950) and ''Add
  mqtt notify platform'' (#115653). Together with const.py''s SUPPORTED_COMPONENTS
  listing, this confirms MQTT notify was born discovery-capable — the platform and
  its discovery wiring shipped in the same release, and upstream tests exercise discovery
  (test_notifypy).'
---

Add mqtt notify platform by jbouwh · Pull Request #115653 · home-assistant/core · GitHub

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

home-assistant

/

core

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
38.5k

Star
90.2k

Merged

Add mqtt notify platform#115653

jbouwh merged 2 commits into
devhome-assistant/core:devfrom
notify-mqtthome-assistant/core:notify-mqttCopy head branch name to clipboard

Conversation

jbouwh

commented

Apr 15, 2024

•

edited

Loading

Uh oh!

There was an error while loading. Please reload this page.

Copy link

Copy Markdown

Contributor

Proposed change

Add mqtt notify platform

Type of change

Dependency upgrade

Bugfix (non-breaking change which fixes an issue)

New integration (thank you!)

New feature (which adds functionality to an existing integration)

Deprecation (breaking change to happen in the future)

Breaking change (fix/feature causing existing functionality to break)

Code quality improvements to existing code or addition of tests

Additional information

This PR fixes or closes issue: fixes #

This PR is related to issue:

Link to documentation pull request: Add mqtt notify entity home-assistant.io#32327

Checklist

The code change is tested and works locally.

Local tests pass. Your PR cannot be merged unless tests pass

There is no commented out code in this PR.

I have followed the development checklist

I have followed the perfect PR recommendations

The code has been formatted using Ruff (ruff format homeassistant tests)

Tests have been added to verify that the new code works.

If user exposed functionality or configuration variables are added/changed:

Documentation added/updated for www.home-assistant.io

If the code communicates with devices, web services, or third-party tools:

The manifest file has all fields filled out correctly.

Updated and included derived files by running: python3 -m script.hassfest.

New or updated dependencies have been added to requirements_all.txt.

Updated by running python3 -m script.gen_requirements_all.

For the updated dependencies - a link to the changelog, or at minimum a diff between library versions is added to the PR description.

Untested files have been added to .coveragerc.

To help with the load of incoming pull requests:

I have reviewed two other open pull requests in this repository.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Add mqtt notify platform

b7e2443

home-assistant
Bot

added

cla-signed

core

has-tests

integration: mqtt

new-feature

new-platform

labels

Apr 15, 2024

home-assistant
Bot

assigned

emontnemery

Apr 15, 2024

home-assistant
Bot

added

by-code-owner

Quality Scale: gold

labels

Apr 15, 2024

home-assistant
Bot

commented

Apr 15, 2024

Copy link

Copy Markdown

Contributor

Hey there @emontnemery, mind taking a look at this pull request as it has been labeled with an integration (mqtt) you are listed as a code owner for? Thanks!

Code owner commands

Code owners of mqtt can trigger bot actions by commenting:

@home-assistant close Closes the pull request.

@home-assistant rename Awesome new title Renames the pull request.

@home-assistant reopen Reopen the pull request.

@home-assistant unassign mqtt Removes the current integration label and assignees on the pull request, add the integration domain after the command.

@home-assistant add-label needs-more-information Add a label (needs-more-information, problem in dependency, problem in custom component) to the pull request.

@home-assistant remove-label needs-more-information Remove a label (needs-more-information, problem in dependency, problem in custom component) on the pull request.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

jbouwh

added

docs-missing

and removed

by-code-owner

Quality Scale: gold

labels

Apr 15, 2024

jbouwh

mentioned this pull request

Apr 15, 2024

Add mqtt notify entity
home-assistant/home-assistant.io#32327

Merged

8 tasks

jbouwh

removed
the
docs-missing

label

Apr 15, 2024

jbouwh

marked this pull request as ready for review

April 15, 2024 16:31

jbouwh

requested a review
from emontnemery
as a code owner

April 15, 2024 16:31

emontnemery

approved these changes

Apr 17, 2024

View reviewed changes

emontnemery

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

LGTM, just one minor comment. OK to merge when that's addressed 👍

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

Comment thread

homeassistant/components/mqtt/notify.py

Outdated

Show resolved

Hide resolved

Uh oh!

There was an error while loading. Please reload this page.

jbouwh

commented

Apr 17, 2024

View reviewed changes

Comment thread

homeassistant/components/mqtt/notify.py

Outdated

Show resolved

Hide resolved

Uh oh!

There was an error while loading. Please reload this page.

Stale docstring

8413da3

jbouwh

commented

Apr 17, 2024

Copy link

Copy Markdown

Contributor

Author

Thnx!

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

jbouwh

merged commit 8275512
into

dev

Apr 17, 2024

jbouwh

deleted the

notify-mqtt

branch

April 17, 2024 18:07

github-actions
Bot

locked and limited conversation to collaborators

Apr 18, 2024

This file contains hidden or bidirectional Unicode text that may be interpreted or compiled differently than what appears below. To review, open the file in an editor that reveals hidden Unicode characters.
Learn more about bidirectional Unicode characters

Show hidden characters

Sign up for free
to subscribe to this conversation on GitHub.
Already have an account?
Sign in.

Reviewers

emontnemery

emontnemery approved these changes

Assignees

emontnemery

Labels

cla-signed

core

has-tests

integration: mqtt

new-feature

new-platform

Projects

None yet

Milestone

No milestone

Development

Successfully merging this pull request may close these issues.

Uh oh!

There was an error while loading. Please reload this page.

2 participants

Add this suggestion to a batch that can be applied as a single commit.This suggestion is invalid because no changes were made to the code.Suggestions cannot be applied while the pull request is closed.Suggestions cannot be applied while viewing a subset of changes.Only one suggestion per line can be applied in a batch.Add this suggestion to a batch that can be applied as a single commit.Applying suggestions on deleted lines is not supported.You must change the existing code in this line in order to create a valid suggestion.Outdated suggestions cannot be applied.This suggestion has been applied or marked resolved.Suggestions cannot be applied from pending reviews.Suggestions cannot be applied on multi-line comments.Suggestions cannot be applied while the pull request is queued to merge.Suggestion cannot be applied right now. Please check back later.

You can’t perform that action at this time.

## Related

- [[comments]]
