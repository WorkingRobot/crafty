use crate::{condition::Condition, executor::Executor, simulator::Simulator, Input, State};

#[derive(Debug)]
pub struct NoRngSimulator<'s> {
    state: &'s mut State,
    input: Input,
}

impl Simulator for NoRngSimulator<'_> {
    fn get_state(&self) -> &State {
        self.state
    }

    fn get_state_mut(&mut self) -> &mut State {
        self.state
    }

    fn get_input(&self) -> &Input {
        &self.input
    }

    fn roll_condition(&mut self) -> Condition {
        Condition::Normal
    }

    fn roll_success_raw(&mut self, success_rate: f32) -> bool {
        success_rate == 1.0
    }
}

impl Executor for NoRngSimulator<'_> {
    fn set_input(&mut self, input: Input) {
        self.input = input;
    }
}

impl<'s> NoRngSimulator<'s> {
    pub fn new(state: &'s mut State, input: Input) -> Self {
        Self { state, input }
    }
}
