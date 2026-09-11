use super::IMM5_BIT_COUNT;
use crate::{
    operation::{sign_extend, BinaryOp, BinaryOpMode, BinaryOperands},
    register::Register,
};

/// Represents an LC-3 ADD operation.
///
/// - `dr`: Destination register.
/// - `sr1`: First source register.
/// - `mode`: ADD mode [`AddMode::Register`] or [`AddMode::Immediate`]
///
/// The operation adds the value in `SR1` to either the value in `SR2` or
/// a sign-extended 5-bit immediate operand. The result is stored in `DR`,
/// and the condition code is updated based on the result.
///
/// Arithmetic uses 16-bit wrapping semantics.
#[derive(Debug)]
pub struct AddOp {
    dr: Register,
    sr1: Register,
    mode: BinaryOpMode,
}

impl BinaryOp for AddOp {
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
        lhs.wrapping_add(rhs)
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
    fn create_add_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x1FFF));

        let add_op = AddOp::decode(decoded);

        assert!(add_op.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn register_mode_add() {
        // ADD R2, R3, R4
        let decoded = DecodedInstruction::from(RawInstruction::from(0x14C4));
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0001);
        vm.write_register(Register::R4, 0x0001);

        let add_op = AddOp::decode(decoded).unwrap();
        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0002);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Pos);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn immediate_mode_add() {
        // ADD R2, R3, #7
        let decoded = DecodedInstruction::from(RawInstruction::from(0x14E7));
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0001);

        let add_op = AddOp::decode(decoded).unwrap();
        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x008);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Pos);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn immediate_add_negative() {
        // ADD R2, R3, #-1
        let decoded = DecodedInstruction::from(RawInstruction::from(0x14FF));
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0005);

        let add_op = AddOp::decode(decoded).unwrap();
        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0004);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Pos);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn immediate_add_overflow_to_negative() {
        // ADD R2, R3, #1
        let decoded = DecodedInstruction::from(RawInstruction::from(0x14E1));
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x7FFF);

        let add_op = AddOp::decode(decoded).unwrap();
        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x8000);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Neg);
    }
}
