use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Register {
    AX,
    DX,
    CX,
    R10,
    R11
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AX => write!(f, "eax"),
            Register::DX => write!(f, "edx"),
            Register::CX => write!(f, "ecx"),
            Register::R10 => write!(f, "r10d"),
            Register::R11 => write!(f, "r11d"),
        }
    }
}