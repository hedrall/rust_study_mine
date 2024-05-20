pub mod action;
pub mod factory;
pub mod getter;
pub mod mine;
pub mod mutation;
pub mod printer;

use super::cell::types::{FlagCellResult, OpenCellResult};
use super::cell::Cell;
use super::point::Point;
use crate::setting::{MINE_COUNT, SIZE};
use colored::Colorize;

pub struct Board {
    // 10 x 10 マスで固定
    cells: Vec<Vec<Cell>>,
}

impl Board {}
