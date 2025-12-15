use std::collections::HashMap;
use crate::language::logger::Logger;
use crate::language::analyzing::types::VariableType;
use crate::language::analyzing::symbol_table::SymbolTable;



#[derive(Debug)]
pub struct FunctionSignature {
    parameters: Vec<VariableType>,
    return_type: VariableType
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


