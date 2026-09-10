use std::fmt::{Display, Formatter};
use crate::core::intermediate::operator::UnaryOperator;
use crate::core::lowering::operand::Operand;

#[derive(Debug, Clone)]
pub enum MachineInstruction {
    MOVE { src: Operand, dst: Operand },
    UNARY { operator: UnaryOperator, operand: Operand },
    ALLOCATE_STACK(i32),
    RETURN
}

impl Display for MachineInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineInstruction::MOVE { src, dst } => write!(f, "mov {} {}", src, dst),
            MachineInstruction::UNARY { operator, operand } => write!(f, "{} {}", operator, operand),
            MachineInstruction::ALLOCATE_STACK(v) => write!(f, "alloc {}", v),
            MachineInstruction::RETURN => write!(f, "ret"),
        }
    }
}