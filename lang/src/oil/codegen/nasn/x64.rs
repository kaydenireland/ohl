use std::fmt::format;
use std::io::Error;
use crate::oil::codegen::codegen::AssemblyGenerator;
use crate::oil::machine::program::MachineProgram;
use crate::{indent_dec, indent_inc, indent_reset, info, log_debug};
use crate::oil::machine::instruction::MachineInstruction;
use crate::oil::machine::operand::Operand;
use crate::oil::machine::register::Register;
use crate::oil::machine::operator::{MachineBinaryOperator, MachineUnaryOperator};

pub struct X64CodeGenerator {
    output: String,
}

impl X64CodeGenerator {
    pub fn new(_debug: bool) -> X64CodeGenerator {
        log_debug!(_debug);
        X64CodeGenerator {
            output: String::new(),
        }
    }
}

// AT&T ASM
impl AssemblyGenerator for X64CodeGenerator {
    fn generate(&mut self, program: MachineProgram ) -> Result<String, Error> {
        self.output.clear();
        indent_reset!();
        info!("generate_file_x64()");
        indent_inc!();

        self.output.push_str(&format!(".globl {}\n", "main"));
        for function in program.functions {
            self.generate_function_definition(function.name)?;
            self.generate_instructions(function.instructions)?;
        }
        self.output.push_str("\n");


        indent_dec!();

        Ok(self.output.clone())
    }

}

impl X64CodeGenerator {
    fn generate_function_definition(&mut self, name: String) -> Result<(), Error> {
        info!("generate_function_definition_x64()");
        indent_inc!();

        self.output.push_str(&format!("{}:\n", name));
        self.output.push_str(&format!("\tpushq\t%rbp\n"));
        self.output.push_str(&format!("\tmovq\t%rsp, %rbp\n"));

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

    fn generate_operand(&self, operand: &Operand) -> Result<String, Error> {
        info!("generate_operand_x64()");
        indent_inc!();

        let asm: String = match operand {
            Operand::REG(Register::R10) => "%r10d".to_string(),
            Operand::REG(Register::R11) => "%r11d".to_string(),
            Operand::REG(Register::AX) => "%eax".to_string(),
            Operand::REG(Register::DX) => "%edx".to_string(),
            Operand::REG(Register::CX) => "%ecx".to_string(),
            Operand::STACK(v) => format!("{}(%rbp)", v),
            Operand::IMM(v) => format!("${}", v),

            _ => panic!("Unexpected operand type: {}", operand),
        };

        indent_dec!();
        Ok(asm)
    }

    fn generate_unary_operator(&self, operand: &MachineUnaryOperator) -> String {
        info!("generate_unary_operator_x64()");
        indent_inc!();

        let asm: String = match operand {
            MachineUnaryOperator::NEGATE => "negl".to_string(),
            MachineUnaryOperator::NOT => "notl".to_string(),
        };


        indent_dec!();
        asm
    }

    fn generate_binary_operator(&self, operand: &MachineBinaryOperator) -> String {
        info!("generate_binary_operator_x64()");
        indent_inc!();

        let asm: String = match operand {
            MachineBinaryOperator::ADD => "addl".to_string(),
            MachineBinaryOperator::SUBTRACT => "subl".to_string(),
            MachineBinaryOperator::MULTIPLY => "imull".to_string(),

            MachineBinaryOperator::BIT_AND => "andl".to_string(),
            MachineBinaryOperator::BIT_OR => "orl".to_string(),
            MachineBinaryOperator::BIT_XOR => "xorl".to_string(),
            MachineBinaryOperator::SHIFT_LEFT => "sall".to_string(),
            MachineBinaryOperator::SHIFT_RIGHT => "sarl".to_string(),
            _ => String::new()
        };


        indent_dec!();
        asm
    }

    fn generate_instruction(&mut self, instruction: MachineInstruction) -> Result<(), Error> {
        info!("generate_instruction_x64()");
        indent_inc!();

        match instruction {
            MachineInstruction::MOVE { src, dst} => self.output.push_str(&format!("\tmovl\t{}, {}\n", self.generate_operand(&src)?, self.generate_operand(&dst)?)),
            MachineInstruction::UNARY { operator, operand } => self.output.push_str(&format!("\t{}\t{}\n", self.generate_unary_operator(&operator), self.generate_operand(&operand)?)),
            MachineInstruction::BINARY { operator, operand1, operand2} => {
                let src = match operator {
                    MachineBinaryOperator::SHIFT_LEFT | MachineBinaryOperator::SHIFT_RIGHT => {
                        match operand2 {
                            Operand::REG(Register::CX) => "%cl".to_string(),
                            _ => self.generate_operand(&operand2)?,
                        }
                    }

                    _ => self.generate_operand(&operand2)?,
                };

                self.output.push_str(&format!(
                    "\t{}\t{}, {}\n",
                    self.generate_binary_operator(&operator),
                    src,
                    self.generate_operand(&operand1)?
                ));
            },
            MachineInstruction::COMPARE { operand1, operand2 } => self.output.push_str(&format!("\tcmpl\t{},\t{}\n", operand1, operand2)),
            MachineInstruction::IDIV(operand) => self.output.push_str(&format!("\tidivl\t{}\n", self.generate_operand(&operand)?)),
            MachineInstruction::CDQ => self.output.push_str(&format!("\tcdq\n")),
            MachineInstruction::JUMP(id) => self.output.push_str(&format!("\tjmp\t.L{}\n", id)),
            MachineInstruction::JUMP_CC { condition, identifier } => self.output.push_str(&format!("\tj{}\t.L{}\n", condition, identifier)),
            MachineInstruction::SET_CC { condition, operand } => self.output.push_str(&format!("\tset{}\t{}\n", condition, operand)),
            MachineInstruction::LABEL(id) => self.output.push_str(&format!("\t.L{}:\n", id)),
            MachineInstruction::ALLOCATE_STACK(v) => self.output.push_str(&format!("\tsubq\t${}, %rsp\n", v)),
            MachineInstruction::RETURN => {
                self.output.push_str(&format!("\tmovq\t%rbp, %rsp\n"));
                self.output.push_str(&format!("\tpopq\t%rbp\n"));
                self.output.push_str(&format!("\tret\n"));
            },


            // _ => {}
        };


        indent_dec!();
        Ok(())
    }
}