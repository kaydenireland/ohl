use crate::core::intermediate::definition::IntermediateFunction;
use crate::core::intermediate::program::IntermediateProgram;
use crate::{indent_dec, indent_inc, info};
use crate::core::intermediate::instruction::IntermediateInstruction;
use crate::core::intermediate::operator::Value;
use crate::core::lowering::definition::AssemblyFunction;
use crate::core::lowering::instruction::AssemblyInstruction;
use crate::core::lowering::operand::{Operand, Register};

pub struct MachineProgram {
    pub functions: Vec<AssemblyFunction>
}

impl MachineProgram {
    pub fn new() -> MachineProgram {
        MachineProgram {
            functions: Vec::new()
        }
    }

    pub fn lower(&mut self, program: IntermediateProgram) {
        info!("machine_lower_program()");
        indent_inc!();

        for function in program.functions {
            let new_function = self.lower_function(function);
            self.functions.push(new_function);
        }

        indent_dec!();
    }

    fn lower_function(&mut self, function: IntermediateFunction) -> AssemblyFunction {
        info!("machine_lower_function()");
        indent_inc!();

        let mut instructions: Vec<AssemblyInstruction> = Vec::new();

        for instruction in function.instructions {
            instructions.extend(self.lower_instruction(instruction));
        }

        indent_dec!();
        AssemblyFunction { name: function.name, instructions }
    }

    fn lower_instruction(&mut self, instruction: IntermediateInstruction) -> Vec<AssemblyInstruction> {
        info!("machine_lower_instruction()");
        indent_inc!();

        let mut new_instructions: Vec<AssemblyInstruction> = Vec::new();

        match instruction {
            IntermediateInstruction::RETURN(value) => {
                new_instructions.push(
                    AssemblyInstruction::MOVE { src: self.lower_value(value), dst: Operand::REG(Register::AX)}
                );
                new_instructions.push(AssemblyInstruction::RETURN);
            },
            IntermediateInstruction::UNARY { operator, src, dst} => {
                let dst_val = self.lower_value(dst);
                new_instructions.push(
                    AssemblyInstruction::MOVE { src: self.lower_value(src), dst: dst_val.clone()}
                );
                new_instructions.push(
                    AssemblyInstruction::UNARY { operator, operand: dst_val }
                );
            }
        }

        indent_dec!();
        new_instructions
    }

    fn lower_value(&mut self, val: Value) -> Operand {
        info!("machine_lower_value()");
        indent_inc!();

        let operand = match val {
            Value::INT(v) => Operand::IMM(v),
            Value::VAR(v) => Operand::PSEUDO(v)
        };

        indent_dec!();
        operand
    }

    pub fn dump(&self) {
        println!("\nDumping Machine :");
        for function in &self.functions {
            println!("\nfn {}", function.name);
            for instruction in &function.instructions {
                println!("   {}", instruction);
            }
        }
    }
}