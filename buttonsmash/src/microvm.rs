use crate::consts::*;
use crate::bindings::*;
use crate::opcodes::Opcode;

/// Executes actions using a program.
pub struct Executor<const BINDINGS: usize> {
    /// Current selected Layer
    current_layer: LayerIdx,
    bindings: BindingList<BINDINGS>,
    opcodes: [Opcode; 1024],
    procedures: [usize; MAX_PROCEDURES],
}

impl<const BN: usize> Executor<BN> {
    pub fn new() -> Self {
        // Initialize with an empty program.
        Self {
            current_layer: 0,
            bindings: BindingList::new(),
            opcodes: [Opcode::Noop; 1024],
            procedures: [0; MAX_PROCEDURES],
        }
    }

    pub fn load_static(&mut self, program: &[Opcode]) {
        for (idx, opcode) in program.iter().enumerate() {
            self.opcodes[idx] = *opcode;
        }
        self.index_code();
        self.execute(0);
    }

    pub fn emit(&self, command: Command) {
        println!("Emiting {:?}", command);
    }

    /// Helper: Bind input/trigger to a call to a given procedure.
    fn bind_proc(&mut self, idx: InIdx, trigger: Trigger, proc_idx: ProcIdx) {
        self.bindings.bind(
            Binding {
                idx,
                trigger,
                layer: self.current_layer,
                action: Action::Proc(proc_idx),
            }
        );
    }

    /// Helper: Bind input/trigger to single command.
    fn bind_single(&mut self, idx: InIdx, trigger: Trigger, command: Command) {
        self.bindings.bind(
            Binding {
                idx,
                trigger,
                layer: self.current_layer,
                action: Action::Single(command),
            }
        );
    }

    pub fn execute(&mut self, proc: ProcIdx) {
        let mut pc = self.procedures[proc as usize];
        assert_eq!(self.opcodes[pc], Opcode::Start(proc));
        loop {
            pc += 1;
            match self.opcodes[pc] {
                Opcode::Noop => {
                    /* Noop */
                }
                Opcode::Stop => {
                    break;
                }
                Opcode::Start(_) => {
                    panic!("Invalid opcode: Start");
                }
                Opcode::Call(proc_id) => {
                    // TODO: Own stack?
                    self.execute(proc_id);
                }

                Opcode::Toggle(out_idx) => {
                    self.emit(Command::ToggleOutput(out_idx));
                }
                Opcode::Activate(out_idx) => {
                    self.emit(Command::ActivateOutput(out_idx));
                }
                Opcode::Deactivate(out_idx) => {
                    self.emit(Command::DeactivateOutput(out_idx));
                }

                // Enable a layer (later: push layer onto a layer stack)
                Opcode::LayerPush(layer) => {
                    assert!(layer as usize <= MAX_LAYERS);
                    self.current_layer = layer;
                }
                // Clear the layer stack - back to default layer.
                Opcode::LayerDefault => {
                    self.current_layer = 0;
                }

                // WaitForRelease - maybe?
                // Procedure 0 is executed after loading and it can map the actions initially

                // Clear all the bindings.
                Opcode::BindClearAll => {
                    self.bindings.clear();
                }

                Opcode::BindShortCall(in_idx, proc_idx) => {
                    self.bind_proc(in_idx, Trigger::ShortClick, proc_idx);
                }
                Opcode::BindLongCall(in_idx, proc_idx) => {
                    self.bind_proc(in_idx, Trigger::LongClick, proc_idx);
                }
                Opcode::BindActivateCall(in_idx, proc_idx) => {
                    self.bind_proc(in_idx, Trigger::Activated, proc_idx);
                }
                Opcode::BindDeactivateCall(in_idx, proc_idx) => {
                    self.bind_proc(in_idx, Trigger::Deactivated, proc_idx);
                }
                Opcode::BindLongActivate(in_idx, proc_idx) => {
                    self.bind_proc(in_idx, Trigger::LongActivated, proc_idx);
                }
                Opcode::BindLongDeactivate(in_idx, proc_idx) => {
                    self.bind_proc(in_idx, Trigger::LongDeactivated, proc_idx);
                }


                /*
                 * Shortcuts
                 */
                // Trivial configuration shortcuts.
                Opcode::BindShortToggle(in_idx, out_idx) => {
                    self.bind_single(in_idx, Trigger::ShortClick, Command::ToggleOutput(out_idx));
                }

                Opcode::BindLongToggle(in_idx, out_idx) => {
                    self.bind_single(in_idx, Trigger::LongClick, Command::ToggleOutput(out_idx));
                }

                Opcode::BindLayerHold(in_idx, layer_idx) => {
                    // When this is in use + ShortClick means something, then
                    // the shortclick should be defined on that layer.
                    self.bind_single(in_idx, Trigger::Activated, Command::ActivateLayer(layer_idx));
                    self.bind_single(in_idx, Trigger::Deactivated, Command::DeactivateLayer(layer_idx));
                }

                // Hypothetical?
                // Read input value (local) into register
                /*
                Opcode::ReadInput(in_idx) => {
                },
                /// Read input value (local) into register
                Opcode::ReadOutput(OutIdx) => {
                },
                /// Call first if register is True, second one if False.
                Opcode::CallConditionally(proc_idx, proc_idx) => {
                },
                */
            }
        }
    }

    /// Index procedures starts
    fn index_code(&mut self) {
        for i in 0..MAX_PROCEDURES {
            self.procedures[i] = 0;
        }

        for (idx, opcode) in self.opcodes.iter().enumerate() {
            if let Opcode::Start(proc_idx) = opcode {
                self.procedures[*proc_idx as usize] = idx;
            }
        }
    }

    /// Reads events and reacts to it.
    fn parse_event(&mut self, event: &Event) {
        match event {
            Event::ButtonEvent(event) => {
                event.switch_id;
                event.state;
                todo!("find action related");
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_handles_code() {

        const PROGRAM: [Opcode; 16] = [
            // Setup proc.
            Opcode::Start(0),
            Opcode::LayerDefault,
            Opcode::BindShortToggle(1, 1),
            Opcode::BindShortToggle(2, 2),
            Opcode::BindShortToggle(3, 3),
            Opcode::BindShortToggle(4, 4),
            Opcode::BindShortToggle(5, 5),
            Opcode::BindShortToggle(6, 6),
            Opcode::BindShortToggle(7, 7),
            Opcode::BindShortToggle(8, 8),
            Opcode::BindShortToggle(9, 9),
            Opcode::BindShortToggle(10, 10),
            Opcode::Stop,

            // Random proc.
            Opcode::Start(1),
            Opcode::Toggle(1),
            Opcode::Stop,
        ];

        // let (event_sender, event_receiver) = mpsc::channel(32);
        let mut executor: Executor<30> = Executor::new();
        executor.load_static(&PROGRAM);

        // let event = channel.recv().await;
        executor.parse_event(&Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Activated,
            }));

        todo!();
    }
}
