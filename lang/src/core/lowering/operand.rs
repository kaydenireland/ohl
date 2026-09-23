use std::fmt::{Display, Formatter};
use crate::core::lowering::register::Register;

#[derive(Debug, Clone)]
pub enum Operand {
    IMM(i32),
    REG(Register),
    PSEUDO(String),
    STACK(i32)
}

impl Operand {
    pub fn is_memory(&self) -> bool {
        matches!(self, Operand::STACK(_))
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::IMM(v) => write!(f, "${}", v),
            Operand::REG(v) => write!(f, "%{}", v),
            Operand::PSEUDO(v) => write!(f, "{}", v),
            Operand::STACK(v) => write!(f, "{}(%rbp)", v)
        }
    }
}