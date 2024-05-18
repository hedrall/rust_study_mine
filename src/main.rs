use std::{io, num::ParseIntError};

use inquire::{required, Text};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn get_user_input() -> Result<String, io::Error> {
    let input = Text::new("座標を入力してください")
        .with_help_message("ex) 7, 2")
        .with_validator(required!("必須です。"))
        .prompt();
    let input = input.map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    Ok(input)
}

fn parse_input(input: &str) -> Result<Point, ParseIntError> {
    let point: Vec<&str> = input.split_whitespace().collect();
    let x = point[0].parse::<u32>()?;
    let y = point[1].parse::<u32>()?;
    Ok(Point { x, y })
}

fn get_next_point() -> Result<Point, io::Error> {
    let input = get_user_input()?;
    let point = parse_input(&input).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Result::Ok(point)
}
fn main() {
    // ユーザ入力を取る
    let next_point = get_next_point();

    match next_point {
        Ok(ref point) => println!("{} {}", point.x, point.y),
        Err(_) => println!("エラー"),
    }
}
