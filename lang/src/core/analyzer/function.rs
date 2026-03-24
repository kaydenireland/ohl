use crate::core::analyzer::variable::VariableType;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<VariableType>,
    pub return_type: VariableType,
    pub called: bool
}

impl FunctionSignature {
    pub fn new(name: String, parameters: Vec<VariableType>, return_type: VariableType, called: bool) -> FunctionSignature {
        FunctionSignature {
            name,
            parameters,
            return_type,
            called
        }
    }

    pub fn call(&mut self) {
        self.called = true;
    }
}