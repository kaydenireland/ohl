use crate::core::lowering::program::MachineProgram;
use crate::{indent_dec, indent_inc, indent_reset, info};
use crate::core::lowering::instruction::MachineInstruction;
use crate::core::lowering::operand::Operand;
use crate::core::lowering::operator::MachineBinaryOperator;
use crate::core::lowering::register::Register;

impl MachineProgram {
    pub fn allocate_stack(&mut self) {
        indent_reset!();
        info!("allocate_stack()");
        indent_inc!();

        for function in &mut self.functions {
            function.allocate_stack();
        }

        indent_dec!();
    }

    pub fn legalize_instructions(&mut self) {
        indent_reset!();
        info!("legalize_instructions()");

        for function in &mut self.functions {
            let mut new_instructions: Vec<MachineInstruction> = Vec::new();
            for instruction in &mut function.instructions.drain(..) {
                match instruction {
                    MachineInstruction::MOVE { src, dst } => {
                        if src.is_memory() && dst.is_memory() {
                            new_instructions.push(
                                MachineInstruction::MOVE {
                                    src,
                                    dst: Operand::REG(Register::R10),
                                }
                            );

                            new_instructions.push(
                                MachineInstruction::MOVE {
                                    src: Operand::REG(Register::R10),
                                    dst,
                                }
                            );
                        } else {
                            new_instructions.push(
                                MachineInstruction::MOVE { src, dst }
                            );
                        }
                    },

                    MachineInstruction::IDIV(operand) => {
                        new_instructions.push(
                            MachineInstruction::MOVE { src: operand, dst: Operand::REG(Register::R10) }
                        );
                        new_instructions.push(
                            MachineInstruction::IDIV(Operand::REG(Register::R10))
                        );
                    },
                    MachineInstruction::BINARY { operator, operand1, operand2 } => {
                        match operator {
                            MachineBinaryOperator::ADD | MachineBinaryOperator::SUBTRACT
                            | MachineBinaryOperator::BIT_AND | MachineBinaryOperator::BIT_OR | MachineBinaryOperator::BIT_XOR => {
                                if operand1.is_memory() && operand2.is_memory() {
                                    new_instructions.push(
                                        MachineInstruction::MOVE {
                                            src: operand2,
                                            dst: Operand::REG(Register::R10),
                                        }
                                    );

                                    new_instructions.push(
                                        MachineInstruction::BINARY {
                                            operator,
                                            operand1,
                                            operand2: Operand::REG(Register::R10),
                                        }
                                    );
                                } else {
                                    new_instructions.push(
                                        MachineInstruction::BINARY { operator, operand1, operand2 }
                                    );
                                }
                            },
                            MachineBinaryOperator::MULTIPLY => {
                                new_instructions.push(
                                    MachineInstruction::MOVE { src: operand1.clone(), dst: Operand::REG(Register::R11) }
                                );
                                new_instructions.push(
                                    MachineInstruction::BINARY { operator: MachineBinaryOperator::MULTIPLY, operand1: Operand::REG(Register::R11), operand2 }
                                );
                                new_instructions.push(
                                    MachineInstruction::MOVE { src: Operand::REG(Register::R11), dst: operand1 }
                                );
                            },
                            MachineBinaryOperator::SHIFT_LEFT | MachineBinaryOperator::SHIFT_RIGHT => {
                                new_instructions.push(
                                    MachineInstruction::MOVE {
                                        src: operand2,
                                        dst: Operand::REG(Register::CX),
                                    }
                                );

                                new_instructions.push(
                                    MachineInstruction::BINARY {
                                        operator,
                                        operand1,
                                        operand2: Operand::REG(Register::CX),
                                    }
                                );
                            }
                        }

                    },
                    MachineInstruction::COMPARE { operand1, operand2 } => {
                        if operand1.is_memory() && operand2.is_memory() {
                            new_instructions.push(
                                MachineInstruction::MOVE {
                                    src: operand1,
                                    dst: Operand::REG(Register::R10),
                                }
                            );

                            new_instructions.push(
                                MachineInstruction::COMPARE {
                                    operand1: Operand::REG(Register::R10),
                                    operand2,
                                }
                            );
                        } else if operand2.is_immediate() {
                            new_instructions.push(
                                MachineInstruction::MOVE { src: operand2, dst: Operand::REG(Register::R10) }
                            );

                            new_instructions.push(
                                MachineInstruction::COMPARE { operand1, operand2: Operand::REG(Register::R10) }
                            );
                        } else {
                            new_instructions.push(
                                MachineInstruction::COMPARE { operand1, operand2 }
                            );
                        }
                    }

                    instruction => new_instructions.push(instruction.clone())
                }
            }
            function.instructions = new_instructions;
        }
    }
}