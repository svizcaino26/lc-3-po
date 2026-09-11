use super::IMM5_BIT_COUNT;
use crate::{
    operation::{sign_extend, BinaryOp, BinaryOpMode, BinaryOperands},
    register::Register,
};

/// Represents an LC-3 AND operation.
///
/// - `dr`: Destination register.
/// - `sr1`: First source register.
/// - `mode`: And mode [`BinaryOpMode::Register`] or [`BinaryOpMode::Immediate`]
///
/// A bitwise AND `&` operation is performed between `SR1` and the value in `SR2` or
/// a sign-extended 5-bit immediate operand. The result is stored in `DR`,
/// and the condition code is updated based on the result.
#[derive(Debug)]
pub struct AndOp {
    dr: Register,
    sr1: Register,
    mode: BinaryOpMode,
}

impl BinaryOp for AndOp {
    fn from_parts(dr: Register, sr1: Register, mode: BinaryOpMode) -> Self {
        Self { dr, sr1, mode }
    }

    fn operands(self, vm: &crate::vm::VirtualMachine) -> BinaryOperands {
        BinaryOperands {
            dr: self.dr,
            lhs: vm.read_register(self.sr1),
            rhs: match self.mode {
                BinaryOpMode::Register(sr2) => vm.read_register(sr2),
                BinaryOpMode::Immediate(imm5) => sign_extend(imm5, IMM5_BIT_COUNT),
            },
        }
    }

    fn operate(lhs: u16, rhs: u16) -> u16 {
        lhs & rhs
    }
}
