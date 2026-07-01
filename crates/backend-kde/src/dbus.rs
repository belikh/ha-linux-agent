use zbus::proxy;

/// Verified live against kactivitymanagerd (`busctl --user introspect
/// org.kde.ActivityManager /ActivityManager/Activities`) — this API has been
/// stable across Plasma 5/6.
#[proxy(
    interface = "org.kde.ActivityManager.Activities",
    default_service = "org.kde.ActivityManager",
    default_path = "/ActivityManager/Activities"
)]
pub trait Activities {
    #[zbus(name = "CurrentActivity")]
    fn current_activity(&self) -> zbus::Result<String>;

    #[zbus(name = "ActivityName")]
    fn activity_name(&self, id: &str) -> zbus::Result<String>;
}
