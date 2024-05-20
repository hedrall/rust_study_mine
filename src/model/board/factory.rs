use super::{mine, Board, Cell, SIZE};

fn new_cells() -> Vec<Vec<Cell>> {
    Vec::from([0; SIZE].map(|_| new_row()))
}

fn new_row() -> Vec<Cell> {
    Vec::from([0; SIZE].map(|_| Cell::new()))
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board { cells: new_cells() };
        board.set_mines();
        board
    }

    // ボードに爆弾をセットする
    fn set_mines(&mut self) {
        let mine_positions = mine::mine_positions();
        println!("mine positions {:#?}", mine_positions);

        // 爆弾をセット
        for mp in mine_positions {
            self.get_cell(&mp).set_mine();

            // 周辺Cellのneighbor_mine_countをインクリメントする
            for np in self.get_neighbor_cells(&mp) {
                self.get_cell(&np).increment_neighbor_mine_count();
            }
        }
    }
}
