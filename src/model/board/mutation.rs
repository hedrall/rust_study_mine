use super::{Board, Point};

impl Board {
    pub fn open_neighbor_if_no_mines(&mut self, point: &Point) {
        // 自分のmine_countが0であること
        if self.get_cell(&point).neighbor_mine_count != 0 {
            return;
        }

        // 近隣を全て開く
        for n in self.get_neighbor_cells(point) {
            let cell = self.get_cell(&n);
            if !cell.is_open {
                cell.open();
                self.open_neighbor_if_no_mines(&n);
            }
        }
    }
}
