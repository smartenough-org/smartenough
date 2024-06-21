/*
 * Shared, common constants and trivial structures
 */

// Input IO index. `0` is reserved to simplify things.
pub type InIdx = u8;
pub type OutIdx = u8;
pub type LayerIdx = u8;
pub type ProcIdx = u8;
pub const MAX_PROCEDURES: usize = 128;
pub const MAX_LAYERS: usize = 128;
pub const MAX_LAYER_STACK: usize = 5;

// FIXME: Those required?
pub const MAX_INPUTS: usize = 128;
pub const MAX_OUTPUTS: usize = 128;

// TODO: Low/high active?
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
    /// Toggle output...
    ToggleOutput(OutIdx),
    /// Enable output of given ID - Local or remote.
    ActivateOutput(OutIdx),
    /// Deactivate output of given ID - Local or remote
    DeactivateOutput(OutIdx),

    /// Activate layer (public message)
    ActivateLayer(LayerIdx),
    /// Deactivate layer (public message)
    DeactivateLayer(LayerIdx),
    /// No operation
    Noop,
}

/// Buttons can be triggered in multiple ways.
/// TODO: This is after initial detection of short/long click detection. Events can be duplicated for a key:
/// eg. Activated -> LongActivated -> LongClick -> LongDeactivated -> Deactivated.
/// Activated -> ShortClick -> Deactivated
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Trigger {
    /// Short click activation; longer than debounce period, but shorter than a
    /// long click. Triggered on deactivation.
    ShortClick,
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

#[derive(Debug, Copy, Clone)]
pub struct ButtonTrigger {
    pub in_idx: InIdx,
    pub trigger: Trigger,
}

#[derive(Debug, Copy, Clone)]
pub enum SwitchState {
    /// Just pressed
    Activated,
    // Still active
    Active(u32),
    /// Released with a time it was pressed (in quantified ms)
    Deactivated(u32),
}

#[derive(Debug, Copy, Clone)]
pub struct SwitchEvent {
    pub switch_id: InIdx,
    pub state: SwitchState,
}

#[derive(Debug)]
pub enum LayerEvent {
    Activate(u8),
    Deactivate(u8),
}

#[derive(Debug)]
pub enum Event {
    /// Button event
    ButtonTrigger(ButtonTrigger),
    /*
    /// External information about layer change
    LayerEvent(LayerEvent),
    */
}

impl Event {
    pub fn new_button_trigger(in_idx: InIdx, trigger: Trigger) -> Self {
        Event::ButtonTrigger(ButtonTrigger {
            in_idx,
            trigger
        })
    }
}
