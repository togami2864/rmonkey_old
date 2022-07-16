use crate::{ast, object::Object};

pub fn eval(node: ast::Program) {
    for stmt in node.stmts.iter() {
        let obj = eval_stmt(stmt);
        println!("{}", obj);
    }
}

pub fn eval_stmt(stmt: &ast::Stmt) -> Object {
    match stmt {
        ast::Stmt::LetStatement { ident, value } => todo!(),
        ast::Stmt::ReturnStatement { value } => todo!(),
        ast::Stmt::ExpressionStatement { expr } => eval_expr(expr),
        ast::Stmt::BlockStatement { stmts } => todo!(),
    }
}

pub fn eval_expr(expr: &ast::Expr) -> Object {
    match expr {
        ast::Expr::Ident(_) => todo!(),
        ast::Expr::Int(val) => Object::Integer(*val),
        ast::Expr::Boolean(val) => Object::Boolean(*val),
        ast::Expr::PrefixExpr { op, right } => todo!(),
        ast::Expr::InfixExpr { left, right, op } => todo!(),
        ast::Expr::IfExpr {
            condition,
            consequence,
            alternative,
        } => todo!(),
        ast::Expr::FuncLiteral { parameters, body } => todo!(),
        ast::Expr::CallExpr { function, args } => todo!(),
    }
}
