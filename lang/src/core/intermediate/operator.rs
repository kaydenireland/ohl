use std::fmt::Display;
use crate::core::converter::operator::Operator;

#[derive(Debug, Clone)]
pub enum Value {
    INT(i32),
    VAR(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::INT(x) => write!(f, "{}", x),
            Value::VAR(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntermediateUnaryOperator {
    NEGATE,
    NOT
}

impl IntermediateUnaryOperator {
    pub fn from(typ: Operator) -> IntermediateUnaryOperator {
        match typ {
            Operator::NEGATE => IntermediateUnaryOperator::NEGATE,
            Operator::COMPLEMENT => IntermediateUnaryOperator::NOT,

            _ => panic!("Unknown unary operator {:?}", typ)
        }
    }
}

impl Display for IntermediateUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntermediateUnaryOperator::NOT => write!(f, "not"),
            IntermediateUnaryOperator::NEGATE => write!(f, "neg"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntermediateBinaryOperator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER
}


impl IntermediateBinaryOperator {
    pub fn from(typ: Operator) -> IntermediateBinaryOperator {
        match typ {
            Operator::ADD => IntermediateBinaryOperator::ADD,
            Operator::SUBTRACT | Operator::NEGATE => IntermediateBinaryOperator::SUBTRACT,
            Operator::MULTIPLY => IntermediateBinaryOperator::MULTIPLY,
            Operator::DIVIDE | Operator::RECIPRICOL => IntermediateBinaryOperator::DIVIDE,
            Operator::REMAINDER => IntermediateBinaryOperator::REMAINDER,

            _ => panic!("Unknown binary operator {:?}", typ)
        }
    }
}

impl Display for IntermediateBinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntermediateBinaryOperator::ADD => write!(f, "add"),
            IntermediateBinaryOperator::SUBTRACT => write!(f, "sub"),
            IntermediateBinaryOperator::MULTIPLY => write!(f, "mlt"),
            IntermediateBinaryOperator::DIVIDE => write!(f, "div"),
            IntermediateBinaryOperator::REMAINDER => write!(f, "mod"),
        }
    }
}
