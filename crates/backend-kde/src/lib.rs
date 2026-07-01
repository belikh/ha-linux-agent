mod dbus;

use async_trait::async_trait;
use dbus::ActivitiesProxy;
use ha_agent_core::model::{SensorDescriptor, SensorState};
use ha_agent_core::SensorBackend;
use tracing::warn;
use zbus::Connection;

/// KDE Plasma sensors, currently just the active Activity — via
/// kactivitymanagerd's `org.kde.ActivityManager` session-bus API, which has
/// been stable across Plasma 5 and 6 (verified live with `busctl --user
/// introspect org.kde.ActivityManager /ActivityManager/Activities`).
///
/// Deliberately *not* attempting active-window title/app tracking here: KWin
/// has no stable, scripting-free D-Bus method for that (it requires loading
/// a KWin script at runtime), which is a bigger commitment than this v1
/// warrants. A window-tracking follow-up is a natural PR — see the
/// workspace README's "adding a DE backend" section.
pub struct KdeBackend {
    session_bus: Connection,
}

impl KdeBackend {
    /// Available only inside a running Plasma session with kactivitymanagerd
    /// registered on the session bus.
    pub async fn detect() -> bool {
        let is_kde = std::env::var("XDG_CURRENT_DESKTOP")
            .map(|v| v.to_ascii_uppercase().contains("KDE"))
            .unwrap_or(false);
        if !is_kde {
            return false;
        }
        match Connection::session().await {
            Ok(conn) => ActivitiesProxy::new(&conn).await.is_ok(),
            Err(_) => false,
        }
    }

    pub async fn new() -> anyhow::Result<Self> {
        let session_bus = Connection::session().await?;
        Ok(Self { session_bus })
    }
}

#[async_trait]
impl SensorBackend for KdeBackend {
    fn id(&self) -> &str {
        "kde"
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        vec![SensorDescriptor::sensor("kde_activity", "Active Activity").with_icon("mdi:widgets")]
    }

    async fn poll(&self) -> Vec<SensorState> {
        let mut out = Vec::new();
        match ActivitiesProxy::new(&self.session_bus).await {
            Ok(proxy) => match proxy.current_activity().await {
                Ok(id) => {
                    let name = proxy.activity_name(&id).await.unwrap_or(id);
                    out.push(SensorState::new("kde_activity", name));
                }
                Err(e) => warn!("kde backend: CurrentActivity failed: {e}"),
            },
            Err(e) => warn!("kde backend: ActivityManager proxy: {e}"),
        }
        out
    }
}
