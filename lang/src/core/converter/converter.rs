use crate::core::{converter::stree::STree, parser::mtree::MTree, lexer::token_type::TokenType};
use crate::core::analyzer::signature::VariableType;
use crate::{indent_reset, indent_inc, indent_dec, info, log_debug};

pub struct Converter { }

impl Converter {
    pub fn new(_debug: bool) -> Converter {
        log_debug!(_debug);
        Converter {  }
    }

    pub fn convert_tree(&mut self, node: &MTree) -> Result<STree, String> {
        let converted_tree_result = self.convert(node);
        indent_reset!();
        converted_tree_result
    }

    fn convert(&mut self, node: &MTree) -> Result<STree, String> {
        
        match &node.token.token_type {

            // Program Root: All Children are Classes
            TokenType::START => {
                info!("convert_program()");
                indent_inc!();

                let mut classes = Vec::new();
                for child in &node.children {
                    let next = self.convert(child)?;
                    classes.push(next);
                }
                indent_dec!();
                Ok(STree::START { classes })
            }

            // Expected Function Declaration Children
            // [ ScopeModifier, ID, ClassBody ]
            TokenType::CLASS => {
                info!("convert_class()");
                indent_inc!();

                let scope = node.children.get(0).unwrap();
                let name_node = node.children[1].token.token_type.clone();
                let class_name: String = match &name_node {
                    TokenType::ID { name } => name.clone(),
                    _ => return Err("Expected ID in Class Declaration".into()),
                };

                let block_node = &node.children[2];
                let body = self.convert(&block_node)?;

                indent_dec!();

                indent_dec!();
                Ok(STree::CLASS { scope: scope.token.token_type.clone(), name: class_name, body: Box::new(body) })
            }

            // Expected Class Body
            // [ Vec<ClassVariable>, Vec<Function> ]
            TokenType::CLASS_BODY => {
                info!("convert_class_body()");
                indent_inc!();


                let mut variables = Vec::new();
                let mut functions = Vec::new();
                for child in &node.children {

                    if child.token.token_type == TokenType::CLASS_VARIABLE {
                        info!("convert_class_variable()");
                        indent_inc!();

                        let next = self.convert(child)?;
                        variables.push(next);
                    } else {
                        info!("convert_function()");
                        indent_inc!();

                        let next = self.convert(child)?;
                        functions.push(next);
                    }

                    indent_dec!();
                }
                indent_dec!();
                Ok(STree::CLASS_BODY { variables, functions })
            }

            // Expected Variable Declaration Children
            // [ Scope, ID(name), Option<VARIABLE_TYPE>, Option<Expression> ]
            TokenType::CLASS_VARIABLE => {
                info!("convert_class_variable()");
                indent_inc!();

                let mut mutable = true;

                let scope = node.children.get(0).unwrap().token.token_type.clone();

                let id_node = node.children.get(1).ok_or("Variable Missing ID")?;
                let id = match &id_node.token.token_type {
                    TokenType::ID { name } => name.clone(),
                    _ => return Err("Unexpected ID in Variable".into()),
                };

                let declared_type = &node.children[2].token.token_type;

                let expression = if node.children.len() >= 4 {
                    self.convert(&node.children[3])?
                } else {
                    STree::NULL
                };

                let var_type = match declared_type {
                    TokenType::INFER => {
                        self.infer_expression_type(&expression)?
                    }
                    _ => self.token_to_variable_type(declared_type, false)?
                };

                indent_dec!();

                Ok(STree::CLASS_VAR_DECL { scope, id, var_type, mutable, expression: Box::new(expression) })
            }

            // Expected Function Declaration Children
            // [ Scope, ID(name) PARAM_LIST, ReturnType, BLOCK ]
            TokenType::FUNCTION => {

                info!("convert_function_decl()");
                
                let scope = node.children[0].token.token_type.clone();

                let name_node = node.children[1].token.token_type.clone();
                let function_name: String = match &name_node {
                    TokenType::ID { name } => name.clone(),
                    _ => return Err("Expected ID in Function Declaration".into()),
                };

                info!("convert_param_list()");
                indent_inc!();

                let params_node = &node.children[2];
                let mut params: Vec<(String, VariableType)> = Vec::new();
                for param_node in &params_node.children {
                    info!("convert_param()");

                    let id_node = param_node.children.get(0).ok_or("Param Missing ID")?;
                    let type_node = param_node.children.get(1).ok_or("Param Missing Type")?;


                    let param_name = match &id_node.token.token_type {
                        TokenType::ID { name } => name,
                        _ => return Err("Expected ID in param".into()),
                    };
                    let param_type_token = type_node.token.token_type.clone();
                    let param_type = self.token_to_variable_type(&param_type_token, false)?;
                    params.push((param_name.to_string(), param_type));
                }

                indent_dec!();

                let return_type_token = node.children[3].token.token_type.clone();
                let return_type = self.token_to_variable_type(&return_type_token, true)?;


                info!("convert_block()");
                indent_inc!();
                // Block
                let block_node = &node.children[4];
                let body = self.convert(&block_node)?;
                
                //indent_dec!();

                Ok(
                    STree::FUNCTION {
                        scope,
                        name: function_name,
                        params,
                        return_type,
                        body: Box::new(body),
                    }
                )
            }

            TokenType::BLOCK => {
                let mut statements = Vec::new();
                for child in &node.children {
                    let stmt = self.convert(child)?;
                    statements.push(stmt);
                }
                Ok(STree::BLOCK { statements })
            }

            // Expected Variable Declaration Children
            // [ ID(name), Option<VARIABLE_TYPE>, Option<Expression> ]
            TokenType::VARIABLE => {
                info!("convert_var_statement()");
                indent_inc!();

                let id_node = node.children
                    .get(0)
                    .ok_or("Variable Missing ID")?;

                let id = match &id_node.token.token_type {
                    TokenType::ID { name } => name.clone(),
                    _ => return Err("Expected ID in Variable".into()),
                };

                let declared_type = &node.children[1].token.token_type;

                let expression = if node.children.len() >= 3 {
                    self.convert(&node.children[2])?
                } else {
                    STree::NULL
                };

                let var_type = match declared_type {
                    TokenType::INFER => {
                        self.infer_expression_type(&expression)?
                    }

                    _ => {
                        self.token_to_variable_type(declared_type, false)?
                    }
                };

                indent_dec!();

                Ok(STree::VAR_DECL {
                    id,
                    var_type,
                    mutable: true,
                    expression: Box::new(expression),
                })
            }

            // Expected Assignment Children
            // [ ID(name), VARIABLE_TYPE, Option<Expression> ]
            TokenType::ASSIGN => {
                info!("convert_assignment()");
                indent_inc!();

                if node.children.len() != 2 {
                    return Err("Assignment must have left and right side".into());
                }

                let left = &node.children[0];
                let id = match &left.token.token_type {
                    TokenType::ID { name } => name.clone(),
                    _ => return Err("Left side of assignment must be an ID".into()),
                };
                let right = self.convert(&node.children[1])?;

                indent_dec!();

                Ok(STree::VAR_ASSIGN { id, expression: Box::new(right) })
            }


            /*
            TokenType::ADD_ASSIGN | TokenType::SUB_ASSIGN | TokenType::MULT_ASSIGN |TokenType::DIV_ASSIGN
            | TokenType::REM_ASSIGN | TokenType::POWER_ASSIGN | TokenType::ROOT_ASSIGN => {
                info!("convert_expression_assignment()");
                indent_inc!();

                let variable_node = node.children.get(0).ok_or("Assignment missing left side")?;
                let variable = self.convert(variable_node)?;
                let name = match &variable_node.token.token_type {
                    TokenType::ID { name } => name.clone(),
                    _ => return Err("Left side of assignment must be an ID".into()),
                };

                let right_node = node.children.get(1).ok_or("Assignment missing right side")?;
                let expression = self.convert(right_node)?;

                let operator = node.token.token_type;

                let combined = STree::EXPR { left: Box::new(variable.clone()), operator, right: Box::new(expression) };

                indent_dec!();

                Ok(STree::VAR_ASSIGN { id: name, expression: Box::new(combined) })
            }*/

            // Expected Print Children
            // [ Expression ]
            TokenType::PRINT => {
                info!("convert_return()");
                indent_inc!();

                let expression_node = node.children.get(0).unwrap();
                let expression = self.convert(expression_node)?;

                indent_dec!();
                Ok(STree::PRINT { expression: Box::new(expression) })
            }

            // Expected Return Children
            // [ Expression ]
            TokenType::RETURN => {
                info!("convert_return()");
                indent_inc!();

                let expression_node = node.children.get(0);
                match expression_node {
                    Some(_) => {
                        let expression = self.convert(expression_node.unwrap())?;
                        indent_dec!();
                        Ok(STree::RETURN_STMT { expression: Some(Box::new(expression)) })
                    },
                    None => {
                        indent_dec!();
                        Ok(STree::RETURN_STMT { expression: None })
                    }
                }
            }

            // Unary Prefix Only Operators 
            TokenType::NOT => {
                info!("convert_unary_op()");
                indent_inc!();

                if node.children.len() != 1 {
                    return Err("Unary Prefix NOT must have one child".into());
                }

                let child = self.convert(&node.children[0])?;

                indent_dec!();

                Ok(STree::PRFX_EXPR { operator: TokenType::NOT, right: Box::new(child) })
            }

            // Binary Operators
            TokenType::PLUS | TokenType::DASH 
            | TokenType::STAR | TokenType::SLASH | TokenType::PERCENT 
            | TokenType::POWER | TokenType::ROOT 
            | TokenType::EQUAL | TokenType::NOT_EQUAL 
            | TokenType::LESS | TokenType::GREATER 
            | TokenType::LESS_EQUAL | TokenType::GREATER_EQUAL 
            | TokenType::AND | TokenType::OR | TokenType::XOR => {

                // Check for Unary
                if node.children.len() == 1 {
                    info!("convert_unary_op()");
                    indent_inc!();

                    let child = self.convert(&node.children[0])?;
                    let operator = node.token.token_type.clone();

                    indent_dec!();

                    Ok(STree::PRFX_EXPR { operator, right: Box::new(child) })
                } else if node.children.len() == 2 {
                    info!("convert_binary_op()");
                    indent_inc!();

                    let left = self.convert(&node.children[0])?;
                    let right = self.convert(&node.children[1])?;
                    let operator = node.token.token_type.clone();

                    indent_dec!();
                    Ok(STree::EXPR { left: Box::new(left), operator, right: Box::new(right) })
                } else {
                    return Err("Operator must have either one or two children".into());
                }
            },

            // Expected If Children
            // [ Expression, Body, Else(Else if) ]
            TokenType::IF => {
                info!("convert_if()");
                indent_inc!();

                // condition
                let condition_node = node.children.get(0).ok_or("If statement missing condition")?;
                let condition = self.convert(condition_node)?;

                // then block
                let then_node = node.children.get(1).ok_or("If statement missing then block")?;
                let then_block = self.convert(then_node)?;

                // else or else-if
                let else_block = if node.children.len() > 2 {
                    let else_node = &node.children[2];
                    Some(Box::new(self.convert(else_node)?))
                } else {
                    None
                };

                indent_dec!();
                Ok(STree::IF_STMT {
                    condition: Box::new(condition),
                    then_block: Box::new(then_block),
                    else_block,
                })
            },

            // Expected While Children
            // [ Expression, Body ]
            TokenType::WHILE => {
                info!("convert_while()");
                indent_inc!();

                let condition_node = node.children.get(0).ok_or("While missing condition")?;
                let condition = self.convert(condition_node)?;

                let body_node = node.children.get(1).ok_or("While missing body")?;
                let body = self.convert(body_node)?;

                indent_dec!();

                Ok(STree::WHILE_STMT { condition: Box::new(condition), body: Box::new(body) })
            },

            // Expected Do-While Children
            // [ Body, Expression ]
            TokenType::DO => {
                info!("convert_while()");
                indent_inc!();

                let condition_node = node.children.get(1).ok_or("While missing condition")?;
                let condition = self.convert(condition_node)?;

                let body_node = node.children.get(0).ok_or("While missing body")?;
                let body = self.convert(body_node)?;

                indent_dec!();

                Ok(STree::DO_WHILE_STMT { condition: Box::new(condition), body: Box::new(body) })
            },

            TokenType::BREAK => Ok(STree::BREAK),
            TokenType::CONTINUE => Ok(STree::CONTINUE),
            TokenType::REPEAT => Ok(STree::REPEAT),

            // Expected Call Children
            // [ Id/Dot, Arg_List ]
            TokenType::CALL => {
                info!("convert_call()");
                indent_inc!();

                
                let callee_node = node.children.get(0).ok_or("Call missing callee")?;
                let callee = self.convert(callee_node)?;

                // Remaining children are args (depends on your parser shape)
                let mut args = Vec::new();

                if node.children.len() > 1 {
                    let args_node = &node.children[1];

                    for arg_node in &args_node.children {
                        args.push(self.convert(arg_node)?);
                    }
                }

                indent_dec!();

                Ok(STree::FUNCTION_CALL {
                    callee: Box::new(callee),
                    args,
                })
            },

            TokenType::PERIOD => {
                info!("convert_dot()");
                indent_inc!();

                let left = self.convert(&node.children[0])?;

                let right_node = &node.children[1];
                let member = match &right_node.token.token_type {
                    TokenType::ID { name } => name.clone(),
                    _ => return Err("Right side of '.' must be an identifier".into()),
                };

                indent_dec!();

                Ok(STree::MEMBER_CALL {
                    object: Box::new(left),
                    member,
                })
            }

            // Identifier
            TokenType::ID { name } => {
                info!("convert_identifier()");
                Ok(STree::ID { name: name.clone() })
            }

            TokenType::LIT_INT { value } => Ok(STree::LIT_INT { value: *value }),
            TokenType::LIT_FLOAT { value } => Ok(STree::LIT_FLOAT { value: *value }),
            TokenType::TRUE => Ok(STree::LIT_BOOL { value: true }),
            TokenType::FALSE => Ok(STree::LIT_BOOL { value: false }),

            TokenType::LIT_CHAR { value } => Ok(STree::LIT_CHAR { value: *value }),
            TokenType::LIT_STRING { value } => Ok(STree::LIT_STRING { value: value.clone() }),
            TokenType::NULL => Ok(STree::NULL),

            TokenType::SEMICOLON => Ok(STree::BLANK),

            TokenType::INT | TokenType::FLOAT
            | TokenType::BOOLEAN
            | TokenType::CHAR 
            | TokenType::STRING => {
            

                Ok(STree::VAR_TYPE { var_type: node.token.token_type.clone() })
            }


            other => {
                indent_dec!();
                Err(format!("Unrecognized token in semantic conversion: {:?}", other ))
            }
        }
    }

}

impl Converter {

    pub fn infer_expression_type(&self, expression: &STree ) -> Result<VariableType, String> {
        match expression {
            STree::LIT_INT { .. } => Ok(VariableType::INT),
            STree::LIT_FLOAT { .. } => Ok(VariableType::FLOAT),
            STree::LIT_BOOL { .. } => Ok(VariableType::BOOLEAN),
            STree::LIT_CHAR { .. } => Ok(VariableType::CHAR),
            STree::LIT_STRING { .. } => Ok(VariableType::STRING),

            _ => Err("Unable to infer expression type".into()),
        }
    }
    
    pub fn token_to_variable_type(&self, token_type: &TokenType, allow_void: bool) -> Result<VariableType, String> {
        match token_type {
            TokenType::STRING => Ok(VariableType::STRING),
            TokenType::CHAR => Ok(VariableType::CHAR),
            TokenType::INT => Ok(VariableType::INT),
            TokenType::FLOAT => Ok(VariableType::FLOAT),
            TokenType::BOOLEAN => Ok(VariableType::BOOLEAN),

            TokenType::INFER => Ok(VariableType::UNKNOWN),

            TokenType::VOID => {
                if allow_void {
                    Ok(VariableType::VOID)
                } else {
                    Err("Invalid variable type".to_string())
                }
            },
            
            _ => Err("Invalid token for variable type".to_string())
        }
    }

}
