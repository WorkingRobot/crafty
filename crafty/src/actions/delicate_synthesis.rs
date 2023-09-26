use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct DelicateSynthesis;

impl ActionTrait for DelicateSynthesis {
    fn level() -> u8 {
        76
    }

    fn increases_progress() -> bool {
        true
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        32
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.0
    }
}
