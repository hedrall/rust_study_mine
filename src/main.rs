mod input;
mod setting;
use input::UserAction;
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
        let action = input::get_user_action();

        match action {
            UserAction::OpenCell(point) => {
                println!("Open !!!, {} {}", point.x, point.y);
                let res = board.open_cell(&point);
                match res {
                    board::OpenCellResult::OK => println!("OK !!!"),
                    board::OpenCellResult::AlreadyOpened => println!("⚠️ すでに開いています。"),
                    board::OpenCellResult::CannotOpenBecauseFlaged => println!("⚠️ フラグがついているので開けません。"),
                    board::OpenCellResult::Mine => {
                        board.print_with_result();
                        panic!("❌ Boooom !!!!!!!");
                    }
                }
            }
            UserAction::FlagCell(point) => {
                println!("Flag !!!, {} {}", point.x, point.y);
                let res = board.flag_cell(&point);
                match res {
                    board::FlagCellResult::Added => println!("フラグを追加しました。"),
                    board::FlagCellResult::Removed => println!("フラグさ削除しました。"),
                }
            }
            UserAction::Error(e) => match e {
                input::error::UserPointInputError::Inquire(e) => panic!("@@@ Inquier, {}", e),
                input::error::UserPointInputError::Parse(e) => println!("@@@ Parse, {}", e),
                input::error::UserPointInputError::InvalidValueCount => println!("@@@ Invalid Value Count, {}", e),
            },
        }
    }
}
