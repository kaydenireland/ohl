
use std::io::Error;
use crate::core::codegen::codegen::AssemblyGenerator;
use crate::core::lowering::program::MachineProgram;
use crate::{indent_dec, indent_inc, indent_reset, info, log_debug};
use crate::core::intermediate::operator::UnaryOperator;
use crate::core::lowering::instruction::MachineInstruction;
use crate::core::lowering::operand::{Operand, Register};

pub struct X64CodeGenerator {
}

impl X64CodeGenerator {
    pub fn new(_debug: bool) -> X64CodeGenerator {
        log_debug!(_debug);
        X64CodeGenerator {}
    }
}

// AT&T ASM
impl AssemblyGenerator for X64CodeGenerator {
    fn generate(&mut self, program: MachineProgram ) -> Result<(), Error> {
        indent_reset!();
        info!("generate_file_x64()");
        indent_inc!();

        println!(".globl {}", "main");
        for function in program.functions {
            self.generate_function_definition(function.name)?;
            self.generate_instructions(function.instructions)?;
        }


        indent_dec!();

        Ok(())
    }

}

impl X64CodeGenerator {
    fn generate_function_definition(&mut self, name: String) -> Result<(), Error> {
        info!("generate_function_definition_x64()");
        indent_inc!();

        println!("{}:", name);
        println!("\tpushq\t%rbp");
        println!("\tmovq\t%rsp, %rbp");

        indent_dec!();
        Ok(())
    }

    fn generate_instructions(&mut self, instructions: Vec<MachineInstruction>) -> Result<(), Error> {
        info!("generate_instructions_x64()");
        indent_inc!();

        for instruction in instructions {
            match instruction {
                MachineInstruction::MOVE { src, dst} => println!("\tmovl\t{}, {}", self.generate_operand(src)?, self.generate_operand(dst)?),
                MachineInstruction::RETURN => {
                    println!("\tmovq\t%rbp, %rsp");
                    println!("\tpopq\t%rbp");
                    println!("\tret");
                },
                MachineInstruction::UNARY { operator, operand } => println!("\t{}\t{}", self.generate_unary_operator(operator), self.generate_operand(operand)?),
                MachineInstruction::ALLOCATE_STACK(v) => println!("\tsubq\t${}, %rsp", v)
            }
        }

        indent_dec!();
        Ok(())

    }

    fn generate_operand(&mut self, operand: Operand) -> Result<String, Error> {
        info!("generate_operand_x64()");
        indent_inc!();

        let asm: String = match operand {
            Operand::REG(Register::R10) => format!("%r10d"),
            Operand::REG(Register::AX) => format!("%eax"),
            Operand::STACK(v) => format!("{}(%rbp)", v),
            Operand::IMM(v) => format!("${}", v),
            // TODO: Operand::PSEUDO(_) => return Error::new(),

            _ => String::new()
        };

        indent_dec!();
        Ok(asm)
    }

    fn generate_unary_operator(&mut self, operand: UnaryOperator) -> String {
        info!("generate_unary_operator_x64()");
        indent_inc!();

        let asm: String = match operand {
            UnaryOperator::NEGATE => format!("negl"),
            UnaryOperator::NOT => format!("notl"),
        };


        indent_dec!();
        asm
    }
}