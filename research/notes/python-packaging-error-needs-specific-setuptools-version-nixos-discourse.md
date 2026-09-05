---
title: Python packaging error, needs specific setuptools version - NixOS Discourse
id: python-packaging-error-needs-specific-setuptools-version-nixos-discourse
tags:
- linux-agent-jupiteros-fleet-15537b
- nixos
- adopt-vs-build
- discourse
- nixpkgs
- locus-adopt-vs-build-honest-verdict
- gap-05
- lnxlink
- packaging
created: '2026-09-02T16:34:35.236936Z'
updated: '2026-09-02T17:37:22.633995Z'
source: https://discourse.nixos.org/t/python-packaging-error-needs-specific-setuptools-version/71434
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T16:34:35.221908Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'Community packaging attempt (Oct 2025, Discourse): a user packaging lnxlink
  2025.10.0 with buildPythonPackage hit setuptools-version errors from pyproject.toml
  — evidence lnxlink has no official nixpkgs presence and its packaging is non-trivial
  for individuals. Their derivation lists the dependency surface: paho-mqtt, psutil,
  pygobject3, jeepney (D-Bus), ewmh, xlib, pulsectl, pyalsaaudio, opencv-python, flask,
  waitress, docker — a heavy GUI/audio/X11-coupled Python stack. Related: GitHub issue
  #334089 (Aug 2024) ''Namespace GLib not available'' fixed by wrapGAppsHook4, mentioning
  an (unmerged) ''Ensure the Python package is installed when lnxlink is enabled''
  — a NixOS option someone was drafting but never upstreamed.'
---

Python packaging error, needs specific setuptools version - NixOS Discourse

Python packaging error, needs specific setuptools version

CronyAkatsuki

October 28, 2025, 11:32am

1

Hello, I’m trying to package a program called lnxlink, but my build is talking about an error where it requires a specific setuptools version set by pyproject.toml.

My current derivation with buildPythonPackage :
{pkgs ? import <nixpkgs> {}}:
with pkgs.python3Packages;
buildPythonPackage rec {
pname = "lnxlink";
version = "2025.10.0";
format = "pyproject";
disabled = pythonOlder "3.8";

src = fetchPypi {
inherit pname version;
hash = "sha256-kiHnMw+wZlezW9kT38Iy+L7ray0drRPceDRpbDlYL1g=";
};

propagatedBuildInputs = [
# System Dependencies
distro
pyyaml
paho-mqtt
requests
psutil
inotify
jeepney
aiohttp
setuptools
wheel

# Module dependencies
pygobject3
speechrecognition
docker
ewmh
flask
mss
numpy
opencv-python
pulsectl
pyalsaaudio
xlib
vdf
waitress
];
}

The error that I get is this:
> ERROR Missing dependencies:
>      setuptools~=68.0.0
>      wheel~=0.40.0

Sigmanificient

October 28, 2025, 11:39am

2

Hi, you need to relax the version requirements for setuptools:
postPatch = ''
substituteInPlace pyproject.toml \
--replace-fail "setuptools~=68.0.0" "setuptools"
'';

You might also want to replace format = "pyproject" woth pyproject = true; and split your propagatedBuildInputs into build-system/dependencies to follow nixpkgs conventions

2 Likes

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.

## Related

- [[lnxlink]]
