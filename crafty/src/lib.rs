#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::enum_glob_use)]

mod action_set;
mod action_states;
mod actions;
mod condition;
mod craft_context;
mod craft_state;
mod executor;
mod intrinsics;
mod player;
mod recipe;
mod simulator;
mod simulator_no_rng;
mod solver;
mod tree;

use action_set::ActionSet;
pub use actions::Action;
pub use condition::Condition;
pub use craft_context::{CraftOptions, Input};
pub use craft_state::{CraftResult, Effects, State};
pub use player::Player;
pub use recipe::Recipe;
pub use solver::{SearchOptions, Solver};
