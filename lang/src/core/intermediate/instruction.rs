use std::fmt::Display;
use crate::core::intermediate::operator::{IntermediateBinaryOperator, IntermediateUnaryOperator, Value};

#[derive(Debug, Clone)]
pub enum IntermediateInstruction {
    RETURN(Value),
    UNARY { operator: IntermediateUnaryOperator, src: Value, dst: Value },
    BINARY { operator: IntermediateBinaryOperator, src1: Value, src2: Value, dst: Value },
}

impl Display for IntermediateInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntermediateInstruction::RETURN(v) => write!(f, "ret {}", v),
            IntermediateInstruction::UNARY { operator, src, dst } => write!(f, "{} = {} {}", dst, operator, src),
            IntermediateInstruction::BINARY { operator, src1, src2, dst } => write!(f, "{} = {} {} {}", dst, src1, operator, src2),
        }
    }
}

