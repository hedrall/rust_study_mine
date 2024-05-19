use crate::model::point::Point;
use crate::setting::SIZE;

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
        if !self.is_open {
            print!("■");
        } else {
            if self.is_mine {
                print!("💣")
            } else {
                print!("□");
            }
        }
    }
}

pub struct Board {
    // 10 x 10 マスで固定
    cells: Vec<Vec<Cell>>,
}

const LENGTH: usize = 8;

impl Board {
    pub fn new() -> Board {
        Board {
            cells: Vec::from([0; SIZE].map(|_| Board::row())),
        }
    }
    fn row() -> Vec<Cell> {
        Vec::from([0; SIZE].map(|_| Cell::new()))
    }
    fn get_cell_of_point(&self, point: Point) -> &Cell {
        let row = &self.cells[point.x];
        &row[point.y]
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
}
