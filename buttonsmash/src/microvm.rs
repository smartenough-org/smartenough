use tokio::sync::mpsc;

use crate::bindings::*;
use crate::consts::*;
use crate::layers::Layers;
use crate::opcodes::Opcode;

/// Executes actions using a program.
pub struct Executor<const BINDINGS: usize> {
    /// Current selected Layer
    // current_layer: LayerIdx,
    layers: Layers,
    bindings: BindingList<BINDINGS>,
    opcodes: [Opcode; 1024],
    procedures: [usize; MAX_PROCEDURES],

    command_queue: mpsc::Sender<Command>,
}

impl<const BN: usize> Executor<BN> {
    pub fn new(queue: mpsc::Sender<Command>) -> Self {
        Self {
            layers: Layers::new(),
            bindings: BindingList::new(),
            opcodes: [Opcode::Noop; 1024],
            procedures: [0; MAX_PROCEDURES],

            command_queue: queue,
        }
    }

    pub async fn load_static(&mut self, program: &[Opcode]) {
        for (idx, opcode) in program.iter().enumerate() {
            self.opcodes[idx] = *opcode;
        }
        self.index_code();
        self.execute(0).await;
        // Finish on default layer
        self.layers.reset();
    }

    pub async fn emit(&self, command: Command) {
        println!("Emiting {:?}", command);
        // TODO: Maybe some timeout in case it breaks and we don't want to hang?
        self.command_queue.send(command).await.unwrap();
    }

    /// Helper: Bind input/trigger to a call to a given procedure.
    fn bind_proc(&mut self, idx: InIdx, trigger: Trigger, proc_idx: ProcIdx) {
        self.bindings.bind(Binding {
            idx,
            trigger,
            layer: self.layers.current,
            action: Action::Proc(proc_idx),
        });
    }

    /// Helper: Bind input/trigger to single command.
    fn bind_single(&mut self, idx: InIdx, trigger: Trigger, command: Command) {
        self.bindings.bind(Binding {
            idx,
            trigger,
            layer: self.layers.current,
            action: Action::Single(command),
        });
    }

    async fn execute_opcode(&mut self, opcode: Opcode) -> bool {
        match opcode {
            Opcode::Noop => { /* Noop */ }
            Opcode::Stop => {
                return true;
            }
            Opcode::Start(_) => {
                panic!("Invalid opcode: Start");
            }
            Opcode::Call(proc_id) => {
                // TODO: Own stack?
                Box::pin(self.execute(proc_id)).await;
            }

            Opcode::Toggle(out_idx) => {
                self.emit(Command::ToggleOutput(out_idx)).await;
            }
            Opcode::Activate(out_idx) => {
                self.emit(Command::ActivateOutput(out_idx)).await;
            }
            Opcode::Deactivate(out_idx) => {
                self.emit(Command::DeactivateOutput(out_idx)).await;
            }

            // Enable a layer (TODO: push layer onto a layer stack?)
            Opcode::LayerPush(layer) => {
                assert!(layer as usize <= MAX_LAYERS);
                // Use a `virtual` input idx of 0 when forcing a layer activation.
                self.layers.activate(0, layer);
            }
            Opcode::LayerPop => {
                // Deactivate last virtual 0 input.
                self.layers.maybe_deactivate(0);
            }
            Opcode::LayerSet(layer) => {
                self.layers.reset();
                self.layers.activate(0, layer);
            }

            // Clear the layer stack - back to default layer.
            Opcode::LayerDefault => {
                self.layers.reset();
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
                // When this is in use + ShortClick is defined for the same key,
                // then the shortclick should be defined on new layer.
                self.bind_single(
                    in_idx,
                    Trigger::Activated,
                    Command::ActivateLayer(layer_idx),
                );

                // NOTE: Layer deactivation is handled automatically and should
                // not be bound.
            } // Hypothetical?
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
        false
    }

    pub async fn execute(&mut self, proc: ProcIdx) {
        let mut pc = self.procedures[proc as usize];
        assert_eq!(self.opcodes[pc], Opcode::Start(proc));
        loop {
            pc += 1;
            let opcode = self.opcodes[pc];
            if self.execute_opcode(opcode).await {
                break;
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
    pub async fn parse_event(&mut self, event: &Event) {
        match event {
            Event::ButtonTrigger(data) => {
                if data.trigger == Trigger::Deactivated && self.layers.maybe_deactivate(data.in_idx)
                {
                    // Deactivated layer that was previously activated using
                    // this key. TODO: Warning! Event order might be important.
                    // longclick, longdeactivate first, then deactivate?
                    return;
                }

                let binding = self.bindings.filter(
                    data.in_idx,
                    Some(self.layers.current),
                    Some(data.trigger),
                );
                if let Some(binding) = binding {
                    println!("Found matching event {:?}", binding.action);

                    match binding.action {
                        Action::Noop => {}
                        Action::Single(cmd) => match cmd {
                            Command::ActivateLayer(layer) => {
                                self.layers.activate(data.in_idx, layer);
                                // self.current_layer = layer
                            }
                            Command::DeactivateLayer(layer) => {
                                todo!("deactivation is based on stack list");
                                // self.current_layer = 0
                            }
                            _ => self.emit(cmd).await,
                        },
                        Action::Proc(proc_idx) => {
                            self.execute(proc_idx).await;
                        }
                    }
                } else {
                    println!("Not found binding {:?}!", data);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    async fn get_prepared() -> (Executor<30>, mpsc::Receiver<Command>) {
        const PROGRAM: [Opcode; 16] = [
            // Setup proc.
            Opcode::Start(0),
            Opcode::LayerDefault,
            Opcode::BindShortToggle(1, 10),
            Opcode::BindShortToggle(2, 11),
            Opcode::BindLongToggle(3, 20),
            Opcode::BindShortToggle(3, 21),
            Opcode::BindShortCall(4, 1),
            Opcode::BindLayerHold(5, 66),
            Opcode::LayerPush(66),
            Opcode::BindShortToggle(1, 13),
            Opcode::Stop,
            // Test proc.
            Opcode::Start(1),
            Opcode::Activate(100),
            Opcode::Activate(101),
            Opcode::Deactivate(110),
            Opcode::Stop,
        ];

        let (event_src, event_handler) = mpsc::channel(32);
        let mut executor: Executor<30> = Executor::new(event_src);
        executor.load_static(&PROGRAM).await;

        (executor, event_handler)
    }


    #[tokio::test]
    async fn it_handles_basic_code() {

        let (mut executor, mut event_handler) = get_prepared().await;

        executor
            .parse_event(&Event::new_button_trigger(3, Trigger::LongClick))
            .await;
        executor
            .parse_event(&Event::new_button_trigger(3, Trigger::ShortClick))
            .await;
        assert!(!event_handler.is_empty());
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ToggleOutput(20));
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ToggleOutput(21));
        assert!(event_handler.is_empty());

        // Try procedure execution
        executor
            .parse_event(&Event::new_button_trigger(4, Trigger::ShortClick))
            .await;
        assert!(!event_handler.is_empty());
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ActivateOutput(100));
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ActivateOutput(101));
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::DeactivateOutput(110));
    }

    #[tokio::test]
    async fn it_handles_layers() {
        let (mut executor, mut event_handler) = get_prepared().await;

        // Try layer differentiator
        assert!(event_handler.is_empty());

        // Normal activation on layer 0 -> 10 output
        executor
            .parse_event(&Event::new_button_trigger(1, Trigger::ShortClick))
            .await;
        // Holds layer 66 active.
        executor
            .parse_event(&Event::new_button_trigger(5, Trigger::Activated))
            .await;
        // Now activates 13 instead.
        executor
            .parse_event(&Event::new_button_trigger(1, Trigger::ShortClick))
            .await;
        // ...twice.
        executor
            .parse_event(&Event::new_button_trigger(1, Trigger::ShortClick))
            .await;
        // Back to layer 1
        executor
            .parse_event(&Event::new_button_trigger(5, Trigger::Deactivated))
            .await;
        // Activates 10.
        executor
            .parse_event(&Event::new_button_trigger(1, Trigger::ShortClick))
            .await;

        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ToggleOutput(10));
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ToggleOutput(13));
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ToggleOutput(13));
        let cmd = event_handler.recv().await.unwrap();
        assert_eq!(cmd, Command::ToggleOutput(10));
        assert!(event_handler.is_empty());

        // TODO: Multiple layers test.
    }
}
