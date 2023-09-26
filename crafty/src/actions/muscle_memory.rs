use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct MuscleMemory;

impl ActionTrait for MuscleMemory {
    fn level() -> u8 {
        54
    }

    fn increases_progress() -> bool {
        true
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        6
    }

    fn efficiency(_: &impl Simulator) -> f32 {
        3.0
    }

    fn can_use(sim: &impl Simulator) -> bool {
        sim.is_first_step() && Self::can_use_default(sim)
    }

    fn use_success(sim: &mut impl Simulator) {
        Self::use_success_default(sim);
        sim.get_state_mut().effects.muscle_memory = 5;
    }
}
