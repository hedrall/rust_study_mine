use super::{Board, Cell, Point, MINE_COUNT, SIZE};

impl Board {
    pub fn in_board(point: &Point) -> bool {
        let board_range = 1..=SIZE;
        board_range.contains(&point.x) & board_range.contains(&point.y)
    }

    pub fn get_neighbor_cells(&mut self, point: &Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        for x in (point.x - 1)..=(point.x + 1) {
            for y in (point.y - 1)..=(point.y + 1) {
                let p = Point { x, y };
                if Board::in_board(&p) {
                    neighbors.push(p);
                }
            }
        }
        neighbors
    }

    pub fn get_cell(&mut self, point: &Point) -> &mut Cell {
        &mut self.cells[point.x - 1][point.y - 1]
    }

    pub(super) fn not_open_cell_count(&self) -> usize {
        let mut count: usize = 0;
        for row in &self.cells {
            for cell in row {
                if !cell.is_open {
                    count = count + 1;
                }
            }
        }
        count
    }

    pub(super) fn flaged_cell_count(&self) -> usize {
        let mut count: usize = 0;
        for row in &self.cells {
            for cell in row {
                if cell.is_flag {
                    count = count + 1;
                }
            }
        }
        count
    }

    pub(super) fn check_is_win(&self) -> bool {
        self.not_open_cell_count() == MINE_COUNT
    }
}
