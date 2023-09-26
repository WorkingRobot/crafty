use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct GreatStrides;

impl ActionTrait for GreatStrides {
    fn level() -> u8 {
        21
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        32
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.get_state().effects.great_strides = 3;
    }
}
