use std::fmt::{Display, Formatter};
use crate::core::intermediate::operator::UnaryOperator;
use crate::core::lowering::operand::Operand;

#[derive(Debug, Clone)]
pub enum AssemblyInstruction {
    MOVE {src: Operand, dst: Operand},
    UNARY {operator: UnaryOperator, operand: Operand},
    ALLOCATE_STACK(i32),
    RETURN
}

impl Display for AssemblyInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            AssemblyInstruction::MOVE { src, dst } => write!(f, "mov {} {}", src, dst),
            AssemblyInstruction::UNARY { operator, operand } => write!(f, "{} {}", operator, operand),
            AssemblyInstruction::ALLOCATE_STACK(v) => write!(f, "alloc {}", v),
            AssemblyInstruction::RETURN => write!(f, "ret"),
        }
    }
}