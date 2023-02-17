use crate::{
    gameplay_state::GameplayState, input::Input, menu_state::MenuState, modifier::Modifier,
};

/// A general state of the game.
///
/// This type is a simple wrapper for either a [`MenuState`] or a
/// [`GameplayState`]. It automatically handles transformations between the two
/// when applicable.
///
/// The `MODIFIER` const generic specifies game modifiers - see [`Modifier`] for
/// supported modifiers.
#[derive(Clone, Debug)]
pub enum State<const MODIFIER: Modifier> {
    MenuState(MenuState<MODIFIER>),
    GameplayState(GameplayState<MODIFIER>),
}

impl State<{ Modifier::empty() }> {
    /// Creates a `State` with an "empty" [`Modifier`].
    ///
    /// Equivalent to `State::<{ Modifier::empty() }>::new_with_modifier`.
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_modifier()
    }
}

impl<const MODIFIER: Modifier> State<MODIFIER> {
    /// Creates a `State` with a [`Modifier`].
    ///
    /// Example:
    /// ```
    /// use meta_nestris::{Modifier, State};
    ///
    /// const MODIFIER: Modifier = Modifier {
    ///     uncapped_score: true,
    ///     ..Modifier::empty()
    /// };
    ///
    /// // both equivalent:
    /// let state_a = State::<MODIFIER>::new_with_modifier();
    /// let state_b: State<MODIFIER> = State::new_with_modifier();
    /// ```
    #[must_use]
    pub fn new_with_modifier() -> Self {
        Self::MenuState(MenuState::new_with_modifier())
    }

    /// Steps to the next state.
    pub fn step(&mut self, input: Input) {
        match self {
            State::MenuState(state) => {
                if let Some(gameplay_state) = state.step(input) {
                    *self = Self::GameplayState(gameplay_state);
                }
            }
            State::GameplayState(state) => state.step(input),
        }
    }
}

impl<const MODIFIER: Modifier> Default for State<MODIFIER> {
    fn default() -> Self {
        Self::new_with_modifier()
    }
}
