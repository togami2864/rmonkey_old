use crate::{
    ast::{Expr, Program, Stmt},
    error::{MonkeyError, Result},
    lexer::Lexer,
    operator::{Infix, Precedence, Prefix},
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
            cur_token: Token::Illegal('\u{0}'.to_string()),
            peek_token: Token::Illegal('\u{0}'.to_string()),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) -> &Token {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
        &self.cur_token
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut program = Program::new();
        while self.cur_token != Token::Eof {
            match self.parse_stmt() {
                Ok(stmt) => program.stmts.push(stmt),
                Err(err) => return Err(MonkeyError::Custom(format!("stmt error: {}", err))),
            }
            self.next_token();
        }
        Ok(program)
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.cur_token {
            Token::Let => self.parse_let_stmt(),
            Token::Return => self.parse_return_stmt(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt> {
        self.next_token();
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
            self.next_token();
        }

        Ok(Stmt::LetStatement {
            ident: Expr::Ident(ident),
            value: Expr::Ident("placeholder".to_string()),
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::ReturnStatement {
            value: Expr::Ident("placeholder".to_string()),
        })
    }

    fn parse_block_stmt(&mut self) -> Result<Stmt> {
        self.expect_peek(Token::LBrace)?;
        let mut stmts: Vec<Stmt> = vec![];
        self.next_token();
        while !self.cur_token_is(Token::RBrace) && !self.cur_token_is(Token::Eof) {
            match self.parse_stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(_) => todo!(),
            }
            self.next_token();
        }
        self.next_token();
        Ok(Stmt::BlockStatement { stmts })
    }

    fn parse_expr_statement(&mut self) -> Result<Stmt> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::ExpressionStatement { expr })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr> {
        let mut left = match self.cur_token.clone() {
            Token::Ident(ident) => Expr::Ident(ident),
            Token::Int(val) => Expr::Int(val),
            Token::True => Expr::Boolean(true),
            Token::False => Expr::Boolean(false),
            Token::Minus | Token::Bang => self.parse_prefix_expression()?,
            Token::LParen => self.parse_group_expression()?,
            Token::If => self.parse_if_expression()?,
            _ => todo!(),
        };

        while !self.cur_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left = self.parse_infix_expression(left)?;
        }
        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expr> {
        let op = match self.cur_token {
            Token::Minus => Prefix::Minus,
            Token::Bang => Prefix::Bang,
            _ => todo!(),
        };
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Expr::PrefixExpr {
            op,
            right: Box::new(right),
        })
    }

    fn parse_infix_expression(&mut self, left: Expr) -> Result<Expr> {
        let op = match self.cur_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Slash,
            Token::Asterisk => Infix::Asterisk,
            Token::Eq => Infix::Eq,
            Token::NotEq => Infix::NotEq,
            Token::Lt => Infix::Lt,
            Token::Gt => Infix::Gt,
            _ => unimplemented!(),
        };
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(Expr::InfixExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
        })
    }

    fn parse_group_expression(&mut self) -> Result<Expr> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RParen)? {
            return Err(MonkeyError::UnexpectedToken(
                Token::RParen,
                self.peek_token.clone(),
            ));
        }
        Ok(expr)
    }

    fn parse_if_expression(&mut self) -> Result<Expr> {
        self.expect_peek(Token::LParen)?;
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(Token::RParen)?;

        let consequence = self.parse_block_stmt()?;
        let mut alternative = None;
        if self.cur_token_is(Token::Else) {
            alternative = Some(Box::new(self.parse_block_stmt()?));
        }
        Ok(Expr::IfExpr {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
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
            self.next_token();
            return Ok(true);
        }
        Err(MonkeyError::UnexpectedToken(
            expected,
            self.peek_token.clone(),
        ))
    }

    fn peek_precedence(&mut self) -> Precedence {
        Token::precedence(self.peek_token.clone())
    }
    fn cur_precedence(&mut self) -> Precedence {
        Token::precedence(self.cur_token.clone())
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

    #[test]
    fn test_ident_expression() {
        let input = "foobar";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 1);
    }

    #[test]
    fn test_int_expression() {
        let input = "5";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 1);
    }

    #[test]
    fn test_prefix_expression() {
        let input = "-5;
!5;";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 2);
    }

    #[test]
    fn test_infix_expression() {
        let input = "5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 + 5 * 5;
        5 * 5 - 5 * 5 + 1;
        -a * b;
        !-a;
        a + b * c + d / e - f;
        5 > 4 == 3 < 4;
        5 < 4 != 3 > 4;
        3 + 4 * 5 == 3 * 1 + 4 * 5;
        ";
        let expected = vec![
            "(5 + 5)",
            "(5 - 5)",
            "(5 * 5)",
            "(5 / 5)",
            "(5 + (5 * 5))",
            "(((5 * 5) - (5 * 5)) + 1)",
            "((-a) * b)",
            "(!(-a))",
            "(((a + (b * c)) + (d / e)) - f)",
            "((5 > 4) == (3 < 4))",
            "((5 < 4) != (3 > 4))",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_boolean() {
        let input = "true;
        false;
        3 > 5 == false;
        3 < 5 == true;
        !true
        ";
        let expected = vec![
            "true",
            "false",
            "((3 > 5) == false)",
            "((3 < 5) == true)",
            "(!true)",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_group() {
        let input = "1 + (2 + 3) + 4;
        (5 + 5) * 2;
        2 / (5 + 5);
        -(5 + 5);
        !(true == true);
        ";
        let expected = vec![
            "((1 + (2 + 3)) + 4)",
            "((5 + 5) * 2)",
            "(2 / (5 + 5))",
            "(-(5 + 5))",
            "(!(true == true))",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_if_expression() {
        let input = r#"if(x < y){x};
        if(a<b){a}else{b};"#;
        let expected = vec!["if((x < y)){x}", "if((a < b)){a}else{b}"];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }
}
