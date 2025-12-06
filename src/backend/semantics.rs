use std::collections::HashMap;

use crate::backend::token::Token;
use crate::backend::mtree::MTree;
use crate::backend::logger::Logger;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    PUBLIC,
    PROTECTED,
    PRIVATE
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    INT,
    FLOAT,
    BOOLEAN,
    CHAR,
    STRING,
    NULL,
}

#[derive(Debug)]
pub struct SymbolTable {
    variables: HashMap<String, VariableType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
        }
    }

    // TODO: Proper Error Handling
    pub fn declare_variable(&mut self, name: String, var_type: VariableType) {
        if self.variables.contains_key(&name) {
            panic!("Variable '{}' is already declared.", name);
        } else {
            self.variables.insert(name, var_type);
        }
    }

    pub fn check_variable(&self, name: &String) -> Result<VariableType, String> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Variable '{}' is not declared.", name))

    }
}


// Semantic AST
#[derive(Debug)]
pub enum STree {
    START { functions: Vec<STree> },
    FUNCTION { function_type: FunctionType, return_type: VariableType, name: String, params: Vec<(String, VariableType)>, body: Box<STree> },
    BLOCK { statements: Vec<STree> },
    LET_STMT { id: String, var_type: VariableType, expression: Option<Box<STree>> },
    ASSIGN_STMT { id: String, expression: Box<STree> },
    IF_EXPR { condition: Box<STree>, then_block: Box<STree>, else_block: Option<Box<STree>> },
    WHILE_EXPR { condition: Box<STree>, body: Box<STree> },
    LOOP_EXPR { condition: Box<STree>, body: Box<STree> },
    RETURN_STMT { expression: Option<Box<STree>> },
    PRINT_STMT { expression: Box<STree> },
    EXPR { left: Box<STree>, operator: Token, right: Box<STree> },
    CALL { name: String, arguments: Vec<STree> },
    ID { name: String },
    BREAK,
    CONTINUE,
    REPEAT,
    LIT_INT { value: i32 },
    LIT_FLOAT { value: f32 },
    LIT_BOOL { value: bool },
    LIT_CHAR { value: char },
    LIT_STRING { value: String },
}


pub struct Converter {
    log: Logger,
}

impl Converter {
    pub fn new(debug: bool) -> Converter {
        let log = Logger::new(debug);
        Converter { log }
    }

    pub fn convert_tree(&mut self, node: &MTree) -> Result<STree, String> {
        
        match node.token {

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
            // [ FunctionType, ReturnType, Id(name), PARAM_LIST, BLOCK ]
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
                // Block
                let block_node = iterator.next().ok_or("Missing Function Block")?;
                let body = self.convert_tree(block_node)?;

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



            _ => {
                self.log.indent_dec();
                Err(format!("Unrecognized node in semantic conversion: {:?}", node.token))
            }
        }
    }
}