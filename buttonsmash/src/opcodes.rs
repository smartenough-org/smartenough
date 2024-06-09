use crate::consts::{OutIdx, InIdx, ProcIdx, LayerIdx};

/// Opcodes of the internal micro vm.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Opcode {
    /// No operation
    Noop,
    /// Start a procedure with ID.
    Start(u8),
    /// Return from procedure or end a program.
    Stop,
    /// Call a procedure
    Call(u8),

    /// Direct output control: Toggle IO
    Toggle(OutIdx),
    /// Direct output control: Activate IO (no matter state)
    Activate(OutIdx),
    /// Direct output control: Deactivate IO (no matter state)
    Deactivate(OutIdx),

    /// Enable a layer (later: push layer onto a layer stack)
    LayerPush(LayerIdx),
    /// Clear the layer stack - back to default layer.
    LayerDefault,

    /// Clear all bindings.
    BindClearAll,
    /// Map Input short click to a procedure (on current layer)
    BindShortCall(InIdx, ProcIdx),
    /// Map Input long click to a procedure (on current layer)
    BindLongCall(InIdx, ProcIdx),
    /// Map immediate activate of input to a procedure (on a current layer)
    BindActivateCall(InIdx, ProcIdx),
    /// Map immediate deactivation to a procedure (on a current layer)
    BindDeactivateCall(InIdx, ProcIdx),
    /// Map activate that takes longer than a short click to a procedure (on a current layer)
    BindLongActivate(InIdx, ProcIdx),
    /// Map deactivation after over short click time to a procedure (on a current layer)
    BindLongDeactivate(InIdx, ProcIdx),


    /*
     * Shortcuts
     */
    /// Bind short click to a toggle of an output
    BindShortToggle(InIdx, OutIdx),

    /// Bind long click to a toggle of an output
    BindLongToggle(InIdx, OutIdx),

    /// Bind layer to activate/deactivate triggers.
    BindLayerHold(InIdx, LayerIdx),


    // Hypothetical?
    /*
    /// Read input value (local) into register
    ReadInput(InIdx),
    /// Read input value (local) into register
    ReadOutput(OutIdx),
    /// Call first if register is True, second one if False.
    CallConditionally(ProcIdx, ProcIdx),

    // WaitForRelease - maybe?
    // Procedure 0 is executed after loading and it can map the actions initially

    */
}
