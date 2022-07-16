use crate::operator::Precedence;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal(String),
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

    pub fn precedence(tok: Token) -> Precedence {
        match tok {
            Token::Eq => Precedence::Equals,
            Token::NotEq => Precedence::Equals,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Asterisk => Precedence::Product,
            Token::Slash => Precedence::Product,
            Token::Lt => Precedence::LessGreater,
            Token::Gt => Precedence::LessGreater,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}
