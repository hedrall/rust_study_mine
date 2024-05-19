use crate::model::point::Point;
use crate::setting::SIZE;

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
                true => print!("💣"),
                false => print!("□"),
            },
        }
    }
    fn open(&mut self) -> OpenCellResult {
        let res: OpenCellResult;
        match self.is_open {
            true => res = OpenCellResult::AlreadyOpened,
            false => match self.is_mine {
                true => res = OpenCellResult::Mine,
                false => {
                    res = OpenCellResult::OK;
                    self.is_open = true;
                    println!("Open Cell: {:?}", self);
                }
            },
        }
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

impl Board {
    pub fn new() -> Board {
        Board {
            cells: Vec::from([0; SIZE].map(|_| Board::row())),
        }
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
