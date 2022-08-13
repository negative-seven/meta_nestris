use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameMode {
    CopyrightScreen = 0,
    TitleScreen = 1,
    GameTypeSelect = 2,
    LevelSelect = 3,
    Gameplay = 4,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
