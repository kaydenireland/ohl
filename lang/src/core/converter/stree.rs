#![allow(warnings)]

use crate::core::analyzer::variable::VariableType;
use crate::core::lexer::token_type::TokenType;

// Semantic AST
#[derive(Debug, Clone, PartialEq)]
pub enum STree {
    START { classes: Vec<STree> },
    CLASS { scope: TokenType, name: String, body: Box<STree> },
    CLASS_BODY { variables: Vec<STree>, functions: Vec<STree> },
    FUNCTION { scope: TokenType, name: String, params: Vec<(String, VariableType)>, return_type: VariableType, body: Box<STree> },
    BLOCK { statements: Vec<STree> },
    VAR_TYPE { var_type: TokenType },

    // Expressions
    EXPR { left: Box<STree>, operator: TokenType, right: Box<STree> },
    PRFX_EXPR { operator: TokenType, right: Box<STree> },
    // PTFX_EXPR { left: Box<STree>, operator: TokenType },

    // Literals
    ID { name: String },
    LIT_INT { value: i32 },
    LIT_FLOAT { value: f32 },
    LIT_BOOL { value: bool },
    LIT_STRING { value: String },
    LIT_CHAR { value: char },

    // Statements
    CLASS_VAR_DECL { scope: TokenType, id: String, var_type: VariableType, mutable: bool, expression: Box<STree> },
    VAR_DECL { id: String, var_type: VariableType, mutable: bool, expression: Box<STree> },
    VAR_ASSIGN { id: String, expression: Box<STree> },
    RETURN_STMT { expression: Option<Box<STree>>},
    IF_STMT { condition: Box<STree>, then_block: Box<STree>, else_block: Option<Box<STree>> },
    WHILE_STMT { condition: Box<STree>, body: Box<STree> },
    DO_WHILE_STMT { condition: Box<STree>, body: Box<STree> },
    BREAK,
    CONTINUE,
    REPEAT,

    // Calls
    FUNCTION_CALL { callee: Box<STree>, args: Vec<STree> },
    MEMBER_CALL { object: Box<STree>, member: String },

    NULL,
    BLANK,
    PRINT { expression: Box<STree> }
}

impl STree {
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            STree::LIT_INT { .. }
            | STree::LIT_FLOAT { .. }
            | STree::LIT_BOOL { .. }
            | STree::LIT_CHAR { .. }
            | STree::LIT_STRING { .. }
        )
    }
}