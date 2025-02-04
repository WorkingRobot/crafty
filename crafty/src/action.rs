use crate::CraftState;
use enumflags2::bitflags;
use std::{cmp, fmt};

pub struct Attributes {
    pub level: u32,
    pub progress_efficiency: Option<f32>,
    pub quality_efficiency: Option<f32>,
    pub durability_cost: Option<i8>,
    pub cp_cost: Option<u32>,
    pub effect: Option<fn(&mut CraftState)>,
}

macro_rules! optional {
    () => {
        None
    };
    ($e:expr) => {
        Some($e)
    };
}

macro_rules! create_actions {
    (
        $(
            [$action_name:ident, $label:expr]
                level $level:expr,
                $(progress $progress:expr,)?
                $(quality $quality:expr,)?
                $(durability $durability:expr,)?
                $(cp $cp:expr,)?
                $(effect $effect:expr,)?
        )+ $(,)?
    ) => {
        #[bitflags]
        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Action {
            $($action_name,)*
        }

        impl Action {
            pub const ACTIONS: &'static [Action] = &[
                $(Action::$action_name,)*
            ];

            pub fn attributes(&self) -> Attributes {
                match *self {
                    $(
                        Action::$action_name => Attributes {
                            level: $level,
                            progress_efficiency: optional!($( $progress )?),
                            quality_efficiency: optional!($( $quality )?),
                            durability_cost: optional!($( $durability )?),
                            cp_cost: optional!($( $cp )?),
                            effect: optional!($( $effect )?),
                        },
                    )*
                }
            }

            pub fn name(&self) -> &'static str {
                match *self {
                    $(Action::$action_name => stringify!($action_name),)*
                }
            }

            pub fn label(&self) -> &'static str {
                match *self {
                    $(Action::$action_name => $label,)*
                }
            }
        }

        #[derive(Debug)]
        pub struct ActionParseError;

        impl std::str::FromStr for Action {
            type Err = ActionParseError;

            fn from_str(s: &str) -> Result<Action, ActionParseError> {
                match s {
                    $(stringify!($action_name) => Ok(Action::$action_name),)*
                    _ => Err(ActionParseError)
                }
            }
        }
    };
}

// https://na.finalfantasyxiv.com/crafting_gathering_guide/carpenter/
create_actions!(
    [BasicSynthesis, "Basic Synthesis"]
        level 1,
        progress 1.0,
        durability 10,
    [BasicTouch, "Basic Touch"]
        level 5,
        quality 1.0,
        durability 10,
        cp 18,
        effect |state| {
            state.next_combo_action = Some(Action::StandardTouch);
        },
    [MastersMend, "Master's Mend"]
        level 7,
        durability 0,  // indicates that this move is not a buff
        cp 88,
        effect |state| {
            state.durability = cmp::min(state.durability + 30, state.context.durability_max);
        },
    // HastyTouch
    // RapidSynthesis
    [Observe, "Observe"]
        level 13,
        durability 0,  // indicates that this move is not a buff
        cp 7,
        effect |state| {
            state.observe = true;
        },
    // TricksOfTheTrade
    [WasteNot, "Waste Not"]
        level 15,
        cp 56,
        effect |state| {
            state.buffs.waste_not = 4;
            state.buffs.waste_not_ii = 0;
        },
    [Veneration, "Veneration"]
        level 15,
        cp 18,
        effect |state| {
            state.buffs.veneration = 4;
        },
    [StandardTouch, "Standard Touch"]
        level 18,
        quality 1.25,
        durability 10,
        cp 32,
        effect |state| {
            if state.next_combo_action == Some(Action::StandardTouch) {
                state.next_combo_action = Some(Action::AdvancedTouch);
            }
        },
    [GreatStrides, "Great Strides"]
        level 21,
        cp 32,
        effect |state| {
            state.buffs.great_strides = 3;
        },
    [Innovation, "Innovation"]
        level 26,
        cp 18,
        effect |state| {
            state.buffs.innovation = 4;
        },
    [BasicSynthesisTraited, "Basic Synthesis"]
        level 31,
        progress 1.2,
        durability 10,
    // FinalAppraisal
    [WasteNotII, "Waste Not II"]
        level 47,
        cp 98,
        effect |state| {
            state.buffs.waste_not = 0;
            state.buffs.waste_not_ii = 8;
        },
    [ByregotsBlessing, "Byregot's Blessing"]
        level 50,
        quality 0.0,  // a placeholder to indicate this action *does* affect quality
        durability 10,
        cp 24,
    // PreciseTouch
    [MuscleMemory, "Muscle Memory"]
        level 54,
        progress 3.0,
        durability 10,
        cp 6,
        effect |state| {
            state.buffs.muscle_memory = 5;
        },
    [CarefulSynthesis, "Careful Synthesis"]
        level 62,
        progress 1.5,
        durability 10,
        cp 7,
    [Manipulation, "Manipulation"]
        level 65,
        cp 96,
        effect |state| {
            state.buffs.manipulation = 8;
        },
    [PrudentTouch, "Prudent Touch"]
        level 66,
        quality 1.0,
        durability 5,
        cp 25,
    [FocusedSynthesis, "Focused Synthesis"]
        level 67,
        progress 2.0,
        durability 10,
        cp 5,
    [FocusedTouch, "Focused Touch"]
        level 68,
        quality 1.5,
        durability 10,
        cp 18,
    [Reflect, "Reflect"]
        level 69,
        quality 1.0,
        durability 10,
        cp 6,
    [PreparatoryTouch, "Preparatory Touch"]
        level 71,
        quality 2.0,
        durability 20,
        cp 40,
    [Groundwork, "Groundwork"]
        level 72,
        progress 3.0,
        durability 20,
        cp 18,
    [DelicateSynthesis, "Delicate Synthesis"]
        level 76,
        progress 1.0,
        quality 1.0,
        durability 10,
        cp 32,
    // Intensive Synthesis
    [TrainedEye, "Trained Eye"]
        level 80,
        quality 0.0, // a placeholder to indicate this action *does* affect quality
        durability 0,
        cp 250,
    [CarefulSynthesisTraited, "Careful Synthesis"]
        level 82,
        progress 1.8,
        durability 10,
        cp 7,
    [AdvancedTouch, "Advanced Touch"]
        level 84,
        quality 1.5,
        durability 10,
        cp 46,
        effect |state| {
            state.next_combo_action = None;
        },
    [GroundworkTraited, "Groundwork"]
        level 86,
        progress 3.6,
        durability 20,
        cp 18,
    [PrudentSynthesis, "Prudent Synthesis"]
        level 88,
        progress 1.8,
        durability 5,
        cp 18,
    [TrainedFinesse, "Trained Finesse"]
        level 90,
        quality 1.0,
        cp 32,
);

impl Action {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn calc_progress_increase(state: &CraftState, efficiency: f32) -> u32 {
        let base = state.context.progress_factor;

        let mut multiplier = 1.0;
        if state.buffs.veneration > 0 {
            multiplier += 0.5;
        }
        if state.buffs.muscle_memory > 0 {
            multiplier += 1.0;
        }

        (base * efficiency * multiplier).floor() as u32
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn calc_quality_increase(state: &CraftState, efficiency: f32) -> u32 {
        if state.action == Some(Action::TrainedEye) {
            return state.context.quality_target - state.quality;
        }

        let base = state.context.quality_factor;

        let mut efficiency = efficiency;

        if state.action == Some(Action::ByregotsBlessing) {
            efficiency = 1.0 + f32::from(state.buffs.inner_quiet) * 0.2;
        }

        let mut modifier = 1.0 + f32::from(state.buffs.inner_quiet) / 10.0;

        let mut multiplier = 1.0;
        if state.buffs.innovation > 0 {
            multiplier += 0.5;
        }
        if state.buffs.great_strides > 0 {
            multiplier += 1.0;
        }

        modifier *= multiplier;

        (base * efficiency * modifier).floor() as u32
    }

    pub fn calc_durability_cost(state: &CraftState, base_cost: i8) -> i8 {
        if state.buffs.waste_not > 0 || state.buffs.waste_not_ii > 0 {
            return base_cost / 2;
        }
        base_cost
    }

    pub fn calc_cp_cost(state: &CraftState, base_cost: u32) -> u32 {
        // test for basic touch combo
        if state.action.is_some() && state.action == state.next_combo_action {
            return 18;
        }
        base_cost
    }

    pub fn macro_text(&self) -> String {
        let mut label = self.label().to_string();
        if label.contains(' ') {
            label = format!("\"{label}\"");
        }

        let attrs = self.attributes();
        let is_buff = attrs.progress_efficiency.is_none()
            && attrs.quality_efficiency.is_none()
            && attrs.durability_cost.is_none();
        let wait_time = if is_buff { 2 } else { 3 };

        format!("/ac {label} <wait.{wait_time}>")
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}
