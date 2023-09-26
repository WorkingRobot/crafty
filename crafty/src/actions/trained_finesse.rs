use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct TrainedFinesse;

impl ActionTrait for TrainedFinesse {
    fn level() -> u8 {
        90
    }

    fn increases_quality() -> bool {
        true
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        32
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.0
    }

    fn can_use(sim: &impl Simulator) -> bool {
        sim.get_state().effects.inner_quiet == 10 && Self::can_use_default(sim)
    }
}
