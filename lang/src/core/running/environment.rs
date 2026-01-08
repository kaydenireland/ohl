use std::collections::HashMap;

use crate::core::{analyzing::stree::STree, running::value::{Binding, Value}};


pub struct Scope {
    pub bindings: HashMap<String, Binding>,
    pub deferred: Vec<STree>
}

pub struct Environment {
    scopes: Vec<Scope>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { 
            scopes: vec![
                Scope {
                    bindings: HashMap::new(),
                    deferred: Vec::new()
                }
            ]
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope { bindings: HashMap::new(), deferred: Vec::new() });
    }

    pub fn peek_scope(&mut self) -> Option<&Scope> {
        if self.scopes.len() > 1 {
            self.scopes.last()
        } else {
            None
        }
    }

    pub fn pop_scope(&mut self) -> Option<Scope> {
        if self.scopes.len() > 1 {
            self.scopes.pop()
        } else {
            None
        }
    }

    pub fn declare(&mut self, name: String, value: Value, mutable: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.bindings.insert(name, Binding::new(value, mutable));
        }
    }


    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(binding) = scope.bindings.get_mut(name) {
                return binding.set(value);
            }
        }
        Err(format!("Variable '{}' not found", name))
    }


    pub fn get(&self, name: &str) -> Result<Value, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(binding) = scope.bindings.get(name) {
                return Ok(binding.value().clone());
            }
        }
        Err(format!("Variable '{}' not found", name))
    }

    pub fn is_mutable(&self, name: &str) -> Result<bool, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(binding) = scope.bindings.get(name) {
                return Ok(binding.mutable.clone());
            }
        }
        Err(format!("Variable '{}' not found", name))
    }
    
    pub fn defer(&mut self, stmt: STree) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.deferred.push(stmt);
        }
    }

}