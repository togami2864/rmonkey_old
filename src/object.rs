use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
        }
    }
}

impl Object {
    pub fn obj_type(&self) -> String {
        match self {
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Boolean(_) => "BOOLEAN".to_string(),
            Object::Null => "NULL".to_string(),
            Object::ReturnValue(_) => unimplemented!(),
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
