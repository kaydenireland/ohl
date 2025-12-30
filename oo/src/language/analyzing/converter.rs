use crate::language::tokenizing::token::Token;
use crate::language::parsing::mtree::MTree;
use crate::language::logger::Logger;
use crate::language::analyzing::operator::Operator;
use crate::language::analyzing::types::{FunctionType, VariableType};
use crate::language::analyzing::stree::STree;


pub struct Converter {
    log: Logger,
}

impl Converter {
    pub fn new(_debug: bool) -> Converter {
        let log = Logger::new(_debug);
        Converter { log }
    }

    pub fn convert_tree(&mut self, node: &MTree) -> Result<STree, String> {
        
        match &node.token {

            // Program Root: All Children are Functions
            Token::START => {
                self.log.info("convert_program()");
                self.log.indent_inc();

                let mut functions = Vec::new();
                for child in &node.children {
                    self.log.info("convert_function()");
                    self.log.indent_inc();

                    let next = self.convert_tree(child)?;
                    functions.push(next);

                    self.log.indent_dec();
                }
                self.log.indent_dec();
                Ok(STree::START { functions })
            }

            // Expected Function Declaration Children
            // [ FunctionType, ReturnType, ID(name), PARAM_LIST, BLOCK ]
            Token::FUNC_DECL => {
                let mut iterator = node.children.iter();

                self.log.info("convert_function_type()");
                // Function Type 
                let function_type_node = iterator.next().ok_or("Missing Function Type")?;
                let function_type: FunctionType = match function_type_node.token {
                    Token::PUBLIC => FunctionType::PUBLIC,
                    Token::PROTECTED => FunctionType::PROTECTED,
                    Token::PRIVATE => FunctionType::PRIVATE,
                    _ => return Err("Unexpected Function Type".into())
                };

                self.log.info("convert_return_type()");
                // Return Type
                let return_type_node = iterator.next().ok_or("Missing Function Return Type")?;
                let return_type: VariableType = match return_type_node.token {
                    Token::INT => VariableType::INT,
                    Token::FLOAT => VariableType::FLOAT,
                    Token::BOOLEAN => VariableType::BOOLEAN,
                    Token::CHAR => VariableType::CHAR,
                    Token::STRING => VariableType::STRING,
                    Token::NULL => VariableType::NULL,
                    _ => return Err("Unexpected Return Type".into()),
                };

                self.log.info("convert_function_name()");
                // Name
                let name_node = iterator.next().ok_or("Missing Function Name")?;
                let function_name: String = match &name_node.token {
                    Token::ID { name } => name.clone(),
                    _ => return Err("Expected ID in Function Declaration".into()),
                };

                self.log.info("convert_param_list()");
                self.log.indent_inc();
                // Param List
                let params_node = iterator.next().ok_or("Missing Param List")?;
                let mut params: Vec<(String, VariableType)> = Vec::new();
                for param_node in &params_node.children {
                    self.log.info("convert_param()");

                    let id_node = param_node.children.get(0).ok_or("Param Missing ID")?;
                    let type_node = param_node.children.get(1).ok_or("Param Missing Type")?;

                    let param_name = match &id_node.token {
                        Token::ID { name } => name,
                        _ => return Err("Expected ID in param".into()),
                    };
                    let param_type = match type_node.token {
                        Token::INT => VariableType::INT,
                        Token::FLOAT => VariableType::FLOAT,
                        Token::BOOLEAN => VariableType::BOOLEAN,
                        Token::CHAR => VariableType::CHAR,
                        Token::STRING => VariableType::STRING,
                        _ => return Err("Unexpected Return Type".into()),
                    };
                    params.push((param_name.to_string(), param_type));
                }
                self.log.indent_dec();

                self.log.info("convert_block()");
                self.log.indent_inc();
                // Block
                let block_node = iterator.next().ok_or("Missing Function Block")?;
                let body = self.convert_tree(block_node)?;
                
                self.log.indent_dec();

                Ok(
                    STree::FUNCTION {
                        function_type, 
                        return_type, 
                        name: function_name,
                        params,
                        body: Box::new(body),
                    }
                )
            }

            Token::BLOCK => {
                let mut statements = Vec::new();
                for child in &node.children {
                    let stmt = self.convert_tree(child)?;
                    statements.push(stmt);
                }
                Ok(STree::BLOCK { statements })
            }

            // Expected Variable Declaration Children
            // [ ID(name), VARIABLE_TYPE, MUTABLE/IMMUTABLE, Option<Expression> ]
            Token::VAR_DECL => {
                self.log.info("convert_let_statement()");
                self.log.indent_inc();

                let id_node = node.children.get(0).ok_or("Let Missing ID")?;
                let id = match &id_node.token {
                    Token::ID { name } => name.clone(),
                    _ => return Err("Unexpected ID in Let".into()),
                };

                let type_node = &node.children[1];
                let variable_type: VariableType = match &type_node.token {
                    Token::INT => VariableType::INT,
                    Token::FLOAT => VariableType::FLOAT,
                    Token::BOOLEAN => VariableType::BOOLEAN,
                    Token::CHAR => VariableType::CHAR,
                    Token::STRING => VariableType::STRING,
                    _ => return Err("Unexpected Variable Type".into()),
                };

                let mutable = node.children[2].token == Token::MUTABLE;

                let expression: Box<STree>;
                if node.children.len() >= 4 {
                    let expression_node = &node.children[3];
                    expression = Box::new(self.convert_tree(expression_node)?);
                } else {
                    expression = Box::new(STree::NULL)
                }

                self.log.indent_dec();

                Ok(STree::LET_STMT { id, var_type: variable_type, mutable, expression })
            }

            // Expected Assignment Children
            // [ ID(name), VARIABLE_TYPE, Option<Expression> ]
            Token::ASSIGN => {
                self.log.info("convert_assignment()");
                self.log.indent_inc();

                if node.children.len() != 2 {
                    return Err("Assignment must have left and right side".into());
                }

                let left = &node.children[0];
                let id = match &left.token {
                    Token::ID { name } => name.clone(),
                    _ => return Err("Left side of assignment must be an ID".into()),
                };
                let right = self.convert_tree(&node.children[1])?;

                self.log.indent_dec();

                Ok(STree::ASSIGN_STMT { id, expression: Box::new(right) })
            }

            Token::RANGE_INCL | Token::RANGE_EXCL => {
                self.log.info("convert_range()");
                self.log.indent_inc();

                if node.children.len() != 2 {
                    return Err("Assignment must have left and right side".into());
                }

                let inclusive = node.token == Token::RANGE_INCL;

                let start = self.convert_tree(&node.children[0])?;
                let end = self.convert_tree(&node.children[1])?;

                self.log.indent_dec();

                Ok(STree::RANGE { start: Box::new(start), end: Box::new(end), inclusive })
            }


            Token::ADD_ASSIGN | Token::SUB_ASSIGN | Token::MULT_ASSIGN |Token::DIV_ASSIGN
            | Token::REM_ASSIGN | Token::POWER_ASSIGN | Token::ROOT_ASSIGN => {
                self.log.info("convert_expression_assignment()");
                self.log.indent_inc();

                let variable_node = node.children.get(0).ok_or("Assignment missing left side")?;
                let variable = self.convert_tree(variable_node)?;
                let name = match &variable_node.token {
                    Token::ID { name } => name.clone(),
                    _ => return Err("Left side of assignment must be an ID".into()),
                };

                let right_node = node.children.get(1).ok_or("Assignment missing right side")?;
                let expression = self.convert_tree(right_node)?;

                let operator = match &node.token {
                    Token::ADD_ASSIGN => Operator::ADD,
                    Token::SUB_ASSIGN => Operator::SUBTRACT,
                    Token::MULT_ASSIGN => Operator::MULTIPLY,
                    Token::DIV_ASSIGN => Operator::DIVIDE,
                    Token::REM_ASSIGN => Operator::REMAINDER,
                    Token::POWER_ASSIGN => Operator::POWER,
                    Token::ROOT_ASSIGN => Operator::ROOT,
                    _ => return Err("Invalid expression assignment operator".into())
                };

                let combined = STree::EXPR { left: Box::new(variable.clone()), operator, right: Box::new(expression) };

                self.log.indent_dec();

                Ok(STree::ASSIGN_STMT { id: name, expression: Box::new(combined) })
            }

            // Expected Return Children
            // [ Expression ]
            Token::RTRN_STMT => {
                self.log.info("convert_return()");
                self.log.indent_inc();

                let expression_node = node.children.get(0);
                match expression_node {
                    Some(_) => {
                        let expression = self.convert_tree(expression_node.unwrap())?;
                        self.log.indent_dec();
                        Ok(STree::RETURN_STMT { expression: Some(Box::new(expression)) })
                    },
                    None => {
                        self.log.indent_dec();
                        Ok(STree::RETURN_STMT { expression: None })
                    }
                }
            }

            // Expected For-Loop Children
            // [ Option<Assign>, Expression, Expression, Block ]
            Token::FOR => {

                // detect for-each 
                if node.children.len() == 3 {
                    self.log.info("convert_for_each()");
                    self.log.indent_inc();

                    let variable_node = node.children.get(0).ok_or("Missing loop variable in for-each")?;
                    let variable_name = match &variable_node.token {
                        Token::ID { name } => name.clone(),
                        _ => return Err("Expected identifier as loop variable".into()),
                    };

                    let iterable_node = node.children.get(1).ok_or("Missing iterable in for-each")?;
                    let iterable = self.convert_tree(iterable_node)?;

                    let body_node = node.children.get(2).ok_or("Missing body block in for-each")?;
                    let body = self.convert_tree(body_node)?;

                    self.log.indent_dec();

                    return Ok(STree::FOR_EACH { variable: variable_name, iterable: Box::new(iterable), body: Box::new(body)});
                }

                self.log.info("convert_for()");
                self.log.indent_inc();

                // optional initial assignment
                let init_node = node.children.get(0).ok_or("For loop missing init statement")?;

                let init = match init_node.token {
                    Token::VAR_DECL | Token::ASSIGN => {
                        Some(Box::new(self.convert_tree(init_node)?))
                    }
                    _ => return Err("Invalid init section in for-loop".into()),
                };

                // condition 
                let condition_node = node.children.get(1).ok_or("For loop missing condition expression")?;
                let condition = self.convert_tree(condition_node)?;

                // increment
                let modifier_node = node.children.get(2).ok_or("For loop missing increment expression")?;

                let modifier = match modifier_node.token {
                    Token::INCREMENT | Token::DECREMENT | Token::SQUARE | Token::ASSIGN => {
                        Some(Box::new(self.convert_tree(modifier_node)?))
                    }
                    _ => return Err("Invalid increment section in for-loop".into()),
                };

                // body
                let body_node = node.children.get(3).ok_or("For loop missing body block")?;
                let body = self.convert_tree(body_node)?;

                self.log.indent_dec();

                Ok(STree::FOR_EXPR {
                    init,
                    condition: Box::new(condition),
                    modifier,
                    body: Box::new(body),
                })
            }


            // Expected While Children
            // [ Expression, Body ]
            Token::WHILE => {
                self.log.info("convert_while()");
                self.log.indent_inc();

                let condition_node = node.children.get(0).ok_or("While missing condition")?;
                let condition = self.convert_tree(condition_node)?;

                let body_node = node.children.get(1).ok_or("While missing body")?;
                let body = self.convert_tree(body_node)?;

                self.log.indent_dec();

                Ok(STree::WHILE_EXPR { condition: Box::new(condition), body: Box::new(body) })
            }

            // Expected Loop Children
            // [ Expression, Body ]
            Token::LOOP => {
                self.log.info("convert_loop()");
                self.log.indent_inc();

                let condition_node = node.children.get(0).ok_or("While missing condition")?;
                let condition = self.convert_tree(condition_node)?;

                let body_node = node.children.get(1).ok_or("While missing body")?;
                let body = self.convert_tree(body_node)?;

                self.log.indent_dec();

                Ok(STree::LOOP_EXPR { condition: Box::new(condition), body: Box::new(body) })
            }

            Token::BREAK => Ok(STree::BREAK),
            Token::CONTINUE => Ok(STree::CONTINUE),
            Token::REPEAT => Ok(STree::REPEAT),

            // Expected If Children
            // [ Expression, Body, Else(Else if) ]
            Token::IF_STMT => {
                self.log.info("convert_if()");
                self.log.indent_inc();

                // condition
                let condition_node = node.children.get(0).ok_or("If statement missing condition")?;
                let condition = self.convert_tree(condition_node)?;

                // then block
                let then_node = node.children.get(1).ok_or("If statement missing then block")?;
                let then_block = self.convert_tree(then_node)?;

                // else or else-if
                let else_block = if node.children.len() > 2 {
                    let else_node = &node.children[2];

                    match else_node.token {
                        Token::IF_STMT => {
                            Some(Box::new(self.convert_tree(else_node)?))
                        }
                        Token::BLOCK => {
                            Some(Box::new(self.convert_tree(else_node)?))
                        }
                        _ => return Err("Invalid else clause in if-statement".into()),
                    }
                } else {
                    None
                };

                self.log.indent_dec();
                Ok(STree::IF_EXPR {
                    condition: Box::new(condition),
                    then_block: Box::new(then_block),
                    else_block,
                })
            } 

            // Expected Match Children
            // [Expression, Vec<Arm>]
            Token::MATCH => {
                self.log.info("convert_match()");
                self.log.indent_inc();

                let expression_node = node.children.get(0).ok_or("Invalid expression for MATCH".to_string())?;
                let expression = self.convert_tree(expression_node)?;

                let mut arms: Vec<STree> = Vec::new();
                for child in node.children.iter().skip(1) {
                    match child.token {
                        Token::MATCH_ARM => arms.push(self.convert_tree(child)?),
                        _ => return Err(format!("Match child expected MATCH_ARM, got {:?}", child.token))
                    }
                }

                self.log.indent_dec();
                Ok(STree::MATCH_STMT { expression: Box::new(expression), arms })
            }

            // Expected Match Arm Children
            // [Expression, Expression/Block]
            Token::MATCH_ARM => {
                self.log.info("convert_match_arm()");
                self.log.indent_inc();

                let pattern_node = node.children.get(0)
                    .ok_or("MATCH_ARM missing pattern".to_string())?;

                let body_node = node.children.get(1)
                    .ok_or("MATCH_ARM missing body".to_string())?;

                if node.children.len() != 2 {
                    return Err(format!(
                        "MATCH_ARM expects 2 children, got {}",
                        node.children.len()
                    ));
                }

                let pattern = self.convert_tree(pattern_node)?;
                let body = self.convert_tree(body_node)?;

                self.log.indent_dec();
                Ok(STree::MATCH_ARM {
                    expression: Box::new(pattern),
                    body: Box::new(body),
                })
            }

            Token::DEFAULT => Ok(STree::DEFAULT),


            // Expected Defer Children
            // [Block/Expression]
            Token::DEFER => {
                self.log.info("convert_defer()");
                self.log.indent_inc();

                let body_node = node.children.get(0).ok_or("Defer must have a body".to_string())?;
                let body = self.convert_tree(body_node)?;

                self.log.indent_dec();
                Ok(STree::DEFER_STMT { body: Box::new(body) })
            }

            // Unary Prefix Operators 
            Token::NOT => {
                self.log.info("convert_unary_op()");
                self.log.indent_inc();

                if node.children.len() != 1 {
                    return Err("Unary Prefix NOT must have one child".into());
                }

                let child = self.convert_tree(&node.children[0])?;

                self.log.indent_dec();

                Ok(STree::PRFX_EXPR { operator: Operator::NOT, right: Box::new(child) })
            }

            // Unary Postfix Operators 
            Token::INCREMENT | Token::DECREMENT | Token::SQUARE => {
                self.log.info("convert_unary_op()");
                self.log.indent_inc();

                if node.children.len() != 1 {
                    return Err("Unary postfix operator must have one child".into());
                }

                let child = self.convert_tree(&node.children[0])?;

                let operator = match &node.token {
                    Token::INCREMENT => Operator::INCREMENT,
                    Token::DECREMENT => Operator::DECREMENT,
                    Token::SQUARE => Operator::SQUARE,
                    _ => return Err("Invalid Unary Postfix Operator".into())
                };

                self.log.indent_dec();

                Ok(STree::PTFX_EXPR { left: Box::new(child) , operator })
            }

            // Binary Operators
            Token::ADD | Token::SUB | Token::MULT | Token::DIV | Token::REM | Token::POWER | Token::ROOT 
            | Token::EQUAL | Token::NEQ | Token::LT | Token::GT | Token::NLT | Token::NGT 
            | Token::AND | Token::OR | Token::XOR | Token::NULL_COAL => {
                // Check for Unary
                if node.children.len() == 1 {
                    self.log.info("convert_unary_op()");
                    self.log.indent_inc();

                    let child = self.convert_tree(&node.children[0])?;
                    let operator = match &node.token {
                        Token::SUB => Operator::NEGATIVE,
                        Token::DIV => Operator::RECIPROCAL,
                        _ => return Err("Invalid Unary Prefix Operator".into())
                    };

                    self.log.indent_dec();

                    Ok(STree::PRFX_EXPR { operator, right: Box::new(child) })
                } else if node.children.len() == 2 {
                    self.log.info("convert_binary_op()");
                    self.log.indent_inc();

                    let left = self.convert_tree(&node.children[0])?;
                    let right = self.convert_tree(&node.children[1])?;
                    let operator = match &node.token {
                        Token::ADD => Operator::ADD,
                        Token::SUB => Operator::SUBTRACT,
                        Token::MULT => Operator::MULTIPLY,
                        Token::DIV => Operator::DIVIDE,
                        Token::REM => Operator::REMAINDER,
                        Token::POWER => Operator::POWER,
                        Token::ROOT => Operator::ROOT,
                        Token::EQUAL => Operator::EQUAL,
                        Token::NEQ => Operator::NOT_EQUAL,
                        Token::LT => Operator::LESS_THAN,
                        Token::GT => Operator::GREATER_THAN,
                        Token::NLT => Operator::NOT_LESS_THAN,
                        Token::NGT => Operator::NOT_GREATER_THAN,
                        Token::AND => Operator::AND,
                        Token::OR => Operator::OR,
                        Token::XOR => Operator::XOR,
                        Token::NULL_COAL => Operator::NULL_COAL,
                        _ => return Err("Invalid Binary Operator".into())
                    };

                    self.log.indent_dec();
                    Ok(STree::EXPR { left: Box::new(left), operator, right: Box::new(right) })
                } else {
                    return Err("Operator must have either one or two children".into());
                }
            }

            // Function/Module Calls
            Token::CALL => {
                self.log.info("convert_call()");
                self.log.indent_inc();

                if node.children.is_empty() {
                    return Err("CALL node missing callee".into());
                }

                let callee_node = &node.children[0];

                let mut arguments = Vec::new();
                for arg_node in node.children.iter().skip(1) {
                    arguments.push(self.convert_tree(arg_node)?);
                }

                let mut path = Vec::new();
                self.extract_path(callee_node, &mut path)?;

                self.log.indent_dec();
                Ok(STree::CALL { path, arguments })
            }

            // Periods
            Token::POINT => Err("Member access is only allowed as a call target".into()),


            // Identifier
            Token::ID { name } => {
                self.log.info("convert_identifier()");
                Ok(STree::ID { name: name.clone() })
            }




            Token::LIT_INT { value } => Ok(STree::LIT_INT { value: *value }),
            Token::LIT_FLOAT { value } => Ok(STree::LIT_FLOAT { value: *value }),
            Token::LIT_BOOL { value } => Ok(STree::LIT_BOOL { value: *value }),
            Token::LIT_CHAR { value } => Ok(STree::LIT_CHAR { value: *value }),
            Token::LIT_STRING { value } => Ok(STree::LIT_STRING { value: value.clone() }),
            Token::NULL => Ok(STree::NULL),

            Token::BLANK_STMT => Ok(STree::BLANK_STMT),

            other => {
                self.log.indent_dec();
                Err(format!("Unrecognized token in semantic conversion: {:?}", other ))
            }
        }
    }
}

// Helpers
impl Converter {
    fn extract_path(&self, node: &MTree, out: &mut Vec<String>) -> Result<(), String> {
        match &node.token {
            Token::ID { name } => {
                out.push(name.clone());
                Ok(())
            }
            Token::POINT => {
                if node.children.len() != 2 {
                    return Err("POINT must have exactly 2 children".into());
                }
                self.extract_path(&node.children[0], out)?;
                self.extract_path(&node.children[1], out)?;
                Ok(())
            }
            _ => Err(format!("Expected ID or POINT in qualified name, got {:?}", node.token)),
        }
    }

}