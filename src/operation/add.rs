use super::{DR_FIELD, IMM5_BIT_COUNT, IMM5_FIELD, MODE_FIELD, SR1_FIELD, SR2_FIELD};
use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::{sign_extend, BinaryOpMode, Execute},
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

impl TryFrom<DecodedInstruction> for AddOp {
    type Error = InstructionError;

    /// Creates an ADD operation [`AddOp`] from a [`DecodedInstruction`]
    ///
    /// # Errors
    ///
    /// Returns [`InstructionError`] if a bit field cannot be extracted from
    /// the underlying [`RawInstruction`].
    #[allow(clippy::unreachable)]
    fn try_from(instruction: DecodedInstruction) -> Result<Self, Self::Error> {
        let raw = instruction.raw();
        let dr = raw.decode_register(DR_FIELD)?;
        let sr1 = raw.decode_register(SR1_FIELD)?;
        let mode = match raw.bits(MODE_FIELD)? {
            0 => BinaryOpMode::Register(raw.decode_register(SR2_FIELD)?),
            1 => BinaryOpMode::Immediate(raw.bits(IMM5_FIELD)?),
            _ => unreachable!(),
        };

        Ok(Self { dr, sr1, mode })
    }
}

impl Execute for AddOp {
    fn execute(self, vm: &mut crate::vm::VirtualMachine) {
        let sr2 = match self.mode {
            BinaryOpMode::Register(sr2) => vm.read_register(sr2),
            BinaryOpMode::Immediate(imm5) => sign_extend(imm5, IMM5_BIT_COUNT),
        };

        let result = vm.read_register(self.sr1).wrapping_add(sr2);

        vm.write_register(self.dr, result);

        vm.set_cond(result);
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::{instruction::RawInstruction, vm::VirtualMachine};

    use super::*;

    #[test]
    fn create_add_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x1FFF));

        let add_op = AddOp::try_from(decoded);

        assert!(add_op.is_ok());
    }

    #[test]
    fn sign_extension() {
        assert_eq!(sign_extend(0b00000, 5), 0x0000);
        assert_eq!(sign_extend(0b00001, 5), 0x0001);
        assert_eq!(sign_extend(0b01111, 5), 0x000F);
        assert_eq!(sign_extend(0b10000, 5), 0xFFF0);
        assert_eq!(sign_extend(0b10001, 5), 0xFFF1);
        assert_eq!(sign_extend(0b11111, 5), 0xFFFF);
    }

    #[test]
    fn register_mode_add() {
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0001);
        vm.write_register(Register::R4, 0x0001);

        let add_op = AddOp {
            dr,
            sr1: Register::R3,
            mode: BinaryOpMode::Register(Register::R4),
        };

        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0002);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Pos);
    }

    #[test]
    fn immediate_mode_add() {
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0001);

        let add_op = AddOp {
            dr,
            sr1: Register::R3,
            mode: BinaryOpMode::Immediate(0b11111),
        };

        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Zro);
    }

    #[test]
    fn immediate_add_negative() {
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x0005);

        let add_op = AddOp {
            dr,
            sr1: Register::R3,
            mode: BinaryOpMode::Immediate(0b11111),
        };

        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0004);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Pos);
    }

    #[test]
    fn immediate_add_negative_result() {
        let mut vm = VirtualMachine::default();
        let dr = Register::R2;

        vm.write_register(Register::R3, 0x7FFF);

        let add_op = AddOp {
            dr,
            sr1: Register::R3,
            mode: BinaryOpMode::Immediate(0b1),
        };

        add_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x8000);
        assert_matches!(vm.read_cond(), crate::register::ConditionCode::Neg);
    }
}
