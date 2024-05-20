pub mod action;
pub mod printer;
pub mod types;
use types::{FlagCellResult, OpenCellResult};

#[derive(Debug)]
pub struct Cell {
    pub is_open: bool,
    pub is_mine: bool,
    pub is_flag: bool,
    pub neighbor_mine_count: u32,
}

impl Cell {
    pub(in crate::model) fn new() -> Cell {
        Cell {
            is_open: false,
            is_mine: false,
            is_flag: false,
            neighbor_mine_count: 0,
        }
    }

    pub(in crate::model) fn set_mine(&mut self) {
        self.is_mine = true;
    }
    pub(in crate::model) fn increment_neighbor_mine_count(&mut self) {
        self.neighbor_mine_count = self.neighbor_mine_count + 1;
    }
}
