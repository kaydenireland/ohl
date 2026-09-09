use crate::core::codegen::operator::UnaryOperator::{COMPLEMENT, NEGATE};
use crate::core::converter::operator::Operator;

#[derive(Debug, Clone)]
pub enum Value {
    INT(i32),
    VAR(String),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    NEGATE,
    COMPLEMENT
}

impl UnaryOperator {
    pub fn from(typ: Operator) -> UnaryOperator {
        match typ {
            Operator::NEGATE => NEGATE,
            Operator::COMPLEMENT => COMPLEMENT,

            _ => panic!("Unknown unary operator {:?}", typ)
        }
    }
}