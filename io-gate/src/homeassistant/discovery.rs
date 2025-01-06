use crate::config;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;

/// Device identifier
#[derive(Serialize, Debug, Default)]
pub struct DeviceId {
    pub name: String,
    /// Device CAN bus ID + CAN Address to uniquely identify device.
    pub identifiers: Vec<String>,
    pub manufacturer: String,
    // hw, sw, ...
}

/// Discovery origin - this software identifier.
#[derive(Serialize, Debug, Default)]
pub struct Origin {
    name: String,
    sw_version: String,
    support_url: String,
}

/// Represents a component - a part of Device defined by Discovery
#[derive(Serialize, Debug)]
pub struct Component {
    name: String,
    platform: String,
    /// Changes icon; outlet or switch
    device_class: String,
    unique_id: String,
    /// smartenough/
    command_topic: String,
    state_topic: String,
}

impl Component {
    pub fn new_switch(name: &str, device_addr: u8, idx: u8) -> Self {
        Self {
            name: name.to_string(),
            platform: "switch".to_string(),
            device_class: "switch".to_string(),
            unique_id: format!("io-gate-{}-{}", device_addr, idx),
            command_topic: format!("smartenough/{}/switch/{}/set", device_addr, idx),
            state_topic: format!("smartenough/{}/switch/{}/get", device_addr, idx),
        }
    }
}

// config topic: homeassistant/binary_sensor/garden/config
// <discovery_prefix>/<component>/[<node_id>/]<object_id>/config
// component == switch, node_id == omit, object_id == unique_id
// If using device discovery then component is == `device`.
#[derive(Serialize, Debug)]
pub struct Discovery {
    pub device: DeviceId,
    pub origin: Origin,

    pub components: HashMap<String, Component>,
}

impl Discovery {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).expect("All should be serializable")
    }
}

pub fn new_device(name: &str, config: &config::DeviceConfig) -> Discovery {
    let origin = Origin {
        name: crate::consts::GATE_NAME.to_string(),
        sw_version: crate::consts::GATE_VERSION.to_string(),
        support_url: crate::consts::GATE_URL.to_string(),
    };

    let device_id = DeviceId {
        name: name.to_string(),
        identifiers: vec![format!("gate-{}", config.addr)],
        manufacturer: "smartenough".to_string(),
    };

    let mut components = HashMap::new();

    for i in 0..config.outputs.count {
        let name = if let Some(label) = config.outputs.labels.get(i as usize) {
            label.to_string()
        } else {
            format!("{}-{}", name, i)
        };
        let component = Component::new_switch(&name, config.addr, i);

        components.insert(name, component);
    }

    Discovery {
        origin,
        device: device_id,
        components,
    }
}
