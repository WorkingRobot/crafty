use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct Innovation;

impl ActionTrait for Innovation {
    fn level() -> u8 {
        26
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        18
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.get_state().effects.innovation = 4;
    }
}
