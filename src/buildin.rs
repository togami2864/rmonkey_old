use crate::error::{MonkeyError, Result};
use crate::object::Object;

macro_rules! buildin {
    ($name:ident) => {
        BuildIn {
            name: stringify!($name),
            buildin: Object::BuildIn($name),
        }
    };
}

#[derive(Debug)]
pub struct BuildIn {
    pub name: &'static str,
    pub buildin: Object,
}

pub const BUILDIN: &[BuildIn] = &[buildin!(len)];

pub fn lookup(name: &str) -> Option<Object> {
    for func in BUILDIN {
        if func.name == name {
            return Some(func.buildin.clone());
        }
    }
    None
}

fn len(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(MonkeyError::Custom(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }
    match &args[0] {
        Object::String(val) => {
            let val = val.len().try_into().unwrap();
            Ok(Object::Integer(val))
        }
        Object::Array { elements } => Ok(Object::Integer(elements.len().try_into()?)),
        arg => {
            return Err(MonkeyError::Custom(format!(
                "arg to `len` not supported, got {}",
                arg.obj_type()
            )))
        }
    }
}
