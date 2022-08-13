#[derive(Clone, Eq, PartialEq)]
pub enum PlayState {
    MoveTetrimino = 1,
    LockTetrimino = 2,
    CheckForCompletedRows = 3,
    DoNothing = 4,
    UpdateLinesAndStatistics = 5,
    SkipTo7 = 6,
    SkipTo8 = 7,
    SpawnNextTetrimino = 8,
}
