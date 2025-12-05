use std::collections::HashMap;

use crate::backend::token::Token;
use crate::backend::mtree::MTree;

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    INT,
    FLOAT,
    BOOLEAN,
    CHAR,
    STRING,
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
        self.variables.get(name).cloned().ok_or_else(format!( || "Variable '{}' is not declared.", name))
    }
}


// Semantic AST
#[derive(Debug)]
pub enum STree {
    START { functions: Vec<STree> },
    FUNCTION { name: String, params: Vec<(String, VariableType)>, return_type: VariableType, body: Box<STree> },
    BLOCK { statements: Vec<STree> },
    LET_STMT { id: String, var_type: VariableType, expression: Option<Box<STree>> },
    ASSIGN_STMT { id: String, expression: Box<STree> },
    IF_EXPR { condition: Box<STree>, then_block: Box<STree>, else_block: Option<Box<STree>> },
    WHILE_EXPR { condition: Box<STree>, body: Box<STree> },
    RETURN_STMT { expression: Option<Box<STree>> },
    PRINT_STMT { expression: Box<STree> },
    EXPR { left: Box<STree>, operator: Token, right: Box<STree> },
    CALL { name: String, arguments: Vec<STree> },
    ID { name: String },
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
    pub fn new(tree: MTree, debug: bool) -> Converter {
        let log = Logger::new(debug);
        Converter { log }
    }

    pub fn convert_tree(&mut self, node: MTree) -> Result<STree, String> {
        self.log.info("convert_tree()");
        self.log.indent_inc();
        
        match node.token {

            // Program Root: All Children are Functions
            Token::START => {
                let mut functions = Vec::new();
                for child in node.children {
                    let next = self.convert_tree(child)?;
                    functions.push(next);
                }
                self.log.indent_dec();
                Ok(STree::START { functions })
            }





            _ => {
                self.log.indent_dec();
                Err(format!("Unrecognized node in semantic conversion: {:?}", node.token))
            }
        }
    }
}