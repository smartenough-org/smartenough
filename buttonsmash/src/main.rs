/*
 * Use cases:
 * - Outputs can be active-high or active-low.
 * - Inputs can be active-high or active-low.
 * - Inputs and outputs can use native µC pins or expander IOs.
 * - Output can have a maximum activation time and be disabled even if button is still held (shutters).
 *
 * - A push-button toggles state of a single local output.
 * - Multiple buttons controls a IO that might be local or remote.
 * - Button can control multiple IOs. Up to.. X? "Scene".
 * - Button can control a scene that `has a state`?
 * - Stair or long corridor control. Button TOGGLES the output, no matter what is the
 *   state. One controller has the output and 1 or more inputs correlated.
 * - Button has different action when short-clicked or when held longer.
 * - Holding a button can cause a single-shot action (toggle light/scene) or change a layer.
 * - Layer changes actions locally assigned to buttons. Some buttons can be
 *   "transparent" on a layer and keep the same action.
 * - MAYBE: layers could be triggered remotely.
 * - Input/output configuration can be static/compiled. It's hardware dependent.
 * - Input->output mapping and scenes should be dynamically programmable.
 * - Pressing a button activates Output for a set amount of time.
 *
 * Analysis:
 * Living room + kitchen + dining room setup + terrace
 * Inputs: (2+3+2) * 2 inputs = 14 inputs.
 * Lights:
 * - Kitchen lights: 2 + Leds?
 * - Dining room: 1+1+1
 * - Living room: 1+1+1 (+1?)
 * - Terrace: 2
 * - Corridor light (remote, not physical): 1
 * Drapes: 5 with 2 directions -> 10. Not physical, remote.
 * Total: 11 outputs... or 21.
 *
 * Possible "scenes" ("partial" scenes?):
 * - Everything on/off (including wall-lights/ambients?)
 * - Kitchen on/off (ceiling + island)
 * - Ambient lighting (wall light ON, rest OFF).
 * - Ceiling lights: kitchen
 * - Ceiling lights: living room
 * - Ceiling lights: dining room
 * - Ceiling lights all on (or off) - as a group
 * - Kitchen island lights
 * - Living room dining table
 * - Kitchen island + dining table
 * - Ambient dining (table + island)
 *
 * Disabling a scene should...:
 * - turn off all things that are turned on by a scene and set off all the
 *   things that are turned off by it? Or rather turn off what is turned on and
 *   keep the rest as is?
 *
 * - toggling a scene needs information about its state or "if at least one of
 *   its lights is enabled assume the scene is on".
 */
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use buttonsmash::{
    consts::*,
    opcodes::Opcode,
    microvm::*
};

/*
 * Layers in a distributed system.
 *
 * Those would work a bit differently than in QMK. Layer being active would need
 * a regular "yeah it's still up" and have a timeout. Deactivate not strictly
 * needed. Still. Do we need this?
 *
 * IOs can be remote, but layers should work within the single controller. Also,
 * someone hitting a button upstrairs, shouldn't not alter the behaviour
 * downstairs.
 *
 */

struct OutputMock {
    pub commands: Mutex<Vec<Command>>,
}

impl OutputMock {
    pub fn new() -> Self {
        OutputMock {
            commands: Mutex::new(Vec::new()),
        }
    }

    pub async fn set(&self, command: Command) {
        self.commands.lock().await.push(command);
    }

    pub async fn clear(&self) {
        self.commands.lock().await.clear();
    }
}

async fn main_event_handler(mut channel: mpsc::Receiver<Event>, outputs: Arc<OutputMock>) {
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

    let mut executor: Executor<30> = Executor::new();
    executor.load_static(&PROGRAM);

    loop {
        let event = channel.recv().await;
        println!("Got event {:?}", event);
    }
}

#[tokio::main]
async fn main() {
    panic!();

    let output = Arc::new(OutputMock::new());
    let (event_sender, event_receiver) = mpsc::channel(32);
    let out = output.clone();
    let handler = tokio::spawn(main_event_handler(event_receiver, out));

    println!("Main starts!");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    loop {
        /* Click 1 */
        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Activated,
            }))
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Deactivated(30),
            }))
            .await
            .unwrap();

        /* Press 2 long */
        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Activated,
            }))
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Active(30),
            }))
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        /* Press 1 again */
        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Activated,
            }))
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Deactivated(30),
            }))
            .await
            .unwrap();

        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Active(60),
            }))
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        event_sender
            .send(Event::ButtonEvent(SwitchEvent {
                switch_id: 1,
                state: SwitchState::Deactivated(30),
            }))
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        println!("Main waits for next cycle");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    handler.await.unwrap();
}
