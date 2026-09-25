use std::fmt::Display;
use crate::oil::intermediate::operator::{IntermediateBinaryOperator, IntermediateUnaryOperator};

#[derive(Debug, Clone)]
pub enum MachineUnaryOperator {
    NEGATE,
    NOT
}

impl MachineUnaryOperator {
    pub fn from(typ: IntermediateUnaryOperator) -> MachineUnaryOperator {
        match typ {
            IntermediateUnaryOperator::NEGATE => MachineUnaryOperator::NEGATE,
            IntermediateUnaryOperator::COMPLEMENT | IntermediateUnaryOperator::NOT => MachineUnaryOperator::NOT,

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

    BIT_AND,
    BIT_OR,
    BIT_XOR,
    SHIFT_LEFT,
    SHIFT_RIGHT
}


impl MachineBinaryOperator {
    pub fn from(typ: IntermediateBinaryOperator) -> MachineBinaryOperator {
        match typ {
            IntermediateBinaryOperator::ADD => MachineBinaryOperator::ADD,
            IntermediateBinaryOperator::SUBTRACT  => MachineBinaryOperator::SUBTRACT,
            IntermediateBinaryOperator::MULTIPLY => MachineBinaryOperator::MULTIPLY,

            IntermediateBinaryOperator::BIT_AND => MachineBinaryOperator::BIT_AND,
            IntermediateBinaryOperator::BIT_OR => MachineBinaryOperator::BIT_OR,
            IntermediateBinaryOperator::BIT_XOR => MachineBinaryOperator::BIT_XOR,
            IntermediateBinaryOperator::SHIFT_LEFT => MachineBinaryOperator::SHIFT_LEFT,
            IntermediateBinaryOperator::SHIFT_RIGHT => MachineBinaryOperator::SHIFT_RIGHT,

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

            MachineBinaryOperator::BIT_AND => write!(f, "band"),
            MachineBinaryOperator::BIT_OR => write!(f, "bor"),
            MachineBinaryOperator::BIT_XOR => write!(f, "bxor"),
            MachineBinaryOperator::SHIFT_LEFT => write!(f, "slt"),
            MachineBinaryOperator::SHIFT_RIGHT => write!(f, "srt"),
        }
    }
}
