use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i64),
    Assign,    // =
    Plus,      // +
    Minus,     // -
    Asterisk,  // *
    Slash,     // /
    Gt,        // <
    Lt,        // >
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    //{
    RBrace,    //}
    Bang,      // !
    Eq,        // ==
    NotEq,     // !=

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn keyword(c: &str) -> Option<Token> {
        match c {
            "fn" => Some(Token::Function),
            "let" => Some(Token::Let),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "return" => Some(Token::Return),
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
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Function => write!(f, "function"),
            Token::Let => write!(f, "let"),
            Token::Gt => write!(f, "<"),
            Token::Lt => write!(f, ">"),
            Token::Bang => write!(f, "!"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}
