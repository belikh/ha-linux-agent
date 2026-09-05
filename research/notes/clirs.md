---
title: niri src/cli.rs — niri msg CLI definition (no socket flag)
id: clirs
tags:
- linux-agent-jupiteros-fleet-15537b
- systemd
- niri
- locus-fleet-service-model-by-host-class
- broker-config
- version-ground-truth
- gap-06
- cli
created: '2026-09-02T16:59:14.148652Z'
updated: '2026-09-02T17:37:22.665180Z'
source: https://raw.githubusercontent.com/niri-wm/niri/main/src/cli.rs
source_domain: raw.githubusercontent.com
fetched_at: '2026-09-02T16:59:13.296691Z'
fetch_provider: builtin
status: review
type: note
deprecated: false
summary: 'niri src/cli.rs: the niri msg CLI has NO --socket flag and no config for
  socket path — subcommands (Outputs, Workspaces, Windows, Layers, KeyboardLayouts,
  FocusedOutput, FocusedWindow, PickWindow, PickColor, Action, Output, EventStream,
  Version, RequestError, OverviewState, Casts) all route through the $NIRI_SOCKET
  env var resolved by the niri-ipc library. Consequence for out-of-session access:
  an external service cannot use ''niri msg'' without injecting NIRI_SOCKET into its
  environment; it must either set the env var explicitly (requires knowing the compositor''s
  PID) or speak the JSON protocol directly over the socket file via Socket::connect_to()/socat.'
---

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use niri_ipc::{Action, OutputAction};

use crate::utils::version;

#[derive(Parser)]
#[command(author, version = version(), about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
#[command(subcommand_value_name = "SUBCOMMAND")]
#[command(subcommand_help_heading = "Subcommands")]
pub struct Cli {
/// Path to config file (default: `$XDG_CONFIG_HOME/niri/config.kdl`).
///
/// This can also be set with the `NIRI_CONFIG` environment variable. If both are set, the
/// command line argument takes precedence.
#[arg(short, long)]
pub config: Option,
/// Import environment globally to systemd and D-Bus, run D-Bus services.
///
/// Set this flag in a systemd service started by your display manager, or when running
/// manually as your main compositor instance. Do not set when running as a nested window, or
/// on a TTY as your non-main compositor instance, to avoid messing up the global environment.
#[arg(long)]
pub session: bool,
/// Command to run upon compositor startup.
#[arg(last = true)]
pub command: Vec,

#[command(subcommand)]
pub subcommand: Option,
}

#[derive(Subcommand)]
pub enum Sub {
/// Communicate with the running niri instance.
Msg {
#[command(subcommand)]
msg: Msg,
/// Format output as JSON.
#[arg(short, long)]
json: bool,
},
/// Validate the config file.
Validate {
/// Path to config file (default: `$XDG_CONFIG_HOME/niri/config.kdl`).
///
/// This can also be set with the `NIRI_CONFIG` environment variable. If both are set, the
/// command line argument takes precedence.
#[arg(short, long)]
config: Option,
},
/// Cause a panic to check if the backtraces are good.
Panic,
/// Generate shell completions.
Completions { shell: CompletionShell },
}

#[derive(Subcommand)]
pub enum Msg {
/// List connected outputs.
Outputs,
/// List workspaces.
Workspaces,
/// List open windows.
Windows,
/// List open layer-shell surfaces.
Layers,
/// Get the configured keyboard layouts.
KeyboardLayouts,
/// Print information about the focused output.
FocusedOutput,
/// Print information about the focused window.
FocusedWindow,
/// Pick a window with the mouse and print information about it.
PickWindow,
/// Pick a color from the screen with the mouse.
PickColor,
/// Perform an action.
Action {
#[command(subcommand)]
action: Action,
},
/// Change output configuration temporarily.
///
/// The configuration is changed temporarily and not saved into the config file. If the output
/// configuration subsequently changes in the config file, these temporary changes will be
/// forgotten.
Output {
/// Output name.
///
/// Run `niri msg outputs` to see the output names.
#[arg()]
output: String,
/// Configuration to apply.
#[command(subcommand)]
action: OutputAction,
},
/// Start continuously receiving events from the compositor.
EventStream,
/// Print the version of the running niri instance.
Version,
/// Request an error from the running niri instance.
RequestError,
/// Print the overview state.
OverviewState,
/// List screencasts.
Casts,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum CompletionShell {
Bash,
Elvish,
Fish,
PowerShell,
Zsh,
Nushell,
}

impl TryFrom for Shell {
type Error = &'static str;

fn try_from(shell: CompletionShell) -> Result {
match shell {
CompletionShell::Bash => Ok(Shell::Bash),
CompletionShell::Elvish => Ok(Shell::Elvish),
CompletionShell::Fish => Ok(Shell::Fish),
CompletionShell::PowerShell => Ok(Shell::PowerShell),
CompletionShell::Zsh => Ok(Shell::Zsh),
CompletionShell::Nushell => Err("Nushell should be handled separately"),
}
}
}

## Related

- [[d-bus]]
