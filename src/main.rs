mod input;
mod setting;
use input::error::UserPointInputError as InputError;
mod model;
use model::{board::Board, point::Point};

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
                board.open_cell(&point);
            }
            Err(e) => match e {
                InputError::Inquire(e) => {
                    panic!("@@@ Inquier, {}", e);
                }
                InputError::Parse(e) => println!("@@@ Parse, {}", e),
                InputError::InvalidValueCount => println!("@@@ Invalid Value Count, {}", e),
            },
        }
    }
}
