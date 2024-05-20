use super::{ Board, Point};

impl Board {
    pub fn open_neighbor_if_no_mines(&mut self, point: &Point) {
        // 自分のmine_countが0であること
        if self.get_cell(&point).neighbor_mine_count != 0 {
            return;
        }

        let neighbors = self.get_neighbor_cells(point);

        // 近隣を全て開く
        for n in neighbors {
            let cell = self.get_cell(&n);
            match cell.is_open {
                true => continue,
                false => {
                    cell.open();

                    // 開いたcellのneighbor_mine_countが0の場合は再帰実行する
                    if cell.neighbor_mine_count == 0 {
                        self.open_neighbor_if_no_mines(&n);
                    }
                }
            }
        }
    }
}
