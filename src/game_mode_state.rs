#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GameModeState {
    HandleGameplay = 2,
    HandleStartButton = 7,
    Unpause = 8,
}