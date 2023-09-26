use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct IntensiveSynthesis;

impl ActionTrait for IntensiveSynthesis {
    fn level() -> u8 {
        78
    }

    fn increases_progress() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        6
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        4.0
    }

    fn can_use(sim: &impl Simulator) -> bool {
        Self::can_use_default(sim) && sim.is_in_good_condition()
    }

    fn use_success(sim: &mut impl Simulator) {
        Self::use_success_default(sim);
        sim.consume_good_condition();
    }
}
