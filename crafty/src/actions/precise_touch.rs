use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct PreciseTouch;

impl ActionTrait for PreciseTouch {
    fn level() -> u8 {
        53
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        18
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.5
    }

    fn can_use(sim: &impl Simulator) -> bool {
        Self::can_use_default(sim) && sim.is_in_good_condition()
    }

    fn use_success(sim: &mut impl Simulator) {
        Self::use_success_default(sim);
        sim.get_state_mut().effects.inner_quiet += 1;
        sim.consume_good_condition();
    }
}
