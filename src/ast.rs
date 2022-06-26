use std::fmt;

use crate::operator::Prefix;

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in self.stmts.iter() {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    LetStatement { ident: Expr, value: Expr },
    ReturnStatement { value: Expr },
    ExpressionStatement { expr: Expr },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::LetStatement { ident, value } => {
                write!(f, "let {} = {}", ident, value)
            }
            Stmt::ReturnStatement { value } => {
                write!(f, "return {}", value)
            }
            Stmt::ExpressionStatement { expr } => {
                write!(f, "{}", expr)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Ident(String),
    Int(i64),
    PrefixExpr { op: Prefix, right: Box<Expr> },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::Int(val) => write!(f, "{}", val),
            Expr::PrefixExpr { op, right } => write!(f, "({}{})", op, right),
        }
    }
}
