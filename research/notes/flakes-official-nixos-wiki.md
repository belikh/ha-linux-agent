---
title: Flakes - Official NixOS Wiki
id: flakes-official-nixos-wiki
tags:
- linux-agent-jupiteros-fleet-15537b
- nixos
- flakes
created: '2026-09-02T06:45:00.871241Z'
updated: '2026-09-05T10:51:22.098897Z'
source: https://wiki.nixos.org/wiki/Flakes
source_domain: wiki.nixos.org
fetched_at: '2026-09-02T06:45:00.869164Z'
fetch_provider: builtin
status: evergreen
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'Official NixOS Wiki Flakes page (successor to the bot-walled nixos.wiki):
  flakes are an experimental Nix feature (since 2.4, 2021) providing uniform project
  structure, dependency pinning via flake.lock, and URL-like input syntax with a registry
  (github:NixOS/nixpkgs). Documents the full flake schema — description/inputs/outputs/nixConfig
  top-level attrs and the outputs schema incl. packages.<system>, nixosModules.<name>,
  nixosConfigurations.<hostname> (used by nixos-rebuild switch --flake .#<hostname>),
  checks (run by nix flake check), devShells, overlays, hydraJobs, templates. Multi-arch
  requires explicit per-system outputs (forAllSystems pattern) or libs like flake-utils/flake-parts.
  Key warnings: flake contents are copied world-readable to the Nix store (no unencrypted
  secrets); in git repos only git-added files are copied; pure evaluation forbids
  builtins.getEnv/currentSystem. Cites RFC 49 (2019) as the original flakes specification.'
---

Flakes - Official NixOS Wiki

Jump to content

Official NixOS Wiki

Search

Search

Create account

Log in

Personal tools

Create account
Log in

Flakes

From Official NixOS Wiki

This page contains changes which are not marked for translation.

Other languages:
English

español

français

русский

中文

日本語

⚟︎
This article or section needs cleanup.  Please edit the article, paying special attention to fixing any formatting issues, inconsistencies, grammar, or phrasing. Make sure to consult the Manual of Style for guidance.

Nix flakes are an experimental feature first introduced in the 2.4 Nix release,[1][2] aiming to address a number of areas of improvement for the Nix ecosystem: they provide a uniform structure for Nix projects, allow for pinning specific versions of each dependencies, and sharing these dependencies via lock files, and overall make it more convenient to write reproducible Nix expressions.

A flake is a directory which directly contains a Nix file called flake.nix, that follows a very specific structure. Flakes introduce a URL-like syntax[3] for specifying remote resources. To simplify the URL syntax, flakes use a registry of symbolic identifiers,[4] allowing the direct specification of resources through syntax such as github:NixOS/nixpkgs.

Flakes also allow for locking references and versions, which can then be queried and updated programmatically via the inputs [5][6]. Additionally, an experimental CLI utility accepts flake references for expressions that build, run, and deploy packages.[7]

Flake file structure

Minimally, a flake file contains a description of the flake, a set of input dependencies and an output. You can generate a very basic flake file after #Setup using nix flake init. This will populate the current directory with a file called flake.nix that will contain something akin to:

❄︎ flake.nix
{
description = "A very basic flake";

inputs = {
nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
};

outputs = { self, nixpkgs }: {
packages.x86_64-linux = {
default = self.packages.x86_64-linux.hello;
hello = nixpkgs.legacyPackages.x86_64-linux.hello;
};
};
}

In the example above, you can see the description, the input specified as a GitHub repository with a specific branch (here nixos/nixpkgs on the nixos-unstable branch), and an output that makes use of the input. The output simply specifies that the flake contains one package for the x86_64 architecture called hello. Even if your flake's output wouldn't use its input (however, in practice, that is highly unlikely), the output still needs to be a Nix function.

Note: Flakes require you to specify its outputs for each architecture separately. For more information, read the related section below.

Nix configuration

It is possible to override the global Nix configuration set in your nix.conf file for the purposes of evaluating a flake. This can be useful, for example, for setting up binary caches specific to certain projects, while keeping the global configuration untouched. The flake file can contain a nixConfig attribute with any relevant configuration settings supplied. For example, enabling the nix-community binary cache would be achieved by:

❄︎ flake.nix
{
...
nixConfig = {
extra-substituters = [
"https://nix-community.cachix.org"
];
extra-trusted-public-keys = [
"nix-community.cachix.org-1:...="
];
}
}

Note: If you are used to configuring your Nix settings via the NixOS configuration, these options are under nix.settings and not nix. For example, you cannot specify the automatic storage optimisation under nix.optimisation.enable.

Setup

Enabling flakes temporarily

When using any nix command, add the following command-line options:

--experimental-features 'nix-command flakes'

Enabling flakes permanently

NixOS

Add the following to the NixOS configuration:

nix.settings.experimental-features = [ "nix-command" "flakes" ];

Home Manager

Add the following to your home manager config:

nix.settings.experimental-features = [ "nix-command" "flakes" ];

Nix standalone

Note:  The  Determinate Nix Installer enables flakes by default, but installs the proprietary Determinate Nix.

Add the following to ~/.config/nix/nix.conf or /etc/nix/nix.conf:

experimental-features = nix-command flakes

Usage

⚠︎
Warning:  Since contents of flake files are copied to the world-readable Nix store folder, do not put any unencrypted secrets in flake files. You should instead use a secret managing scheme.

Note:  For flakes in git repositories, only files in the working tree will be copied to the store.
Therefore, if you use git for your flake, ensure to git add any project files after you first create them.

The nix flakes command
Main article: Nix (command)

The nix flake subcommand is described in
command reference page of the Nix manual.

This flake produces a single flake output packages. And within that, x86_64-linux is a system-specific attribute set. And within that, two package derivations default and hello. You can find outputs with the
show command of a flake as shown below:

$ nix flake show
path:/path/to/flake
└───packages
└───x86_64-linux
├───default: package 'hello-2.12.3'
└───hello: package 'hello-2.12.3'

Development shells

A devShell is a Nix-provided development environment defined within a flake. It lets you declare a reproducible shell environment with the tools, libraries, and environment variables you need for the development of a specific project. This is flake equivalent to defining a nix-shell.

{
description = "Example flake with a devShell";

inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

outputs = { self, nixpkgs }:
let
system = "x86_64-linux";
pkgs = import nixpkgs { inherit system; };
in {
devShells.x86_64-linux.default = pkgs.mkShell {
buildInputs = with pkgs; [
hello
];
shellHook = ''
echo "Welcome to the devShell!"
'';
};
};
}

To enter the development shell environment:

$ nix develop

Note: You don’t need to define a devShell to enter a development shell using nix develop.
If no devShell is defined, nix develop will drop you into an environment containing the default build dependencies of the flake (if any).

Build specific attributes in a flake repository

Running nix build will look in the legacyPackages and packages output attributes for the corresponding derivation and then your system architecture and build the default output. If you want to specify a build attribute in a flake repository, you can run nix build .#<attr>. In the example above, if you wanted to build the packages.x86_64-linux.hello attribute, run:

$ nix build .#hello

Likewise, you can specify an attribute with the run command: nix run .#hello and the develop command: nix develop .#hello.

Flake schema

The flake.nix file is a Nix file but that has special restrictions (more on that later).

It has 4 top-level attributes:

description is a string describing the flake.

inputs is an attribute set of all the dependencies of the flake. The schema is described below.

outputs is a function of one argument that takes an attribute set of all the realized inputs, and outputs another attribute set whose schema is described below.

nixConfig is an attribute set of values which reflect the values given to nix.conf. This can extend the normal behavior of a user's nix experience by adding flake-specific configuration, such as a binary cache.

Input schema

The nix flake inputs manual.

The nix flake references manual.

The inputs attribute defines the dependencies of the flake. For example, nixpkgs has to be defined as a dependency for a system flake in order for the system to build properly.

Nixpkgs can be defined using the following code:

inputs.nixpkgs.url = "github:NixOS/nixpkgs/<branch name>";

Nixpkgs can alternatively also point to an url cached by the NixOS organization:

inputs.nixpkgs.url = "https://nixos.org/channels/nixpkgs-unstable/nixexprs.tar.xz";

In this example the input would point to the `nixpkgs-unstable` channel.

For any repository with its own flake.nix file, the website must also be defined. Nix knows where the nixpkgs repository is, so stating that it's on GitHub is unnecessary.

For example, adding Hyprland as an input would look something like this:

inputs.hyprland.url = "github:hyprwm/Hyprland";

If you want to make Hyprland follow the nixpkgs input to avoid having multiple versions of nixpkgs, this can be done using the following code:

inputs.hyprland.inputs.nixpkgs.follows = "nixpkgs";

Using curly brackets ({}), we can shorten all of this and put it in a table. The code will look something like this:

inputs = {
nixpkgs.url = "github:NixOS/nixpkgs/<branch name>";
hyprland = {
url = "github:hyprwm/Hyprland";
inputs.nixpkgs.follows = "nixpkgs";
};
};

By default, Git submodules in package src's won't get copied to the nix store, this may cause the build to fail. Flakes in Git repositories can declare that they need Git submodules to be enabled. Since Nix version 2.27, you can enable submodules by:

inputs.self.submodules = true;

Output schema

The output schema is described the nix flake check manual page.

Once the inputs are resolved, they're passed to the function `outputs` along with with `self`, which is the directory of this flake in the store. `outputs` returns the outputs of the flake, according to the following schema.

Where:

<system> is something like "x86_64-linux", "aarch64-linux", "i686-linux", "x86_64-darwin"

<name> is an attribute name like "hello".

<flake> is a flake name like "nixpkgs".

<store-path> is a /nix/store.. path

{ self, ... }@inputs:
{
# Executed by `nix flake check`
checks."<system>"."<name>" = derivation;
# Executed by `nix build .#<name>`
packages."<system>"."<name>" = derivation;
# Executed by `nix build .`
packages."<system>".default = derivation;
# Executed by `nix run .#<name>`
apps."<system>"."<name>" = {
type = "app";
program = "<store-path>";
meta = {description = "..."; inherit otherMetaAttrs; };
};
# Executed by `nix run . -- <args?>`
apps."<system>".default = { type = "app"; program = "..."; meta = {description = "..."; inherit otherMetaAttrs; }; };

# Formatter (alejandra, nixfmt, treefmt-nix or nixpkgs-fmt)
formatter."<system>" = derivation;
# Used for nixpkgs packages, also accessible via `nix build .#<name>`
legacyPackages."<system>"."<name>" = derivation;
# Overlay, consumed by other flakes
overlays."<name>" = final: prev: { };
# Default overlay
overlays.default = final: prev: { };
# Nixos module, consumed by other flakes
nixosModules."<name>" = { config, ... }: { options = {}; config = {}; };
# Default module
nixosModules.default = { config, ... }: { options = {}; config = {}; };
# Used with `nixos-rebuild switch --flake .#<hostname>`
# nixosConfigurations."<hostname>".config.system.build.toplevel must be a derivation
nixosConfigurations."<hostname>" = {};
# Used by `nix develop .#<name>`
devShells."<system>"."<name>" = derivation;
# Used by `nix develop`
devShells."<system>".default = derivation;
# Hydra build jobs
hydraJobs."<attr>"."<system>" = derivation;
# Used by `nix flake init -t <flake>#<name>`
templates."<name>" = {
path = "<store-path>";
description = "template description goes here?";
};
# Used by `nix flake init -t <flake>`
templates.default = { path = "<store-path>"; description = ""; };
}

You can also define additional arbitrary attributes, but these are the outputs that Nix knows about.

Core usage patterns

Making your evaluations pure

Nix flakes are evaluated in a pure evaluation mode, meaning that access to the external environment is restricted to ensure reproducibility. To maintain purity when working with flakes, consider the following:

fetchurl and fetchzip require a sha256 argument to be considered pure.

builtins.currentSystem is non-hermetic and impure as it reflects the host system performing the evaluation. This can usually be avoided by passing the system (i.e., x86_64-linux) explicitly to derivations requiring it.

builtins.getEnv is also impure. Avoid reading from environment variables and likewise, do not reference files outside of the flake's directory.

Defining a flake for multiple architectures

Flakes force you to specify a program for each supported architecture. An example below shows how to write a flake that targets multiple architectures.

{
description = "A flake targeting multiple architectures";

inputs = {
nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
};

outputs = { self, nixpkgs }: let
systems = [ "x86_64-linux" "aarch64-linux" ];
forAllSystems = f: builtins.listToAttrs (map (system: {
name = system;
value = f system;
}) systems);
in {
packages = forAllSystems (system: let
pkgs = nixpkgs.legacyPackages.${system};
in {
hello = pkgs.hello;
default = pkgs.hello;
});
};
}

You can also use third-parties projects like flake-utils or flake-parts that automatically provide code to avoid this boilerplate. To avoid re-defining the program multiple times, refer to Flake Utils#Defining a flake for multiple architectures

Using overlays

To use Overlays with flakes, refer to Overlays#In a Nix flake page.

Enable unfree software

To allow for unfree software in a flake project, you need to explicitly allow it by setting config.allowUnfree = true; when importing Nixpkgs.

{
inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
outputs = { self, nixpkgs, flake-compat }:
let
system = "x86_64-linux";
pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
in {
...
};
}

NixOS configuration with flakes

It is possible to manage a NixOS system configuration using flakes, gaining the benefits of reproducible, declarative inputs and streamlined updates.

For details and examples, see NixOS system configuration#Defining NixOS as a flake.

Development tricks

Automatically switch nix shells with direnv

It is possible to automatically activate different Nix shells when navigating between project directories by using Direnv. Additional Nix integration with Direnv can be achieved with nix-direnv.

Pushing Flakes to Cachix

https://docs.cachix.org/pushing#flakes

Flake support in projects without flakes

🟆︎
Tip: Consider building your project in a manner where the flake.nix is merely importing or callPackageing stable Nix code to maximize compatibility for everyone as well as avoiding adding workaround dependencies to your project

The flake-compat library provides a compatibility layer that allows projects using traditional default.nix and shell.nix files to operate with flakes. For more details and usage examples, see the Flake Compat page.

Another project that allows consuming flakes from non-flake projects is flake-inputs.

Accessing flakes from Nix expressions

If you want to access a flake from within a regular Nix expression on a system that has flakes enabled, you can use something like (builtins.getFlake "/path/to/directory").packages.x86_64-linux.default, where 'directory' is the directory that contains your flake.nix.

Efficiently build multiple flake outputs

To push all flake outputs automatically, checkout devour-flake.

Build a package added in a PR

$ nix build github:nixos/nixpkgs?ref=pull/<PR_NUMBER>/head#<PACKAGE>

this allows building a package that has not yet been added to nixpkgs.

note that this will download a full source tarball of nixpkgs.  if you already have a local clone, using that may be faster due to delta compression:

$ git fetch upstream pull/<PR_NUMBER>/head && git checkout FETCH_HEAD && nix build .#PACKAGE

this allows building a package that has not yet been added to nixpkgs.

How to add a file locally in git but not include it in commits

When a git folder exists, flake will only copy files added in git to maximize reproducibility (this way if you forgot to add a local file in your repo, you will directly get an error when you try to compile it). However, for development purpose you may want to create an alternative flake file, for instance containing configuration for your preferred editors as described here… of course without committing this file since it contains only your own preferred tools. You can do so by doing something like that (say for a file called extra/flake.nix):

$ git add --intent-to-add extra/flake.nix
$ git update-index --skip-worktree --assume-unchanged extra/flake.nix

Rapid iteration of a direct dependency

One common pain point with using Nix as a development environment is the need to completely rebuild dependencies and re-enter the dev shell every time they are updated. The nix develop --redirect <flake> <directory> command allows you to provide a mutable dependency to your shell as if it were built by Nix.

Consider a situation where your executable, consumexe, depends on a library, libdep. You're trying to work on both at the same time, where changes to libdep are reflected in real time for consumexe. This workflow can be achieved like so:

$ cd ~/libdep-src-checkout/
$ nix develop # Or `nix-shell` if applicable.
$ export prefix="./install" # configure nix to install it here
$ buildPhase   # build it like nix does
$ installPhase # install it like nix does

Now that you've built the dependency, consumexe can take it as an input. In another terminal:

$ cd ~/consumexe-src-checkout/
$ nix develop --redirect libdep ~/libdep-src-checkout/install
$ echo $buildInputs | tr " " "\n" | grep libdep
$ # Output should show ~/libdep-src-checkout/ so you know it worked

If Nix warns you that your redirected flake isn't actually used as an input to the evaluated flake, try using the --inputs-from . flag. If all worked well you should be able to buildPhase && installPhase when the dependency changes and rebuild your consumer with the new version without exiting the development shell.

See also

Official sources

Flakes - nix.dev

Nix flake command reference manual - Many additional details about flakes, and their parts.

spec describing flake inputs in more detail

RFC 49 (2019) - Original flakes specification

Guides

Flakes aren't real and can't hurt you (Jade Lovelace, 2024)

NixOS & Flakes Book(Ryan4yin, 2023) - 🛠️ ❤️ An unofficial NixOS & Flakes book for beginners.

Nix Flakes: an Introduction (Xe Iaso, 2022)

Practical Nix Flakes (Alexander Bantyev, 2021) - Intro article on working with Nix and Flakes

Nix Flakes, Part 1: An introduction and tutorial (Eelco Dolstra, 2020)

Nix Flakes, Part 2: Evaluation caching (Eelco Dolstra, 2020)

Nix Flakes, Part 3: Managing NixOS systems (Eelco Dolstra, 2020)

Nix flakes 101: Introduction to nix flakes (Jörg Thalheim, 2020) YouTube video

Useful flake modules

flake-utils: Library to avoid some boiler-code when writing flakes

flake-parts: Library to help write modular and organized flakes

flake-compat: A compatibility layer for flakes

building Rust and Haskell flakes

References

↑ Nix Reference Manual, §13.8. Experimental Features, 📖︎ flakes subsection

↑ Nix Reference Manual, §14.27. 📖︎ Release 2.4 (2021-11-01)

↑ Nix Reference Manual, §8.5.17. nix flake, 📖︎ URL-like syntax subsection

↑ Nix Reference Manual, §8.5.62. 📖︎ nix registry

↑ Nix Reference Manual, §7.5.19. 📖︎ nix flake lock

↑ Nix Reference Manual, §7.5.17. 📖︎ nix flake info

↑ Nix Reference Manual, §8.5.1. 📖︎ nix

Retrieved from "https://wiki.nixos.org/w/index.php?title=Flakes&oldid=33616"

Categories:
Articles that need cleanup
Software
Nix
Nix Language
Flakes
Hidden category:
Articles with issues

Search

Search

Flakes

Add languages

Add topic