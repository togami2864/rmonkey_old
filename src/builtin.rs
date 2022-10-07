use crate::error::{MonkeyError, Result};
use crate::object::Object;

macro_rules! builtin {
    ($name:ident) => {
        BuiltIn {
            name: stringify!($name),
            builtin: Object::BuiltIn($name),
        }
    };
}

#[derive(Debug)]
pub struct BuiltIn {
    pub name: &'static str,
    pub builtin: Object,
}

pub const BUILTIN: &[BuiltIn] = &[
    builtin!(len),
    builtin!(first),
    builtin!(last),
    builtin!(rest),
    builtin!(push),
    builtin!(puts),
];

pub fn lookup(name: &str) -> Option<Object> {
    for func in BUILTIN {
        if func.name == name {
            return Some(func.builtin.clone());
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

fn first(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(MonkeyError::Custom(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }
    match &args[0] {
        Object::Array { elements } => {
            if elements.is_empty() {
                return Err(MonkeyError::Custom("this array is empty".to_string()));
            }
            match elements.get(0) {
                Some(obj) => Ok(obj.clone()),
                None => {
                    return Err(MonkeyError::Custom(format!(
                        "wrong number of arguments. got={}, want=1",
                        args.len()
                    )));
                }
            }
        }
        arg => {
            return Err(MonkeyError::Custom(format!(
                "arg to `first` not supported, got {}",
                arg.obj_type()
            )))
        }
    }
}

fn last(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(MonkeyError::Custom(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }
    match &args[0] {
        Object::Array { elements } => {
            if elements.is_empty() {
                return Err(MonkeyError::Custom("this array is empty".to_string()));
            }
            let last_index = elements.len() - 1;
            match elements.get(last_index) {
                Some(obj) => Ok(obj.clone()),
                None => {
                    return Err(MonkeyError::Custom(format!(
                        "wrong number of arguments. got={}, want=1",
                        args.len()
                    )));
                }
            }
        }
        arg => {
            return Err(MonkeyError::Custom(format!(
                "arg to `last` not supported, got {}",
                arg.obj_type()
            )))
        }
    }
}

fn rest(args: Vec<Object>) -> Result<Object> {
    if args.len() != 1 {
        return Err(MonkeyError::Custom(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        )));
    }
    match &args[0] {
        Object::Array { elements } => {
            if elements.is_empty() {
                return Err(MonkeyError::Custom("this array is empty".to_string()));
            }
            let elements: Vec<Object> = elements.clone().drain(1..).collect();
            Ok(Object::Array { elements })
        }
        arg => {
            return Err(MonkeyError::Custom(format!(
                "arg to `last` not supported, got {}",
                arg.obj_type()
            )))
        }
    }
}

fn push(args: Vec<Object>) -> Result<Object> {
    if args.len() != 2 {
        return Err(MonkeyError::Custom(format!(
            "wrong number of arguments. got={}, want=2",
            args.len()
        )));
    }

    match &args[0] {
        Object::Array { elements } => {
            let mut new_ele = elements.clone();
            let len = new_ele.len();
            new_ele.insert(len, args[1].clone());
            Ok(Object::Array { elements: new_ele })
        }
        arg => {
            return Err(MonkeyError::Custom(format!(
                "arg to `push` not supported, got {}",
                arg.obj_type()
            )))
        }
    }
}

fn puts(args: Vec<Object>) -> Result<Object> {
    for a in args.iter() {
        println!("{}", a);
    }
    Ok(Object::Null)
}
