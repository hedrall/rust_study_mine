use core::panic;
use std::isize::MIN;

use super::point;
use crate::model::point::Point;
use crate::setting::{MINE_COUNT, SIZE};
use rand::Rng;

const DEBUG_MODE_KEY: &str = "DEBUG_MODE";
fn is_debug_mode() -> bool {
    let debug_mode = std::env::var(DEBUG_MODE_KEY);
    match debug_mode {
        Ok(value) => value == "true",
        Err(_) => false,
    }
}

#[derive(Debug)]
pub struct Cell {
    pub is_open: bool,
    pub is_mine: bool,
    pub neighbor_mine_count: u32,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            is_open: false,
            is_mine: false,
            neighbor_mine_count: 0,
        }
    }
    fn print(&self) {
        match is_debug_mode() {
            true => {
                match self.is_mine {
                    true => print!("×"),
                    false => print!("{}", self.neighbor_mine_count),
                }
            }
            false => {
                match self.is_open {
                    false => print!("■"),
                    true => match self.is_mine {
                        true => print!("×"),
                        false => print!("{}", self.neighbor_mine_count),
                    },
                }
            }
        }
    }
    fn set_mine(&mut self) {
        self.is_mine = true;
    }
    fn increment_neighbor_mine_count(&mut self) {
        self.neighbor_mine_count = self.neighbor_mine_count + 1;
    }
    fn open(&mut self) -> OpenCellResult {
        let res: OpenCellResult;
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
}

pub struct Board {
    // 10 x 10 マスで固定
    cells: Vec<Vec<Cell>>,
}

pub enum OpenCellResult {
    OK,
    AlreadyOpened,
    Mine,
}

fn mine_positions() -> Vec<Point> {
    let mut rng = rand::thread_rng();

    // 長さがMINE_COUNTのpointの配列
    let mut p: Vec<Point> = Vec::from([0; MINE_COUNT].map(|_| Point { x: 0, y: 0 }));

    for index in 0..MINE_COUNT {
        println!("index {}", index);
        // 1 ~ 10 までの乱数を生成
        p[index] = Point {
            x: rng.gen_range(1..=SIZE),
            y: rng.gen_range(1..=SIZE),
        };
    }
    p
}
fn set_mines(board: &mut Board) {
    let positions = mine_positions();
    println!("mine positions {:#?}", positions);

    // 周辺の爆弾数をセットする
    for p in positions {
        board.get_cell(&p).set_mine();
        for np in board.get_neighbor_cells(&p) {
            board.get_cell(&np).increment_neighbor_mine_count();
        }
    }
}
impl Board {
    // MINEの場所一覧を作成
    pub fn new() -> Board {
        let mut board = Board {
            cells: Board::new_cells(),
        };
        set_mines(&mut board);
        board
    }
    fn new_cells() -> Vec<Vec<Cell>> {
        Vec::from([0; SIZE].map(|_| Board::new_row()))
    }
    fn new_row() -> Vec<Cell> {
        Vec::from([0; SIZE].map(|_| Cell::new()))
    }
    fn print_row(row: &Vec<Cell>) {
        for cell in row {
            cell.print();
            print!(" ");
        }
    }
    pub fn print(&self) {
        for row in &self.cells {
            Board::print_row(&row);
            println!("");
        }
    }

    pub fn get_cell(&mut self, point: &Point) -> &mut Cell {
        &mut self.cells[point.x - 1][point.y - 1]
    }

    pub fn in_board(point: &Point) -> bool {
        let board_range = 1..=SIZE;
        board_range.contains(&point.x) & board_range.contains(&point.y)
    }

    pub fn get_neighbor_cells(&mut self, point: &Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        for x in (point.x - 1)..=(point.x + 1) {
            for y in (point.y - 1)..=(point.y + 1) {
                let p = Point { x, y };
                if Board::in_board(&p) {
                    neighbors.push(p);
                }
            }
        }
        neighbors
    }

    pub fn open_cell(&mut self, point: &Point) -> OpenCellResult {
        self.get_cell(point).open()
    }
}
