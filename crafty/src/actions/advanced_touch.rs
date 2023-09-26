use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct AdvancedTouch;

impl ActionTrait for AdvancedTouch {
    fn level() -> u8 {
        84
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(sim: &impl Simulator) -> u32 {
        if sim.get_state().action_states.touch_combo_step == 2 {
            18
        } else {
            46
        }
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.5
    }
}
