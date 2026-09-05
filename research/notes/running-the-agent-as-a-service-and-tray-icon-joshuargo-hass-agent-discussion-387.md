---
title: 'Running the agent as a service and tray icon · joshuar/go-hass-agent · Discussion
  #387 · GitHub'
id: running-the-agent-as-a-service-and-tray-icon-joshuargo-hass-agent-discussion-387
tags:
- linux-agent-jupiteros-fleet-15537b
- locus-adopt-vs-build-honest-verdict
- adopt-vs-build
created: '2026-09-02T12:05:37.293121Z'
updated: '2026-09-02T17:37:22.503044Z'
source: https://github.com/joshuar/go-hass-agent/discussions/387
source_domain: github.com
fetched_at: '2026-09-02T12:05:37.291768Z'
fetch_provider: builtin
status: review
type: note
tier: ground_truth
content_type: code
deprecated: false
summary: 'Answered Q&A discussion (asked by hapklaar Jan 12 2025, answered by maintainer
  joshuar Jan 18 2025): a user running ''systemctl --user enable go-hass-agent'' as
  a service wanted the tray icon too; the maintainer replied that on a desktop you
  should use the desktop environment''s autostart instead of a systemctl service,
  and stated verbatim: ''The systemctl service is primarily for "headless" usage (i.e.,
  servers) not running any desktop environment. If you run the agent both as a a systemctl
  service and in your desktop, you''ll have two instances sending duplicate data,
  which might not be desired''.'
---

Running the agent as a service and tray icon · joshuar/go-hass-agent · Discussion #387 · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

joshuar

/

go-hass-agent

Public

Uh oh!

There was an error while loading. Please reload this page.

Notifications
You must be signed in to change notification settings

Fork
30

Star
575

Running the agent as a service and tray icon

#387

Answered

by
joshuar

hapklaar

asked this question in
Q&A

Running the agent as a service and tray icon

#387

hapklaar

Jan 12, 2025
·
1 comment

Answered

by
joshuar

Return to top

Discussion options

Uh oh!

There was an error while loading. Please reload this page.

{{title}}

Something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

Quote reply

edited

Uh oh!

There was an error while loading. Please reload this page.

{{editor}}'s edit

{{actor}} deleted this content
.

{{editor}}'s edit

Something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

hapklaar

Jan 12, 2025

I installed the agent as a service with systemctl --user enable go-hass-agent && systemctl --user start go-hass-agent to have it auto start on boot. I would also like to have the tray icon present, but this is not active.

To start it I can separately issue a go-hass-agent run on which the icon will show. Is this the way or can I have the tray icon auto start another way?

PS Thanks for creating this, really appreciate it!

1
You must be logged in to vote

All reactions

Answered by

joshuar

Jan 18, 2025

Hey there, thanks for trying out Go Hass Agent!

If you are running Go Hass Agent in a desktop, then you don't need to worry about configuring and starting it as a systemctl service. You can just use the autostart functionality of your desktop environment, for example in KDE or Gnome, or see the documentation for your desktop environment (the Arch Wiki might have some useful information).

The systemctl service is primarily for "headless" usage (i.e., servers) not running any desktop environment. If you run the agent both as a a systemctl service and in your desktop, you'll have two instances sending duplicate data, which might not be desired 😄

View full answer

Replies:

1 comment

Comment options

Uh oh!

There was an error while loading. Please reload this page.

{{title}}

Something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

Quote reply

joshuar

Jan 18, 2025

Maintainer

Hey there, thanks for trying out Go Hass Agent!

If you are running Go Hass Agent in a desktop, then you don't need to worry about configuring and starting it as a systemctl service. You can just use the autostart functionality of your desktop environment, for example in KDE or Gnome, or see the documentation for your desktop environment (the Arch Wiki might have some useful information).

The systemctl service is primarily for "headless" usage (i.e., servers) not running any desktop environment. If you run the agent both as a a systemctl service and in your desktop, you'll have two instances sending duplicate data, which might not be desired 😄

Marked as answer

1
You must be logged in to vote

All reactions

0 replies

Answer selected by
joshuar

Sign up for free
to join this conversation on GitHub.
Already have an account?
Sign in to comment

Category

🙏

Q&A

Labels

None yet

2 participants

Heading

Bold

Italic

Quote

Code

Link

Numbered list

Unordered list

Task list

Attach files

Mention

Reference

Menu

Heading

Bold

Italic

Quote

Code

Link

Numbered list

Unordered list

Task list

Attach files

Mention

Reference

Select a reply

Loading

Uh oh!

There was an error while loading. Please reload this page.

Create a new saved reply

👍
1
reacted with thumbs up emoji

👎
1
reacted with thumbs down emoji

😄
1
reacted with laugh emoji

🎉
1
reacted with hooray emoji

😕
1
reacted with confused emoji

❤️
1
reacted with heart emoji

🚀
1
reacted with rocket emoji

👀
1
reacted with eyes emoji

You can’t perform that action at this time.