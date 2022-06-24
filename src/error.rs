use std::fmt;

use crate::token::Token;

#[derive(Debug)]
pub enum MonkeyError {
    Custom(String),
    UnsupportedNumError,
    UnexpectedToken(Token, Token),
}

impl fmt::Display for MonkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonkeyError::Custom(msg) => write!(f, "{}", msg),
            MonkeyError::UnsupportedNumError => write!(
                f,
                "UnsupportedNumError: Monkey only supports integer numbers"
            ),
            MonkeyError::UnexpectedToken(expected, actual) => {
                write!(f, "expected {:?}, but got {:?}", expected, actual)
            }
        }
    }
}

impl From<std::num::ParseIntError> for MonkeyError {
    fn from(_: std::num::ParseIntError) -> Self {
        MonkeyError::UnsupportedNumError
    }
}

pub type Result<T> = std::result::Result<T, MonkeyError>;
