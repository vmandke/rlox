/*
State for the Scopes

Simple Stack Based Environment;;
This wont work for closures..
    Not even sure how closures scopes will be retianed.
    FIXME(vin): Look into Rc / RefCell as pointed in the Project Docs
 */

use crate::grammar::InterpretedResult;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, InterpretedResult>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: InterpretedResult) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> InterpretedResult {
        self.values
            .get(key)
            .cloned()
            .unwrap_or(InterpretedResult::Nil)
    }
}
