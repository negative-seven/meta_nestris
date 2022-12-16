#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GameModeState {
    HandleGameplay,
    HandleStartButton,
    Unpause,
}
