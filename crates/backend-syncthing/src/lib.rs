//! Syncthing per-folder sync status sensors, read from Syncthing's local
//! REST API (<https://docs.syncthing.net/dev/rest.html>).
//!
//! Implementation note: this speaks to the REST API over HTTP with the
//! `X-API-Key` header (rather than shelling out to a CLI, since Syncthing has
//! no equivalent to `syncthingctl` in wide use) using `reqwest`. It was
//! implemented against Syncthing's *documented* REST API shape, not verified
//! against a live daemon (none is available in this sandbox) — every
//! endpoint path and JSON field name below that isn't rock-solid from the
//! docs is flagged inline with `unverified:` so it can be checked against a
//! real instance before this is trusted in production.
//!
//! Because the folder list is itself only discoverable via an API call, and
//! `SensorBackend::sensors()` is synchronous and must return a fixed set,
//! folder discovery happens once up front in the async `new()` constructor
//! (mirroring how this codebase's `GenericBackend::new()` does async D-Bus
//! discovery before `sensors()`/`poll()` ever run) — `sensors()` and
//! `poll()` only ever read the already-resolved folder list stored on the
//! struct.

use async_trait::async_trait;
use ha_agent_core::model::{SensorDescriptor, SensorState};
use ha_agent_core::SensorBackend;
use serde::Deserialize;
use tracing::warn;

/// One Syncthing-configured folder, as discovered from `/rest/config` at
/// startup.
struct Folder {
    /// Syncthing's own folder ID (e.g. "abcde-fghij"), used as the query
    /// param against `/rest/db/status`.
    id: String,
    /// Sanitized (lowercase, non-alphanumeric -> `_`) form of `id`, used to
    /// build this folder's entity IDs.
    slug: String,
    /// Human-readable folder label, used in entity names. Falls back to
    /// `id` if the folder has no label set.
    label: String,
}

/// Syncthing sync-status sensors: overall connection count plus, per
/// configured folder, its sync state and an out-of-sync binary sensor.
pub struct SyncthingBackend {
    client: reqwest::Client,
    address: String,
    api_key: String,
    folders: Vec<Folder>,
}

fn sanitize(id: &str) -> String {
    id.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

impl SyncthingBackend {
    /// Lightweight reachability probe. Unlike other backends' synchronous
    /// `detect()` (env var / binary-on-PATH checks), this one needs network
    /// I/O to know whether a Syncthing daemon is actually listening and
    /// accepting our API key, so it's async — call it from an async context
    /// during backend registration.
    ///
    /// unverified: `/rest/system/ping` is documented to return
    /// `{"ping":"pong"}` on success and is the cheapest documented
    /// healthcheck endpoint; falling back to a plain reachability read is
    /// avoided since malformed/missing API keys should count as "not
    /// available" too (Syncthing returns 403 for those).
    pub async fn detect(address: &str, api_key: &str) -> bool {
        let client = match reqwest::Client::builder().build() {
            Ok(c) => c,
            Err(e) => {
                warn!("building reqwest client for syncthing detect: {e}");
                return false;
            }
        };

        let url = format!("{}/rest/system/ping", address.trim_end_matches('/'));
        match client.get(&url).header("X-API-Key", api_key).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(e) => {
                warn!("probing syncthing at {url}: {e}");
                false
            }
        }
    }

    /// Builds the backend, doing an up-front `/rest/config` fetch to
    /// discover the fixed folder list `sensors()` will report. Fails if
    /// that initial discovery call fails — without it there's no sensor
    /// set to publish.
    pub async fn new(address: String, api_key: String) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder().build()?;
        let folders = fetch_folders(&client, &address, &api_key).await?;
        Ok(Self {
            client,
            address,
            api_key,
            folders,
        })
    }

    fn folder_state_id(folder: &Folder) -> String {
        format!("syncthing_folder_{}_state", folder.slug)
    }

    fn folder_out_of_sync_id(folder: &Folder) -> String {
        format!("syncthing_folder_{}_out_of_sync", folder.slug)
    }
}

/// Config shapes we care about, deserialized from `/rest/config`.
///
/// unverified: the top-level `/rest/config` response is documented to
/// contain a `folders` array of objects with (at least) `id` and `label`
/// fields — this is stable across Syncthing versions per the docs, but the
/// exact field set on each folder object is large and only `id`/`label` are
/// consumed here.
#[derive(Deserialize)]
struct ConfigResponse {
    folders: Vec<ConfigFolder>,
}

#[derive(Deserialize)]
struct ConfigFolder {
    id: String,
    #[serde(default)]
    label: String,
}

/// Response shape of `/rest/db/status?folder=<id>`.
///
/// unverified: `state` (one of "idle"/"scanning"/"syncing"/"error", etc.) is
/// documented. `needTotalItems` (count of items needed to bring the folder
/// in sync) is documented on this endpoint as the aggregate "need" count,
/// but the *exact* field name has shifted across Syncthing API versions in
/// the past (older docs/some versions use nested `need.total` shapes on
/// `/rest/db/need` instead) — verify against a live instance's
/// `/rest/db/status?folder=<id>` response before relying on this field name.
#[derive(Deserialize)]
struct DbStatusResponse {
    #[serde(default)]
    state: String,
    #[serde(default)]
    #[serde(rename = "needTotalItems")]
    need_total_items: u64,
}

/// Response shape of `/rest/system/connections`.
///
/// unverified: documented as `{"connections": {<device-id>: {"connected":
/// bool, ...}, ...}, "total": {...}}`. The per-device key being the
/// device's full ID string (not an index) is per docs; only `connected` is
/// consumed here.
#[derive(Deserialize)]
struct ConnectionsResponse {
    connections: std::collections::HashMap<String, ConnectionEntry>,
}

#[derive(Deserialize)]
struct ConnectionEntry {
    #[serde(default)]
    connected: bool,
}

async fn get_json<T: serde::de::DeserializeOwned>(
    client: &reqwest::Client,
    address: &str,
    api_key: &str,
    path_and_query: &str,
) -> anyhow::Result<T> {
    let url = format!("{}{}", address.trim_end_matches('/'), path_and_query);
    let resp = client
        .get(&url)
        .header("X-API-Key", api_key)
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json::<T>().await?)
}

async fn fetch_folders(
    client: &reqwest::Client,
    address: &str,
    api_key: &str,
) -> anyhow::Result<Vec<Folder>> {
    let cfg: ConfigResponse = get_json(client, address, api_key, "/rest/config").await?;
    Ok(cfg
        .folders
        .into_iter()
        .map(|f| {
            let label = if f.label.is_empty() {
                f.id.clone()
            } else {
                f.label
            };
            Folder {
                slug: sanitize(&f.id),
                id: f.id,
                label,
            }
        })
        .collect())
}

#[async_trait]
impl SensorBackend for SyncthingBackend {
    fn id(&self) -> &str {
        "syncthing"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        let mut out = vec![SensorDescriptor::sensor(
            "syncthing_connections",
            "Syncthing Connected Devices",
        )
        .with_icon("mdi:sync")];

        for folder in &self.folders {
            out.push(
                SensorDescriptor::sensor(
                    Self::folder_state_id(folder),
                    format!("Syncthing {} Folder State", folder.label),
                )
                .with_icon("mdi:folder-sync"),
            );
            out.push(
                SensorDescriptor::binary_sensor(
                    Self::folder_out_of_sync_id(folder),
                    format!("Syncthing {} Out Of Sync", folder.label),
                )
                .with_device_class("problem")
                .with_icon("mdi:folder-alert"),
            );
        }

        out
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut out = Vec::new();

        match get_json::<ConnectionsResponse>(
            &self.client,
            &self.address,
            &self.api_key,
            "/rest/system/connections",
        )
        .await
        {
            Ok(conns) => {
                let connected = conns.connections.values().filter(|c| c.connected).count();
                out.push(SensorState::new("syncthing_connections", connected as u64));
            }
            Err(e) => warn!("fetching syncthing connections: {e}"),
        }

        for folder in &self.folders {
            let path = format!("/rest/db/status?folder={}", folder.id);
            match get_json::<DbStatusResponse>(&self.client, &self.address, &self.api_key, &path)
                .await
            {
                Ok(status) => {
                    out.push(SensorState::new(
                        Self::folder_state_id(folder),
                        status.state.clone(),
                    ));
                    let out_of_sync = status.state != "idle" || status.need_total_items > 0;
                    out.push(SensorState::binary(
                        Self::folder_out_of_sync_id(folder),
                        out_of_sync,
                    ));
                }
                Err(e) => warn!(
                    "fetching syncthing db status for folder {}: {e}",
                    folder.id
                ),
            }
        }

        out
    }
}
