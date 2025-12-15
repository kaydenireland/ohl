#![allow(warnings)]

use crate::language::analyzing::operator::Operator;
use crate::language::analyzing::types::{FunctionType, VariableType};

// Semantic AST
#[derive(Debug, Clone)]
pub enum STree {
    START { functions: Vec<STree> },
    FUNCTION { function_type: FunctionType, return_type: VariableType, name: String, params: Vec<(String, VariableType)>, body: Box<STree> },
    BLOCK { statements: Vec<STree> },
    LET_STMT { id: String, var_type: VariableType, expression: Option<Box<STree>> },
    ASSIGN_STMT { id: String, expression: Box<STree> },
    FOR_EXPR { init: Option<Box<STree>>, condition: Box<STree>, modifier: Option<Box<STree>>, body: Box<STree> },
    FOR_EACH { variable: String, iterable: Box<STree>, body: Box<STree> },
    IF_EXPR { condition: Box<STree>, then_block: Box<STree>, else_block: Option<Box<STree>> },
    WHILE_EXPR { condition: Box<STree>, body: Box<STree> },
    LOOP_EXPR { condition: Box<STree>, body: Box<STree> },
    RETURN_STMT { expression: Option<Box<STree>> },
    PRINT_STMT { expression: Box<STree> },
    EXPR { left: Box<STree>, operator: Operator, right: Box<STree> },
    PRFX_EXPR { operator: Operator, right: Box<STree> },
    PTFX_EXPR { left: Box<STree>, operator: Operator },
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