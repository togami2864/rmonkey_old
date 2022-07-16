use crate::{
    ast::{self, Expr},
    error::Result,
    object::Object,
    operator::{Infix, Prefix},
};

pub fn eval(node: ast::Program) -> Result<Vec<Object>> {
    let mut result: Vec<Object> = Vec::new();
    for stmt in node.stmts.iter() {
        let obj = eval_stmt(stmt)?;
        result.push(obj);
    }
    Ok(result)
}

pub fn eval_stmt(stmt: &ast::Stmt) -> Result<Object> {
    match stmt {
        ast::Stmt::LetStatement { ident, value } => todo!(),
        ast::Stmt::ReturnStatement { value } => todo!(),
        ast::Stmt::ExpressionStatement { expr } => eval_expr(expr),
        ast::Stmt::BlockStatement { stmts } => eval_block_stmt(stmts),
    }
}

pub fn eval_block_stmt(stmts: &[ast::Stmt]) -> Result<Object> {
    let mut result = Object::Null;
    for s in stmts.iter() {
        result = eval_stmt(s)?;
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
        let input = r#"5;
10;
-5;
-10;
5 + 5 + 5 + 5 - 10;
2 * 2 * 2 * 2 * 2;
-50 + 100 + -50;
5 * 2 + 10;
5 + 2 * 10;
50 / 2 * 2 + 10;
2 * (5 + 10);
3 * 3 * 3 + 10;
3 * (3 * 3) + 10;
(5 + 10 * 2 + 15 / 3) * 2 + -10;
        "#;
        let expected = [
            "5", "10", "-5", "-10", "10", "32", "0", "20", "25", "60", "30", "37", "37", "50",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        let results = eval(program).unwrap();
        for (i, r) in results.iter().enumerate() {
            assert_eq!(r.to_string(), expected[i])
        }
    }

    #[test]
    fn test_bang_ope() {
        let input = r#"!true;
!false;
!5;
!!true;
!!false;
!!5;
        "#;
        let expected = ["false", "true", "false", "true", "false", "true"];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        let results = eval(program).unwrap();
        for (i, r) in results.iter().enumerate() {
            assert_eq!(r.to_string(), expected[i])
        }
    }

    #[test]
    fn test_boolean_expr() {
        let input = r#"1 < 2;
1 > 2;
1 < 1;
1 > 1;
1 == 1;
1 != 1;
1 == 2;
1 != 2;
true == true
false == false;
(1 < 2) == true;
(1 < 2) == false;
(1 > 2) == true;
(1 > 2) == false;
        "#;
        let expected = [
            "true", "false", "false", "false", "true", "false", "false", "true", "true", "true",
            "true", "false", "false", "true",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        let results = eval(program).unwrap();
        for (i, r) in results.iter().enumerate() {
            assert_eq!(r.to_string(), expected[i])
        }
    }

    #[test]
    fn test_if_else_expr() {
        let case = [("if(true){10}", "10"), ("if (false) { 10 }", "null")];
        for (input, expected) in case.iter() {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let results = eval(program).unwrap();
            for r in results.iter() {
                assert_eq!(r.to_string(), *expected)
            }
        }
    }
}
