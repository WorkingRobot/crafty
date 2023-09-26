use super::action::ActionTrait;
use crate::simulator::Simulator;

pub struct FinalAppraisal;

impl ActionTrait for FinalAppraisal {
    fn level() -> u8 {
        42
    }

    fn durability_cost() -> i8 {
        0
    }

    fn increases_step_count() -> bool {
        false
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        1
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.get_state().effects.final_appraisal = 5;
    }
}
