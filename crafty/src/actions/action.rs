use crate::{action_states::ActionStates, simulator::Simulator};

pub trait ActionTrait: Sized {
    fn level() -> u8;

    fn increases_progress() -> bool {
        false
    }

    fn increases_quality() -> bool {
        false
    }

    fn durability_cost() -> i8 {
        10
    }

    fn increases_step_count() -> bool {
        true
    }

    fn cp_cost(sim: &impl Simulator) -> u32;

    fn efficiency(_: &impl Simulator) -> f32 {
        0.0
    }

    fn success_rate(_: &impl Simulator) -> f32 {
        1.0
    }

    fn can_use(sim: &impl Simulator) -> bool {
        Self::can_use_default(sim)
    }

    fn use_action(sim: &mut impl Simulator) {
        Self::use_action_default(sim)
    }

    fn use_success(sim: &mut impl Simulator) {
        Self::use_success_default(sim)
    }

    fn mutate_state(state: &mut ActionStates) {
        Self::mutate_state_default(state)
    }
}

pub(super) trait ActionImpl: ActionTrait {
    fn can_use_default(sim: &impl Simulator) -> bool {
        let state = sim.get_state();
        sim.at_crafter_level(Self::level()) && state.cp >= Self::cp_cost(sim)
    }

    fn use_success_default(sim: &mut impl Simulator) {
        let efficiency = Self::efficiency(sim);
        if efficiency != 0.0 {
            if Self::increases_progress() {
                sim.increase_progress(efficiency);
            }
            if Self::increases_quality() {
                sim.increase_quality(efficiency);
            }
        }
    }

    fn use_action_default(sim: &mut impl Simulator) {
        if sim.roll_success(Self::success_rate(sim)) {
            Self::use_success(sim);
        }

        sim.reduce_cp(Self::cp_cost(sim));
        sim.reduce_durability(Self::durability_cost());

        let state = sim.get_state();
        if state.durability > 0 && state.effects.manipulation > 0 {
            sim.restore_durability(5);
        }

        if Self::increases_step_count() {
            sim.get_state_mut().step += 1;
            sim.tick_condition();
        }

        let state = sim.get_state_mut();
        Self::mutate_state(&mut state.action_states);
        state.action += 1;

        state.effects.decrement_timers();
    }

    fn mutate_state_default(state: &mut ActionStates) {
        state.has_observed = false;
        state.touch_combo_step = 0;
    }
}

impl<A: ActionTrait> ActionImpl for A {}
