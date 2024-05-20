use super::{Board, Cell, Point, MINE_COUNT, SIZE};

// 二次元vecのBoard.cellsを一次元のiterに変換する
fn cells_iter(board: &Board) -> impl Iterator<Item = &Cell> {
    board.cells.iter().flat_map(|cell| cell)
}

// 条件に一致するCellの総数をカウントする
fn count_cell<F>(board: &Board, condition_fn: F) -> usize
where
    F: Fn(&&Cell) -> bool,
{
    cells_iter(board).filter(condition_fn).count()
}

impl Board {
    pub fn in_board(point: &Point) -> bool {
        let board_range = 1..=SIZE;
        board_range.contains(&point.x) & board_range.contains(&point.y)
    }

    pub fn get_neighbor_cells(&mut self, point: &Point) -> Vec<Point> {
        let x_range = (point.x - 1)..=(point.x + 1);
        let y_range = || (point.y - 1)..=(point.y + 1);
        x_range
            .flat_map(|x| y_range().map(move |y| Point { x, y }))
            .filter(Board::in_board)
            .collect()
    }

    pub fn get_cell(&mut self, point: &Point) -> &mut Cell {
        &mut self.cells[point.x - 1][point.y - 1]
    }

    pub(super) fn 未開封の_cell_の総数(&self) -> usize {
        count_cell(self, |cell| !cell.is_open)
    }

    pub(super) fn フラグを立てた_cell_の総数(&self) -> usize {
        count_cell(self, |cell| cell.is_flag)
    }

    pub(super) fn check_is_win(&self) -> bool {
        self.未開封の_cell_の総数() == MINE_COUNT
    }
}
