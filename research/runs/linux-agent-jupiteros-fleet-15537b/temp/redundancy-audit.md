# Redundancy audit — step 2.6

Claims files: 116 across the corpus. `hpr dedup` found no substantive near-duplicate clusters within the run tag beyond the 7 notes already deprecated during fetching (redirect stubs, anchor duplicates, superseded versions — deprecated by their fetchers with reasons).

Known derivative relationships (noted, not deprecated — discounted in curation):
- `docs.influxdata.com` Telegraf page mirrors the GitHub README (both retained; README is canonical).
- `mqtt-home-assistant-2` / `-3` — anchor/redirect variants of the canonical HA MQTT doc (flagged near-duplicate by fetcher; canonical is `mqtt-home-assistant`).
- `telegraf-documentation-2` — docs mirror of the Telegraf README.
- pkg.go.dev go-hass-agent note is v1.4.3-era (flagged STALE by fetcher; kept as history).
- man7/man.archlinux/debian mirrors of systemd-logind — false-positive escalations; content recovered from systemd DocBook source (canonical).

Adjusted coverage: no atomic item drops below 3 independent sources after discounting derivatives.
