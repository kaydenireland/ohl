use std::collections::HashMap;
use crate::core::lowering::instruction::MachineInstruction;
use crate::core::lowering::operand::Operand;

#[derive(Debug, Clone)]
pub struct MachineFunction {
    pub name: String,
    pub instructions: Vec<MachineInstruction>
}

impl MachineFunction {
    pub fn new(name: String, instructions: Vec<MachineInstruction>) -> MachineFunction {
        MachineFunction { name, instructions }
    }

    pub fn allocate_stack(&mut self) {
        let mut stack_size: i32 = 0;    // in bytes
        let mut vars: HashMap<String, i32> = HashMap::new();
        for instruction in &mut self.instructions {
            match instruction {
                MachineInstruction::UNARY { operator, operand } => Self::allocate_operand(operand, &mut vars, &mut stack_size),
                MachineInstruction::MOVE { src, dst} => {
                    Self::allocate_operand(src, &mut vars, &mut stack_size);
                    Self::allocate_operand(dst, &mut vars, &mut stack_size)
                },

                _ => {}
            }
        }
        self.instructions.insert(0, MachineInstruction::ALLOCATE_STACK(stack_size))
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