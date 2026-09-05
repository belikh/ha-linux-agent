# Synthesis conflicts — step 11.2

## Conflict 1: Is the launcher select "architecturally inexpressible" in the incumbents?

- Draft A/C say: go-hass-agent's control vocabulary is button/switch/number with explicitly unsynced state; its changelog has zero select occurrences; the launcher group select cannot be expressed.
- Draft B says: the select gap is thinner than claimed — lnxlink ships a Steam game-launcher dropdown (a select of launchable games) and a bash-command module creating sensors/buttons/switches from shell scripts; a mutual-exclusion session select is one bash module over `systemctl`, "which is where the mutual exclusion already lives anyway"; the launcher backend is "a systemd-and-polkit design wearing a Rust backend as a coat".
- Source check: lnxlink's module catalogue verbatim confirms "Steam game launcher dropdown" and the bash-command module exists. go-hass-agent's changelog verbatim has zero "select"/state-sync occurrences; its README admits unsynced control states. Both sides' facts are accurate.
- **Verdict:** This is a genuine argumentative fork, not a factual error — resolve in the SYNTHESIS, not by discarding either side. The synthesizer must engage B's "the select is a shell script away" honestly: the counter is not that lnxlink cannot emit a select (it can), but (a) lnxlink is not packaged in nixpkgs and its Python/GUI dependency stack resisted DIY packaging (audited: failed Oct 2025 packaging thread, 21+ propagated deps), so "one bash module" carries a heavy deployment cost on this fleet; (b) mutual exclusion with state round-trip and per-profile binary_sensor pairing is typed, tested surface in the current backend vs shell glue; (c) the decision then reduces to the maintenance calculus both drafts already frame. Commit to: improve — but present B's framing as the strongest objection with the packaging counter.

## Conflict 2: Test count (3 vs 8)

- Draft A (and the run's early audit): "3 tests in the entire workspace".
- Draft B and the testing interim: 8 tests (3 discovery + 5 launcher, feature-gated), of which ZERO run in any gate (flake check runs no tests).
- Source check: the testing-gate investigator recounted from the tree: 8 test functions; craneLib.cargoTest is absent from flake.nix so none run.
- **Verdict:** 8 is correct; the synthesizer uses 8-with-zero-running (the deeper point).

## Conflict 3: Who owns the session-switch design?

- Draft B: the launcher's bespoke content lives in systemd units and polkit rules "which adoption leaves untouched" — adopting keeps the fleet's design.
- Draft A/C: the launcher backend IS the jupiterOS-native differentiation; incumbents can't express it natively.
- Source check: both accurate at different layers — the MECHANISM (systemctl) is fleet-owned; the HA-facing ENTITY SURFACE (declarative select + paired sensors + groups from jupiter.services.haAgent.launcherApps) is agent-owned.
- **Verdict:** Synthesizer presents the two-layer decomposition honestly (mechanism vs surface) and argues the typed declarative surface + state round-trip is what justifies the build path — while conceding the mechanism would survive any adoption.

No other substantive factual conflicts found across drafts.
