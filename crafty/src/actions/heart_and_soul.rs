use super::action::{ActionImpl, ActionTrait};
use crate::{action_states::ActionStates, simulator::Simulator};

pub struct HeartAndSoul;

impl ActionTrait for HeartAndSoul {
    fn level() -> u8 {
        86
    }

    fn durability_cost() -> i8 {
        0
    }

    fn increases_step_count() -> bool {
        false
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        0
    }

    fn can_use(sim: &impl Simulator) -> bool {
        Self::can_use_default(sim)
            && sim.get_input().is_specialist
            && !sim.get_state().action_states.used_heart_and_soul
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.get_state_mut().effects.heart_and_soul = true;
    }

    fn mutate_state(state: &mut ActionStates) {
        state.used_heart_and_soul = true;
        Self::mutate_state_default(state);
    }
}
