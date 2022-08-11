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
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::LetStatement {
            ident: Expr::Ident(ident),
            value,
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::ReturnStatement {
            value: return_value,
        })
    }

    fn parse_block_stmt(&mut self) -> Result<Stmt> {
        self.expect_peek(Token::LBrace)?;
        let mut stmts: Vec<Stmt> = vec![];
        self.next_token();
        while !self.cur_token_is(Token::RBrace) && !self.cur_token_is(Token::Eof) {
            match self.parse_stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => return Err(MonkeyError::Custom(e.to_string())),
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
            Token::String(val) => Expr::String(val),
            Token::Int(val) => Expr::Int(val),
            Token::True => Expr::Boolean(true),
            Token::False => Expr::Boolean(false),
            Token::Minus | Token::Bang => self.parse_prefix_expression()?,
            Token::LParen => self.parse_group_expression()?,
            Token::If => self.parse_if_expression()?,
            Token::Function => self.parse_func()?,
            Token::LBrace => self.parse_hash_literal()?,
            Token::LBracket => self.parse_array_literal()?,
            e => return Err(MonkeyError::Custom(format!("{:?}", e))),
        };
        while !self.cur_token_is(Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left = match self.cur_token {
                Token::LParen => self.parse_call_expression(left)?,
                Token::LBracket => self.parse_index_expression(left)?,
                _ => self.parse_infix_expression(left)?,
            }
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
            _ => return Err(MonkeyError::Custom("not yet".to_string())),
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

    fn parse_index_expression(&mut self, left: Expr) -> Result<Expr> {
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(Token::RBracket)?;
        Ok(Expr::IndexExpr {
            left: Box::new(left),
            index: Box::new(index),
        })
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

    fn parse_func(&mut self) -> Result<Expr> {
        let parameters = self.parse_func_params()?;
        let body = self.parse_block_stmt()?;
        Ok(Expr::FuncLiteral {
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_func_params(&mut self) -> Result<Vec<Expr>> {
        let mut params: Vec<Expr> = Vec::new();

        self.expect_peek(Token::LParen)?;
        self.next_token();
        if self.cur_token_is(Token::RParen) {
            return Ok(params);
        }

        while !self.cur_token_is(Token::RParen) {
            let param = self.parse_expression(Precedence::Lowest)?;
            params.push(param);
            self.next_token();
            if self.cur_token_is(Token::Comma) {
                self.next_token();
            }
        }
        Ok(params)
    }

    fn parse_call_expression(&mut self, func: Expr) -> Result<Expr> {
        let args = self.parse_call_args(Token::RParen)?;
        Ok(Expr::CallExpr {
            function: Box::new(func),
            args,
        })
    }

    fn parse_call_args(&mut self, end: Token) -> Result<Vec<Expr>> {
        let mut args: Vec<Expr> = Vec::new();
        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Ok(args);
        }
        self.next_token();
        let first_arg = self.parse_expression(Precedence::Lowest)?;
        args.push(first_arg);
        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            let arg = self.parse_expression(Precedence::Lowest)?;
            args.push(arg);
        }
        self.expect_peek(end)?;
        Ok(args)
    }

    pub fn parse_array_literal(&mut self) -> Result<Expr> {
        let elements = self.parse_call_args(Token::RBracket)?;
        Ok(Expr::ArrayLiteral { elements })
    }

    pub fn parse_hash_literal(&mut self) -> Result<Expr> {
        let mut pairs: Vec<(Expr, Expr)> = Vec::new();
        while !self.peek_token_is(Token::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;
            if !self.expect_peek(Token::Colon)? {
                todo!()
            };
            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;
            pairs.push((key, value));
            if !self.peek_token_is(Token::RBrace) && !self.expect_peek(Token::Comma)? {
                todo!()
            }
        }
        if !self.expect_peek(Token::RBrace)? {
            todo!()
        }
        Ok(Expr::HashLiteral { pairs })
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
let foo = "bar"
"#;
        let expected = vec![
            "let x = 5",
            "let y = 10",
            "let foobar = 838383",
            r#"let foo = "bar""#,
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());

        for (i, stmt) in program.stmts.iter().enumerate() {
            assert_eq!(stmt.to_string(), expected[i])
        }
    }
    #[test]
    fn test_return_stmt() {
        let input = r#"return 5;
return 10;
return "10"
"#;
        let expected = vec!["return 5", "return 10", r#"return "10""#];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, stmt) in program.stmts.iter().enumerate() {
            assert_eq!(stmt.to_string(), expected[i])
        }
    }

    #[test]
    fn test_ident_expression() {
        let input = r#"let foobar = "foo""#;
        let expected = vec![r#"let foobar = "foo""#];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, stmt) in program.stmts.iter().enumerate() {
            assert_eq!(stmt.to_string(), expected[i])
        }
    }

    #[test]
    fn test_int_expression() {
        let input = "5";
        let expected = vec!["5"];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, stmt) in program.stmts.iter().enumerate() {
            assert_eq!(stmt.to_string(), expected[i])
        }
    }

    #[test]
    fn test_prefix_expression() {
        let input = "-5;
!5;";
        let expected = vec!["(-5)", "(!5)"];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, stmt) in program.stmts.iter().enumerate() {
            assert_eq!(stmt.to_string(), expected[i])
        }
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
    fn test_array() {
        let input = "[1, 2 * 2, 3 + 3];
        a * [1, 2, 3, 4][b * c] * d;
        add(a * b[2], b[1], 2 * [1, 2][1]);
        ";
        let expected = vec![
            "[1, (2 * 2), (3 + 3)]",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
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
    fn test_string() {
        let input = r#""foobar""#;
        let expected = vec![r#""foobar""#];
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

    #[test]
    fn test_function_literal() {
        let input = r#"fn(x,y){x+y};
        fn(){1+1};"#;
        let expected = vec!["fn(x,y){(x + y)}", "fn(){(1 + 1)}"];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_call_expr() {
        let input = r#"add(1, 2 * 3, 4 + 5);"#;
        let expected = vec!["add(1, (2 * 3), (4 + 5))"];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_hash_literal() {
        let input = r#"{"one": 1, "two": 2, "three": 3};
        {};
        {"one": 0 + 1, "two": 10 - 8, "three": 15 / 5};
        "#;
        let expected = vec![
            r#"{"one": 1, "two": 2, "three": 3}"#,
            "{}",
            r#"{"one": (0 + 1), "two": (10 - 8), "three": (15 / 5)}"#,
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }
}
