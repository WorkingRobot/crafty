use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct CarefulSynthesis;

impl ActionTrait for CarefulSynthesis {
    fn level() -> u8 {
        62
    }

    fn increases_progress() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        7
    }

    fn efficiency(sim: &impl Simulator) -> f32 {
        // Careful Synthesis Mastery Trait
        if sim.at_crafter_level(82) {
            1.80
        } else {
            1.50
        }
    }
}
