use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct BasicSynthesis;

impl ActionTrait for BasicSynthesis {
    fn level() -> u8 {
        1
    }

    fn increases_progress() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        0
    }

    fn efficiency(sim: &impl Simulator) -> f32 {
        // Basic Synthesis Mastery Trait
        if sim.at_crafter_level(31) {
            1.2
        } else {
            1.0
        }
    }
}
