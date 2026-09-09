use crate::core::codegen::operator::{UnaryOperator, Value};

#[derive(Debug, Clone)]
pub enum Instruction {
    RETURN(Value),
    UNARY { operator: UnaryOperator, src: Value, dst: Value },
}