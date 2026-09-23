use std::fmt::{Display, Formatter};
use crate::core::intermediate::operator::IntermediateBinaryOperator;

#[derive(Clone, Debug)]
pub enum ConditionCode {
    EQUAL,
    NOT_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL
}

impl Display for ConditionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionCode::EQUAL => write!(f, "e"),
            ConditionCode::NOT_EQUAL => write!(f, "ne"),
            ConditionCode::GREATER => write!(f, "g"),
            ConditionCode::GREATER_EQUAL => write!(f, "ge"),
            ConditionCode::LESS => write!(f, "l"),
            ConditionCode::LESS_EQUAL => write!(f, "le"),
        }
    }
}

impl ConditionCode {
    pub fn from_inter_operator(op: &IntermediateBinaryOperator) -> ConditionCode {
        match op { 
            IntermediateBinaryOperator::EQUAL => ConditionCode::EQUAL,
            IntermediateBinaryOperator::NOT_EQUAL => ConditionCode::NOT_EQUAL,
            IntermediateBinaryOperator::GREATER_THAN => ConditionCode::GREATER,
            IntermediateBinaryOperator::GREATER_OR_EQUAL => ConditionCode::GREATER_EQUAL,
            IntermediateBinaryOperator::LESS_THAN => ConditionCode::LESS,
            IntermediateBinaryOperator::LESS_OR_EQUAL => ConditionCode::LESS_EQUAL,
            
            _ => panic!("Invalid conditional operator: {}", op)
        }
    }
}