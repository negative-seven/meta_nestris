use bitmask_enum::bitmask;

/// Flags to modify original game behaviors.
///
/// This type's only intended use is to be passed as a const generic to
/// [`State`](crate::state::State) and
/// [`MenuState`](crate::menu_state::MenuState).

#[bitmask(u8)]
pub enum Modifier {
    /// Allows the use of select+start to add 20 to the selected level number,
    /// much like the already-present A+start combo to add 10 to the selected
    /// level number.
    SelectAdds20Levels = (1 << 0),
}
