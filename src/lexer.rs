use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    cur: char,
    peek: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Self {
            input: input.chars(),
            cur: '\u{0}',
            peek: '\u{0}',
        };
        l.read_char();
        l.read_char();
        l
    }

    fn read_char(&mut self) -> char {
        let c = self.cur;
        self.cur = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
        c
    }

    fn peek_char(&self, c: char) -> bool {
        self.peek == c
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.cur {
            '=' => {
                if self.peek_char('=') {
                    // consume peek_char
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '!' => {
                if self.peek_char('=') {
                    // consume peek_char
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '<' => Token::Gt,
            '>' => Token::Lt,
            '\u{0}' => Token::Eof,
            c => {
                if is_letter(c) {
                    return self.read_identifier();
                } else if is_digit(c) {
                    return self.read_integer();
                } else {
                    return Token::Illegal(c.to_string());
                }
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while is_letter(self.cur) {
            ident.push(self.read_char());
        }
        if let Some(tok) = Token::keyword(&ident) {
            return tok;
        }
        Token::Ident(ident)
    }

    fn read_integer(&mut self) -> Token {
        let mut integer = String::new();
        while is_digit(self.cur) {
            integer.push(self.read_char());
        }
        match integer.parse::<i64>() {
            Ok(int) => Token::Int(int),
            Err(_) => Token::Illegal(integer),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.cur.is_whitespace() {
            self.read_char();
        }
    }
}

fn is_letter(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c)
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

mod test {

    use super::*;

    fn assert_tokens(input: &str, expected: Vec<Token>) {
        let mut l = Lexer::new(input);
        for token in expected.iter() {
            assert_eq!(l.next_token(), *token);
        }
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},!-/*5;";
        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Eof,
        ];
        assert_tokens(input, expected);
    }

    #[test]
    fn test_let_stmt() {
        let input = "let five = 5;";
        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Eof,
        ];
        assert_tokens(input, expected);
    }

    #[test]
    fn test_func() {
        let input = "let add = fn(x, y){x + y};";
        let expected = vec![
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::RBrace,
            Token::Semicolon,
            Token::Eof,
        ];
        assert_tokens(input, expected);
    }

    #[test]
    fn test_gt_le() {
        let input = "5 < 10 > 5";
        let expected = vec![
            Token::Int(5),
            Token::Gt,
            Token::Int(10),
            Token::Lt,
            Token::Int(5),
            Token::Eof,
        ];
        assert_tokens(input, expected);
    }
    #[test]
    fn test_if_stmt() {
        let input = "if(5 < 10){return true} else {return false};";
        let expected = vec![
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Gt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::RBrace,
            Token::Semicolon,
            Token::Eof,
        ];
        assert_tokens(input, expected);
    }

    #[test]
    fn test_eq() {
        let input = "10 == 10; 10 != 9";
        let expected = vec![
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Eof,
        ];
        assert_tokens(input, expected);
    }
}
