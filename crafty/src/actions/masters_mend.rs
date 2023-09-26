use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct MastersMend;

impl ActionTrait for MastersMend {
    fn level() -> u8 {
        7
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        88
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.restore_durability(30);
    }
}
