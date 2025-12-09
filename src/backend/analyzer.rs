use std::collections::HashMap;
use crate::backend::{converter::VariableType, logger::Logger};


#[derive(Debug)]
pub struct FunctionSignature {
    parameters: Vec<VariableType>,
    return_type: VariableType
}

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

pub struct Analyzer {
    pub scopes: Vec<HashMap<String, VariableType>>, 
    pub functions: HashMap<String, FunctionSignature>,
    pub errors: Vec<String>,
    pub log: Logger,
}

impl Analyzer {
    pub fn new(_debug: bool) -> Analyzer {
        let log = Logger::new(_debug);
        Analyzer {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            errors: vec![],
            log,
        }
    }
}

