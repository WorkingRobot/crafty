use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct WasteNot2;

impl ActionTrait for WasteNot2 {
    fn level() -> u8 {
        47
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        98
    }

    fn durability_cost() -> i8 {
        0
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.get_state().effects.waste_not_ii = 8;
    }
}
