#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameModeState {
    HandleGameplay,
    HandleStartButton,
    Unpause,
}
