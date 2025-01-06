use super::discovery;

/// Things we sent to HA.
#[derive(Debug)]
pub enum Outgoing {
    Initial,
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
}
