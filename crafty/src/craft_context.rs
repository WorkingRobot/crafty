use crate::{Action, ActionSet, Player, Recipe};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct CraftContext {
    pub player_job_level: u32,
    pub recipe_job_level: u32,
    /// Multiply by synthesis action efficiency for increase in progress
    pub progress_factor: f32,
    /// Multiply by touch action efficiency for increase in quality
    pub quality_factor: f32,
    pub step_max: u8,
    pub progress_target: u32,
    pub starting_quality: u32,
    pub quality_target: u32,
    pub durability_max: i8,
    pub cp_max: u32,
    pub is_expert: bool,
    pub action_pool: ActionSet,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct CraftOptions {
    pub max_steps: u8,
    pub starting_quality: Option<u32>,
    pub quality_target: Option<u32>,
}

fn get_player_clvl(_player_job_level: u32) -> Option<u32> {
    unimplemented!()
}

impl CraftContext {
    #[allow(clippy::cast_precision_loss)]
    fn factors(player: &Player, recipe: &Recipe) -> (f32, f32) {
        // https://github.com/ffxiv-teamcraft/simulator/blob/72f4a6037baa3cd7cd78dfe34207283b824881a2/src/model/actions/crafting-action.ts#L176

        let progress_div = recipe.progress_div as f32;
        let mut progress_factor: f32 = (player.craftsmanship * 10) as f32 / progress_div + 2.0;

        let quality_div = recipe.quality_div as f32;
        let mut quality_factor: f32 = (player.control * 10) as f32 / quality_div + 35.0;

        if let Some(base_recipe_level) = get_player_clvl(player.job_level) {
            if base_recipe_level <= recipe.recipe_level {
                progress_factor *= recipe.progress_mod as f32 / 100.0;
                quality_factor *= recipe.quality_mod as f32 / 100.0;
            }
        }

        (progress_factor.floor(), quality_factor.floor())
    }

    fn determine_action_pool(player: &Player, recipe: &Recipe) -> ActionSet {
        let mut pool = ActionSet::new();

        for action in Action::ACTIONS {
            let attrs = action.attributes();
            if player.job_level >= attrs.level && player.cp >= attrs.cp_cost.unwrap_or(0) {
                if action == &Action::TrainedEye
                    && player.job_level.saturating_sub(recipe.job_level) < 10
                {
                    continue;
                }

                pool.set(*action);
            }
        }

        {
            use Action::*;
            if pool.contains(BasicSynthesisTraited) && pool.contains(BasicSynthesis) {
                pool.unset(BasicSynthesis);
            }
            if pool.contains(CarefulSynthesisTraited) && pool.contains(CarefulSynthesis) {
                pool.unset(CarefulSynthesis);
            }
            if pool.contains(GroundworkTraited) && pool.contains(Groundwork) {
                pool.unset(Groundwork);
            }
        }

        pool
    }

    pub fn new(player: &Player, recipe: &Recipe, options: CraftOptions) -> Self {
        let (progress_factor, quality_factor) = Self::factors(player, recipe);
        Self {
            player_job_level: player.job_level,
            recipe_job_level: recipe.job_level,
            progress_factor,
            quality_factor,
            step_max: options.max_steps,
            progress_target: recipe.progress,
            starting_quality: options.starting_quality.unwrap_or(0),
            quality_target: options.quality_target.unwrap_or(recipe.quality),
            durability_max: recipe.durability,
            cp_max: player.cp,
            is_expert: recipe.is_expert,
            action_pool: Self::determine_action_pool(player, recipe),
        }
    }
}
