use std::fmt;

#[derive(Debug)]
pub enum MonkeyError {
    UnsupportedNumError,
}

impl fmt::Display for MonkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonkeyError::UnsupportedNumError => write!(
                f,
                "UnsupportedNumError: Monkey only supports integer numbers"
            ),
        }
    }
}

impl From<std::num::ParseIntError> for MonkeyError {
    fn from(_: std::num::ParseIntError) -> Self {
        MonkeyError::UnsupportedNumError
    }
}

pub type Result<T> = std::result::Result<T, MonkeyError>;
