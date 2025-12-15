use std::collections::HashMap;
use crate::language::analyzing::types::VariableType;

#[derive(Debug)]
pub struct SymbolTable {
    variables: HashMap<String, VariableType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
        }
    }

    // TODO: Proper Error Handling
    pub fn declare_variable(&mut self, name: String, var_type: VariableType) {
        if self.variables.contains_key(&name) {
            panic!("Variable '{}' is already declared.", name);
        } else {
            self.variables.insert(name, var_type);
        }
    }

    pub fn check_variable(&self, name: &String) -> Result<VariableType, String> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Variable '{}' is not declared.", name))

    }
}