use super::action::{ActionImpl, ActionTrait};
use crate::simulator::Simulator;

pub struct Manipulation;

impl ActionTrait for Manipulation {
    fn level() -> u8 {
        65
    }

    fn durability_cost() -> i8 {
        0
    }

    fn cp_cost(_: &impl Simulator) -> u32 {
        96
    }

    fn can_use(sim: &impl Simulator) -> bool {
        sim.get_input().can_use_manipulation && Self::can_use_default(sim)
    }

    fn use_action(sim: &mut impl Simulator) {
        sim.get_state_mut().effects.manipulation = 8;

        sim.reduce_cp(Self::cp_cost(sim));

        // same as base.Use(s), but manipulation effect never kicks in, even if manip is active before

        sim.get_state_mut().step += 1;
        sim.tick_condition();

        Self::mutate_state(&mut sim.get_state_mut().action_states);
        sim.get_state_mut().action += 1;

        sim.get_state_mut().effects.decrement_timers();
    }
}
