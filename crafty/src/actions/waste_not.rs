use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct WasteNot;

impl ActionTrait for WasteNot {
    fn level() -> u8 {
        15
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        56
    }

    fn durability_cost() -> i8 {
        0
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.get_state().effects.waste_not = 4;
    }
}
