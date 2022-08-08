use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prefix {
    Minus,
    Bang,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Minus => write!(f, "-"),
            Prefix::Bang => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Infix {
    Eq,
    NotEq,
    Lt,
    Gt,
    Plus,
    Minus,
    Slash,
    Asterisk,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Infix::Eq => write!(f, "=="),
            Infix::NotEq => write!(f, "!="),
            Infix::Lt => write!(f, ">"),
            Infix::Gt => write!(f, "<"),
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Slash => write!(f, "/"),
            Infix::Asterisk => write!(f, "*"),
        }
    }
}
