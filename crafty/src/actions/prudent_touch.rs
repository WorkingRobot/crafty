use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct PrudentTouch;

impl ActionTrait for PrudentTouch {
    fn level() -> u8 {
        66
    }

    fn increases_quality() -> bool {
        true
    }

    fn durability_cost() -> i8 {
        5
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        25
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.0
    }

    fn can_use(sim: &impl Simulator) -> bool {
        !sim.get_state().effects.is_waste_not_active() && Self::can_use_default(sim)
    }
}
