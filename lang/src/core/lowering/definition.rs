use crate::core::lowering::instruction::AssemblyInstruction;

#[derive(Debug, Clone)]
pub struct AssemblyFunction {
    pub name: String,
    pub instructions: Vec<AssemblyInstruction>
}

impl AssemblyFunction {
    pub fn new(name: String, instructions: Vec<AssemblyInstruction>) -> AssemblyFunction {
        AssemblyFunction { name, instructions }
    }
}