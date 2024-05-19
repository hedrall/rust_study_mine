use crate::model::point::Point;
use crate::setting::{MINE_COUNT, SIZE};
use rand::Rng;

#[derive(Debug)]
pub struct Cell {
    pub is_open: bool,
    pub is_mine: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            is_open: false,
            is_mine: false,
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
        self.is_open = true;
        match self.is_mine {
            true => OpenCellResult::Mine,
            false => match self.is_open {
                true => OpenCellResult::AlreadyOpened,
                false => OpenCellResult::OK,
            },
        }
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
    for p in positions {
        board.cells[p.x - 1][p.y - 1].set_mine();
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

    pub fn open_cell(&mut self, point: &Point) -> OpenCellResult {
        self.cells[point.x - 1][point.y - 1].open()
    }
}
