use super::action::{ActionImpl, ActionTrait};
use crate::{action_states::ActionStates, simulator::Simulator};

pub struct BasicTouch;

impl ActionTrait for BasicTouch {
    fn level() -> u8 {
        5
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        18
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.0
    }

    fn mutate_state(state: &mut ActionStates) {
        Self::mutate_state_default(state);
        state.touch_combo_step = 1;
    }
}
