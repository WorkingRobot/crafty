use crate::{simulator::Simulator, Action, Input, State};

#[derive(PartialEq)]
pub enum CompletionState {
    Incomplete,
    ProgressComplete,
    NoMoreDurability,

    InvalidAction,
    MaxActionCountReached,
    NoMoreActions,
}

#[derive(PartialEq)]
pub enum ActionResponse {
    UsedAction,
    SynthesisAlreadyComplete,
    ActionNotUnlocked,
    NotEnoughCP,
    NoDurability,
    CannotUseAction,
}

pub trait Executor: Simulator + Sized {
    fn is_complete(&self) -> bool {
        self.get_completion_state() != CompletionState::Incomplete
    }

    fn get_completion_state(&self) -> CompletionState {
        let state = self.get_state();
        if state.progress >= self.get_input().progress_target {
            CompletionState::ProgressComplete
        } else if state.durability == 0 {
            CompletionState::NoMoreDurability
        } else {
            CompletionState::Incomplete
        }
    }

    fn set_state(&mut self, state: State) {
        *self.get_state_mut() = state;
    }

    fn set_input(&mut self, input: Input);

    fn execute(&mut self, action: Action) -> ActionResponse {
        if self.is_complete() {
            return ActionResponse::SynthesisAlreadyComplete;
        }

        if !action.can_use(self) {
            if !self.at_crafter_level(action.level())
                || (action == Action::Manipulation && !self.get_input().can_use_manipulation)
            {
                ActionResponse::ActionNotUnlocked
            } else if action.cp_cost(self) > self.get_state().cp {
                ActionResponse::NotEnoughCP
            } else {
                ActionResponse::CannotUseAction
            }
        } else {
            action.use_action(self);
            ActionResponse::UsedAction
        }
    }

    fn execute_on(&mut self, state: &mut State, action: Action) -> ActionResponse {
        self.set_state(state.clone());
        let response = self.execute(action);
        *state = self.get_state().clone();
        response
    }
}
