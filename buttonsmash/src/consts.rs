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
    ButtonEvent(SwitchEvent),
    /*
    /// External information about layer change
    LayerEvent(LayerEvent),
    */
}
