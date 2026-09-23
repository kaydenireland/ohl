use crate::core::intermediate::instruction::IntermediateInstruction;

#[derive(Debug, Clone)]
pub struct IntermediateFunction {
    pub name: String,
    pub instructions: Vec<IntermediateInstruction>
}

impl IntermediateFunction {
    pub fn new(name: String, instructions: Vec<IntermediateInstruction>) -> IntermediateFunction {
        IntermediateFunction { name, instructions }
    }
}