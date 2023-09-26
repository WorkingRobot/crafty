use super::action::{ActionImpl, ActionTrait};
use crate::{action_states::ActionStates, simulator::Simulator};

pub struct StandardTouch;

impl ActionTrait for StandardTouch {
    fn level() -> u8 {
        18
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(sim: &impl Simulator) -> u32 {
        if sim.get_state().action_states.touch_combo_step == 1 {
            18
        } else {
            32
        }
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.25
    }

    fn mutate_state(state: &mut ActionStates) {
        let advance_combo = state.touch_combo_step == 1;
        Self::mutate_state_default(state);
        if advance_combo {
            state.touch_combo_step = 2;
        }
    }
}
