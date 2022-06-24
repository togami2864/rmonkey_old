use std::fmt;

use crate::token::Token;

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
        // self.stmts.to_string()
        unimplemented!()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    LetStatement { ident: Expr, value: Expr },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Ident(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
