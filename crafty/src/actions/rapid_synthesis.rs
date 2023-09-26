use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct RapidSynthesis;

impl ActionTrait for RapidSynthesis {
    fn level() -> u8 {
        9
    }

    fn increases_progress() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        0
    }

    fn efficiency(sim: &impl Simulator) -> f32 {
        // Rapid Synthesis Mastery Trait
        if sim.at_crafter_level(63) {
            5.0
        } else {
            2.5
        }
    }

    fn success_rate(_: &impl Simulator) -> f32 {
        0.5
    }
}
