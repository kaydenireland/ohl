use crate::core::codegen::definition::Function;
use crate::{indent_dec, indent_inc, indent_reset, info, log_debug};
use crate::core::codegen::instruction::Instruction;
use crate::core::codegen::instruction::Instruction::UNARY;
use crate::core::codegen::operator::{UnaryOperator, Value};
use crate::core::converter::stree::STree;

pub struct Program {
    pub functions: Vec<Function>,
    pub counter: usize,
}

impl Program {
    pub fn new(_debug: bool) -> Program {
        log_debug!(_debug);
        Program { functions: Vec::new(), counter: 0 }
    }

    pub fn make_temporary_name(&mut self) -> String {
        let name: String = format!("tmp.{}", self.counter);
        self.counter += 1;
        name
    }

    pub fn lower(&mut self, tree: STree) -> (Vec<Instruction>, Value) {
        match tree {

            // File
            STree::START { classes } => {
                info!("lower_file()");
                indent_inc!();

                for class in classes {
                    self.lower(class);
                }

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            // Classes
            STree::CLASS { scope, name, body } => {
                info!("lower_class_declaration()");
                indent_inc!();

                self.lower(*body);

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            STree::CLASS_BODY {variables, functions} => {
                info!("lower_class_body()");
                indent_inc!();

                for function in functions {
                    self.lower(function);
                }

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            // Functions
            STree::FUNCTION { scope, name, params, return_type, body } => {
                info!("lower_function_declaration()");
                indent_inc!();

                let (instructions, _) = self.lower(*body);
                self.functions.push(Function::new(name, instructions));

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            STree::BLOCK { statements } => {
                info!("lower_block()");
                indent_inc!();

                let mut instructions: Vec<Instruction> = Vec::new();

                for statement in statements {
                    let (new_instructions, _) = self.lower(statement);
                    instructions.extend(new_instructions);
                }

                indent_dec!();
                (instructions, Value::INT(0))
            }


            // Expressions
            STree::PRFX_EXPR { operator, right } => {
                info!("lower_prefix_expression()");
                indent_inc!();

                let unary_op = UnaryOperator::from(operator);
                let (mut instructions, value) = self.lower(*right);

                let dst = Value::VAR(self.make_temporary_name());

                instructions.push(UNARY { operator: unary_op, src: value, dst: dst.clone() });

                indent_dec!();
                (instructions, dst)
            }


            // Statements
            STree::RETURN_STMT { expression } => {
                info!("lower_return()");
                indent_inc!();

                let (mut instructions, value) = match expression {
                    Some(expression) => self.lower(*expression),
                    None => (Vec::new(), Value::INT(0))
                };

                instructions.push(Instruction::RETURN(value));

                indent_dec!();
                (instructions, Value::INT(0))
            }

            // Literals
            STree::LIT_INT { value } => (Vec::new(), Value::INT(value)),
            STree::ID { name } => (Vec::new(), Value::VAR(name)),




            _ => (Vec::new(), Value::INT(0)),

        }
    }

    pub fn dump(&self) {
        println!("\nDumping Program");
        for function in &self.functions {
            println!("Function: {}", function.name);
            for instruction in &function.instructions {
                match instruction {
                    Instruction::RETURN(val) => {
                        println!("  Return {:?}", val);
                    }
                    Instruction::UNARY { operator, src, dst } => {
                        println!("  Unary {:?} {:?} {:?}", operator, src, dst);
                    }
                }
            }
        }
    }

}