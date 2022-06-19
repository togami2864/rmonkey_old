use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i64),
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

impl Token {
    pub fn keyword(c: &str) -> Option<Token> {
        match c {
            "fn" => Some(Token::Function),
            "let" => Some(Token::Let),
            _ => None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Int(num) => write!(f, "{}", num),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Function => write!(f, "function"),
            Token::Let => write!(f, "let"),
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}
