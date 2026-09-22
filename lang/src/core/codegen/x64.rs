
use std::io::Error;
use crate::core::codegen::codegen::AssemblyGenerator;
use crate::core::lowering::program::MachineProgram;
use crate::{indent_dec, indent_inc, indent_reset, info, log_debug};
use crate::core::lowering::instruction::MachineInstruction;
use crate::core::lowering::operand::{Operand, Register};
use crate::core::lowering::operator::{MachineBinaryOperator, MachineUnaryOperator};

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
        println!();


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
            self.generate_instruction(instruction)?;
        }

        indent_dec!();
        Ok(())

    }

    fn generate_operand(&mut self, operand: Operand) -> Result<String, Error> {
        info!("generate_operand_x64()");
        indent_inc!();

        let asm: String = match operand {
            Operand::REG(Register::R10) => format!("%r10d"),
            Operand::REG(Register::R11) => format!("%r11d"),
            Operand::REG(Register::AX) => format!("%eax"),
            Operand::REG(Register::DX) => format!("%edx"),
            Operand::STACK(v) => format!("{}(%rbp)", v),
            Operand::IMM(v) => format!("${}", v),

            _ => panic!("Unexpected operand type: {}", operand),
        };

        indent_dec!();
        Ok(asm)
    }

    fn generate_unary_operator(&mut self, operand: MachineUnaryOperator) -> String {
        info!("generate_unary_operator_x64()");
        indent_inc!();

        let asm: String = match operand {
            MachineUnaryOperator::NEGATE => "negl".to_string(),
            MachineUnaryOperator::NOT => "notl".to_string(),
        };


        indent_dec!();
        asm
    }

    fn generate_binary_operator(&mut self, operand: MachineBinaryOperator) -> String {
        info!("generate_binary_operator_x64()");
        indent_inc!();

        let asm: String = match operand {
            MachineBinaryOperator::ADD => "addl".to_string(),
            MachineBinaryOperator::SUBTRACT => "subl".to_string(),
            MachineBinaryOperator::MULTIPLY => "imull".to_string(),
            _ => String::new()
        };


        indent_dec!();
        asm
    }

    fn generate_instruction(&mut self, instruction: MachineInstruction) -> Result<(), Error> {
        info!("generate_instruction_x64()");
        indent_inc!();

        match instruction {
            MachineInstruction::MOVE { src, dst} => println!("\tmovl\t{}, {}", self.generate_operand(src)?, self.generate_operand(dst)?),
            MachineInstruction::RETURN => {
                println!("\tmovq\t%rbp, %rsp");
                println!("\tpopq\t%rbp");
                println!("\tret");
            },
            MachineInstruction::UNARY { operator, operand } => println!("\t{}\t{}", self.generate_unary_operator(operator), self.generate_operand(operand)?),
            MachineInstruction::BINARY { operator, operand1, operand2, } => {
                println!("\t{}\t{}, {}", self.generate_binary_operator(operator), self.generate_operand(operand2)?, self.generate_operand(operand1)?);
            },
            MachineInstruction::IDIV(operand) => println!("\tidivl\t{}", self.generate_operand(operand)?),
            MachineInstruction::CDQ => println!("\tcdq"),
            MachineInstruction::ALLOCATE_STACK(v) => println!("\tsubq\t${}, %rsp", v),

            _ => {}
        };


        indent_dec!();
        Ok(())
    }
}