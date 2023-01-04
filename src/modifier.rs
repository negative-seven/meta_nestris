/// Options to modify game behavior.
///
/// This type's only intended use is as a const generic for
/// [`State`](crate::state::State),
/// [`MenuState`](crate::menu_state::MenuState) and
/// [`GameplayState`](crate::gameplay_state::GameplayState).

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Modifier {
    /// Allows for the use of select + start on the level selection screen to
    /// add 20 to the selected level number, alongside the A + start button
    /// combination from the original game.
    pub select_adds_20_levels: bool,
}

impl Modifier {
    pub const fn empty() -> Self {
        Self {
            select_adds_20_levels: false,
        }
    }
}
