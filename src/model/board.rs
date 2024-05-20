use core::panic;
use std::isize::MIN;

use crate::model::point::Point;
use crate::setting::{MINE_COUNT, SIZE};
use rand::Rng;

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
        match self.is_open {
            false => print!("■"),
            true => match self.is_mine {
                true => print!("×"),
                false => print!("□"),
            },
        }
    }
    fn set_mine(&mut self) {
        self.is_mine = true;
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
    // ex)
    //    point          (1, 5)
    // => cell index     (0, 4)
    // => neighbor cells x       x       x
    //                   (0, 3), (0, 4), (0, 5)
    //                   (1, 3), (1, 4), (1, 5)
    for p in positions {
        let row_i = p.x - 1;
        let col_i = p.y - 1;
        board.cells[row_i][col_i].set_mine();
        // let xrange = std::cmp::max(row_i - 1, 0)..=std::cmp::min(row_i + 1, SIZE - 1);
        // println!("{:?}", xrange);
        // for x in xrange {
        //     let yrange = std::cmp::max(col_i - 1, 0)..=std::cmp::min(col_i + 1, SIZE - 1);
        //     println!("{:?}", yrange);
        //     for y in yrange {
        //         println!("{} {}", x, y);
        //     }
        // }
        // panic!("test");
    }
}
impl Board {
    // MINEの場所一覧を作成
    pub fn new() -> Board {
        let mut board = Board {
            cells: Vec::from([0; SIZE].map(|_| Board::row())),
        };
        set_mines(&mut board);
        board
    }
    fn row() -> Vec<Cell> {
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
    pub fn open_cell(&mut self, point: &Point) -> OpenCellResult {
        self.get_cell(point).open()
    }
}
