use event_core::DuoChannel;
use events::main_x_control::{MainFromControl, MainToControl};
use specs::{RunArg, System};
use utils::Delta;

#[derive(Debug)]
pub struct ControlSystem<ID>
    where ID: Send + Eq
{
    main_channel: DuoChannel<ID, MainFromControl, MainToControl>,
}

impl<ID> ControlSystem<ID>
    where ID: Send + Eq
{
    pub fn new(main_channel: DuoChannel<ID, MainFromControl, MainToControl>) -> ControlSystem<ID> {
        ControlSystem {
            main_channel: main_channel,
        }
    }

    fn process_event(&mut self, event: &MainToControl) {
        match event {

        }
    }
}

impl<ID> System<Delta> for ControlSystem<ID>
    where ID: Send + Eq
{
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        arg.fetch(|_| ());
    }
}
