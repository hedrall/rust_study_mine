use std::num::ParseIntError;
use std::{error, fmt};

use inquire::{required, InquireError, Text};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum UserPointInputError {
    Inquire(InquireError),
    Parse(ParseIntError),
    // x, yとちょうど2つの値が指定されていない
    InvalidValueCount,
}

impl fmt::Display for UserPointInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserPointInputError::Inquire(e) => write!(f, "IO Error: {}", e),
            UserPointInputError::Parse(e) => write!(f, "Parse Error {}", e),
            UserPointInputError::InvalidValueCount => write!(f, "Parse Error 入力する数値はx, yの2値です。"),
        }
    }
}

impl error::Error for UserPointInputError {}

impl From<InquireError> for UserPointInputError {
    fn from(err: InquireError) -> UserPointInputError {
        UserPointInputError::Inquire(err)
    }
}

impl From<ParseIntError> for UserPointInputError {
    fn from(err: ParseIntError) -> UserPointInputError {
        UserPointInputError::Parse(err)
    }
}

fn get_user_input() -> Result<String, UserPointInputError> {
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

fn get_next_point() -> Result<Point, UserPointInputError> {
    let input = get_user_input()?;
    let point = parse_input(&input)?;
    Ok(point)
}
fn main() {
    // ユーザ入力を取る
    let next_point = get_next_point();

    match next_point {
        Ok(ref point) => println!("{} {}", point.x, point.y),
        Err(e) => match e {
            UserPointInputError::Inquire(e) => println!("@@@ Inquier, {}", e),
            UserPointInputError::Parse(e) => println!("@@@ Parse, {}", e),
            UserPointInputError::InvalidValueCount => println!("@@@ Invalid Value Count, {}", e),
        },
    }
}
