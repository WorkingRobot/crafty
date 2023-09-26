use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct TricksOfTheTrade;

impl ActionTrait for TricksOfTheTrade {
    fn level() -> u8 {
        13
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        0
    }

    fn can_use(sim: &impl Simulator) -> bool {
        sim.is_in_good_condition() && Self::can_use_default(sim)
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.restore_cp(20);
        sim.consume_good_condition();
    }
}
