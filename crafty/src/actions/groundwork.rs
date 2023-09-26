use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct Groundwork;

impl ActionTrait for Groundwork {
    fn level() -> u8 {
        72
    }

    fn increases_progress() -> bool {
        true
    }

    fn durability_cost() -> i8 {
        20
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        18
    }

    fn efficiency(sim: &impl Simulator) -> f32 {
        // Groundwork Mastery Trait
        let state = sim.get_state();
        let eff = if sim.at_crafter_level(86) { 3.6 } else { 3.0 };
        if state.durability < sim.calculate_durability_cost(Self::durability_cost()) {
            eff / 2.0
        } else {
            eff
        }
    }
}
