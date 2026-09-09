use std::collections::HashMap;
use crate::core::lowering::instruction::AssemblyInstruction;
use crate::core::lowering::operand::Operand;

#[derive(Debug, Clone)]
pub struct AssemblyFunction {
    pub name: String,
    pub instructions: Vec<AssemblyInstruction>
}

impl AssemblyFunction {
    pub fn new(name: String, instructions: Vec<AssemblyInstruction>) -> AssemblyFunction {
        AssemblyFunction { name, instructions }
    }

    pub fn allocate_stack(&mut self) {
        let mut stack_size: i32 = 0;    // in bytes
        let mut vars: HashMap<String, i32> = HashMap::new();
        for instruction in &mut self.instructions {
            match instruction {
                AssemblyInstruction::UNARY { operator, operand } => Self::allocate_operand(operand, &mut vars, &mut stack_size),
                AssemblyInstruction::MOVE { src, dst} => {
                    Self::allocate_operand(src, &mut vars, &mut stack_size);
                    Self::allocate_operand(dst, &mut vars, &mut stack_size)
                },

                _ => {}
            }
        }
        self.instructions.insert(0, AssemblyInstruction::ALLOCATE_STACK(stack_size))
    }

    fn allocate_operand(operand: &mut Operand, stack_slots: &mut HashMap<String, i32>, stack_size: &mut i32) {
        if let Operand::PSEUDO(name) = operand {
            let offset = match stack_slots.get(name) {
                Some(offset) => *offset,
                None => {
                    *stack_size += 4;
                    let offset = -*stack_size;
                    stack_slots.insert(name.clone(), offset);
                    offset
                }
            };

            *operand = Operand::STACK(offset);
        }
    }
}