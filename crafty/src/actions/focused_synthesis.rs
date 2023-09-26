use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct FocusedSynthesis;

impl ActionTrait for FocusedSynthesis {
    fn level() -> u8 {
        67
    }

    fn increases_progress() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        5
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        2.0
    }

    fn success_rate(sim: &impl Simulator) -> f32 {
        if sim.get_state().action_states.has_observed {
            1.00
        } else {
            0.50
        }
    }
}
