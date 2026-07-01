//! ZFS pool health sensors.
//!
//! Publishes, per configured (or auto-discovered) ZFS pool, a capacity
//! percentage sensor and a "problem" binary sensor derived from `zpool
//! list`'s health column. This is a read-only, sensor-only backend for v1 —
//! starting a scrub or otherwise mutating pool state needs root and is out
//! of scope here, so there's no `CommandBackend` impl.
//!
//! Note: this relies on `zpool list -H [-p] -o <fields>` output being stable,
//! tab-separated plain text across OpenZFS versions (verified against
//! upstream docs, not against a live system in this sandbox — there's no
//! `zpool` binary available here to test against). If a future OpenZFS
//! release changes this format, `poll()` will simply fail to parse a line
//! and skip that pool (logged via `tracing::warn!`) rather than panicking.

use async_trait::async_trait;
use ha_agent_core::model::{SensorDescriptor, SensorState};
use ha_agent_core::SensorBackend;
use tokio::process::Command;
use tracing::warn;

pub struct ZfsBackend {
    /// Resolved pool list. If the caller passed an empty `pools` to `new()`,
    /// this is populated by auto-discovery (`zpool list -H -o name`) at
    /// construction time via a blocking call — `sensors()` is a sync trait
    /// method and needs the final list up front to publish discovery
    /// configs, so discovery can't be deferred to the first async `poll()`.
    pools: Vec<String>,
}

impl ZfsBackend {
    /// True if the `zpool` binary is on `$PATH`.
    pub fn detect() -> bool {
        which_zpool()
    }

    /// Construct a backend for the given pools. If `pools` is empty, the
    /// pool list is auto-discovered synchronously (blocking) via `zpool
    /// list -H -o name`.
    pub fn new(pools: Vec<String>) -> Self {
        let pools = if pools.is_empty() {
            discover_pools_blocking()
        } else {
            pools
        };
        Self { pools }
    }
}

impl Default for ZfsBackend {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

fn which_zpool() -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join("zpool").is_file()))
        .unwrap_or(false)
}

/// Sanitize a pool name for use as (part of) an MQTT topic / entity id
/// segment: lowercase, alphanumeric-and-underscore only.
fn sanitize_pool_id(pool: &str) -> String {
    pool.to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn discover_pools_blocking() -> Vec<String> {
    match std::process::Command::new("zpool")
        .args(["list", "-H", "-o", "name"])
        .output()
    {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect(),
        Ok(out) => {
            warn!(
                "zpool list -H -o name failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            Vec::new()
        }
        Err(e) => {
            warn!("running zpool list -H -o name: {e}");
            Vec::new()
        }
    }
}

async fn zpool_lines(args: &[&str]) -> Option<Vec<String>> {
    let mut cmd = Command::new("zpool");
    cmd.args(args);
    match cmd.output().await {
        Ok(out) if out.status.success() => Some(
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|l| l.to_string())
                .collect(),
        ),
        Ok(out) => {
            warn!(
                "zpool {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr)
            );
            None
        }
        Err(e) => {
            warn!("running zpool {}: {e}", args.join(" "));
            None
        }
    }
}

/// `zpool list -H -p -o cap <pool>` — one line, plain integer percent (no
/// `%` suffix thanks to `-p`).
async fn pool_capacity_percent(pool: &str) -> Option<f64> {
    let lines = zpool_lines(&["list", "-H", "-p", "-o", "cap", pool]).await?;
    let line = lines.first()?.trim();
    line.parse::<f64>().ok()
}

/// `zpool list -H -o health <pool>` — one line, e.g. "ONLINE", "DEGRADED".
async fn pool_health(pool: &str) -> Option<String> {
    let lines = zpool_lines(&["list", "-H", "-o", "health", pool]).await?;
    lines.first().map(|l| l.trim().to_string())
}

#[async_trait]
impl SensorBackend for ZfsBackend {
    fn id(&self) -> &str {
        "zfs"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        self.pools
            .iter()
            .flat_map(|pool| {
                let sid = sanitize_pool_id(pool);
                vec![
                    SensorDescriptor::sensor(
                        format!("zfs_{sid}_capacity_percent"),
                        format!("ZFS Pool {pool} Capacity"),
                    )
                    .with_unit("%")
                    .with_icon("mdi:harddisk"),
                    SensorDescriptor::binary_sensor(
                        format!("zfs_{sid}_problem"),
                        format!("ZFS Pool {pool} Problem"),
                    )
                    .with_device_class("problem")
                    .with_icon("mdi:alert"),
                ]
            })
            .collect()
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut out = Vec::new();
        for pool in &self.pools {
            let sid = sanitize_pool_id(pool);

            if let Some(cap) = pool_capacity_percent(pool).await {
                out.push(SensorState::new(format!("zfs_{sid}_capacity_percent"), cap));
            }

            if let Some(health) = pool_health(pool).await {
                let problem = health != "ONLINE";
                out.push(SensorState::binary(format!("zfs_{sid}_problem"), problem));
            }
        }
        out
    }
}
