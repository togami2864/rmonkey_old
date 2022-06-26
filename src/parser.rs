use crate::{
    ast::{Expr, Program, Stmt},
    error::{MonkeyError, Result},
    lexer::Lexer,
    token::Token,
};

#[derive(Debug)]
pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut p = Self {
            l,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) -> Result<&Token> {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token()?;
        Ok(&self.cur_token)
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut program = Program::new();
        while self.cur_token != Token::Eof {
            match self.parse_stmt() {
                Ok(stmt) => program.stmts.push(stmt),
                Err(_) => return Err(MonkeyError::Custom("stmt error".to_string())),
            }
            self.next_token()?;
        }
        Ok(program)
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.cur_token {
            Token::Let => self.parse_let_stmt(),
            Token::Return => self.parse_return_stmt(),
            _ => unimplemented!(),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt> {
        self.next_token()?;
        let ident = match self.cur_token.clone() {
            Token::Ident(ident) => ident,
            tok => {
                return Err(MonkeyError::UnexpectedToken(
                    tok,
                    Token::Ident("".to_string()),
                ))
            }
        };
        self.expect_peek(Token::Assign)?;
        while !self.cur_token_is(Token::Semicolon) {
            self.next_token()?;
        }

        Ok(Stmt::LetStatement {
            ident: Expr::Ident(ident),
            value: Expr::Ident("placeholder".to_string()),
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        while !self.cur_token_is(Token::Semicolon) {
            self.next_token()?;
        }
        Ok(Stmt::ReturnStatement {
            value: Expr::Ident("placeholder".to_string()),
        })
    }

    fn cur_token_is(&self, t: Token) -> bool {
        self.cur_token == t
    }

    fn peek_token_is(&self, t: Token) -> bool {
        self.peek_token == t
    }

    fn expect_peek(&mut self, t: Token) -> Result<bool> {
        let expected = t.clone();
        if self.peek_token_is(t) {
            self.next_token()?;
            return Ok(true);
        }
        Err(MonkeyError::UnexpectedToken(
            expected,
            self.peek_token.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_stmt() {
        let input = r#"let x = 5;
let y = 10;
let foobar = 838383;
"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 3);
        let expected = vec!["x", "y", "foobar"];
        for (i, stmt) in program.stmts.iter().enumerate() {
            if let Stmt::LetStatement { ident, .. } = stmt {
                if let Expr::Ident(ident) = ident {
                    assert_eq!(ident, expected[i]);
                }
            };
        }
    }
    #[test]
    fn test_return_stmt() {
        let input = r#"return 5;
return 10;
"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 2);
    }
}
