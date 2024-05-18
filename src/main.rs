mod input;
use input::error::UserPointInputError;
mod model;

fn main() {
    // ユーザ入力を取る
    let next_point = input::get_next_point();

    match next_point {
        Ok(point) => println!("{} {}", point.x, point.y),
        Err(e) => match e {
            UserPointInputError::Inquire(e) => println!("@@@ Inquier, {}", e),
            UserPointInputError::Parse(e) => println!("@@@ Parse, {}", e),
            UserPointInputError::InvalidValueCount => println!("@@@ Invalid Value Count, {}", e),
        },
    }
}
