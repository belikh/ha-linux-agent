## Coverage Matrix — query phrase → atomic item mapping

| Query phrase (verbatim) | Mapped atomic item(s) | Scope check | Gap? |
|---|---|---|---|
| "use the repo part of @hyperresearch on this repo" | (executed pre-pipeline: `hpr repo map` → note `repo-map-ha-linux-agent`; `hpr repo wiki` → 13-word note, DeepWiki unindexed) + Sub-Q1, entity "ha-linux-agent (this repo)" | OK — both repo tools executed, map is the substantial artifact | No |
| "broaden the research topic to improving it" | Sub-Qs 2-8, required format "ordered improvement roadmap", headings 1-8 | OK — broadened beyond the audit into lifecycle, comparison, fleet, features, testing, roadmap | No |
| "currently it's in a sloppy state - never reliable" | Sub-Q1 (verified defects), Sub-Q2 (MQTT lifecycle), heading 1 + 2, required format "verified-defect audit with file:line evidence and observed build/test/clippy exit codes" | OK — treated as a claim to verify against code, not vibes | No |
| "probably lacking functions and features" | Sub-Q3 (proper Linux agent sensor/command set), Sub-Q6 (jupiterOS-specific gaps), headings 3 + 6 | OK — both the generic agent comparison AND jupiterOS-specific gaps covered | No |
| "a proper 'Linux Agent'" | Sub-Q3 + entities HASS.Agent, Telegraf, Glances, Netdata, node_exporter, HA companion apps; headings 3 + 4 | OK — full comparison set, not narrowed to one | No |
| "or at least a 'jupiterOS agent (nixos)'" | Sub-Q4 (NixOS fleet-native design), Sub-Q6, entity "NixOS module system", heading 5 | OK — NixOS fleet architecture covered | No |
| "I want it to run on EVERY host in jupiterOS, not just the kiosks" | Sub-Q4 + Sub-Q5, entities "jupiterOS fleet", "TCx Wave kiosks", "Headless hosts (europa...)", heading 5, required format "fleet deployment architecture recommendation for all jupiterOS hosts" | OK — every host including headless, with explicit host-class split | No |
| (implied) build-vs-buy | Sub-Q5, heading 4 | OK — explicit dialectic | No |
| (implied) testing / reliability engineering | Sub-Q2, Sub-Q7, heading 7 | OK | No |

Zero `Gap? = YES` rows — decomposition covers every phrase at full scope.
