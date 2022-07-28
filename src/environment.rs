use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug)]
pub struct Environment {
    pub(crate) store: HashMap<String, Object>,
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
        }
    }
    pub fn get(&self, key: String) -> Option<&Object> {
        match self.store.get(&key) {
            Some(val) => Some(val),
            None => None,
        }
    }

    pub fn set(&mut self, key: String, val: Object) {
        self.store.insert(key, val);
    }
}
