use crate::model::cell::{
    types::{FlagCellResult, OpenCellResult},
    Cell,
    printer::*
};
use crate::model::point::Point;
use crate::setting::{MINE_COUNT, SIZE};
use colored::Colorize;
use rand::Rng;

pub struct Board {
    // 10 x 10 マスで固定
    cells: Vec<Vec<Cell>>,
}

fn mine_positions() -> Vec<Point> {
    let mut rng = rand::thread_rng();

    // 長さがMINE_COUNTのpointの配列
    let mut p: Vec<Point> = vec![];

    while p.len() < MINE_COUNT {
        let new_p = Point {
            x: rng.gen_range(1..=SIZE),
            y: rng.gen_range(1..=SIZE),
        };
        if !p.contains(&new_p) {
            p.push(new_p);
        }
    }
    p
}
fn set_mines(board: &mut Board) {
    let positions = mine_positions();
    println!("mine positions {:#?}", positions);

    // 周辺の爆弾数をセットする
    for p in positions {
        board.get_cell(&p).set_mine();
        for np in board.get_neighbor_cells(&p) {
            board.get_cell(&np).increment_neighbor_mine_count();
        }
    }
}
impl Board {
    // MINEの場所一覧を作成
    pub fn new() -> Board {
        let mut board = Board {
            cells: Board::new_cells(),
        };
        set_mines(&mut board);
        board
    }
    fn new_cells() -> Vec<Vec<Cell>> {
        Vec::from([0; SIZE].map(|_| Board::new_row()))
    }
    fn new_row() -> Vec<Cell> {
        Vec::from([0; SIZE].map(|_| Cell::new()))
    }
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

    pub fn get_cell(&mut self, point: &Point) -> &mut Cell {
        &mut self.cells[point.x - 1][point.y - 1]
    }

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

    fn not_open_cell_count(&self) -> usize {
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
    fn flaged_cell_count(&self) -> usize {
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
    fn check_is_win(&self) -> bool {
        self.not_open_cell_count() == MINE_COUNT
    }
    pub fn show_stats(&self) {
        let to_open_cell_count = self.not_open_cell_count() - MINE_COUNT;
        let mines_not_flagged = MINE_COUNT - self.flaged_cell_count();
        println!(
            "あと{}マス, 💣残: {}",
            to_open_cell_count, mines_not_flagged
        );
    }
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
