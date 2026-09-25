use crate::ohl::intermediate::function::IntermediateFunction;
use crate::ohl::intermediate::program::IntermediateProgram;
use crate::{indent_dec, indent_inc, info, log_debug};
use crate::ohl::intermediate::instruction::IntermediateInstruction;
use crate::ohl::intermediate::operator::{IntermediateBinaryOperator, IntermediateUnaryOperator, Value};
use crate::oil::machine::condition::ConditionCode;
use crate::oil::machine::function::MachineFunction;
use crate::oil::machine::instruction::MachineInstruction;
use crate::oil::machine::operand::Operand;
use crate::oil::machine::register::Register;
use crate::oil::machine::operator::{MachineBinaryOperator, MachineUnaryOperator};

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
        machine.legalize_instructions();

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
            IntermediateInstruction::UNARY { operator: IntermediateUnaryOperator::NOT, src, dst} => {
                let dst_val = self.lower_value(dst);
                new_instructions.push(
                    MachineInstruction::COMPARE { operand1: Operand::IMM(0), operand2: self.lower_value(src) }
                );
                new_instructions.push(
                    MachineInstruction::MOVE { src: Operand::IMM(0), dst: dst_val.clone() }
                );
                new_instructions.push(
                    MachineInstruction::SET_CC { condition: ConditionCode::EQUAL, operand: dst_val }
                );
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
                    IntermediateBinaryOperator::EQUAL | IntermediateBinaryOperator::NOT_EQUAL |
                    IntermediateBinaryOperator::GREATER_THAN | IntermediateBinaryOperator::GREATER_OR_EQUAL
                    | IntermediateBinaryOperator::LESS_THAN | IntermediateBinaryOperator::LESS_OR_EQUAL => {
                        new_instructions.push(
                            MachineInstruction::COMPARE { operand1: self.lower_value(src2), operand2: self.lower_value(src1) }
                        );
                        new_instructions.push(
                            MachineInstruction::MOVE { src: Operand::IMM(0), dst: dst_val.clone() }
                        );
                        new_instructions.push(
                            MachineInstruction::SET_CC { condition: ConditionCode::from_inter_operator(&operator), operand: dst_val }
                        )
                    },
                    _ => {
                        new_instructions.push(MachineInstruction::MOVE { src: self.lower_value(src1), dst: dst_val.clone() });
                        new_instructions.push(MachineInstruction::BINARY { operator: MachineBinaryOperator::from(operator), operand1: dst_val, operand2: self.lower_value(src2) });
                    }
                }
            },
            IntermediateInstruction::COPY { src, dst } => {
                let dst_val = self.lower_value(dst);
                new_instructions.push(
                    MachineInstruction::MOVE { src: self.lower_value(src), dst: dst_val }
                );
            }
            IntermediateInstruction::JUMP { target } => new_instructions.push(MachineInstruction::JUMP(target)),
            IntermediateInstruction::JUMP_IF_ZERO { condition, target } => {
                new_instructions.push(
                    MachineInstruction::COMPARE { operand1: Operand::IMM(0), operand2: self.lower_value(condition) }
                );
                new_instructions.push(
                    MachineInstruction::JUMP_CC { condition: ConditionCode::EQUAL, identifier: target.clone() }
                )
            },
            IntermediateInstruction::JUMP_IF_NOT_ZERO { condition, target } => {
                new_instructions.push(
                    MachineInstruction::COMPARE { operand1: Operand::IMM(0), operand2: self.lower_value(condition) }
                );
                new_instructions.push(
                    MachineInstruction::JUMP_CC { condition: ConditionCode::NOT_EQUAL, identifier: target.clone() }
                )
            },
            IntermediateInstruction::LABEL { label } => new_instructions.push(MachineInstruction::LABEL(label)),
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