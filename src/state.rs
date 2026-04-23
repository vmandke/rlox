/*
Chain-based Environment

Option<Rc<Environment>>:: Reference copied parent scope.
        Multiple owners, possibly will work for closures
        TODO(vin) Test!!!

Rc<RefCell<InterpretedResult>>>
Reference Counting + RefCell (mutability across various scopes)
            a = 1
            {
                ...
            }
            a = 2
 */

use crate::grammar::InterpretedResult;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    enclosing: Option<Rc<Environment>>,
    values: HashMap<String, Rc<RefCell<InterpretedResult>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(parent: Rc<Environment>) -> Self {
        Environment {
            enclosing: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Rc<RefCell<InterpretedResult>>) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Rc<RefCell<InterpretedResult>>> {
        if let Some(rc) = self.values.get(key) {
            return Some(Rc::clone(rc));
        }
        if let Some(parent) = &self.enclosing {
            return parent.get(key);
        }
        None
    }
}
