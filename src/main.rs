mod input;
mod setting;
use input::error::UserPointInputError as InputError;
mod model;
use model::{board::Board, point::Point};

fn main() {
    // ユーザ入力を取る
    let next_point = input::get_next_point();

    match next_point {
        Ok(point) => println!("{} {}", point.x, point.y),
        Err(e) => match e {
            InputError::Inquire(e) => println!("@@@ Inquier, {}", e),
            InputError::Parse(e) => println!("@@@ Parse, {}", e),
            InputError::InvalidValueCount => println!("@@@ Invalid Value Count, {}", e),
        },
    }

    let board = Board::new();
    board.print();
}
