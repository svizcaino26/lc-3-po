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

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::{
        instruction::{DecodedInstruction, RawInstruction},
        vm::VirtualMachine,
    };

    use super::*;

    #[test]
    fn create_and_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x5FFF));

        let and_op = AndOp::decode(decoded);

        assert!(and_op.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn register_mode_and() {
        // AND R2, R3, R4
        let decoded = DecodedInstruction::from(RawInstruction::from(0x54C4));
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0001);
        vm.write_register(Register::R4, 0x0001);

        let and_op = AndOp::decode(decoded).unwrap();
        and_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0001);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Pos);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn immediate_mode_and() {
        // AND R2, R3, #7
        let decoded = DecodedInstruction::from(RawInstruction::from(0x54E7));
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0001);

        let and_op = AndOp::decode(decoded).unwrap();
        and_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 1);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Pos);
    }
}
