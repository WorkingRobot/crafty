use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct HastyTouch;

impl ActionTrait for HastyTouch {
    fn level() -> u8 {
        9
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        0
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        1.0
    }

    fn success_rate(_: &impl Simulator) -> f32 {
        0.6
    }
}
