use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct PreparatoryTouch;

impl ActionTrait for PreparatoryTouch {
    fn level() -> u8 {
        71
    }

    fn increases_quality() -> bool {
        true
    }

    fn durability_cost() -> i8 {
        20
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        40
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        2.0
    }

    fn use_success(sim: &mut impl Simulator) {
        Self::use_success_default(sim);
        sim.get_state_mut().effects.inner_quiet += 1;
    }
}
