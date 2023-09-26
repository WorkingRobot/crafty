use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct TrainedEye;

impl ActionTrait for TrainedEye {
    fn level() -> u8 {
        80
    }

    fn increases_quality() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        250
    }

    fn can_use(sim: &impl Simulator) -> bool {
        let input = &sim.get_input();
        sim.is_first_step()
            && !input.is_expert
            && sim.at_crafter_level(input.recipe_job_level + 10)
            && Self::can_use_default(sim)
    }

    fn use_success(sim: &mut impl Simulator) {
        sim.increase_quality_raw(sim.get_input().quality_target - sim.get_state().quality)
    }
}
