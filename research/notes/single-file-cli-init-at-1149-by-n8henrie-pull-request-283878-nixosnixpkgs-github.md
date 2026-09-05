---
title: 'single-file-cli: init at 1.1.49 by n8henrie · Pull Request #283878 · NixOS/nixpkgs
  · GitHub'
id: single-file-cli-init-at-1149-by-n8henrie-pull-request-283878-nixosnixpkgs-github
tags:
- linux-agent-jupiteros-fleet-15537b
- testing
- nixos
- source-code
- birth-message
- community-thread
- gap-04
created: '2026-09-02T17:03:39.622453Z'
updated: '2026-09-02T17:39:28.748535Z'
source: https://github.com/NixOS/nixpkgs/pull/283878
source_domain: github.com
fetched_at: '2026-09-02T17:03:39.573045Z'
fetch_provider: builtin
status: review
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'nixpkgs PR #283878 ''single-file-cli: init at 1.1.49'' (n8henrie, merged
  Feb 2024): a real, merged nixpkgs derivation whose checkPhase runs a localhost TCP
  client/server test inside the Linux build sandbox. The nixpkgs-review build log
  shows ''Running phase: checkPhase / Serving HTTP on 127.0.0.1 port 8000 ... "GET
  / HTTP/1.1" 200'' — a python -m http.server bound to 127.0.0.1:8000 serving requests
  from the test client, both inside the sandbox. (The later failure in that log is
  selenium/browser-context flakiness in the test itself, not networking.) Direct precedent
  for the testing-gate ladder''s step 5: the same pattern (spawn a server on 127.0.0.1
  inside checkPhase, connect from the test) is what a mosquitto-subprocess harness
  inside craneLib.cargoTest needs.'
---

single-file-cli: init at 1.1.49 by n8henrie · Pull Request #283878 · NixOS/nixpkgs · GitHub

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

single-file-cli: init at 1.1.49#283878

kirillrdy merged 2 commits into
NixOS:masterNixOS/nixpkgs:masterfrom
n8henrie:init_single_file_clin8henrie/nixpkgs:init_single_file_cliCopy head branch name to clipboard

Conversation

n8henrie

commented

Jan 25, 2024

•

edited

Loading

Uh oh!

There was an error while loading. Please reload this page.

Copy link

Copy Markdown

Contributor

This is a CLI tool for saving a faithful copy of a complete web page in a single HTML file

Fixes #270124

Description of changes

Things done

Built on platform(s)

x86_64-linux

aarch64-linux

x86_64-darwin

aarch64-darwin

For non-Linux: Is sandboxing enabled in nix.conf? (See Nix manual)

sandbox = relaxed

sandbox = true

Tested, as applicable:

NixOS test(s) (look inside nixos/tests)

and/or package tests

or, for functions and "core" functionality, tests in lib/tests or pkgs/test

made sure NixOS tests are linked to the relevant packages

Tested compilation of all packages that depend on this change using nix-shell -p nixpkgs-review --run "nixpkgs-review rev HEAD". Note: all changes have to be committed, also see nixpkgs-review usage

Tested basic functionality of all binary files (usually in ./result/bin/)

24.05 Release Notes (or backporting 23.05 and 23.11 Release notes)

(Package updates) Added a release notes entry if the change is major or breaking

(Module updates) Added a release notes entry if the change is significant

(Module addition) Added a release notes entry if adding a new NixOS module

Fits CONTRIBUTING.md.

Add a 👍 reaction to pull requests you find important.

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

❤️
1
solson reacted with heart emoji

All reactions

❤️
1 reaction

n8henrie

force-pushed
the

init_single_file_cli

branch
2 times, most recently
from
fa7c3fc    to
016c7a6
Compare

January 28, 2024 22:40

ofborg
Bot

added

8.has: package (new)

This PR adds a new package

11.by: package-maintainer

This PR was created by a maintainer of all the package it changes.

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

10.rebuild-darwin: 1

This PR causes 1 package to rebuild on Darwin.

10.rebuild-linux: 1-10

This PR causes between 1 and 10 packages to rebuild on Linux.

10.rebuild-linux: 1

This PR causes 1 package to rebuild on Linux.

labels

Jan 28, 2024

raspher

commented

Jan 29, 2024

•

edited

Loading

Uh oh!

There was an error while loading. Please reload this page.

Copy link

Copy Markdown

Member

Result of nixpkgs-review pr 283878 run on aarch64-linux 1

1 package built:

single-file-cli

Builded, however i cannot use this cli. Can you provide sample command to execute and test it works? Could you add some tests to this package itself as well?

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Jan 29, 2024

Copy link

Copy Markdown

Contributor

Author

Thanks for feedback!

Builded, however i cannot use this cli.

Any errors? It needs to be passed the path to a browser executable with a flag -- my initial submission used a wrapper to provide one by default, but then would give errors if a user used a different one (due the duplicated argument), so I left it off to allow users to provide their own as needed.

Can you provide sample command to execute and test it works?

Yes -- in the package itself, as a comment? Or here?

Could you add some tests to this package itself as well?

Happy to, though this is a web scraper, so I assume the sandbox would make any tests fail.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

raspher

commented

Jan 29, 2024

•

edited

Loading

Uh oh!

There was an error while loading. Please reload this page.

Copy link

Copy Markdown

Member

Happy to, though this is a web scraper, so I assume the sandbox would make any tests fail.

You're right, it probably will fail, my bad.

Can you provide sample command to execute and test it works?

Yes -- in the package itself, as a comment? Or here?

Just an example command to pass inside shell of  nix-shell -p nixpkgs-review --run "nixpkgs-review rev HEAD", here as comment will be ok

Any errors?

I'm testing on remote aarch64 VM and got this

$ single-file --back-end jsdom "https://github.com" test.html
Class extends value undefined is not a constructor or null URL: https://github.com
Stack: TypeError: Class extends value undefined is not a constructor or null
at eval (eval at getPageData (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/back-ends/jsdom.js:76:6), <anonymous>:1:3173)
at eval (eval at getPageData (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/back-ends/jsdom.js:76:6), <anonymous>:1:243)
at eval (eval at getPageData (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/back-ends/jsdom.js:76:6), <anonymous>:1:314)
at eval (<anonymous>)
at getPageData (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/back-ends/jsdom.js:76:6)
at async exports.getPageData (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/back-ends/jsdom.js:39:10)
at async capturePage (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/single-file-cli-api.js:256:20)
at async runNextTask (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/single-file-cli-api.js:176:20)
at async Promise.all (index 0)
at async capture (/nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/single-file-cli-api.js:127:2)

i've tried

$ single-file --back-end playwright-chromium --browser-headless "https://github.com" test.html
Cannot find module 'playwright'
Require stack:
- /nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/back-ends/playwright-chromium.js
- /nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/single-file-cli-api.js
- /nix/store/h6h2yb89by5daywr2imrzhg90gkkqngz-single-file-cli-1.1.49/lib/node_modules/single-file-cli/single-file

As mentioned, this is environment without desktop/X/wayland, but chromium is installed

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Feb 6, 2024

Copy link

Copy Markdown

Contributor

Author

Sorry for the long response time, thank you again for your time and for testing.

Seems to work for me on both aarch64-linux as well as x86_64-linux.

$ nix-info -m
- system: `"aarch64-linux"`
- host os: `Linux 6.1.21, NixOS, 23.11 (Tapir), 23.11.20240202.c3e9c0b`
- multi-user?: `yes`
- sandbox: `yes`
- version: `nix-env (Nix) 2.19.3`
- nixpkgs: `/nix/store/7d0pfg1fjznlj4v2wfjl6501asac5qmy-source`
$ nix shell \
github:n8henrie/nixpkgs/init_single_file_cli#single-file-cli \
nixpkgs#chromium \
-c single-file --browser-executable-path=chromium-browser https://n8henrie.com
$ echo $?
0
$ head n8henrie.com\ \(2_6_2024\ 3_32_01\ PM\).html
<!DOCTYPE html> <html class="wf-opensans-n6-active wf-opensans-n3-active wf-arvo-n7-active wf-arvo-i4-active wf-arvo-n4-active wf-inconsolata-n4-active wf-opensans-i6-active wf-opensans-i3-active wf-active"><!--
Page saved with SingleFile
url: https://n8henrie.com
saved date: Tue Feb 06 2024 15:32:01 GMT-0700 (Mountain Standard Time)
--><head><meta charset="utf-8">

<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">

<meta name="google-site-verification" content="lOgYEDSRH_etu9NWHYa82k8iJKsCS-zMBXUeGNVyZSA">

$ nix-info -m
- system: `"x86_64-linux"`
- host os: `Linux 6.6.11, NixOS, 23.11 (Tapir), 23.11.20240204.2b41125`
- multi-user?: `yes`
- sandbox: `yes`
- version: `nix-env (Nix) 2.19.3`
- channels(root): `""`
- nixpkgs: `/nix/store/hnr3ckhy6f9qxrwli0mj3vq2rpcaryg7-source`
$ nix shell \
github:n8henrie/nixpkgs/init_single_file_cli#single-file-cli \
nixpkgs#chromium \
-c single-file --browser-executable-path=chromium-browser https://n8henrie.com
$ echo $?
0
$ head n8henrie.com\ \(2_6_2024\ 3_30_29\ PM\).html
<!DOCTYPE html> <html class="wf-arvo-i4-active wf-arvo-n7-active wf-inconsolata-n4-active wf-arvo-n4-active wf-opensans-i3-active wf-opensans-i6-active wf-opensans-n6-active wf-opensans-n3-active wf-active"><!--
Page saved with SingleFile
url: https://n8henrie.com
saved date: Tue Feb 06 2024 15:30:29 GMT-0700 (Mountain Standard Time)
--><head><meta charset="utf-8">

<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">

<meta name="google-site-verification" content="lOgYEDSRH_etu9NWHYa82k8iJKsCS-zMBXUeGNVyZSA">

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Feb 6, 2024

Copy link

Copy Markdown

Contributor

Author

I'm fairly confident I could make a test with localhost-only networking viapython -m http.server --bind 127.0.0.1. On darwin I think this can be done with the __darwinallowlocalnetworking flag, not sure if there is something analogous for linux.

Could also just make it a nixosTest I guess?

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Feb 6, 2024

Copy link

Copy Markdown

Contributor

Author

Also working on aarch64-darwin, though I think geckodriver may be the only browser that builds on darwin:

$ nix-info -m
- system: `"aarch64-darwin"`
- host os: `Darwin 23.2.0, macOS 14.2.1`
- multi-user?: `yes`
- sandbox: `yes`
- version: `nix-env (Nix) 2.19.2`
- channels(n8henrie): `""`
- channels(root): `""`
- nixpkgs: `/nix/store/64npyyxn3bb6cxciz12yigldfc2p9fh1-source`
$ nix shell \
github:n8henrie/nixpkgs/init_single_file_cli#single-file-cli \
nixpkgs#geckodriver \
-c single-file \
--back-end=webdriver-gecko \
--browser-executable-path=geckodriver \
https://n8henrie.com
$ echo $?
0
$ head n8henrie.com\ \(2_6_2024\ 3_48_21\ PM\).html
<!DOCTYPE html> <html class="wf-opensans-n6-active wf-opensans-n3-active wf-opensans-i6-active wf-arvo-i4-active wf-arvo-n4-active wf-arvo-n7-active wf-inconsolata-n4-active wf-opensans-i3-active wf-active"><!--
Page saved with SingleFile
url: https://n8henrie.com
saved date: Tue Feb 06 2024 15:48:21 GMT-0700 (Mountain Standard Time)
--><head><meta charset="utf-8">

<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1">

<meta name="google-site-verification" content="lOgYEDSRH_etu9NWHYa82k8iJKsCS-zMBXUeGNVyZSA">

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

force-pushed
the

init_single_file_cli

branch
from
016c7a6    to
7d94e03
Compare

February 12, 2024 13:04

kirillrdy

commented

Feb 22, 2024

Copy link

Copy Markdown

Member

Result of nixpkgs-review pr 283878 run on x86_64-linux 1

1 package failed to build:

single-file-cli

here are logs

@nix { "action": "setPhase", "phase": "checkPhase" }
Running phase: checkPhase
Serving HTTP on 127.0.0.1 port 8000 (http://127.0.0.1:8000/) ...
127.0.0.1 - - [22/Feb/2024 20:22:54] "GET / HTTP/1.1" 200 -
Browsing context has been discarded URL: http://127.0.0.1:8000
Stack: NoSuchWindowError: Browsing context has been discarded
at Object.throwDecodedError (/build/source/node_modules/selenium-webdriver/lib/error.js:524:15)
at parseHttpResponse (/build/source/node_modules/selenium-webdriver/lib/http.js:601:13)
at Executor.execute (/build/source/node_modules/selenium-webdriver/lib/http.js:529:28)
at process.processTicksAndRejections (node:internal/process/task_queues:95:5)
at async thenableWebDriverProxy.execute (/build/source/node_modules/selenium-webdriver/lib/webdriver.js:745:17)
at async getPageData (/build/source/back-ends/webdriver-gecko.js:115:3)
at async exports.getPageData (/build/source/back-ends/webdriver-gecko.js:41:10)
at async capturePage (/build/source/single-file-cli-api.js:256:20)
at async runNextTask (/build/source/single-file-cli-api.js:176:20)
at async Promise.all (index 0)

nix-info -m
- system: `"x86_64-linux"`
- host os: `Linux 6.6.16, NixOS, 24.05 (Uakari), 24.05.20240222.278dcd8`
- multi-user?: `yes`
- sandbox: `yes`
- version: `nix-env (Nix) 2.18.1`
- channels(root): `"nixos"`
- nixpkgs: `/home/kirillvr/.cache/nixpkgs-review/pr-283878-1/nixpkgs`

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Feb 22, 2024

Copy link

Copy Markdown

Contributor

Author

@kirillrdy I saw a similar error a few times, seems to be a problem with the test. Wonder if PYTHONUNBUFFERED would help, will try.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

nixos-discourse

commented

Feb 23, 2024

Copy link

Copy Markdown

This pull request has been mentioned on NixOS Discourse. There might be relevant details there:

https://discourse.nixos.org/t/local-networking-in-checkphase/40208/1

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Feb 23, 2024

Copy link

Copy Markdown

Contributor

Author

Result of nixpkgs-review pr 283878 run on x86_64-linux 1

1 package built:

single-file-cli

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

kirillrdy

reviewed

Feb 23, 2024

View reviewed changes

Comment thread

pkgs/by-name/si/single-file-cli/package.nix

Outdated

Show resolved

Hide resolved

Uh oh!

There was an error while loading. Please reload this page.

kirillrdy

commented

Mar 7, 2024

Copy link

Copy Markdown

Member

@n8henrie can you update commit messages to match contribution guidelines

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

force-pushed
the

init_single_file_cli

branch
from
fb6a660    to
f0ecf0a
Compare

March 7, 2024 14:15

n8henrie

commented

Mar 7, 2024

Copy link

Copy Markdown

Contributor

Author

@kirillrdy hopefully that squash takes care of it!

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

kirillrdy

commented

Mar 11, 2024

Copy link

Copy Markdown

Member

@n8henrie sorry, can you make two commits

maintainers: add n8henrie

single-file-cli: init at 1.1.49

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Mar 11, 2024

Copy link

Copy Markdown

Contributor

Author

No problem, will do shortly

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

kirillrdy

commented

Mar 11, 2024

Copy link

Copy Markdown

Member

FYI 2.0 has been released, but we can deal with that in subsequent PR

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

force-pushed
the

init_single_file_cli

branch
from
f0ecf0a    to
80d391b
Compare

March 12, 2024 01:30

maintainers: add n8henrie

8831261

n8henrie

force-pushed
the

init_single_file_cli

branch
from
80d391b    to
ed0bcfd
Compare

March 12, 2024 01:36

n8henrie

commented

Mar 22, 2024

Copy link

Copy Markdown

Contributor

Author

@kirillrdy this is my first "new package" PR -- what is the next step? Do I need to solicit a review?

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

kirillrdy

reviewed

Mar 22, 2024

View reviewed changes

Comment thread

pkgs/by-name/si/single-file-cli/package.nix

Outdated

kirillrdy

Mar 22, 2024

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

error: attribute 'agpl3' missing

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

All reactions

kirillrdy

commented

Mar 22, 2024

Copy link

Copy Markdown

Member

@kirillrdy this is my first "new package" PR -- what is the next step? Do I need to solicit a review?

due to large volume of PR's things do get lost a bit,

as for getting more people to review there is

there is a discourse https://discourse.nixos.org/t/prs-ready-for-review/3032/3635

and matrix https://app.element.io/#/room/#review-requests:nixos.org

as for this PR, i think I am happy to merge once CI is passing, which it currently isn't

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Mar 22, 2024

Copy link

Copy Markdown

Contributor

Author

Interesting, looks like agpl was just removed: 82b45bf

Looks like this should be agpl3Only: https://github.com/gildas-lormeau/single-file-cli/blob/master/LICENSE

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

force-pushed
the

init_single_file_cli

branch
from
ed0bcfd    to
5c53eaa
Compare

March 22, 2024 20:57

kirillrdy

commented

Mar 23, 2024

Copy link

Copy Markdown

Member

Result of nixpkgs-review pr 283878 run on x86_64-linux 1

1 package failed to build:

single-file-cli

ofborg build fails with same error https://github.com/NixOS/nixpkgs/pull/283878/checks?check_run_id=22999755857

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

commented

Mar 23, 2024

Copy link

Copy Markdown

Contributor

Author

Huh. I've seen a similar error a few times as well, I'm guessing it's some kind of race condition in setting up the example Python web server, but that's the best idea I had for a simple localhost-only web server to make a request to.

Just adding a sleep 1 seems pretty cludgy, but is seen elsewhere in nixpkgs so maybe I'll try that.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

kirillrdy

commented

Mar 24, 2024

Copy link

Copy Markdown

Member

Huh. I've seen a similar error a few times as well, I'm guessing it's some kind of race condition in setting up the example Python web server, but that's the best idea I had for a simple localhost-only web server to make a request to.

Just adding a sleep 1 seems pretty cludgy, but is seen elsewhere in nixpkgs so maybe I'll try that.

instead of adding a single sleep, you can do polling using nc

otherwise, i would just remove checkPhase, and rely on something like versionTest

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

force-pushed
the

init_single_file_cli

branch
from
5c53eaa    to
526884b
Compare

March 25, 2024 09:26

n8henrie

commented

Mar 25, 2024

Copy link

Copy Markdown

Contributor

Author

@ofborg build single-file-cli

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

n8henrie

force-pushed
the

init_single_file_cli

branch
from
526884b    to
d719e13
Compare

March 26, 2024 21:05

ofborg
Bot

added

10.rebuild-darwin: 0

This PR does not cause any packages to rebuild on Darwin.

and removed

10.rebuild-darwin: 1

This PR causes 1 package to rebuild on Darwin.

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

labels

Mar 26, 2024

single-file-cli: init at 1.1.49

5d94d22

n8henrie

force-pushed
the

init_single_file_cli

branch
from
d719e13    to
5d94d22
Compare

March 26, 2024 22:38

ofborg
Bot

added

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

10.rebuild-darwin: 1

This PR causes 1 package to rebuild on Darwin.

and removed

10.rebuild-darwin: 0

This PR does not cause any packages to rebuild on Darwin.

labels

Mar 26, 2024

n8henrie

commented

Mar 28, 2024

Copy link

Copy Markdown

Contributor

Author

@kirillrdy that error seemed to be specific to using geckodriver (which I used initially because I don't think any other browsers build on darwin), but I didn't find an easy workaround.

Changed to using chromium for the tests and restricting tests to linux -- seems to be working now.

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

kirillrdy

merged commit a6b7f02
into

NixOS:master

Mar 30, 2024

kirillrdy

commented

Mar 30, 2024

Copy link

Copy Markdown

Member

@n8henrie thank you !, feel free to tag me for 2.0

All reactions

Sorry, something went wrong.

Uh oh!

There was an error while loading. Please reload this page.

This file contains hidden or bidirectional Unicode text that may be interpreted or compiled differently than what appears below. To review, open the file in an editor that reveals hidden Unicode characters.
Learn more about bidirectional Unicode characters

Show hidden characters

Sign up for free
to join this conversation on GitHub.
Already have an account?
Sign in to comment

Reviewers

kirillrdy

kirillrdy left review comments

Assignees

No one assigned

Labels

8.has: package (new)

This PR adds a new package

10.rebuild-darwin: 1-10

This PR causes between 1 and 10 packages to rebuild on Darwin.

10.rebuild-darwin: 1

This PR causes 1 package to rebuild on Darwin.

10.rebuild-linux: 1-10

This PR causes between 1 and 10 packages to rebuild on Linux.

10.rebuild-linux: 1

This PR causes 1 package to rebuild on Linux.

11.by: package-maintainer

This PR was created by a maintainer of all the package it changes.

Projects

None yet

Milestone

No milestone

Development

Successfully merging this pull request may close these issues.

Package request: single-file-cli

Uh oh!

There was an error while loading. Please reload this page.

4 participants

Add this suggestion to a batch that can be applied as a single commit.This suggestion is invalid because no changes were made to the code.Suggestions cannot be applied while the pull request is closed.Suggestions cannot be applied while viewing a subset of changes.Only one suggestion per line can be applied in a batch.Add this suggestion to a batch that can be applied as a single commit.Applying suggestions on deleted lines is not supported.You must change the existing code in this line in order to create a valid suggestion.Outdated suggestions cannot be applied.This suggestion has been applied or marked resolved.Suggestions cannot be applied from pending reviews.Suggestions cannot be applied on multi-line comments.Suggestions cannot be applied while the pull request is queued to merge.Suggestion cannot be applied right now. Please check back later.

You can’t perform that action at this time.

## Related

- [[comments]]
