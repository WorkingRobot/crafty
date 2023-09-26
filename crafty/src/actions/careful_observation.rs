use super::action::{ActionImpl, ActionTrait};
use crate::{action_states::ActionStates, simulator::Simulator};

pub struct CarefulObservation;

impl ActionTrait for CarefulObservation {
    fn level() -> u8 {
        55
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
        sim.get_input().is_specialist && sim.get_state().action_states.careful_observation_count < 3
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.tick_condition()
    }

    fn mutate_state(state: &mut ActionStates) {
        state.careful_observation_count += 1;
        Self::mutate_state_default(state);
    }
}
