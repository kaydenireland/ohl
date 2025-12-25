use std::collections::HashMap;
use crate::language::analyzing::types::VariableType;

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub var_type: VariableType,
    pub used: bool,
    pub mutable: bool
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub variables: HashMap<String, VariableInfo>,
    pub parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_child(parent: &SymbolTable) -> Self {
        SymbolTable {
            variables: HashMap::new(),
            parent: Some(Box::new(parent.clone())),
        }
    }

    pub fn declare_variable(&mut self, name: String, var_type: VariableType, mutable: bool) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            Err(format!(
                "Variable '{}' is already declared in this scope",
                name
            ))
        } else {
            self.variables.insert(name, VariableInfo { var_type, used: false, mutable });
            Ok(())
        }
    }

    pub fn check_variable(&self, name: &String) -> Result<VariableType, String> {
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

    pub fn mark_used(&mut self, name: &str) -> Result<VariableType, String> {
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
