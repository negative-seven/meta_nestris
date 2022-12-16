#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayState {
    MoveTetrimino,
    LockTetrimino,
    CheckForCompletedRows,
    DoNothing,
    UpdateLinesAndStatistics,
    SpawnNextTetrimino,
}
