use std::fmt::{Display, Formatter};
use crate::core::lowering::condition::ConditionCode;
use crate::core::lowering::operand::Operand;
use crate::core::lowering::operator::{MachineBinaryOperator, MachineUnaryOperator};

#[derive(Debug, Clone)]
pub enum MachineInstruction {
    MOVE { src: Operand, dst: Operand },
    UNARY { operator: MachineUnaryOperator, operand: Operand },
    BINARY { operator: MachineBinaryOperator, operand1: Operand, operand2: Operand },
    COMPARE { operand1: Operand, operand2: Operand },
    IDIV(Operand),
    CDQ,
    JUMP(String),
    JUMP_CC { condition: ConditionCode, identifier: String },
    SET_CC { condition: ConditionCode, operand: Operand },
    LABEL(String),
    ALLOCATE_STACK(i32),
    RETURN,

}

impl Display for MachineInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineInstruction::MOVE { src, dst } => write!(f, "mov {} {}", src, dst),
            MachineInstruction::UNARY { operator, operand } => write!(f, "{} {}", operator, operand),
            MachineInstruction::BINARY { operator, operand1, operand2 } => write!(f, "{} {} {}", operator, operand1, operand2),
            MachineInstruction::COMPARE { operand1, operand2 } => write!(f, "cmp {} {}", operand1, operand2),
            MachineInstruction::IDIV(operand) => write!(f, "idiv{}", operand),
            MachineInstruction::CDQ => write!(f, "cdq"),
            MachineInstruction::JUMP(label) => write!(f, "jmp {}", label),
            MachineInstruction::JUMP_CC { condition, identifier } => write!(f, "jmpcc {} {}", condition, identifier),
            MachineInstruction::SET_CC { condition, operand } => write!(f, "setcc {} {}", condition, operand),
            MachineInstruction::LABEL(label) => write!(f, "lbl {}", label),
            MachineInstruction::ALLOCATE_STACK(v) => write!(f, "alloc {}", v),
            MachineInstruction::RETURN => write!(f, "ret"),
        }
    }
}