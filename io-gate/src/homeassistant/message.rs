use super::discovery;

/// Things we sent to HA.
#[derive(Debug)]
pub enum Outgoing {
    /// Subscribe to a new topic given as argument. Not a real message.
    Subscribe(String),
    /// Send on initialization once.
    Initial,
    /// Debugging only
    RawTest(Vec<u8>),
    /// Discovery message, to be sent to
    /// <discovery_prefix>/<component>/[<node_id>/]<object_id>/config
    /// Usually:
    /// homeassistant/device/io-gate-[devaddr]/config
    DiscoveryDevice(discovery::Discovery),
}

/// Things HA sents to us (like: trigger switch)
#[derive(Debug)]
pub enum Incoming {
    RawTest(Vec<u8>),

    /// Set output on a device to given state (on or off)
    SetOutput {
        /// Device address
        device: u8,
        /// Output index
        output: u8,
        /// On or off.
        on: bool,
    }
}
