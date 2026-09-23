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
    COMPLEMENT,
    NOT
}

impl IntermediateUnaryOperator {
    pub fn from(typ: Operator) -> IntermediateUnaryOperator {
        match typ {
            Operator::NEGATE => IntermediateUnaryOperator::NEGATE,
            Operator::COMPLEMENT => IntermediateUnaryOperator::COMPLEMENT,
            Operator::NOT => IntermediateUnaryOperator::NOT,

            _ => panic!("Unknown unary operator {:?}", typ)
        }
    }
}

impl Display for IntermediateUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntermediateUnaryOperator::COMPLEMENT => write!(f, "not"),
            IntermediateUnaryOperator::NEGATE => write!(f, "neg"),
            IntermediateUnaryOperator::NOT => write!(f, "not"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntermediateBinaryOperator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    REMAINDER,

    BIT_AND,
    BIT_OR,
    BIT_XOR,
    SHIFT_LEFT,
    SHIFT_RIGHT,

    EQUAL,
    NOT_EQUAL,
    GREATER_THAN,
    GREATER_OR_EQUAL,
    LESS_THAN,
    LESS_OR_EQUAL,
}


impl IntermediateBinaryOperator {
    pub fn from(typ: Operator) -> IntermediateBinaryOperator {
        match typ {
            Operator::ADD => IntermediateBinaryOperator::ADD,
            Operator::SUBTRACT | Operator::NEGATE => IntermediateBinaryOperator::SUBTRACT,
            Operator::MULTIPLY => IntermediateBinaryOperator::MULTIPLY,
            Operator::DIVIDE | Operator::RECIPRICOL => IntermediateBinaryOperator::DIVIDE,
            Operator::REMAINDER => IntermediateBinaryOperator::REMAINDER,

            Operator::BIT_AND => IntermediateBinaryOperator::BIT_AND,
            Operator::BIT_OR => IntermediateBinaryOperator::BIT_OR,
            Operator::BIT_XOR => IntermediateBinaryOperator::BIT_XOR,
            Operator::SHIFT_LEFT => IntermediateBinaryOperator::SHIFT_LEFT,
            Operator::SHIFT_RIGHT => IntermediateBinaryOperator::SHIFT_RIGHT,

            Operator::EQUAL => IntermediateBinaryOperator::EQUAL,
            Operator::NOT_EQUAL => IntermediateBinaryOperator::NOT_EQUAL,
            Operator::GREATER_THAN => IntermediateBinaryOperator::GREATER_THAN,
            Operator::GREATER_THAN_EQUAL => IntermediateBinaryOperator::GREATER_OR_EQUAL,
            Operator::LESS_THAN => IntermediateBinaryOperator::LESS_THAN,
            Operator::LESS_THAN_EQUAL => IntermediateBinaryOperator::LESS_OR_EQUAL,

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

            IntermediateBinaryOperator::BIT_AND => write!(f, "band"),
            IntermediateBinaryOperator::BIT_OR => write!(f, "bor"),
            IntermediateBinaryOperator::BIT_XOR => write!(f, "bxor"),
            IntermediateBinaryOperator::SHIFT_LEFT => write!(f, "slt"),
            IntermediateBinaryOperator::SHIFT_RIGHT => write!(f, "srt"),

            IntermediateBinaryOperator::EQUAL => write!(f, "eq"),
            IntermediateBinaryOperator::NOT_EQUAL => write!(f, "neq"),
            IntermediateBinaryOperator::GREATER_THAN => write!(f, "gt"),
            IntermediateBinaryOperator::GREATER_OR_EQUAL => write!(f, "ge"),
            IntermediateBinaryOperator::LESS_THAN => write!(f, "lt"),
            IntermediateBinaryOperator::LESS_OR_EQUAL => write!(f, "le"),
        }
    }
}
