use super::Cell;
use super::{FlagCellResult, OpenCellResult};

impl Cell {
    pub(in crate::model) fn open(&mut self) -> OpenCellResult {
        let res: OpenCellResult;
        if self.is_flag {
            return OpenCellResult::CannotOpenBecauseFlaged;
        }
        match self.is_mine {
            true => res = OpenCellResult::Mine,
            false => match self.is_open {
                true => res = OpenCellResult::AlreadyOpened,
                false => res = OpenCellResult::OK,
            },
        }
        self.is_open = true;
        res
    }
    pub(in crate::model) fn flag(&mut self) -> FlagCellResult {
        if self.is_open {
            return FlagCellResult::CannnotFlagOnOpenedCell;
        }
        match self.is_flag {
            false => {
                self.is_flag = true;
                FlagCellResult::Added
            }
            true => {
                self.is_flag = false;
                FlagCellResult::Removed
            }
        }
    }
}
