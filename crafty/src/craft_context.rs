use crate::{Action, ActionSet, Player, Recipe};

#[derive(Debug, Clone)]
pub struct Input {
    pub player_job_level: u8,
    pub recipe_job_level: u8,
    /// Multiply by synthesis action efficiency for increase in progress
    pub progress_factor: f32,
    /// Multiply by touch action efficiency for increase in quality
    pub quality_factor: f32,
    pub action_max: u8,
    pub progress_target: u32,
    pub starting_quality: u32,
    pub quality_target: u32,
    pub durability_max: i8,
    pub cp_max: u32,
    pub is_expert: bool,
    pub can_use_manipulation: bool,
    pub is_specialist: bool,
    pub has_splendorous_tool: bool,
    pub action_pool: ActionSet,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CraftOptions {
    pub max_steps: u8,
    pub starting_quality: Option<u32>,
    pub quality_target: Option<u32>,
}

impl Input {
    #[allow(clippy::cast_precision_loss)]
    fn factors(player: &Player, recipe: &Recipe) -> (f32, f32) {
        // https://github.com/ffxiv-teamcraft/simulator/blob/72f4a6037baa3cd7cd78dfe34207283b824881a2/src/model/actions/crafting-action.ts#L176

        let progress_div = recipe.progress_divider as f32;
        let mut progress_factor: f32 = (player.craftsmanship * 10) as f32 / progress_div + 2.0;

        let quality_div = recipe.quality_divider as f32;
        let mut quality_factor: f32 = (player.control * 10) as f32 / quality_div + 35.0;

        if let Some(clvl) = player.clvl() {
            if clvl <= recipe.rlvl {
                progress_factor *= recipe.progress_modifier as f32 / 100.0;
                quality_factor *= recipe.quality_modifier as f32 / 100.0;
            }
        }

        (progress_factor.floor(), quality_factor.floor())
    }

    fn determine_action_pool(player: &Player, recipe: &Recipe) -> ActionSet {
        let mut pool = ActionSet::default();

        for action in Action::VALUES {
            if player.level >= action.level() {
                if action == &Action::TrainedEye && player.level.saturating_sub(recipe.level) < 10 {
                    continue;
                }

                pool.set(*action);
            }
        }

        pool
    }

    pub fn new(player: &Player, recipe: &Recipe, options: CraftOptions) -> Self {
        let (progress_factor, quality_factor) = Self::factors(player, recipe);
        Self {
            player_job_level: player.level,
            recipe_job_level: recipe.level,
            progress_factor,
            quality_factor,
            action_max: options.max_steps,
            progress_target: recipe.progress_max,
            starting_quality: options.starting_quality.unwrap_or(0),
            quality_target: options.quality_target.unwrap_or(recipe.quality_max),
            durability_max: recipe.durability_max,
            cp_max: player.cp,
            is_expert: recipe.is_expert,
            can_use_manipulation: player.can_use_manipulation,
            has_splendorous_tool: player.has_splendorous_tool,
            is_specialist: player.is_specialist,
            action_pool: Self::determine_action_pool(player, recipe),
        }
    }
}
