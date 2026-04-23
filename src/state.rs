/*
Chain-based Environment

Option<Rc<Environment>>:: Reference copied parent scope.
        Multiple owners, possibly will work for closures
        TODO(vin) Test!!!

Rc<RefCell<InterpretedResult>>>
Reference Counting + RefCell (mutability across various scopes)
            var a = 1
            {
                a = 2
            }
            print a // this should be 2?

            var a = 1
            {
                var a = 2
            }
            print a // this should be 2?
 */

use crate::grammar::InterpretedResult;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Rc<RefCell<InterpretedResult>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Rc<RefCell<InterpretedResult>>) {
        // Set will always happen on own scope
        self.values.insert(key, value);
    }

    pub fn assign(&mut self, key: &str, value: Rc<RefCell<InterpretedResult>>) -> bool {
        // Can update enclosing hashmaps
        if self.values.contains_key(key) {
            self.values.insert(key.to_string(), value);
            return true;
        }
        if let Some(parent) = &self.enclosing {
            return parent.borrow_mut().assign(key, value);
        }
        false
    }

    pub fn get(&self, key: &str) -> Option<Rc<RefCell<InterpretedResult>>> {
        if let Some(rc) = self.values.get(key) {
            return Some(Rc::clone(rc));
        }
        if let Some(parent) = &self.enclosing {
            return parent.borrow().get(key);
        }
        None
    }
}
