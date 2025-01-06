// Message parsing parts moved from io-ctrl. TODO: Move to a shared crate.

// Input IO index. `0` is reserved to simplify things.
pub type InIdx = u8;
pub type OutIdx = u8;
pub type LayerIdx = u8;
pub type ProcIdx = u8;

/// Higher level switch abstraction.
/// eg. Activated -> LongActivated -> LongClick -> LongDeactivated -> Deactivated.
/// Activated -> ShortClick -> Deactivated
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Trigger {
    /// Short click activation; longer than debounce period, but shorter than a
    /// long click. Triggered on deactivation.
    ShortClick = 0,
    /// Longer than a short click. Triggered on deactivation.
    LongClick,
    /// Triggered right after debouncing period is over.
    Activated,
    /// Triggered immediately on deactivation, no matter time.
    Deactivated,
    /// Activation that exceeds the shortclick time. A bit delayed.
    LongActivated,
    /// Deactivation after LongActivated was triggered
    LongDeactivated,
}

/// Software version
pub const GATE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GATE_NAME: &str = "io-gate";
pub const GATE_URL: &str = env!("CARGO_PKG_HOMEPAGE");

pub const HA_DISCOVERY_TOPIC: &str = "homeassistant";
