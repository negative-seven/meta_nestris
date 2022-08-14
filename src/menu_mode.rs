use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuMode {
    CopyrightScreen = 0,
    TitleScreen = 1,
    GameTypeSelect = 2,
    LevelSelect = 3,
    InitializingGame = 4,
}

impl Display for MenuMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
