use super::{Board, FlagCellResult, OpenCellResult, Point};

impl Board {
    pub fn open_cell(&mut self, point: &Point) -> OpenCellResult {
        let res = self.get_cell(point).open();
        match res {
            OpenCellResult::OK => {
                self.open_neighbor_if_no_mines(point);
                // 勝利判定をする
                if self.check_is_win() {
                    return OpenCellResult::Win;
                }
                res
            }
            _ => res,
        }
    }

    pub fn flag_cell(&mut self, point: &Point) -> FlagCellResult {
        let res = self.get_cell(point).flag();
        res
    }
}
