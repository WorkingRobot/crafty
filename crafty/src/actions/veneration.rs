use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct Veneration;

impl ActionTrait for Veneration {
    fn level() -> u8 {
        15
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        18
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.get_state().effects.veneration = 4;
    }
}
