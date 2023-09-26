use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct Observe;

impl ActionTrait for Observe {
    fn level() -> u8 {
        13
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        7
    }
}
