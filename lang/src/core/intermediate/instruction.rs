use std::fmt::Display;
use crate::core::intermediate::operator::{IntermediateBinaryOperator, IntermediateUnaryOperator, Value};

#[derive(Debug, Clone)]
pub enum IntermediateInstruction {
    RETURN(Value),
    UNARY { operator: IntermediateUnaryOperator, src: Value, dst: Value },
    BINARY { operator: IntermediateBinaryOperator, src1: Value, src2: Value, dst: Value },
    COPY {src: Value, dst: Value},
    JUMP { target: String },
    JUMP_IF_ZERO { condition: Value, target: String },
    JUMP_IF_NOT_ZERO { condition: Value, target: String },
    LABEL { label: String },
}

impl Display for IntermediateInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntermediateInstruction::RETURN(v) => write!(f, "ret {}", v),
            IntermediateInstruction::UNARY { operator, src, dst } => write!(f, "{} = {} {}", dst, operator, src),
            IntermediateInstruction::BINARY { operator, src1, src2, dst } => write!(f, "{} = {} {} {}", dst, src1, operator, src2),
            
            IntermediateInstruction::COPY { src, dst } => write!(f, "copy {} {}", src, dst),
            IntermediateInstruction::JUMP { target } => write!(f, "jmp {}", target),
            IntermediateInstruction::JUMP_IF_ZERO { condition, target } => write!(f, "jmpi {} {}", condition, target),
            IntermediateInstruction::JUMP_IF_NOT_ZERO { condition, target } => write!(f, "jmpin {} {}", condition, target),
            IntermediateInstruction::LABEL { label } => write!(f, "lbl {}", label),
        }
    }
}

