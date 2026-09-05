---
title: How to check if Tailscale is running – alexwlchan
id: how-to-check-if-tailscale-is-running-alexwlchan
tags:
- linux-agent-jupiteros-fleet-15537b
- source-code
- birth-message
- script-sensors
- windows-only
created: '2026-09-02T05:38:56.249258Z'
updated: '2026-09-05T10:51:21.939825Z'
source: https://alexwlchan.net/notes/2025/check-if-tailscale-is-running/
source_domain: alexwlchan.net
fetched_at: '2026-09-02T05:38:50.531965Z'
fetch_provider: builtin
status: evergreen
type: note
tier: practitioner
content_type: blog
deprecated: false
summary: 'alexwlchan practitioner note (28 Apr 2025, named practitioner / former Tailscale
  engineer): check whether Tailscale is up by running ''tailscale status --json''
  and reading the BackendState key — ''can return seven different states, but in practice
  I only ever see Running and Stopped''. Provides a wrapper script ensure_tailscale_running.sh
  that extracts BackendState via jq, exits 0 on Running, 1 on Stopped, 2 on any unexpected
  state, and is designed to be called from other scripts under set -o errexit so dependent
  scripts fail fast when the tailnet is down. The seven canonical BackendState values
  are documented in the ipnstate package: NoState, NeedsLogin, NeedsMachineAuth, Stopped,
  Starting, Running (plus ipn.State also has ''InLimbo'' variants). Useful pattern
  for an agent''s tailscale-up connectivity gate on jupiterOS hosts.'
---

How to check if Tailscale is running – alexwlchanSkip to main content

How to check if Tailscale is running
Posted 28 April 2025
Use tailscale status and look for the BackendState key.
I have some scripts that talk to nodes in my Tailscale network, but they can only connect if Tailscale is running on my Mac. I wanted a way for my scripts to check if Tailscale is running, and prompt me to start it if not.
You can get the state of Tailscale using the CLI, for example:$ tailscale status --json
{
"Version": "1.82.5-tdec88625e-gec2eb9730",
"TUN": true,
"BackendState": "Running",
…
}
The BackendState key can return seven different states, but in practice I only ever see Running and Stopped.
By inspecting this value, I can check whether Tailscale is running.
A wrapper script
I’ve wrapped this in a script ensure_tailscale_running.sh which I can call from my other scripts, and will exit with a non-zero error code if Tailscale isn’t running.
Because I write all my scripts with set -o errexit, this means the other scripts will fail if I’m not connected to Tailscale.#!/usr/bin/env bash
# Check if Tailscale is running, and prompt you to start it if not.

set -o errexit
set -o nounset

# Print a message in blue to stdout
print_info() {
echo -e "\033[34m$1\033[0m"
}

# Print an error in red to stderr
print_error() {
echo -e "\033[31m$1\033[0m" >&2
}

# Print a warning in yellow to stdout
print_warning() {
echo -e "\033[33m$1\033[0m"
}

print_info "Checking if Tailscale is running…"

# Call the Tailscale CLI to check if Tailscale is running
#
# This usually returns one of two statuses: "Stopped" or "Running"
backend_state=$(
/Applications/Tailscale.app/Contents/MacOS/Tailscale status --json \
| jq -r .BackendState
)

if [[ "$backend_state" = "Running" ]]
then
print_info "Tailscale is running!"
exit 0
elif [[ "$backend_state" = "Stopped" ]]
then
print_error "You need to start Tailscale!"
exit 1
else
print_warning "Unexpected BackendState from Tailscale CLI: $backend_state"
exit 2
fi