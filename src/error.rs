use std::fmt;

use crate::{
    operator::{Infix, Prefix},
    token::Token,
};

#[derive(Debug)]
pub enum MonkeyError {
    Custom(String),
    UnsupportedNumError,
    UnexpectedToken(Token, Token),
    TypeMismatch(String, String, Infix),
    UnknownOperator(String, String, Infix),
    UnknownPrefix(Prefix, String),
    UncaughtRef(String),
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
            MonkeyError::TypeMismatch(left, right, op) => {
                write!(f, "type mismatch: {} {} {}", left, op, right)
            }
            MonkeyError::UnknownOperator(left, right, op) => {
                write!(f, "unknown operator: {} {} {}", left, op, right)
            }
            MonkeyError::UnknownPrefix(prefix, left) => {
                write!(f, "unknown prefix: {}{}", prefix, left)
            }
            MonkeyError::UncaughtRef(ident) => {
                write!(f, "Uncaught ReferenceError: {} is not defined", ident)
            }
        }
    }
}

impl From<std::num::ParseIntError> for MonkeyError {
    fn from(_: std::num::ParseIntError) -> Self {
        MonkeyError::UnsupportedNumError
    }
}

impl From<std::num::TryFromIntError> for MonkeyError {
    fn from(_: std::num::TryFromIntError) -> Self {
        MonkeyError::UnsupportedNumError
    }
}

pub type Result<T> = std::result::Result<T, MonkeyError>;
