use std::fmt::Display;
use crate::core::converter::operator::Operator;
use crate::core::intermediate::operator::{IntermediateBinaryOperator, IntermediateUnaryOperator};

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
pub enum MachineUnaryOperator {
    NEGATE,
    NOT
}

impl MachineUnaryOperator {
    pub fn from(typ: IntermediateUnaryOperator) -> MachineUnaryOperator {
        match typ {
            IntermediateUnaryOperator::NEGATE => MachineUnaryOperator::NEGATE,
            IntermediateUnaryOperator::NOT => MachineUnaryOperator::NOT,

            _ => panic!("Unknown intermediate unary operator {:?}", typ)
        }
    }
}

impl Display for MachineUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineUnaryOperator::NOT => write!(f, "not"),
            MachineUnaryOperator::NEGATE => write!(f, "neg"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MachineBinaryOperator {
    ADD,
    SUBTRACT,
    MULTIPLY,
}


impl MachineBinaryOperator {
    pub fn from(typ: IntermediateBinaryOperator) -> MachineBinaryOperator {
        match typ {
            IntermediateBinaryOperator::ADD => MachineBinaryOperator::ADD,
            IntermediateBinaryOperator::SUBTRACT  => MachineBinaryOperator::SUBTRACT,
            IntermediateBinaryOperator::MULTIPLY => MachineBinaryOperator::MULTIPLY,

            _ => panic!("Unknown intermediate binary operator {:?}", typ)
        }
    }
}

impl Display for MachineBinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineBinaryOperator::ADD => write!(f, "add"),
            MachineBinaryOperator::SUBTRACT => write!(f, "sub"),
            MachineBinaryOperator::MULTIPLY => write!(f, "mlt"),
        }
    }
}
