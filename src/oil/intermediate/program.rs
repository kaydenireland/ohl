use crate::oil::intermediate::function::IntermediateFunction;
use crate::{indent_dec, indent_inc, indent_reset, info, log_debug};
use crate::ohl::converter::operator::Operator;
use crate::oil::intermediate::instruction::IntermediateInstruction;
use crate::oil::intermediate::instruction::IntermediateInstruction::UNARY;
use crate::oil::intermediate::operator::{IntermediateBinaryOperator, IntermediateUnaryOperator, Value};
use crate::ohl::converter::stree::STree;

pub struct IntermediateProgram {
    pub functions: Vec<IntermediateFunction>,
    pub temp_counter: usize,
    pub label_counter: usize
}

impl IntermediateProgram {
    fn new(_debug: bool) -> IntermediateProgram {
        log_debug!(_debug);
        IntermediateProgram { functions: Vec::new(), temp_counter: 0, label_counter: 0 }
    }

    pub fn make_temporary_name(&mut self) -> String {
        let name: String = format!("tmp.{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    pub fn make_label_name(&mut self) -> String {
        let name: String = format!("bl.{}", self.label_counter);
        self.label_counter += 1;
        name
    }
    
    pub fn lower_tree(tree: STree, _debug: bool) -> IntermediateProgram {
        let mut program = IntermediateProgram::new(_debug);
        program.lower(tree);
        program
    }

    fn lower(&mut self, tree: STree) -> (Vec<IntermediateInstruction>, Value) {
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

                let unary_op = IntermediateUnaryOperator::from(operator);
                let (mut instructions, value) = self.lower(*right);

                let dst = Value::VAR(self.make_temporary_name());

                instructions.push(UNARY { operator: unary_op, src: value, dst: dst.clone() });

                indent_dec!();
                (instructions, dst)
            },

            // Short Circuit AND
            STree::EXPR { left, operator: Operator::AND, right } => {
                info!("intermediate_lower_and_expression()");
                indent_inc!();

                let false_label = self.make_label_name();
                let end_label = self.make_label_name();
                let dst = Value::VAR(self.make_temporary_name());

                let mut instructions: Vec<IntermediateInstruction> = Vec::new();

                let (left_instructions, left_value) = self.lower(*left);
                instructions.extend(left_instructions);

                // If left == 0, result is false
                instructions.push(
                    IntermediateInstruction::JUMP_IF_ZERO {
                        condition: left_value,
                        target: false_label.clone(),
                    }
                );

                // Evaluate right side ONLY if left was nonzero
                let (right_instructions, right_value) = self.lower(*right);
                instructions.extend(right_instructions);

                // If right == 0, result is false
                instructions.push(
                    IntermediateInstruction::JUMP_IF_ZERO {
                        condition: right_value,
                        target: false_label.clone(),
                    }
                );

                // Both were true
                instructions.push(
                    IntermediateInstruction::COPY {
                        src: Value::INT(1),
                        dst: dst.clone(),
                    }
                );

                instructions.push(
                    IntermediateInstruction::JUMP {
                        target: end_label.clone(),
                    }
                );

                // False
                instructions.push(
                    IntermediateInstruction::LABEL {
                        label: false_label,
                    }
                );

                instructions.push(
                    IntermediateInstruction::COPY {
                        src: Value::INT(0),
                        dst: dst.clone(),
                    }
                );

                // End
                instructions.push(
                    IntermediateInstruction::LABEL {
                        label: end_label,
                    }
                );

                indent_dec!();
                (instructions, dst)
            },

            // Short Circuit OR
            STree::EXPR { left, operator: Operator::OR, right } => {
                info!("intermediate_lower_or_expression()");
                indent_inc!();

                let true_label = self.make_label_name();
                let end_label = self.make_label_name();
                let dst = Value::VAR(self.make_temporary_name());

                let mut instructions = Vec::new();

                // Evaluate left
                let (left_instructions, left_value) = self.lower(*left);
                instructions.extend(left_instructions);

                // If left != 0, we're done: true
                instructions.push(
                    IntermediateInstruction::JUMP_IF_NOT_ZERO {
                        condition: left_value,
                        target: true_label.clone(),
                    }
                );

                // Only evaluate right if left was zero
                let (right_instructions, right_value) = self.lower(*right);
                instructions.extend(right_instructions);

                // If right != 0, true
                instructions.push(
                    IntermediateInstruction::JUMP_IF_NOT_ZERO {
                        condition: right_value,
                        target: true_label.clone(),
                    }
                );

                // Neither was true
                instructions.push(
                    IntermediateInstruction::COPY {
                        src: Value::INT(0),
                        dst: dst.clone(),
                    }
                );

                instructions.push(
                    IntermediateInstruction::JUMP {
                        target: end_label.clone(),
                    }
                );

                // True
                instructions.push(
                    IntermediateInstruction::LABEL {
                        label: true_label,
                    }
                );

                instructions.push(
                    IntermediateInstruction::COPY {
                        src: Value::INT(1),
                        dst: dst.clone(),
                    }
                );

                instructions.push(
                    IntermediateInstruction::LABEL {
                        label: end_label,
                    }
                );

                indent_dec!();
                (instructions, dst)
            },

            STree::EXPR { left, operator, right } => {
                info!("intermediate_lower_expression()");
                indent_inc!();

                let binary_op = IntermediateBinaryOperator::from(operator);
                let mut instructions: Vec<IntermediateInstruction> = Vec::new();
                let (mut left_instructions, left_value) = self.lower(*left);
                instructions.append(&mut left_instructions);

                let (mut right_instructions, right_value) = self.lower(*right);
                instructions.append(&mut right_instructions);

                let dst = Value::VAR(self.make_temporary_name());
                instructions.push(IntermediateInstruction::BINARY { operator: binary_op, src1: left_value, src2: right_value, dst: dst.clone() });

                indent_dec!();
                (instructions, dst)
            },


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