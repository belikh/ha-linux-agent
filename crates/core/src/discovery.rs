use crate::model::{CommandDescriptor, Component, DeviceInfo, SensorDescriptor};
use serde_json::json;

/// Topic the agent publishes its merged sensor-state JSON to. One topic per
/// device, individual entities pulled out via `value_template`.
pub fn state_topic(device_id: &str) -> String {
    format!("ha-linux-agent/{device_id}/state")
}

/// LWT / availability topic: "online" while connected, "offline" if the
/// agent dies without a clean disconnect.
pub fn availability_topic(device_id: &str) -> String {
    format!("ha-linux-agent/{device_id}/availability")
}

/// Topic HA publishes to in order to invoke a command entity.
pub fn command_topic(device_id: &str, command_id: &str) -> String {
    format!("ha-linux-agent/{device_id}/cmd/{command_id}")
}

/// `homeassistant/<component>/<device_id>/<entity_id>/config`
fn discovery_config_topic(prefix: &str, component: &str, device_id: &str, entity_id: &str) -> String {
    format!("{prefix}/{component}/{device_id}_{entity_id}/config")
}

fn unique_id(device_id: &str, entity_id: &str) -> String {
    format!("{device_id}_{entity_id}")
}

pub fn sensor_discovery(
    prefix: &str,
    device: &DeviceInfo,
    device_id: &str,
    d: &SensorDescriptor,
) -> (String, serde_json::Value) {
    let topic = discovery_config_topic(prefix, d.component.discovery_key(), device_id, &d.id);
    let mut payload = json!({
        "name": d.name,
        "unique_id": unique_id(device_id, &d.id),
        "state_topic": state_topic(device_id),
        "availability_topic": availability_topic(device_id),
        // Bracket notation, not dot notation: entity ids can contain
        // characters (e.g. a config-supplied launcher profile id with a
        // hyphen) that aren't valid Jinja attribute-access syntax — found
        // live via a launcher profile id of "dbus-check", which produced
        // {{ value_json.launcher_dbus-check_active }} and silently broke
        // (parsed as subtraction). `['...']` indexing has no such
        // restriction.
        "value_template": format!("{{{{ value_json['{}'] }}}}", d.id),
        "device": device,
    });
    let obj = payload.as_object_mut().unwrap();
    if let Some(unit) = &d.unit {
        obj.insert("unit_of_measurement".into(), json!(unit));
    }
    if let Some(class) = &d.device_class {
        obj.insert("device_class".into(), json!(class));
    }
    if let Some(state_class) = &d.state_class {
        obj.insert("state_class".into(), json!(state_class));
    }
    if let Some(icon) = &d.icon {
        obj.insert("icon".into(), json!(icon));
    }
    (topic, payload)
}

pub fn command_discovery(
    prefix: &str,
    device: &DeviceInfo,
    device_id: &str,
    d: &CommandDescriptor,
) -> (String, serde_json::Value) {
    let topic = discovery_config_topic(prefix, d.component.discovery_key(), device_id, &d.id);
    let mut payload = json!({
        "name": d.name,
        "unique_id": unique_id(device_id, &d.id),
        "command_topic": command_topic(device_id, &d.id),
        "availability_topic": availability_topic(device_id),
        "device": device,
    });
    let obj = payload.as_object_mut().unwrap();
    if let Some(icon) = &d.icon {
        obj.insert("icon".into(), json!(icon));
    }

    if d.component == Component::Switch || d.component == Component::Number || d.component == Component::Select {
        obj.insert("state_topic".into(), json!(state_topic(device_id)));
        let state_key = if d.component == Component::Switch && d.id.starts_with("launcher_") {
            format!("{}_active", d.id)
        } else {
            d.id.clone()
        };
        obj.insert("value_template".into(), json!(format!("{{{{ value_json['{}'] }}}}", state_key)));
    }

    if let Some(min) = d.min {
        obj.insert("min".into(), json!(min));
    }
    if let Some(max) = d.max {
        obj.insert("max".into(), json!(max));
    }
    if let Some(options) = &d.options {
        obj.insert("options".into(), json!(options));
    }
    (topic, payload)
}
