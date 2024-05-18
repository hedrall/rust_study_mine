pub mod error;
use crate::model::point::Point;
use error::UserPointInputError;
use inquire::{required, Text};

pub fn get_next_point() -> Result<Point, UserPointInputError> {
    let input = get_user_input()?;
    parse_input(&input)
}

fn get_user_input() -> Result<String, error::UserPointInputError> {
    let input = Text::new("座標を入力してください")
        .with_help_message("ex) \"7 2\"")
        .with_validator(required!("必須です。"))
        .prompt()?;
    Ok(input)
}

fn parse_input(input: &str) -> Result<Point, UserPointInputError> {
    let point: Vec<&str> = input.split_whitespace().collect();
    if point.len() != 2 {
        return Err(UserPointInputError::InvalidValueCount);
    }
    let x = point[0].parse::<u32>()?;
    let y = point[1].parse::<u32>()?;
    Ok(Point { x, y })
}
