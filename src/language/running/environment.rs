use std::collections::HashMap;

use crate::language::running::value::{Binding, Value};


pub struct Environment {
    scopes: Vec<HashMap<String, Binding>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { scopes: vec![HashMap::new()] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn declare(&mut self, name: String, value: Value, mutable: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, Binding::new(value, mutable));
        }
    }


    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(binding) = scope.get_mut(name) {
                return binding.set(value);
            }
        }
        Err(format!("Variable '{}' not found", name))
    }


    pub fn get(&self, name: &str) -> Result<Value, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(binding) = scope.get(name) {
                return Ok(binding.value().clone());
            }
        }
        Err(format!("Variable '{}' not found", name))
    }
    
}