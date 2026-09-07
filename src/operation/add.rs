use std::ops::RangeInclusive;

use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::{add::AddMode::Immediate, Execute},
};

const DR_FIELD: RangeInclusive<u8> = 5..=7;
const SR1_FIELD: RangeInclusive<u8> = 8..=10;
const ADD_MODE_FIELD: RangeInclusive<u8> = 11..=11;
const SR2_FIELD: RangeInclusive<u8> = 14..=16;
const IMM5_FIELD: RangeInclusive<u8> = 12..=16;

/// Represents an ADD operation.
///
/// - `dr`: Destination register.
/// - `sr1`: First source register.
/// - `mode`: ADD mode [`AddMode::Register`] or [`AddMode::Immediate`]
#[derive(Debug)]
pub struct AddOp {
    dr: usize,
    sr1: usize,
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
        let dr = usize::from(raw.bits(DR_FIELD)?);
        let sr1 = usize::from(raw.bits(SR1_FIELD)?);
        let mode = match raw.bits(ADD_MODE_FIELD)? {
            0 => AddMode::Register(usize::from(raw.bits(SR2_FIELD)?)),
            1 => AddMode::Immediate(raw.bits(IMM5_FIELD)?),
            _ => unreachable!(),
        };

        Ok(Self { dr, sr1, mode })
    }
}

impl Execute for AddOp {
    fn execute(&mut self, vm: &mut crate::vm::VirtualMachine) {
        todo!()
    }
}

/// Represents ADD mode based on instruction bit 11.
///
/// - 0 = [`AddMode::Register`]
/// - 1 = [`AddMode::Immediate`]
#[derive(Debug)]
enum AddMode {
    Register(usize),
    Immediate(u16),
}

fn sign_extend(value: u16) -> u16 {
    todo!()
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
