use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct FocusedTouch;

impl ActionTrait for FocusedTouch {
    fn level() -> u8 {
        68
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        18
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.5
    }

    fn success_rate(sim: &impl Simulator) -> f32 {
        if sim.get_state().action_states.has_observed {
            1.00
        } else {
            0.50
        }
    }
}
