use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct PrudentSynthesis;

impl ActionTrait for PrudentSynthesis {
    fn level() -> u8 {
        88
    }

    fn increases_progress() -> bool {
        true
    }

    fn durability_cost() -> i8 {
        5
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        18
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.8
    }

    fn can_use(sim: &impl Simulator) -> bool {
        !sim.get_state().effects.is_waste_not_active() && Self::can_use_default(sim)
    }
}
