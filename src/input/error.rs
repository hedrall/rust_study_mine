use inquire::InquireError;
use std::num::ParseIntError;
use std::{error, fmt};

#[derive(Debug)]
pub enum UserPointInputError {
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
            UserPointInputError::InvalidValueCount => {
                write!(f, "Parse Error 入力する数値はx, yの2値です。")
            }
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
