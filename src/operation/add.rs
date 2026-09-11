use std::ops::RangeInclusive;

use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::Execute,
    register::Register,
};

const DR_FIELD: RangeInclusive<u8> = 5..=7;
const SR1_FIELD: RangeInclusive<u8> = 8..=10;
const ADD_MODE_FIELD: RangeInclusive<u8> = 11..=11;
const SR2_FIELD: RangeInclusive<u8> = 14..=16;
const IMM5_FIELD: RangeInclusive<u8> = 12..=16;
const IMM5_BIT_COUNT: u8 = 5;

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
    mode: AddMode,
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
        let mode = match raw.bits(ADD_MODE_FIELD)? {
            0 => AddMode::Register(raw.decode_register(SR2_FIELD)?),
            1 => AddMode::Immediate(raw.bits(IMM5_FIELD)?),
            _ => unreachable!(),
        };

        Ok(Self { dr, sr1, mode })
    }
}

impl Execute for AddOp {
    fn execute(self, vm: &mut crate::vm::VirtualMachine) {
        let sr2 = match self.mode {
            AddMode::Register(sr2) => vm.read_register(sr2),
            AddMode::Immediate(imm5) => sign_extend(imm5, IMM5_BIT_COUNT),
        };

        let result = vm.read_register(self.sr1).wrapping_add(sr2);

        vm.write_register(self.dr, result);

        vm.set_cond(result);
    }
}

/// Represents ADD mode based on instruction bit 11.
///
/// - 0 = [`AddMode::Register`]
/// - 1 = [`AddMode::Immediate`]
#[derive(Debug)]
enum AddMode {
    Register(Register),
    Immediate(u16),
}

/// Sign-extends an LC-3 value to 16 bits using two's complement representation.
///
/// The most significant bit of the value's bit field is used as the sign bit.
/// If set, the unused upper bits are filled with `1`s; otherwise, they remain
/// `0`.
///
/// # Examples
///
/// A 5-bit immediate value of `0b11111` represents `-1` and is sign-extended
/// to `0xFFFF`.
///
/// # Panics
///
/// Panics if `bit_count` is `0` or greater than `16`.
#[allow(clippy::arithmetic_side_effects)]
const fn sign_extend(value: u16, bit_count: u8) -> u16 {
    assert!(bit_count > 0 && bit_count <= 16);

    let shift = bit_count - 1;
    if (value >> shift) & 1 == 1 {
        value | (0xFFFF << bit_count)
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::RawInstruction;

    use super::*;

    #[test]
    fn create_add_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x1FFF));

        let add_op = AddOp::try_from(decoded);

        assert!(add_op.is_ok());
    }
}
