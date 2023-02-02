use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuMode {
    CopyrightScreen,
    TitleScreen,
    GameTypeSelect,
    LevelSelect,
    InitializingGame,
}

impl Display for MenuMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
