#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PlayState {
    MoveTetrimino,
    LockTetrimino,
    CheckForCompletedRows,
    DoNothing,
    UpdateLinesAndStatistics,
    Wait2UntilSpawnNextTetrimino,
    Wait1UntilSpawnNextTetrimino,
    SpawnNextTetrimino,
}
