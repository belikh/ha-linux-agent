use crate::model::{CommandDescriptor, SensorDescriptor, SensorState};
use async_trait::async_trait;

/// A pluggable source of read-only state, published to HA as sensors /
/// binary_sensors. Implement this in a `backend-<name>` crate to add support
/// for a new desktop environment, init system, or subsystem — see the
/// workspace README for the full guide.
#[async_trait]
pub trait SensorBackend: Send + Sync {
    /// Short, stable identifier for this backend (used in logs only).
    fn id(&self) -> &str;

    /// The fixed set of entities this backend publishes. Called once at
    /// startup to build HA discovery configs — must not change at runtime.
    fn sensors(&self) -> Vec<SensorDescriptor>;

    /// Current values for (a subset of) the entities from `sensors()`.
    /// Called once per poll tick. Returning fewer entries than `sensors()`
    /// is fine (e.g. a sensor temporarily unavailable) — HA just won't see
    /// an update for that entity this tick.
    async fn poll(&self) -> Vec<SensorState>;
}

/// A pluggable sink for HA-initiated actions (buttons/switches).
#[async_trait]
pub trait CommandBackend: Send + Sync {
    fn id(&self) -> &str;

    /// The fixed set of command entities this backend exposes.
    fn commands(&self) -> Vec<CommandDescriptor>;

    /// Dispatch one command. `command_id` matches a `CommandDescriptor::id`
    /// from `commands()`; `payload` is the raw MQTT message body.
    async fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()>;
}

// Blanket impls so a backend implementing both traits (e.g. GenericBackend,
// which exposes both sensors and commands) can be wrapped once in an `Arc`
// and registered in both the sensor and command backend lists.
#[async_trait]
impl<T: SensorBackend + ?Sized> SensorBackend for std::sync::Arc<T> {
    fn id(&self) -> &str {
        (**self).id()
    }

    fn sensors(&self) -> Vec<SensorDescriptor> {
        (**self).sensors()
    }

    async fn poll(&self) -> Vec<SensorState> {
        (**self).poll().await
    }
}

#[async_trait]
impl<T: CommandBackend + ?Sized> CommandBackend for std::sync::Arc<T> {
    fn id(&self) -> &str {
        (**self).id()
    }

    fn commands(&self) -> Vec<CommandDescriptor> {
        (**self).commands()
    }

    async fn handle(&self, command_id: &str, payload: &str) -> anyhow::Result<()> {
        (**self).handle(command_id, payload).await
    }
}
