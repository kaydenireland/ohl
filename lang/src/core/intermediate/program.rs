use crate::core::intermediate::definition::IntermediateFunction;
use crate::{indent_dec, indent_inc, indent_reset, info, log_debug};
use crate::core::intermediate::instruction::IntermediateInstruction;
use crate::core::intermediate::instruction::IntermediateInstruction::UNARY;
use crate::core::intermediate::operator::{UnaryOperator, Value};
use crate::core::converter::stree::STree;

pub struct IntermediateProgram {
    pub functions: Vec<IntermediateFunction>,
    pub counter: usize,
}

impl IntermediateProgram {
    pub fn new(_debug: bool) -> IntermediateProgram {
        log_debug!(_debug);
        IntermediateProgram { functions: Vec::new(), counter: 0 }
    }

    pub fn make_temporary_name(&mut self) -> String {
        let name: String = format!("tmp.{}", self.counter);
        self.counter += 1;
        name
    }

    pub fn lower(&mut self, tree: STree) -> (Vec<IntermediateInstruction>, Value) {
        match tree {

            // File
            STree::START { classes } => {
                info!("intermediate_lower_file()");
                indent_inc!();

                for class in classes {
                    self.lower(class);
                }

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            // Classes
            STree::CLASS { scope, name, body } => {
                info!("intermediate_lower_class_declaration()");
                indent_inc!();

                self.lower(*body);

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            STree::CLASS_BODY {variables, functions} => {
                info!("intermediate_lower_class_body()");
                indent_inc!();

                for function in functions {
                    self.lower(function);
                }

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            // Functions
            STree::FUNCTION { scope, name, params, return_type, body } => {
                info!("intermediate_lower_function_declaration()");
                indent_inc!();

                let (instructions, _) = self.lower(*body);
                self.functions.push(IntermediateFunction::new(name, instructions));

                indent_dec!();
                (Vec::new(), Value::INT(0))
            }

            STree::BLOCK { statements } => {
                info!("intermediate_lower_block()");
                indent_inc!();

                let mut instructions: Vec<IntermediateInstruction> = Vec::new();

                for statement in statements {
                    let (new_instructions, _) = self.lower(statement);
                    instructions.extend(new_instructions);
                }

                indent_dec!();
                (instructions, Value::INT(0))
            }


            // Expressions
            STree::PRFX_EXPR { operator, right } => {
                info!("intermediate_lower_prefix_expression()");
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
                info!("intermediate_lower_return()");
                indent_inc!();

                let (mut instructions, value) = match expression {
                    Some(expression) => self.lower(*expression),
                    None => (Vec::new(), Value::INT(0))
                };

                instructions.push(IntermediateInstruction::RETURN(value));

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
        println!("\nDumping Ohl IR:");
        for function in &self.functions {
            println!("\nfn {}", function.name);
            for instruction in &function.instructions {
                println!("   {}", instruction);
            }
        }
        println!();
    }

}