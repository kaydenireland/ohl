#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub class_name: String,
    pub name: String,
    pub parameters: Vec<VariableType>,
    pub return_type: VariableType,
    pub called: bool
}

impl FunctionSignature {
    pub fn new(class_name: String, name: String, parameters: Vec<VariableType>, return_type: VariableType, called: bool) -> FunctionSignature {
        FunctionSignature {
            class_name,
            name,
            parameters,
            return_type,
            called
        }
    }

    pub fn call(&mut self) {
        self.called = true;
    }

    pub fn key(&self) -> String {
        format!("{}::{}", self.class_name, self.name)
    }
}


#[derive(Debug, Clone)]
pub struct VariableSignature {
    pub var_type: VariableType,
    pub used: bool,
    pub mutable: bool
}

impl VariableSignature {
    pub fn new(var_type: VariableType, used: bool, mutable: bool) -> VariableSignature {
        VariableSignature {
            var_type,
            used,
            mutable
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    INT,
    FLOAT,
    CHAR,
    STRING,
    BOOLEAN,

    FUNCTION,
    CLASS,

    OBJECT,
    VOID,

    UNKNOWN
}
