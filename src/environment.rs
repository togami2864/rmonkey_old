use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    pub(crate) store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }
    pub fn get(&mut self, key: String) -> Option<Object> {
        match self.store.get(&key) {
            Some(val) => Some(val.clone()),
            None => match self.outer {
                Some(ref outer) => outer.borrow_mut().get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: String, val: Object) {
        self.store.insert(key, val);
    }

    pub fn new_enclosed_env(outer: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }
}
