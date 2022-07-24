use crate::{
    ast::{self, Expr},
    error::Result,
    object::Object,
    operator::{Infix, Prefix},
};

pub fn eval(node: ast::Program) -> Result<Object> {
    let mut result = Object::Null;
    for stmt in node.stmts.iter() {
        result = eval_stmt(stmt)?;
        if let Object::ReturnValue(_) = result {
            return Ok(result);
        }
    }
    Ok(result)
}

pub fn eval_stmt(stmt: &ast::Stmt) -> Result<Object> {
    match stmt {
        ast::Stmt::LetStatement { ident, value } => todo!(),
        ast::Stmt::ReturnStatement { value } => {
            let right = eval_expr(value)?;
            Ok(Object::ReturnValue(Box::new(right)))
        }
        ast::Stmt::ExpressionStatement { expr } => eval_expr(expr),
        ast::Stmt::BlockStatement { stmts } => eval_block_stmt(stmts),
    }
}

pub fn eval_block_stmt(stmts: &[ast::Stmt]) -> Result<Object> {
    let mut result = Object::Null;
    for s in stmts.iter() {
        result = eval_stmt(s)?;
        if let Object::ReturnValue(_) = result {
            return Ok(result);
        }
    }
    Ok(result)
}

pub fn eval_expr(expr: &ast::Expr) -> Result<Object> {
    match expr {
        ast::Expr::Ident(_) => todo!(),
        ast::Expr::Int(val) => Ok(Object::Integer(*val)),
        ast::Expr::Boolean(val) => Ok(Object::Boolean(*val)),
        ast::Expr::PrefixExpr { op, right } => {
            let right = eval_expr(right)?;
            eval_prefix_expr(op, right)
        }
        ast::Expr::InfixExpr { left, right, op } => {
            let left = eval_expr(left)?;
            let right = eval_expr(right)?;
            eval_infix_expr(left, right, op)
        }
        ast::Expr::IfExpr {
            condition,
            consequence,
            alternative,
        } => {
            if eval_expr(condition)?.is_truthy() {
                eval_stmt(consequence)
            } else {
                match alternative {
                    Some(alt) => eval_stmt(alt),
                    None => Ok(Object::Null),
                }
            }
        }
        ast::Expr::FuncLiteral { parameters, body } => todo!(),
        ast::Expr::CallExpr { function, args } => todo!(),
    }
}

pub fn eval_prefix_expr(op: &Prefix, right: Object) -> Result<Object> {
    match op {
        Prefix::Bang => match right {
            Object::Boolean(val) => Ok(Object::Boolean(!val)),
            Object::Null => Ok(Object::Boolean(true)),
            _ => Ok(Object::Boolean(false)),
        },
        Prefix::Minus => match right {
            Object::Integer(val) => Ok(Object::Integer(-val)),
            _ => Ok(Object::Null),
        },
    }
}

pub fn eval_infix_expr(left: Object, right: Object, op: &Infix) -> Result<Object> {
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
            _ => Ok(Object::Null),
        },
        (Object::Boolean(left), Object::Boolean(right)) => match op {
            Infix::Eq => Ok(Object::Boolean(left == right)),
            Infix::NotEq => Ok(Object::Boolean(left != right)),
            _ => Ok(Object::Null),
        },
        _ => Ok(Object::Null),
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::eval;

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
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = eval(program).unwrap();
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
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = eval(program).unwrap();
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
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = eval(program).unwrap();
            assert_eq!(r.to_string(), expected)
        }
    }

    #[test]
    fn test_if_else_expr() {
        let case = [("if(true){10}", "10"), ("if (false) { 10 }", "null")];
        for (input, expected) in case.iter() {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = eval(program).unwrap();
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
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }

    #[test]
    fn test_error() {
        let case = [
            ("5 + true", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
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
        ];
        for (input, expected) in case.iter() {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }
}
