pub mod error;
use crate::model::point::Point;
use error::UserPointInputError;
use inquire::{required, Text};

pub enum UserAction {
    OpenCell(Point),
    FlagCell(Point),
    Error(UserPointInputError),
}

pub fn get_user_action() -> UserAction {
    let input = get_user_input();
    match input {
        Err(e) => UserAction::Error(e),
        Ok(value) => parse_input(&value),
    }
}

fn get_user_input() -> Result<String, error::UserPointInputError> {
    let input = Text::new("座標を入力してください")
        .with_help_message("ex) \"7 2\", フラグの場合は \"f7 2\"")
        .with_validator(required!("必須です。"))
        .prompt()?;
    Ok(input)
}

fn parse_input_point(input: &str) -> Result<Point, UserPointInputError> {
    let point: Vec<&str> = input.split_whitespace().collect();
    if point.len() != 2 {
        return Err(UserPointInputError::InvalidValueCount);
    }
    let x = point[0].parse::<usize>()?;
    let y = point[1].parse::<usize>()?;
    Ok(Point { x, y })
}

fn parse_input(input: &str) -> UserAction {
    match input.starts_with("f") {
        // フラグの設置操作
        true => {
            let res = parse_input_point(input.trim_start_matches('f'));
            match res {
                Ok(point) => return UserAction::FlagCell(point),
                Err(e) => return UserAction::Error(e),
            }
        }
        // セルをOpenする
        false => {
            let res = parse_input_point(input.trim_start_matches('f'));
            match res {
                Ok(point) => return UserAction::OpenCell(point),
                Err(e) => return UserAction::Error(e),
            }
        }
    }
}
