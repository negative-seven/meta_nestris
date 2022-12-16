use crate::{gameplay_state::GameplayState, input::Input, menu_state::MenuState};

/// A general state of the game.
///
/// This type is a simple wrapper for either a
/// [`MenuState`] or a
/// [`GameplayState`]. It automatically
/// handles transformations between the two when applicable.
#[derive(Clone)]
pub enum State {
    MenuState(MenuState),
    GameplayState(GameplayState),
}

impl State {
    pub fn new() -> Self {
        Self::MenuState(MenuState::new())
    }

    /// Steps to the next state.
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
