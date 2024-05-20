mod input;
mod setting;
use input::error::UserPointInputError as InputError;
mod model;
use crate::model::board;
use model::board::Board;
use rand::prelude::*;

fn main() {
    let mut board = Board::new();
    let mut ターンカウント = 0;
    loop {
        ターンカウント = ターンカウント + 1;
        println!("ターン: {}", ターンカウント);
        board.print();

        // ユーザ入力を取る
        let next_point = input::get_next_point();
        
        match next_point {
            Ok(point) => {
                println!("{} {}", point.x, point.y);
                let res = board.open_cell(&point);
                match res {
                    board::OpenCellResult::OK => println!("OK !!!"),
                    board::OpenCellResult::AlreadyOpened => println!("⚠️ すでに開いています。"),
                    board::OpenCellResult::Mine => {
                        board.print_with_result();
                        panic!("❌ Boooom !!!!!!!");
                    }
                }
            }
            Err(e) => match e {
                InputError::Inquire(e) => panic!("@@@ Inquier, {}", e),
                InputError::Parse(e) => println!("@@@ Parse, {}", e),
                InputError::InvalidValueCount => println!("@@@ Invalid Value Count, {}", e),
            },
        }
    }
}
