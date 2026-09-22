use std::fmt::{Display, Formatter};
use crate::core::lowering::operand::Operand;
use crate::core::lowering::operator::{MachineBinaryOperator, MachineUnaryOperator};

#[derive(Debug, Clone)]
pub enum MachineInstruction {
    MOVE { src: Operand, dst: Operand },
    UNARY { operator: MachineUnaryOperator, operand: Operand },
    BINARY { operator: MachineBinaryOperator, operand1: Operand, operand2: Operand },
    IDIV(Operand),
    CDQ,
    ALLOCATE_STACK(i32),
    RETURN
}

impl Display for MachineInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineInstruction::MOVE { src, dst } => write!(f, "mov {} {}", src, dst),
            MachineInstruction::UNARY { operator, operand } => write!(f, "{} {}", operator, operand),
            MachineInstruction::BINARY { operator, operand1, operand2 } => write!(f, "{} {} {}", operator, operand1, operand2),
            MachineInstruction::IDIV(operand) => write!(f, "idiv{}", operand),
            MachineInstruction::CDQ => write!(f, "cdq"),
            MachineInstruction::ALLOCATE_STACK(v) => write!(f, "alloc {}", v),
            MachineInstruction::RETURN => write!(f, "ret"),
        }
    }
}