use std::fmt;

use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
};

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Box<Object>),
    FunctionLiteral {
        params: Vec<Expr>,
        body: Stmt,
        env: Environment,
    },
    BuildIn,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::String(val) => write!(f, "{}", val),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::FunctionLiteral { body, params, .. } => {
                write!(
                    f,
                    "fn({}){{{}}}",
                    params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    body
                )
            }
            Object::BuildIn => todo!(),
        }
    }
}

impl Object {
    pub fn obj_type(&self) -> String {
        match self {
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Boolean(_) => "BOOLEAN".to_string(),
            Object::String(_) => "STRING".to_string(),
            Object::Null => "NULL".to_string(),
            Object::ReturnValue(_) => todo!(),
            Object::FunctionLiteral { .. } => "FunctionLiteral".to_string(),
            Object::BuildIn => "BUILDIN".to_string(),
        }
    }
    pub fn is_truthy(&mut self) -> bool {
        match self {
            Object::Null => false,
            Object::Boolean(val) => *val,
            _ => true,
        }
    }
}
