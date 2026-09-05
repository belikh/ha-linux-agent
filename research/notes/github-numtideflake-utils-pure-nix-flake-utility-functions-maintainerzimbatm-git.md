---
title: 'GitHub - numtide/flake-utils: Pure Nix flake utility functions [maintainer=@zimbatm]
  · GitHub'
id: github-numtideflake-utils-pure-nix-flake-utility-functions-maintainerzimbatm-git
tags:
- linux-agent-jupiteros-fleet-15537b
- source-code
- repo-source
- primary-source
- birth-message
- repo-map
created: '2026-09-02T06:41:40.109438Z'
updated: '2026-09-05T10:51:22.061923Z'
source: https://github.com/numtide/flake-utils
source_domain: github.com
fetched_at: '2026-09-02T06:41:37.862369Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'flake-utils (numtide, 1.6k stars, MIT, maintained by @zimbatm): pure-Nix
  flake utility library — no nixpkgs dependency. Key API: eachDefaultSystem (iterate
  over defaultSystems = x86_64-linux, aarch64-linux, x86_64-darwin, aarch64-darwin),
  eachSystem for custom system lists, system/allSystems attrset for typo-proof system
  names, mkApp, flattenTree, simpleFlake, and meld (merge subflakes sharing inputs
  for splitting large flakes). The systems list is overridable by following a nix-systems/*
  input (e.g. inputs.systems.url = github:nix-systems/x86_64-linux) so fleet consumers
  can narrow to their architectures. The README shows the exact eachDefaultSystemPassThrough
  pattern for exposing nixosConfigurations/homeConfigurations outside the per-system
  attrset — directly relevant to how ha-linux-agent''s flake should expose a NixOS
  module for every jupiterOS host.'
---

GitHub - numtide/flake-utils: Pure Nix flake utility functions [maintainer=@zimbatm] · GitHub

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

numtide

/

flake-utils

Public

Notifications
You must be signed in to change notification settings

Fork
83

Star
1.6k

main

BranchesTags

Go to fileCode
Open more actions menu

Latest commit

History102 Commits

102 Commits
Folders and filesNameName
Last commit message
Last commit date

.github

.github

examples

examples

.gitignore

.gitignore

LICENSE

LICENSE

README.md

README.md

allSystems.nix

allSystems.nix

check-utils.nix

check-utils.nix

default.nix

default.nix

filterPackages.nix

filterPackages.nix

flake-utils.svg

flake-utils.svg

flake.lock

flake.lock

flake.nix

flake.nix

flattenTree.nix

flattenTree.nix

lib.nix

lib.nix

simpleFlake.nix

simpleFlake.nix

View all files

Repository files navigation

flake-utils

Pure Nix flake utility functions.

A numtide project.

The goal of this project is to build a collection of pure Nix functions that don't
depend on nixpkgs, and that are useful in the context of writing other Nix
flakes.

Usage

system :: { system = system, ... }

A map from system to system built from allSystems:

system = {
x86_64-linux = "x86_64-linux";
x86_64-darwin = "x86_64-darwin";
...
}

It's mainly useful to
detect typos and auto-complete if you use rnix-lsp.

Eg: instead of typing "x86_64-linux", use system.x86_64-linux.

allSystems :: [<system>]

A list of all systems defined in nixpkgs. For a smaller list see defaultSystems.

defaultSystems :: [<system>]

The list of systems to use in eachDefaultSystem and simpleFlake.

The default values are ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"].

It's possible to override and control that list by changing the systems input of this flake.

Eg (in your flake.nix):

{
# 1. Defined a "systems" inputs that maps to only ["x86_64-linux"]
inputs.systems.url = "github:nix-systems/x86_64-linux";

inputs.flake-utils.url = "github:numtide/flake-utils";
# 2. Override the flake-utils default to your version
inputs.flake-utils.inputs.systems.follows = "systems";

outputs = { self, flake-utils, ... }:
# Now eachDefaultSystem is only using ["x86_64-linux"], but this list can also
# further be changed by users of your flake.
flake-utils.lib.eachDefaultSystem (system: {
# ...
});
}

For more details in this pattern, see: https://github.com/nix-systems/nix-systems.

eachSystem :: [<system>] -> (<system> -> attrs)

A common case is to build the same structure for each system. Instead of
building the hierarchy manually or per prefix, iterate over each systems and
then re-build the hierarchy.

Eg:

eachSystem [ system.x86_64-linux ] (system: { hello = 42; })
# => { hello = { x86_64-linux = 42; }; }
eachSystem allSystems (system: { hello = 42; })
# => {
hello.aarch64-darwin = 42,
hello.aarch64-genode = 42,
hello.aarch64-linux = 42,
...
hello.x86_64-redox = 42,
hello.x86_64-solaris = 42,
hello.x86_64-windows = 42
}

eachSystemPassThrough :: [<system>] -> (<system> -> attrs)

Unlike eachSystem, this function does not inject the ${system} key by merely
providing the system argument to the function.

eachDefaultSystem :: (<system> -> attrs)

eachSystem pre-populated with defaultSystems.

Example

$ examples/each-system/flake.nix as nix

{
description = "Flake utils demo";

inputs.flake-utils.url = "github:numtide/flake-utils";

outputs = { self, nixpkgs, flake-utils }:
flake-utils.lib.eachDefaultSystem (system:
let pkgs = nixpkgs.legacyPackages.${system}; in
{
packages = rec {
hello = pkgs.hello;
default = hello;
};
apps = rec {
hello = flake-utils.lib.mkApp { drv = self.packages.${system}.hello; };
default = hello;
};
}
);
}

eachDefaultSystemPassThrough :: (<system> -> attrs)

eachSystemPassThrough pre-populated with defaultSystems.

Example

inputs.flake-utils.lib.eachDefaultSystem (system: {
checks./*<SYSTEM>.*/"<CHECK>" = /* ... */;
devShells./*<SYSTEM>.*/"<DEV_SHELL>" = /* ... */;
packages./*<SYSTEM>.*/"<PACKAGE>" = /* ... */;
})
// inputs.flake-utils.lib.eachDefaultSystemPassThrough (system: {
homeConfigurations."<HOME_CONFIGURATION>" = /* ... */;
nixosConfigurations."<NIXOS_CONFIGURATION>" = /* ... */;
})

meld :: attrs -> [ path ] -> attrs

Meld merges subflakes using common inputs.  Useful when you want to
split up a large flake with many different components into more
manageable parts.

mkApp { drv, name ? drv.pname or drv.name, exePath ? drv.passthru.exePath or "/bin/${name}"

A small utility that builds the structure expected by the special apps and defaultApp prefixes.

flattenTree :: attrs -> attrs

Nix flakes insists on having a flat attribute set of derivations in
various places like the packages and checks attributes.

This function traverses a tree of attributes (by respecting
recurseIntoAttrs) and only returns their derivations, with a flattened
key-space.

Eg:

flattenTree { hello = pkgs.hello; gitAndTools = pkgs.gitAndTools }

Returns:

{
hello = «derivation»;
"gitAndTools/git" = «derivation»;
"gitAndTools/hub" = «derivation»;
# ...
}

simpleFlake :: attrs -> attrs

This function should be useful for most common use-cases where you have a
simple flake that builds a package. It takes nixpkgs and a bunch of other
parameters and outputs a value that is compatible as a flake output.

Input:

{
# pass an instance of self
self
, # pass an instance of the nixpkgs flake
nixpkgs
, # we assume that the name maps to the project name, and also that the
# overlay has an attribute with the `name` prefix that contains all of the
# project's packages.
name
, # nixpkgs config
config ? { }
, # pass either a function or a file
overlay ? null
, # use this to load other flakes overlays to supplement nixpkgs
preOverlays ? [ ]
, # maps to the devShell output. Pass in a shell.nix file or function.
shell ? null
, # pass the list of supported systems
systems ? [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ]
}: null

Example

Here is how it looks like in practice:

$ examples/simple-flake/flake.nix as nix

{
description = "Flake utils demo";

inputs.flake-utils.url = "github:numtide/flake-utils";

outputs = { self, nixpkgs, flake-utils }:
flake-utils.lib.simpleFlake {
inherit self nixpkgs;
name = "simple-flake";
overlay = ./overlay.nix;
shell = ./shell.nix;
};
}

Commercial support

Looking for help or customization?

Get in touch with Numtide to get a quote. We make it easy for companies to
work with Open Source projects: https://numtide.com/contact

About
Pure Nix flake utility functions [maintainer=@zimbatm]
Topics
flakenixnix-flake
Resources
Readme
MIT license
Code of conduct
Code of conduct
Contributing
Contributing
Security policy
Security policy
Activity
Custom properties
Stars
1.6k stars
Watchers
21 watching
Forks
83 forks
Report repository

Releases

Packages

Used by

Contributors

Languages

You can’t perform that action at this time.