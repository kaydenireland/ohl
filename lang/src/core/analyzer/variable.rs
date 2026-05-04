
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
    NULL
}
