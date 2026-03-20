use std::collections::HashMap;
use crate::core::analyzer::variable::VariableSignature;
use crate::core::lexer::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Scope {
    pub variables: HashMap<String, VariableSignature>,
    pub parent: Option<Box<Scope>>
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_child(parent: &Scope) -> Self {
        Scope {
            variables: HashMap::new(),
            parent: Some(Box::new(parent.clone())),
        }
    }

    pub fn declare_variable(&mut self, name: String, var_type: TokenType, mutable: bool) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            Err(format!(
                "Variable '{}' is already declared in this scope",
                name
            ))
        } else {
            self.variables.insert(name, VariableSignature { var_type, used: false, mutable });
            Ok(())
        }
    }

    pub fn check_variable(&self, name: &String) -> Result<TokenType, String> {
        if let Some(v) = self.variables.get(name) {
            Ok(v.var_type.clone())
        } else if let Some(parent) = &self.parent {
            parent.check_variable(name)
        } else {
            Err(format!("Variable '{}' is not declared", name))
        }
    }

    pub fn check_mutability(&self, name: &String) -> Result<bool, String> {
        if let Some(v) = self.variables.get(name) {
            Ok(v.mutable.clone())
        } else if let Some(parent) = &self.parent {
            parent.check_mutability(name)
        } else {
            Err(format!("Variable '{}' is not declared", name))
        }
    }

    pub fn contains_local(&self, name: &String) -> bool {
        self.variables.contains_key(name)
    }

    pub fn mark_used(&mut self, name: &str) -> Result<TokenType, String> {
        if let Some(v) = self.variables.get_mut(name) {
            v.used = true;
            Ok(v.var_type.clone())
        } else if let Some(parent) = &mut self.parent {
            parent.mark_used(name)
        } else {
            Err(format!("Variable '{}' is not declared.", name))
        }
    }

}