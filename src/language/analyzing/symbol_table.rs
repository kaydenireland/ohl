use std::collections::HashMap;
use crate::language::analyzing::types::VariableType;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    variables: HashMap<String, VariableType>,
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    /// Create a root (global) symbol table
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            parent: None,
        }
    }

    /// Create a child scope
    pub fn new_child(parent: &SymbolTable) -> Self {
        SymbolTable {
            variables: HashMap::new(),
            parent: Some(Box::new(parent.clone())),
        }
    }

    /// Declare a variable in the current scope only
    pub fn declare_variable(
        &mut self,
        name: String,
        var_type: VariableType,
    ) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            Err(format!(
                "Variable '{}' is already declared in this scope",
                name
            ))
        } else {
            self.variables.insert(name, var_type);
            Ok(())
        }
    }

    /// Look up a variable, walking up through parent scopes
    pub fn check_variable(&self, name: &String) -> Result<VariableType, String> {
        if let Some(v) = self.variables.get(name) {
            Ok(v.clone())
        } else if let Some(parent) = &self.parent {
            parent.check_variable(name)
        } else {
            Err(format!("Variable '{}' is not declared", name))
        }
    }

    /// Check if a variable exists in the current scope only
    pub fn contains_local(&self, name: &String) -> bool {
        self.variables.contains_key(name)
    }
}
