use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{self, Expr},
    buildin::lookup,
    environment::Environment,
    error::{MonkeyError, Result},
    object::Object,
    operator::{Infix, Prefix},
};

#[derive(Debug)]
pub struct Evaluator {
    pub env: Rc<RefCell<Environment>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn from(env: Environment) -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn set(&mut self, key: String, val: Object) {
        self.env.borrow_mut().set(key, val);
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        self.env.borrow_mut().get(key.to_string())
    }

    pub fn eval(&mut self, node: ast::Program) -> Result<Object> {
        let mut result = Object::Null;
        for stmt in node.stmts.iter() {
            result = self.eval_stmt(stmt)?;
            if let Object::ReturnValue(_) = result {
                return Ok(result);
            }
        }
        Ok(result)
    }

    pub fn eval_stmt(&mut self, stmt: &ast::Stmt) -> Result<Object> {
        match stmt {
            ast::Stmt::LetStatement { ident, value } => {
                let val = self.eval_expr(value)?;
                self.env.borrow_mut().set(ident.to_string(), val);
                Ok(Object::Null)
            }
            ast::Stmt::ReturnStatement { value } => {
                let right = self.eval_expr(value)?;
                Ok(Object::ReturnValue(Box::new(right)))
            }
            ast::Stmt::ExpressionStatement { expr } => self.eval_expr(expr),
            ast::Stmt::BlockStatement { stmts } => self.eval_block_stmt(stmts),
        }
    }

    pub fn eval_block_stmt(&mut self, stmts: &[ast::Stmt]) -> Result<Object> {
        let mut result = Object::Null;
        for s in stmts.iter() {
            result = self.eval_stmt(s)?;
            if let Object::ReturnValue(_) = result {
                return Ok(result);
            }
        }
        Ok(result)
    }

    pub fn eval_expr(&mut self, expr: &ast::Expr) -> Result<Object> {
        match expr {
            ast::Expr::Ident(ident) => match self.env.borrow_mut().get(ident.to_string()) {
                Some(val) => Ok(val),
                None => Err(MonkeyError::UncaughtRef(ident.to_string())),
            },
            ast::Expr::String(val) => Ok(Object::String(val.to_string())),
            ast::Expr::Int(val) => Ok(Object::Integer(*val)),
            ast::Expr::Boolean(val) => Ok(Object::Boolean(*val)),
            ast::Expr::PrefixExpr { op, right } => {
                let right = self.eval_expr(right)?;
                self.eval_prefix_expr(op, right)
            }
            ast::Expr::InfixExpr { left, right, op } => {
                let left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;
                self.eval_infix_expr(left, right, op)
            }
            ast::Expr::IfExpr {
                condition,
                consequence,
                alternative,
            } => {
                if self.eval_expr(condition)?.is_truthy() {
                    self.eval_stmt(consequence)
                } else {
                    match alternative {
                        Some(alt) => self.eval_stmt(alt),
                        None => Ok(Object::Null),
                    }
                }
            }
            ast::Expr::FuncLiteral { parameters, body } => Ok(Object::FunctionLiteral {
                params: parameters.to_vec(),
                body: *body.clone(),
                env: Environment::new_enclosed_env(Rc::clone(&self.env)),
            }),
            ast::Expr::CallExpr { function, args } => {
                let args = self.eval_call_expr(args.to_vec())?;
                if let ast::Expr::Ident(func) = &**function {
                    match lookup(func) {
                        Some(func) => match func {
                            Object::BuildIn(f) => f(args),
                            _ => todo!(),
                        },
                        None => {
                            let func = self.eval_expr(function)?;
                            self.apply_function(func, args)
                        }
                    }
                } else {
                    let func = self.eval_expr(function)?;
                    self.apply_function(func, args)
                }
            }
            Expr::ArrayLiteral { elements } => {
                let elements = self.eval_call_expr(elements.to_vec())?;
                Ok(Object::Array { elements })
            }
            Expr::IndexExpr { left, index } => {
                let left = self.eval_expr(left)?;
                let index = self.eval_expr(index)?;
                match (left, index) {
                    (Object::Array { elements }, Object::Integer(index)) => {
                        match elements.get(index as usize) {
                            Some(obj) => Ok(obj.clone()),
                            None => todo!(),
                        }
                    }
                    _ => Err(MonkeyError::Custom(
                        "index operator not supported".to_string(),
                    )),
                }
            }
        }
    }

    pub fn eval_prefix_expr(&mut self, op: &Prefix, right: Object) -> Result<Object> {
        match op {
            Prefix::Bang => match right {
                Object::Boolean(val) => Ok(Object::Boolean(!val)),
                Object::Null => Ok(Object::Boolean(true)),
                _ => Ok(Object::Boolean(false)),
            },
            Prefix::Minus => match right {
                Object::Integer(val) => Ok(Object::Integer(-val)),
                _ => Err(MonkeyError::UnknownPrefix(
                    op.clone(),
                    "BOOLEAN".to_string(),
                )),
            },
        }
    }

    pub fn eval_infix_expr(&mut self, left: Object, right: Object, op: &Infix) -> Result<Object> {
        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => match op {
                Infix::Plus => Ok(Object::Integer(left + right)),
                Infix::Minus => Ok(Object::Integer(left - right)),
                Infix::Asterisk => Ok(Object::Integer(left * right)),
                Infix::Slash => Ok(Object::Integer(left / right)),
                Infix::Gt => Ok(Object::Boolean(left < right)),
                Infix::Lt => Ok(Object::Boolean(left > right)),
                Infix::Eq => Ok(Object::Boolean(left == right)),
                Infix::NotEq => Ok(Object::Boolean(left != right)),
            },
            (Object::Boolean(left), Object::Boolean(right)) => match op {
                Infix::Eq => Ok(Object::Boolean(left == right)),
                Infix::NotEq => Ok(Object::Boolean(left != right)),
                _ => Err(MonkeyError::UnknownOperator(
                    "BOOLEAN".to_string(),
                    "BOOLEAN".to_string(),
                    op.clone(),
                )),
            },
            (Object::String(left), Object::String(right)) => match op {
                Infix::Plus => Ok(Object::String(format!("{}{}", left, right))),
                _ => Err(MonkeyError::UnknownOperator(
                    "STRING".to_string(),
                    "STRING".to_string(),
                    op.clone(),
                )),
            },
            (left, right) => Err(MonkeyError::TypeMismatch(
                left.obj_type(),
                right.obj_type(),
                op.clone(),
            )),
        }
    }

    pub fn eval_call_expr(&mut self, params: Vec<Expr>) -> Result<Vec<Object>> {
        let mut result: Vec<Object> = Vec::new();
        for p in params.iter() {
            let evaluated = self.eval_expr(p)?;
            result.push(evaluated);
        }
        Ok(result)
    }

    pub fn apply_function(&mut self, function: Object, args: Vec<Object>) -> Result<Object> {
        if let Object::FunctionLiteral { params, body, env } = function {
            let mut env = Evaluator::from(env);
            for (ident, arg) in params.iter().zip(args.iter()) {
                if let ast::Expr::Ident(ident) = ident {
                    env.set(ident.to_owned(), arg.clone())
                }
            }
            match env.eval_stmt(&body) {
                Ok(Object::ReturnValue(val)) => Ok(*val),
                obj => obj,
            }
        } else {
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::Evaluator;

    #[test]
    fn test_integer_ope() {
        let case = [
            ("5", "5"),
            ("10", "10"),
            ("-5", "-5"),
            ("-10", "-10"),
            ("5 + 5 + 5 + 5 - 10", "10"),
            ("2 * 2 * 2 * 2 * 2", "32"),
            ("-50 + 100 - 50", "0"),
            ("5 * 2 + 10", "20"),
            ("5 + 2 * 10", "25"),
            ("50 / 2 * 2 + 10", "60"),
            ("2 * (5 + 10)", "30"),
            ("3 * 3 * 3 + 10", "37"),
            ("3 * (3 * 3) + 10", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
        ];

        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }

    #[test]
    fn test_bang_ope() {
        let case = [
            ("!false", "true"),
            ("!5", "false"),
            ("!!true", "true"),
            ("!!false", "false"),
            ("!!5", "true"),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected);
        }
    }

    #[test]
    fn test_boolean_expr() {
        let case = [
            ("1 < 2", "true"),
            ("1 > 2", "false"),
            ("1 < 1", "false"),
            ("1 > 1", "false"),
            ("1 == 1", "true"),
            ("1 != 1", "false"),
            ("1 == 2", "false"),
            ("1 != 2", "true"),
            ("true == true", "true"),
            ("false == false", "true"),
            ("(1 < 2) == true", "true"),
            ("(1 < 2) == false", "false"),
            ("(1 > 2) == true", "false"),
            ("(1 > 2) == false", "true"),
        ];

        for (input, expected) in case {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), expected)
        }
    }

    #[test]
    fn test_string() {
        let case = [
            (r#""foobar""#, r#""foobar""#),
            (r#""Hello" + " " + "World""#, r#""Hello World""#),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }

    #[test]
    fn test_array() {
        let case = [("[1, 2 * 2, 3 + 3]", "[1, 4, 6]")];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }

    #[test]
    fn test_if_else_expr() {
        let case = [("if(true){10}", "10"), ("if (false) { 10 }", "null")];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }

    #[test]
    fn test_return_stmt() {
        let case = [
            ("return 10", "10"),
            ("return 2 * 5", "10"),
            (
                "if (10 > 1) { if (10 > 1) {
            return 10; }
            return 1; }",
                "10",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }

    #[test]
    fn test_error() {
        let case = [
            ("5 + true", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown prefix: -BOOLEAN"),
            ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if(10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { if (10 > 1) { return true + false} return 1;}",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "Uncaught ReferenceError: foobar is not defined"),
            (r#""Hello" - "World""#, "unknown operator: STRING - STRING"),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program);
            assert_eq!(r.unwrap_err().to_string(), *expected);
        }
    }
    #[test]
    fn test_let_statement() {
        let case = [
            ("let a = 5; a;", "5"),
            ("let a = 5 * 5; a;", "25"),
            ("let a = 5; let b = a; b;", "5"),
            ("let a = 5; let b = a; let c = a + b + 5; c;", "15"),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }
    #[test]
    fn test_function_literal() {
        let case = [
            ("let identity = fn(x){ x; }; identity(5);", "5"),
            ("let identity = fn(x){ return x; }; identity(5);", "5"),
            ("let double = fn(x) { x * 2; }; double(5);", "10"),
            ("let add = fn(x, y){ x + y;}; add(5, 5);", "10"),
            ("let add = fn(x, y){ x + y;}; add(5 + 5, add(5, 5));", "20"),
            // FIXME: parser
            // ("fn(x) { x }(5)", "5"),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }
    #[test]
    fn test_buildin_string_len() {
        let case = [
            (r#"len("")"#, "0"),
            (r#"len("four")"#, "4"),
            (r#"len("hello world")"#, "11"),
            (r#"len(1)"#, "arg to `len` not supported, got INTEGER"),
            (
                r#"len("one", "two")"#,
                "wrong number of arguments. got=2, want=1",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_buildin_array_len() {
        let case = [
            (r#"len([])"#, "0"),
            (r#"len([1,2,3,4])"#, "4"),
            (r#"len(["1","2","3","4"])"#, "4"),
            (r#"len([1,"2",3,"4"])"#, "4"),
            (
                r#"len(["one"], ["two"])"#,
                "wrong number of arguments. got=2, want=1",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_buildin_array_first() {
        let case = [
            (r#"first([])"#, "this array is empty"),
            (r#"first([1,2,3,4])"#, "1"),
            (r#"first(["1","2","3","4"])"#, r#""1""#),
            (
                r#"first(["one"], ["two"])"#,
                "wrong number of arguments. got=2, want=1",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_buildin_array_last() {
        let case = [
            (r#"last([])"#, "this array is empty"),
            (r#"last([1,2,3,4])"#, "4"),
            (r#"last(["1","2","3","4"])"#, r#""4""#),
            (
                r#"last(["one"], ["two"])"#,
                "wrong number of arguments. got=2, want=1",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_buildin_array_rest() {
        let case = [
            (r#"rest([])"#, "this array is empty"),
            (r#"rest([1,2,3,4])"#, "[2, 3, 4]"),
            (r#"rest(["1","2","3","4"])"#, r#"["2", "3", "4"]"#),
            (
                r#"rest(["one"], ["two"])"#,
                "wrong number of arguments. got=2, want=1",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_buildin_array_push() {
        let case = [
            (r#"push([], 0)"#, "[0]"),
            (r#"push([1,2,3,4], 5)"#, "[1, 2, 3, 4, 5]"),
            (
                r#"push(["1","2","3","4"], "5")"#,
                r#"["1", "2", "3", "4", "5"]"#,
            ),
            (
                r#"push(["one"], ["two"], ["three"])"#,
                "wrong number of arguments. got=3, want=2",
            ),
            (
                r#"push("one", "two")"#,
                "arg to `push` not supported, got STRING",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }
}
