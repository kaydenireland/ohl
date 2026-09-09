use crate::core::codegen::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>
}

impl Function {
    pub fn new(name: String, instructions: Vec<Instruction>) -> Function {
        Function { name, instructions }
    }
}