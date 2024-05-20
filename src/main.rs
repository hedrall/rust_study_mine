mod input;
mod model;
mod setting;

use cell::types::{FlagCellResult, OpenCellResult};
use input::UserAction;
use model::board::Board;
use model::cell;
use model::point::Point;
use rand::prelude::*;

fn open_cell(board: &mut Board, point: &Point) {
    println!("Open !!!, {} {}", point.x, point.y);

    // 実行
    let res = board.open_cell(&point);

    // 結果を処理
    match res {
        OpenCellResult::OK => {}

        // スキップする処理
        OpenCellResult::AlreadyOpened => println!("⚠️ すでに開いています。"),
        OpenCellResult::CannotOpenBecauseFlaged => println!("⚠️ フラグがついているので開けません。"),

        // 以下、終了する
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

fn flag_cell(board: &mut Board, point: &Point) {
    println!("Flag !!!, {} {}", point.x, point.y);

    // 実行
    let res = board.flag_cell(&point);

    // 結果を処理
    match res {
        FlagCellResult::Added => println!("フラグを追加しました。"),
        FlagCellResult::Removed => println!("フラグを削除しました。"),

        // スキップする処理
        FlagCellResult::CannnotFlagOnOpenedCell => println!("開封済みのセルにフラグを置けません。"),
    }
}

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
            UserAction::OpenCell(point) => open_cell(&mut board, &point),
            UserAction::FlagCell(point) => flag_cell(&mut board, &point),
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
