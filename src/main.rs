mod input;
mod model;
mod setting;

use cell::types::{FlagCellResult, OpenCellResult};
use input::UserAction;
use model::board::Board;
use model::cell;
use rand::prelude::*;

fn main() {
    let mut board = Board::new();
    let mut ターンカウント = 0;
    loop {
        println!("");
        ターンカウント = ターンカウント + 1;
        println!("ターン: {}", ターンカウント);
        board.show_stats();
        board.print();

        // ユーザ入力を取る
        let action = input::get_user_action();

        match action {
            UserAction::OpenCell(point) => {
                println!("Open !!!, {} {}", point.x, point.y);
                let res = board.open_cell(&point);
                match res {
                    OpenCellResult::OK => {}
                    OpenCellResult::AlreadyOpened => println!("⚠️ すでに開いています。"),
                    OpenCellResult::CannotOpenBecauseFlaged => {
                        println!("⚠️ フラグがついているので開けません。")
                    }
                    OpenCellResult::Win => {
                        board.print_with_result();
                        panic!("🎉 You Win !!!!!!!");
                    }
                    OpenCellResult::Mine => {
                        board.print_with_result();
                        panic!("❌ Boooom !!!!!!!");
                    }
                }
            }
            UserAction::FlagCell(point) => {
                println!("Flag !!!, {} {}", point.x, point.y);
                let res = board.flag_cell(&point);
                match res {
                    FlagCellResult::Added => println!("フラグを追加しました。"),
                    FlagCellResult::Removed => println!("フラグを削除しました。"),
                    FlagCellResult::CannnotFlagOnOpenedCell => {
                        println!("開封済みのセルにフラグを置けません。")
                    }
                }
            }
            UserAction::Error(e) => match e {
                input::error::UserPointInputError::Inquire(e) => panic!("@@@ Inquier, {}", e),
                input::error::UserPointInputError::Parse(e) => println!("@@@ Parse, {}", e),
                input::error::UserPointInputError::InvalidValueCount => {
                    println!("@@@ Invalid Value Count, {}", e)
                }
            },
        }
    }
}
