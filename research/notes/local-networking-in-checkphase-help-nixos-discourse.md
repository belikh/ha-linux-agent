---
title: Local networking in checkPhase? - Help - NixOS Discourse
id: local-networking-in-checkphase-help-nixos-discourse
tags:
- linux-agent-jupiteros-fleet-15537b
- nixos
- rust
- discourse
- nixos-tests
- nixpkgs
- gap-04
created: '2026-09-02T17:03:39.612311Z'
updated: '2026-09-02T17:39:28.169539Z'
source: https://discourse.nixos.org/t/local-networking-in-checkphase/40208
source_domain: discourse.nixos.org
fetched_at: '2026-09-02T17:03:35.632578Z'
fetch_provider: builtin
status: review
type: note
tier: practitioner
content_type: forum
deprecated: false
summary: 'NixOS Discourse (Feb 2024, self-answered Aug 2026) ''Local networking in
  checkPhase?'': the confusion gap-04 asks about, resolved. n8henrie found that a
  server started OUTSIDE the build (python http.server in another terminal) is unreachable
  from checkPhase — curl to 127.0.0.1:8000 fails — because ''there are different loopback
  interfaces in different namespaces: the loopback interface inside the build sandbox
  is not the same one my example python server is listening to''. But the same author''s
  nixpkgs PR #283878, whose checkPhase STARTS the python http.server INSIDE the build,
  works on Linux. Delimits the correct design: cross-boundary (host<->sandbox) loopback
  is blocked, but in-sandbox client+server pairs are fine — exactly the mosquitto-subprocess-harness
  shape (spawn the broker as a test-fixture subprocess inside cargoTest).'
---

Local networking in checkPhase? - Help - NixOS Discourse

Local networking in checkPhase?

Help

n8henrie

February 23, 2024,  6:22pm

1

I’m trying to get some integration tests working that depend on local networking (localhost only).

After discovering __darwinAllowLocalNetworking, I was under the impression that local networking was enabled at build time.

However, running python -m http.server --bind 127.0.0.1 in one pane followed by trying to build this simple flake:
{
outputs = {
self,
nixpkgs,
}: let
system = "x86_64-linux";
pkgs = import nixpkgs {inherit system;};
in {
packages.x86_64-linux.default = pkgs.stdenvNoCC.mkDerivation {
name = "foo";
src = pkgs.writeText "say-foo" "foo";
installPhase = "cp $src $out";
dontUnpack = true;
doCheck = true;
checkPhase = ''
${pkgs.curl}/bin/curl http://127.0.0.1:8000
'';
};
};
}

I don’t see any request to the python server, and the build fails with curl: (7) Failed to connect to 127.0.0.1 port 8000 after 0 ms: Couldn't connect to server

I have a PR open whose checkPhase uses local networking, but the python -m http.server call is in the test, and it seems to work. Is that different somehow?

I’m trying to debug the integration tests for this little project, which is using the rust httpmock library in its integration tests, which uses a localhost server to mock external HTTP calls. These tests pass on Linux but fail on Darwin (even though __darwinAllowLocalNetworking is set by default in the rust builder). Any ideas why that might be?

n8henrie

August 26, 2026,  4:32pm

2

According to ChatGPT this is because there are different loopback interfaces in different namespaces – the loopback interface inside the build sandbox is not the same one my example python server is listening to external to that context. Hopefully that is correct.

1 Like

Powered by Discourse, best viewed with JavaScript enabled

Hosted by Flying Circus.

## Related

- [[src]]
