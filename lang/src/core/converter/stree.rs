#![allow(warnings)]

use crate::core::lexer::token_type::TokenType;

// Semantic AST
#[derive(Debug, Clone, PartialEq)]
pub enum STree {
    START { functions: Vec<STree> },
    FUNCTION { function_type: TokenType, return_type: TokenType, name: String, params: Vec<(String, TokenType)>, body: Box<STree> },
    BLOCK { statements: Vec<STree> },
    VAR_TYPE { var_type: TokenType },

    // Expressions
    EXPR { left: Box<STree>, operator: TokenType, right: Box<STree> },
    PRFX_EXPR { operator: TokenType, right: Box<STree> },
    PTFX_EXPR { left: Box<STree>, operator: TokenType },

    // Literals
    ID { name: String },
    LIT_INT { value: i32 },
    LIT_FLOAT { value: f32 },
    LIT_BOOL { value: bool },
    LIT_STRING { value: String },
    LIT_CHAR { value: char },

    // Statements
    VAR_STMT { id: String, var_type: TokenType, mutable: bool, expression: Box<STree> },
    ASSIGN_STMT { id: String, expression: Box<STree> },
    RETURN_STMT { expression: Option<Box<STree>>},
    IF_STMT { condition: Box<STree>, then_block: Box<STree>, else_block: Option<Box<STree>> },
    WHILE_STMT { condition: Box<STree>, body: Box<STree> },
    DO_WHILE_STMT { condition: Box<STree>, body: Box<STree> },
    BREAK,
    CONTINUE,
    REPEAT,
    DEFER { body: Box<STree> },

    // Calls
    FUNCTION_CALL { callee: Box<STree>, args: Vec<STree> },
    MEMBER_CALL { object: Box<STree>, member: String },

    NULL,
    BLANK_STMT,
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