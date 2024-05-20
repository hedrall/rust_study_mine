use super::{Board, Cell, MINE_COUNT, SIZE};

impl Board {
    fn print_row(row: &Vec<Cell>, with_result: bool) {
        for cell in row {
            match with_result {
                true => cell.print_in_result(),
                false => cell.print(),
            }
            print!(" ");
        }
    }

    pub fn print(&self) {
        print!(" \t");
        for index in 1..=SIZE {
            print!("{} ", index);
        }
        println!("");
        for (index, row) in self.cells.iter().enumerate() {
            print!("{}\t", index + 1);
            Board::print_row(&row, false);
            println!("");
        }
    }

    pub fn print_with_result(&self) {
        for row in &self.cells {
            Board::print_row(&row, true);
            println!("");
        }
    }

    pub fn show_stats(&self) {
        let to_open_cell_count = self.未開封の_cell_の総数() - MINE_COUNT;
        let mines_not_flagged = MINE_COUNT - self.フラグを立てた_cell_の総数();
        println!(
            "あと{}マス, 💣残: {}",
            to_open_cell_count, mines_not_flagged
        );
    }
}
