use crate::{gameplay_state::GameplayState, input::Input, menu_state::MenuState};

#[derive(Clone)]
pub enum State {
    MenuState(MenuState),
    GameplayState(GameplayState),
}

impl State {
    pub fn new() -> Self {
        Self::MenuState(MenuState::new())
    }

    pub fn step(&mut self, input: Input) {
        match self {
            State::MenuState(state) => {
                if let Some(gameplay_state) = state.step(input) {
                    *self = Self::GameplayState(gameplay_state)
                }
            }
            State::GameplayState(state) => state.step(input),
        }
    }
}
