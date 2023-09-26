use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct ByregotsBlessing;

impl ActionTrait for ByregotsBlessing {
    fn level() -> u8 {
        50
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        24
    }

    fn efficiency(sim: &impl Simulator) -> f32 {
        1.0 + 0.2 * sim.get_state().effects.inner_quiet as f32
    }

    fn can_use(sim: &impl Simulator) -> bool {
        Self::can_use_default(sim) && sim.get_state().effects.inner_quiet > 0
    }

    fn use_success(sim: &mut impl Simulator) {
        Self::use_success_default(sim);
        sim.get_state_mut().effects.inner_quiet = 0;
    }
}
