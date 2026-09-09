use std::fmt::Display;
use crate::core::intermediate::operator::{UnaryOperator, Value};

#[derive(Debug, Clone)]
pub enum IntermediateInstruction {
    RETURN(Value),
    UNARY { operator: UnaryOperator, src: Value, dst: Value },
}

impl Display for IntermediateInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntermediateInstruction::RETURN(v) => write!(f, "ret {}", v),
            IntermediateInstruction::UNARY { operator, src, dst } => write!(f, "{} = {} {}", dst, operator, src),
        }
    }
}

