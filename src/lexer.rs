use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    cur: char,
    peek: char,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.cur {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '\u{0}' => Token::Eof,
            c => {
                if is_letter(c) {
                    return self.read_identifier();
                } else if is_digit(c) {
                    return self.read_integer();
                } else {
                    return Token::Illegal;
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
        Token::Int(integer.parse::<i64>().unwrap())
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

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Plus);
        assert_eq!(l.next_token(), Token::LParen);
        assert_eq!(l.next_token(), Token::RParen);
        assert_eq!(l.next_token(), Token::LBrace);
        assert_eq!(l.next_token(), Token::RBrace);
        assert_eq!(l.next_token(), Token::Comma);
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Eof);
    }

    #[test]
    fn test_let_stmt() {
        let input = "let five = 5;";
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Let);
        assert_eq!(l.next_token(), Token::Ident("five".to_string()));
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Int(5));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Eof);
    }

    #[test]
    fn test_func() {
        let input = "let add = fn(x, y){x + y};";
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Let);
        assert_eq!(l.next_token(), Token::Ident("add".to_string()));
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Function);
        assert_eq!(l.next_token(), Token::LParen);
        assert_eq!(l.next_token(), Token::Ident("x".to_string()));
        assert_eq!(l.next_token(), Token::Comma);
        assert_eq!(l.next_token(), Token::Ident("y".to_string()));
        assert_eq!(l.next_token(), Token::RParen);
        assert_eq!(l.next_token(), Token::LBrace);
        assert_eq!(l.next_token(), Token::Ident("x".to_string()));
        assert_eq!(l.next_token(), Token::Plus);
        assert_eq!(l.next_token(), Token::Ident("y".to_string()));
        assert_eq!(l.next_token(), Token::RBrace);
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Eof);
    }
}
