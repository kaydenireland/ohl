use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Operand {
    IMM(i32),
    REG(Register),
    PSEUDO(String),
    STACK(i32)
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::IMM(v) => write!(f, "{}", v),
            Operand::REG(v) => write!(f, "${}", v),
            Operand::PSEUDO(v) => write!(f, "%{}", v),
            Operand::STACK(v) => write!(f, "stack {}", v)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Register {
    AX,
    R10
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AX => write!(f, "ax"),
            Register::R10 => write!(f, "r10"),
        }
    }
}