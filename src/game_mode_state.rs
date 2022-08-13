#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GameModeState {
    InitGameState = 1,
    HandleGameplay = 2,
    HandleStartButton = 7,
    Unpause = 8,
}