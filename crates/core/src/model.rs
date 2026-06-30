use serde::Serialize;

/// HA MQTT discovery component kind. Only the kinds backends actually use —
/// extend as new backend needs arise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Sensor,
    BinarySensor,
    Button,
    Switch,
}

impl Component {
    pub fn discovery_key(&self) -> &'static str {
        match self {
            Component::Sensor => "sensor",
            Component::BinarySensor => "binary_sensor",
            Component::Button => "button",
            Component::Switch => "switch",
        }
    }
}

/// Static metadata for one entity a `SensorBackend` exposes. Published once
/// (per connect) as an HA MQTT discovery config.
#[derive(Debug, Clone)]
pub struct SensorDescriptor {
    /// Unique within the device, e.g. "cpu_usage". Used to build topics and
    /// the `value_json.<id>` template HA uses to pull this field out of the
    /// shared state payload.
    pub id: String,
    pub name: String,
    pub component: Component,
    pub device_class: Option<String>,
    pub unit: Option<String>,
    pub icon: Option<String>,
}

impl SensorDescriptor {
    pub fn sensor(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            component: Component::Sensor,
            device_class: None,
            unit: None,
            icon: None,
        }
    }

    pub fn binary_sensor(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            component: Component::BinarySensor,
            device_class: None,
            unit: None,
            icon: None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_device_class(mut self, class: impl Into<String>) -> Self {
        self.device_class = Some(class.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// One sensor's current value, pre-rendered as the string that should land
/// in the shared state JSON payload (numbers stringified as JSON numbers via
/// `serde_json::Value`, binary sensors as "ON"/"OFF").
#[derive(Debug, Clone)]
pub struct SensorState {
    pub id: String,
    pub value: serde_json::Value,
}

impl SensorState {
    pub fn new(id: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
        }
    }

    pub fn binary(id: impl Into<String>, on: bool) -> Self {
        Self::new(id, if on { "ON" } else { "OFF" })
    }
}

/// Static metadata for one command entity (button/switch) a `CommandBackend`
/// exposes.
#[derive(Debug, Clone)]
pub struct CommandDescriptor {
    pub id: String,
    pub name: String,
    pub component: Component,
    pub icon: Option<String>,
}

impl CommandDescriptor {
    pub fn button(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            component: Component::Button,
            icon: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    // serialized verbatim into HA discovery payloads' "device" key
    pub identifiers: Vec<String>,
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub sw_version: String,
}
