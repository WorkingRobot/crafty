use crate::{
    action_states::ActionStates, simulator::Simulator, Action, ActionSet, Condition, Input,
};
use std::fmt;

#[derive(Debug)]
pub enum CraftResult {
    /// The craft reached 100% progress. Includes the score of the `CraftState`.
    Finished(f32),
    /// No durability remains.
    DurabilityFailure,
    /// The step limit was reached.
    MaxStepsFailure,
    /// No actions are available, or an invalid action was used.
    InvalidActionFailure,
}

#[derive(Default, Debug, Clone)]
pub struct Effects {
    pub inner_quiet: u8,
    pub waste_not: u8,
    pub waste_not_ii: u8,
    pub manipulation: u8,
    pub great_strides: u8,
    pub innovation: u8,
    pub veneration: u8,
    pub muscle_memory: u8,
    pub final_appraisal: u8,
    pub heart_and_soul: bool,
}

impl Effects {
    /// Decrements all buff timers by 1 step
    pub fn decrement_timers(&mut self) {
        // don't decrement inner quiet
        self.waste_not = self.waste_not.saturating_sub(1);
        self.waste_not_ii = self.waste_not_ii.saturating_sub(1);
        self.manipulation = self.manipulation.saturating_sub(1);
        self.great_strides = self.great_strides.saturating_sub(1);
        self.innovation = self.innovation.saturating_sub(1);
        self.veneration = self.veneration.saturating_sub(1);
        self.muscle_memory = self.muscle_memory.saturating_sub(1);
    }

    pub fn is_waste_not_active(&self) -> bool {
        self.waste_not > 0 || self.waste_not_ii > 0
    }
}

#[derive(Default, Debug, Clone)]
pub struct State {
    pub step: u8,
    pub action: u8,
    pub progress: u32,
    pub quality: u32,
    pub durability: i8,
    pub cp: u32,
    pub condition: Condition,
    pub action_states: ActionStates,
    pub effects: Effects,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>5} progress | {:>5} quality | {:>2} durability | {:>3} cp",
            self.progress, self.quality, self.durability, self.cp,
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct Node {
    pub state: State,

    /// The action that led to this state
    pub action: Option<Action>,
    /// Sum of scores from this node onward
    pub score_sum: f32,
    /// Maximum score that can be obtained by following this node
    pub max_score: f32,
    /// Number of times this node has been visited
    pub visits: u32,
    pub available_moves: ActionSet,
}

impl Node {
    pub fn new(context: &Input, state: State) -> Self {
        let mut ret = Self::default();
        ret.set_available_moves(context, false);
        ret
    }

    /// Examine the current craft state and populate `available_moves`.
    /// Enabling `strict` will add more rules that aim to prune as many
    /// suboptimal moves as possible.
    #[allow(clippy::too_many_lines)]
    fn set_available_moves(&mut self, sim: &impl Simulator, strict: bool) -> &mut Self {
        let context = sim.get_input();
        let state = sim.get_state();

        if state.progress >= context.progress_target
            || state.action >= context.action_max
            || state.durability <= 0
        {
            return self;
        }

        let mut available_moves = context.action_pool.clone();
        available_moves.keep(|action| {
            use Action::*;

            if action.cp_cost(sim) > state.cp {
                return false;
            }

            // don't allow quality moves at max quality
            if state.quality >= context.quality_target && action.increases_quality() {
                return false;
            }

            if strict {
                // always used Trained Eye if it's available
                if sim.is_first_step()
                    && context.quality_target > 0
                    && !context.is_expert
                    && context.action_pool.contains(TrainedEye)
                {
                    return action == &TrainedEye;
                }

                // only allow Focused moves after Observe
                if state.action_states.has_observed
                    && action != &FocusedSynthesis
                    && action != &FocusedTouch
                {
                    return false;
                }

                // don't allow quality moves under Muscle Memory for difficult crafts
                if context.recipe_job_level == 90
                    && state.effects.muscle_memory > 0
                    && action.increases_quality()
                {
                    return false;
                }

                // don't allow pure quality moves under Veneration
                if state.effects.veneration > 0
                    && !action.increases_progress()
                    && action.increases_quality()
                {
                    return false;
                }

                if action.increases_progress() {
                    let progress_eff = action.efficiency(sim);
                    let progress_increase = sim.calculate_progress_gain(progress_eff);
                    let would_finish =
                        state.progress + progress_increase >= context.progress_target;

                    if would_finish {
                        // don't allow finishing the craft if there is significant quality remaining
                        if state.quality < context.quality_target / 5 {
                            return false;
                        }
                    } else {
                        // don't allow pure progress moves under Innovation, if it wouldn't finish the craft
                        if state.effects.innovation > 0
                            && !action.increases_quality()
                            && action.increases_progress()
                        {
                            return false;
                        }
                    }
                }
            }

            if !action.can_use(sim) {
                return false;
            }

            match action {
                ByregotsBlessing if strict => state.effects.inner_quiet > 1,
                TrainedFinesse => state.effects.inner_quiet == 10,
                // use of Waste Not should be efficient
                PrudentSynthesis | PrudentTouch | WasteNot | WasteNot2 if strict => {
                    state.effects.waste_not == 0 && state.effects.waste_not_ii == 0
                }
                PrudentSynthesis | PrudentTouch => {
                    state.effects.waste_not == 0 && state.effects.waste_not_ii == 0
                }
                // don't allow Observe if observing; should also have enough CP to follow up
                Observe if strict => !state.action_states.has_observed && state.cp >= 5,
                Observe => !state.action_states.has_observed,
                // only allow focused skills if observing
                FocusedSynthesis | FocusedTouch => state.action_states.has_observed,
                // don't allow Groundwork if it's downgraded
                Groundwork => {
                    let cost = sim.calculate_durability_cost(action.durability_cost());
                    state.durability >= cost
                }
                // don't allow buffs too early
                MastersMend if strict => context.durability_max - state.durability >= 25,
                Manipulation if strict => state.effects.manipulation == 0,
                GreatStrides if strict => state.effects.great_strides == 0,
                Veneration | Innovation if strict => {
                    state.effects.veneration <= 1 && state.effects.innovation <= 1
                }
                _ => true,
            }
        });
        self.available_moves = available_moves;

        self
    }
}

// impl Node {
//     pub fn _new(context: &Input) -> Self {
//         Self {
//             state: State {
//                 quality: context.starting_quality,
//                 durability: context.durability_max,
//                 cp: context.cp_max,
//                 ..Default::default()
//             },
//             ..Default::default()
//         }
//     }

//     pub fn new(context: &Input) -> Self {
//         let mut state = Self::_new(context);
//         state.set_available_moves(context, false);
//         state
//     }

//     pub fn new_strict(context: &Input) -> Self {
//         let mut state = Self::_new(context);
//         state.set_available_moves(context, true);
//         state
//     }

//     pub fn clone_strict(&self, context: &Input) -> Self {
//         let mut state = self.clone();
//         state.set_available_moves(context, true);
//         state
//     }

//     /// Examine the current craft state and populate `available_moves`.
//     /// Enabling `strict` will add more rules that aim to prune as many
//     /// suboptimal moves as possible.
//     #[allow(clippy::too_many_lines)]
//     fn set_available_moves(&mut self, sim: &impl Simulator, strict: bool) -> &mut Self {
//         let context = sim.get_input();
//         let state = sim.get_state();

//         if state.progress >= context.progress_target
//             || state.action >= context.action_max
//             || state.durability <= 0
//         {
//             return self;
//         }

//         let mut available_moves = context.action_pool.clone();
//         available_moves.keep(|action| {
//             use Action::*;

//             if action.cp_cost(sim) > state.cp {
//                 return false;
//             }

//             // don't allow quality moves at max quality
//             if state.quality >= context.quality_target && action.increases_quality() {
//                 return false;
//             }

//             if strict {
//                 // always used Trained Eye if it's available
//                 if sim.is_first_step()
//                     && context.quality_target > 0
//                     && !context.is_expert
//                     && context.action_pool.contains(TrainedEye)
//                 {
//                     return action == &TrainedEye;
//                 }

//                 // only allow Focused moves after Observe
//                 if state.action_states.has_observed
//                     && action != &FocusedSynthesis
//                     && action != &FocusedTouch
//                 {
//                     return false;
//                 }

//                 // don't allow quality moves under Muscle Memory for difficult crafts
//                 if context.recipe_job_level == 90
//                     && state.effects.muscle_memory > 0
//                     && action.increases_quality()
//                 {
//                     return false;
//                 }

//                 // don't allow pure quality moves under Veneration
//                 if state.effects.veneration > 0
//                     && !action.increases_progress()
//                     && action.increases_quality()
//                 {
//                     return false;
//                 }

//                 if action.increases_progress() {
//                     let progress_eff = action.efficiency(sim);
//                     let progress_increase = sim.calculate_progress_gain(progress_eff);
//                     let would_finish =
//                         state.progress + progress_increase >= context.progress_target;

//                     if would_finish {
//                         // don't allow finishing the craft if there is significant quality remaining
//                         if state.quality < context.quality_target / 5 {
//                             return false;
//                         }
//                     } else {
//                         // don't allow pure progress moves under Innovation, if it wouldn't finish the craft
//                         if state.effects.innovation > 0
//                             && !action.increases_quality()
//                             && action.increases_progress()
//                         {
//                             return false;
//                         }
//                     }
//                 }
//             }

//             if !action.can_use(sim) {
//                 return false;
//             }

//             match action {
//                 ByregotsBlessing if strict => state.effects.inner_quiet > 1,
//                 TrainedFinesse => state.effects.inner_quiet == 10,
//                 // use of Waste Not should be efficient
//                 PrudentSynthesis | PrudentTouch | WasteNot | WasteNot2 if strict => {
//                     state.effects.waste_not == 0 && state.effects.waste_not_ii == 0
//                 }
//                 PrudentSynthesis | PrudentTouch => {
//                     state.effects.waste_not == 0 && state.effects.waste_not_ii == 0
//                 }
//                 // don't allow Observe if observing; should also have enough CP to follow up
//                 Observe if strict => !state.action_states.has_observed && state.cp >= 5,
//                 Observe => !state.action_states.has_observed,
//                 // only allow focused skills if observing
//                 FocusedSynthesis | FocusedTouch => state.action_states.has_observed,
//                 // don't allow Groundwork if it's downgraded
//                 Groundwork => {
//                     let cost = sim.calculate_durability_cost(action.durability_cost());
//                     state.durability >= cost
//                 }
//                 // don't allow buffs too early
//                 MastersMend if strict => context.durability_max - state.durability >= 25,
//                 Manipulation if strict => state.effects.manipulation == 0,
//                 GreatStrides if strict => state.effects.great_strides == 0,
//                 Veneration | Innovation if strict => {
//                     state.effects.veneration <= 1 && state.effects.innovation <= 1
//                 }
//                 _ => true,
//             }
//         });
//         self.available_moves = available_moves;

//         self
//     }

//     /// Executes the action against a `CraftState`, and returns a new `CraftState`
//     pub fn execute(&self, action: &Action, strict: bool) -> Self {
//         let mut state = self._execute(action);
//         state.set_available_moves(strict);
//         state
//     }

//     /// An evaluation of the craft. Returns a value from 0 to 1.
//     #[allow(clippy::cast_precision_loss)]
//     pub fn score(&self) -> f32 {
//         fn apply(bonus: f32, value: f32, target: f32) -> f32 {
//             bonus * 1f32.min(value / target)
//         }

//         // bonuses should add up to 1.0

//         // The search only expands on finished states (100% progress) so you may
//         // be thinking, "Why do we need to reward progress if we don't score
//         // unfinished craft states at all?". Two reasons:
//         // 1) Conceptually, I think the progress bonus is still useful as a
//         //    weight against the other bonuses
//         // 2) Practically, it ensures the score of a state is sufficiently above
//         //    zero without having to rely solely on durability, cp, and step
//         //    metrics, which by themselves could provide a bad signal.
//         let progress_bonus = 0.20;
//         let quality_bonus = 0.65;
//         let durability_bonus = 0.05;
//         let cp_bonus = 0.05;
//         let fewer_steps_bonus = 0.05;

//         let progress_score = apply(
//             progress_bonus,
//             self.progress as f32,
//             self.context.progress_target as f32,
//         );

//         let quality_score = apply(
//             quality_bonus,
//             self.quality as f32,
//             self.context.quality_target as f32,
//         );

//         let durability_score = apply(
//             durability_bonus,
//             f32::from(self.durability),
//             f32::from(self.context.durability_max),
//         );

//         let cp_score = apply(cp_bonus, self.cp as f32, self.context.cp_max as f32);

//         let fewer_steps_score =
//             fewer_steps_bonus * (1.0_f32 - f32::from(self.step) / f32::from(self.context.step_max));

//         progress_score + quality_score + durability_score + cp_score + fewer_steps_score
//     }

//     /// Evaluates the craft based on step count since quality doesn't matter.
//     /// Returns a value from 0 to 1.
//     #[allow(clippy::cast_precision_loss)]
//     pub fn score_no_quality(&self) -> f32 {
//         1.0_f32 - f32::from(self.step) / f32::from(self.context.step_max)
//     }

//     pub fn check_result(&self) -> Option<CraftResult> {
//         if self.progress >= self.context.progress_target {
//             let score = if self.context.quality_target > 0 {
//                 self.score()
//             } else {
//                 self.score_no_quality()
//             };
//             Some(CraftResult::Finished(score))
//         } else if self.durability <= 0 {
//             Some(CraftResult::DurabilityFailure)
//         } else if self.step >= self.context.step_max {
//             Some(CraftResult::MaxStepsFailure)
//         } else if self.available_moves.is_empty() {
//             Some(CraftResult::InvalidActionFailure)
//         } else {
//             None
//         }
//     }
// }
