# Post-critic fetch log — step 13

**37 findings scanned across 4 critic files.** Most findings point at evidence
already in the vault (verified by targeted search) — they are citation/provenance
fixes for the patcher, not fetch gaps. No web fetches required this wave; one
local ground-truth note was authored instead.

## Gap assessment per finding cluster

| Finding(s) | Topic | Vault evidence? | Disposition |
|---|---|---|---|
| dialectic D1 (critical) | halinuxcompanion's actionable-notification round-trip | YES — `github-muniterhalinuxcompanion-homeassistant-linux-companion-github` (README describes the aiohttp round-trip); the report even cites it two paragraphs earlier | Citation fix only — the patcher flips the false-absence claim to the stronger corrected form (the sole Linux round-trip lives in a project whose own TODO concedes MQTT is the better transport) |
| dialectic D2 | lnxlink bash module can't emit selects | YES — `lnxlink` module catalogue (bash module creates sensors/binary_sensors/buttons/switches only; the Steam dropdown is a separate custom module) | Fix-in-place with existing citation |
| dialectic D3 | adopter's extend-upstream fork dropped | YES — `interim-report-adopt-vs-build-honest-verdict` | Patcher reassembly |
| dialectic D4 | mesh hub on europa not callisto | YES — jupiter-os ground truth + report's own §1 | Internal consistency fix |
| dialectic D5 | lnxlink packaging failure was a setuptools pin | YES — `python-packaging-error-needs-specific-setuptools-version-nixos-discourse` | Reattribution fix |
| dialectic D6 + depth-03 + width-03 | notification-daemon absence + VFD last mile had NO vault citation target | **WAS A GAP** — the facts lived only in comparisons.md (step-8 gap-wave updates), unverifiable against any note | **FILLED — local ground-truth note authored:** `jupiteros-notification-ground-truth` (direct jupiter-os checkout audit: zero-daemon grep + customer-display.nix verbatim header + the ha-agent.nix:112 false-comment finding). Evidence digest appended. No web fetch needed: this is local repo ground truth, not web material. |
| depth-01 | D6 crash leg: Restart=no / WatchdogSec | YES — systemd service docs fetched in step 5 (`see-also-2` ExecStopPost/DocBook, `see-also` logind) + jupiter-os ha-agent.nix (Restart=on-failure is in the flake module, NOT the deployed jupiter-os module — the deployed one has no Restart=) | Fix-in-place |
| depth-02 | D7 decommission design compressed | YES — `the-homie-convention` (state-first removal order), `corehomeassistantcomponentsmqttdiscoverypy-at-dev-home-assistantcore-github` (cleanup paths) | Patcher expansion |
| depth-04..07, 10-12 | smartd hand-off, DynamicUser, Event-enum, BecomeMonitor, tailscale shell-out, launcher-vs-hardware brightness, haluxcompanion exception | YES — respective interim notes + corpus notes already cited | Fix-in-place |
| width-01 | tailscale unprivileged surface (`--operator=io`, `--socket`, `tailscale wait`, tailscale-online.target) | YES — `tailscale-cli-tailscale-docs` (all four mechanisms documented in it, claims extracted in step 6) | Patcher cites existing note |
| width-02 | NUT/UPS cluster unused | YES — `network-ups-tools-nut-home-assistant` (7 claims) | Patcher adds the §6 UPS row |
| width-04 | niri security model | YES — `security-model-niri-wmniri-wiki-github` | Patcher adds the security sentence |
| width-05 | SMART depth quartet | YES — `answers`, `how-to-check-nvme-ssd-health-newbie-corner-arch-linux-forums`, `best-nvme-ssd-health-monitoring-tools-9-`, smartd.conf man page | Patcher selective use |
| width-06 | sops-nix provenance | YES — `github-mic92sops-nix-atomic-secret-provisioning-for-nixos-based-on-sops-github` | Citation insert |
| width-07 | sandbox empirical precedents | YES — `single-file-cli-init-at-1149-by-n8henrie-pull-request-283878-nixosnixpkgs-github` + `local-networking-in-checkphase-help-nixos-discourse` | Patcher upgrade |
| width-08/10, instr findings | blackbox probe, bloat, backend-kde/gamescope/lutris rows | YES — `github-inovexmqtt_blackbox_exporter-prometheus-exporter-for-mqtt-monitoring-gith`, repo-map, jupiter-os configs | Fix-in-place |

## Fetch-worthy gaps found: 1 (filled locally, zero web fetches)

The one true evidence hole — the notification-daemon/VFD ground truth — was
local fleet configuration, not web content, so it was authored as a
ground-truth vault note from a direct audit of `/home/io/projects/jupiter-os`
rather than fetched. Web fetches this wave: **0** (cap 5).

## Unfilled / acknowledged limitations

None — every finding now has either a vault citation target or the
newly-authored ground-truth note.
