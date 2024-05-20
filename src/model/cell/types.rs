pub enum OpenCellResult {
    OK,
    AlreadyOpened,
    CannotOpenBecauseFlaged,
    Win,
    Mine,
}

pub enum FlagCellResult {
    Added,
    Removed,
    CannnotFlagOnOpenedCell,
}
