use std::fmt;

use crate::operator::{Infix, Prefix};

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
    BlockStatement { stmts: Vec<Stmt> },
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
            Stmt::BlockStatement { stmts } => {
                for stmt in stmts.iter() {
                    write!(f, "{}", stmt)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Boolean(bool),
    PrefixExpr {
        op: Prefix,
        right: Box<Expr>,
    },
    InfixExpr {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Infix,
    },
    IfExpr {
        condition: Box<Expr>,
        consequence: Box<Stmt>,
        alternative: Option<Box<Stmt>>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::Int(val) => write!(f, "{}", val),
            Expr::Boolean(val) => write!(f, "{}", val),
            Expr::PrefixExpr { op, right } => write!(f, "({}{})", op, right),
            Expr::InfixExpr { left, right, op } => write!(f, "({} {} {})", left, op, right),
            Expr::IfExpr {
                condition,
                consequence,
                alternative,
            } => match alternative {
                Some(alt) => write!(f, "if({}){{{}}}else{{{}}}", condition, consequence, alt),
                None => write!(f, "if({}){{{}}}", condition, consequence),
            },
        }
    }
}
