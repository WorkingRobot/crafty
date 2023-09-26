use crate::{Condition, Input, State};

pub trait Simulator {
    fn calculate_progress_gain(input: &Input, state: &State, efficiency: f32) -> u32 {
        let mut buff_modifier = 1.0;
        if state.effects.muscle_memory > 0 {
            buff_modifier += 1.0;
        }
        if state.effects.veneration > 0 {
            buff_modifier += 0.5;
        }
        let condition_modifier = match state.condition {
            Condition::Malleable => 1.5,
            _ => 1.0,
        };

        (input.progress_factor * efficiency * condition_modifier * buff_modifier) as u32
    }

    fn calculate_quality_gain(input: &Input, state: &State, efficiency: f32) -> u32 {
        let mut buff_modifier = 1.0;
        if state.effects.great_strides > 0 {
            buff_modifier += 1.0;
        }
        if state.effects.innovation > 0 {
            buff_modifier += 0.5;
        }

        let iq_modifier = 1.0 + state.effects.inner_quiet as f32 * 0.1;

        let condition_modifier = match state.condition {
            Condition::Poor => 0.5,
            Condition::Good if input.has_splendorous_tool => 1.75,
            Condition::Good => 1.5,
            Condition::Excellent => 4.0,
            _ => 1.0,
        };

        (input.quality_factor * efficiency * condition_modifier * iq_modifier * buff_modifier)
            as u32
    }

    fn calculate_cp_cost(state: &State, amount: u32) -> u32 {
        let mut amount = amount as f64;
        if state.condition == Condition::Pliant {
            amount /= 2.0;
        }
        amount.ceil() as u32
    }

    fn calculate_durability_cost(state: &State, amount: i8) -> i8 {
        let mut amount = amount as f64;
        if state.effects.is_waste_not_active() {
            amount /= 2.0;
        }
        if state.condition == Condition::Sturdy {
            amount /= 2.0;
        }
        amount.ceil() as i8
    }

    fn calculate_success_rate(state: &State, mut success_rate: f32) -> f32 {
        if state.condition == Condition::Centered {
            success_rate += 0.25;
        }
        success_rate.clamp(0.0, 1.0)
    }

    fn roll_condition(&mut self) -> Condition;

    fn roll_success_raw(&mut self, success_rate: f32) -> bool;

    //

    fn increase_progress(input: &Input, state: &mut State, efficiency: f32) {
        Self::increase_progress_raw(
            input,
            state,
            Self::calculate_progress_gain(input, state, efficiency),
        );
    }

    fn increase_progress_raw(input: &Input, state: &mut State, amount: u32) {
        let progress_max = input.progress_target;
        state.progress += amount;
        state.effects.muscle_memory = 0;
        if state.effects.final_appraisal > 0 && state.progress >= progress_max {
            state.progress = progress_max - 1;
            state.effects.final_appraisal = 0;
        }
    }

    fn increase_quality(input: &Input, state: &mut State, efficiency: f32) {
        Self::increase_quality_raw(
            input,
            state,
            Self::calculate_quality_gain(input, state, efficiency),
        );
    }

    fn increase_quality_raw(input: &Input, state: &mut State, amount: u32) {
        state.quality += amount;
        state.effects.great_strides = 0;
        if Self::at_crafter_level(input, 11) {
            state.effects.inner_quiet += 1;
        }
    }

    fn reduce_cp(input: &Input, state: &mut State, amount: u32) {
        Self::reduce_cp_raw(state, Self::calculate_cp_cost(state, amount));
    }

    fn reduce_cp_raw(state: &mut State, amount: u32) {
        state.cp -= amount;
    }

    fn reduce_durability(input: &Input, state: &mut State, amount: i8) {
        Self::reduce_durability_raw(state, Self::calculate_durability_cost(state, amount));
    }

    fn reduce_durability_raw(state: &mut State, amount: i8) {
        state.durability -= amount;
    }

    fn restore_cp(input: &Input, state: &mut State, amount: u32) {
        state.cp = (state.cp + amount).min(input.cp_max);
    }

    fn restore_durability(input: &Input, state: &mut State, amount: i8) {
        state.durability = (state.durability + amount).min(input.durability_max);
    }

    fn tick_condition(&mut self, state: &mut State) {
        state.condition = match state.condition {
            Condition::Poor => Condition::Normal,
            Condition::Good => Condition::Normal,
            Condition::Excellent => Condition::Excellent,
            Condition::GoodOmen => Condition::Good,
            _ => self.roll_condition(),
        }
    }

    fn roll_success(&mut self, success_rate: f32) -> bool {
        self.roll_success_raw(self.calculate_success_rate(success_rate))
    }

    fn at_crafter_level(input: &Input, level: u8) -> bool {
        input.player_job_level >= level
    }

    fn is_first_step(&self) -> bool {
        self.get_state().step == 0
    }

    fn is_in_good_condition(&self) -> bool {
        match self.get_state().condition {
            Condition::Good | Condition::Excellent => true,
            _ => self.get_state().effects.heart_and_soul,
        }
    }

    fn consume_good_condition(&mut self) {
        match self.get_state().condition {
            Condition::Good | Condition::Excellent => (),
            _ => self.get_state_mut().effects.heart_and_soul = false,
        }
    }
}
