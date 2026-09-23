use crate::core::intermediate::definition::IntermediateFunction;
use crate::core::intermediate::program::IntermediateProgram;
use crate::{indent_dec, indent_inc, info, log_debug};
use crate::core::intermediate::instruction::IntermediateInstruction;
use crate::core::intermediate::operator::{IntermediateBinaryOperator, Value};
use crate::core::lowering::function::MachineFunction;
use crate::core::lowering::instruction::MachineInstruction;
use crate::core::lowering::operand::Operand;
use crate::core::lowering::register::Register;
use crate::core::lowering::operator::{MachineBinaryOperator, MachineUnaryOperator};

pub struct MachineProgram {
    pub functions: Vec<MachineFunction>
}

impl MachineProgram {
    fn new(_debug: bool) -> MachineProgram {
        log_debug!(_debug);
        MachineProgram {
            functions: Vec::new()
        }
    }

    pub fn lower(program: IntermediateProgram, _debug: bool) -> MachineProgram {
        let mut machine = MachineProgram::new(_debug);
        info!("machine_lower_program()");
        indent_inc!();

        for function in program.functions {
            let new_function = machine.lower_function(function);
            machine.functions.push(new_function);
        }

        machine.allocate_stack();
        machine.legalize_moves();

        indent_dec!();
        machine
    }

    fn lower_function(&mut self, function: IntermediateFunction) -> MachineFunction {
        info!("machine_lower_function()");
        indent_inc!();

        let mut instructions: Vec<MachineInstruction> = Vec::new();

        for instruction in function.instructions {
            instructions.extend(self.lower_instruction(instruction));
        }

        indent_dec!();
        MachineFunction { name: function.name, instructions }
    }

    fn lower_instruction(&mut self, instruction: IntermediateInstruction) -> Vec<MachineInstruction> {
        info!("machine_lower_instruction()");
        indent_inc!();

        let mut new_instructions: Vec<MachineInstruction> = Vec::new();

        match instruction {
            IntermediateInstruction::RETURN(value) => {
                new_instructions.push(
                    MachineInstruction::MOVE { src: self.lower_value(value), dst: Operand::REG(Register::AX)}
                );
                new_instructions.push(MachineInstruction::RETURN);
            },
            IntermediateInstruction::UNARY { operator, src, dst} => {
                let dst_val = self.lower_value(dst);
                new_instructions.push(
                    MachineInstruction::MOVE { src: self.lower_value(src), dst: dst_val.clone()}
                );
                new_instructions.push(
                    MachineInstruction::UNARY { operator: MachineUnaryOperator::from(operator), operand: dst_val }
                );
            },
            IntermediateInstruction::BINARY { operator, src1, src2, dst } => {
                let dst_val = self.lower_value(dst);

                match operator {
                    IntermediateBinaryOperator::DIVIDE => {
                        new_instructions.push(
                            MachineInstruction::MOVE { src: self.lower_value(src1), dst: Operand::REG(Register::AX)}
                        );
                        new_instructions.push(MachineInstruction::CDQ);
                        new_instructions.push(MachineInstruction::IDIV(self.lower_value(src2)));
                        new_instructions.push(
                            MachineInstruction::MOVE { src: Operand::REG(Register::AX), dst: dst_val }
                        );
                    },
                    IntermediateBinaryOperator::REMAINDER => {
                        new_instructions.push(
                            MachineInstruction::MOVE { src: self.lower_value(src1), dst: Operand::REG(Register::AX)}
                        );
                        new_instructions.push(MachineInstruction::CDQ);
                        new_instructions.push(MachineInstruction::IDIV(self.lower_value(src2)));
                        new_instructions.push(
                            MachineInstruction::MOVE { src: Operand::REG(Register::DX), dst: dst_val }
                        );
                    },
                    _ => {
                        new_instructions.push(MachineInstruction::MOVE { src: self.lower_value(src1), dst: dst_val.clone() });
                        new_instructions.push(MachineInstruction::BINARY { operator: MachineBinaryOperator::from(operator), operand1: dst_val, operand2: self.lower_value(src2) });
                    }
                }
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
        println!("\nDumping Machine IR:");
        for function in &self.functions {
            println!("\nfn {}", function.name);
            for instruction in &function.instructions {
                println!("   {}", instruction);
            }
        }
    }

}