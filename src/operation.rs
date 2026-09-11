use std::ops::RangeInclusive;

use crate::{
    instruction::{DecodedInstruction, InstructionError},
    register::Register,
    vm::VirtualMachine,
};

pub mod add;
pub mod and;
pub mod not;

const DR_FIELD: RangeInclusive<u8> = 5..=7;
const SR1_FIELD: RangeInclusive<u8> = 8..=10;
const MODE_FIELD: RangeInclusive<u8> = 11..=11;
const SR2_FIELD: RangeInclusive<u8> = 14..=16;
const IMM5_FIELD: RangeInclusive<u8> = 12..=16;
const IMM5_BIT_COUNT: u8 = 5;

/// Executes an operation against a virtual machine.
///
/// Implementations mutate the virtual machine according to the semantics
/// of the operation.
pub trait Execute {
    fn execute(self, vm: &mut VirtualMachine);
}

/// Provides shared decoding and execution logic for LC-3 binary operations.
///
/// Binary operations have two source operands and one destination register.
/// The second operand can either be a register or a sign-extended immediate
/// value, depending on the operation's mode.
///
/// Implementors provide the operation-specific construction, operand
/// resolution, and computation while the common decoding and execution
/// logic is provided by this trait.
trait BinaryOp: Sized {
    /// Constructs the operation from its decoded operands.
    fn from_parts(dr: Register, sr1: Register, mode: BinaryOpMode) -> Self;

    /// Decodes the operands of a binary operation from a decoded instruction.
    ///
    /// The destination and first source registers are extracted from their
    /// respective instruction fields. The second operand is decoded as either
    /// a register or a 5-bit immediate value according to the mode bit.
    #[allow(clippy::unreachable)]
    fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError> {
        let raw = instruction.raw();
        let dr = raw.decode_register(DR_FIELD)?;
        let sr1 = raw.decode_register(SR1_FIELD)?;
        let mode = match raw.bits(MODE_FIELD)? {
            0 => BinaryOpMode::Register(raw.decode_register(SR2_FIELD)?),
            1 => BinaryOpMode::Immediate(raw.bits(IMM5_FIELD)?),
            _ => unreachable!(),
        };

        Ok(Self::from_parts(dr, sr1, mode))
    }

    /// Performs the operation-specific computation on two operands.
    fn operate(lhs: u16, rhs: u16) -> u16;

    /// Resolves the operation's operands from the virtual machine state.
    fn operands(self, vm: &VirtualMachine) -> BinaryOperands;

    /// Executes the binary operation and updates the condition code.
    fn execute(self, vm: &mut VirtualMachine) {
        let operands = Self::operands(self, vm);
        let result = Self::operate(operands.lhs, operands.rhs);

        vm.write_register(operands.dr, result);

        vm.set_cond(result);
    }
}

/// Contains the resolved operands required to execute a binary operation.
pub struct BinaryOperands {
    dr: Register,
    lhs: u16,
    rhs: u16,
}

/// Represents the operation mode based on instruction bit 11.
///
/// - 0 = [`BinaryOpMod::Register`]
/// - 1 = [`BinaryOpMode::Immediate`]
#[derive(Debug)]
enum BinaryOpMode {
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
    use super::*;

    #[test]
    fn sign_extension() {
        assert_eq!(sign_extend(0b00000, 5), 0x0000);
        assert_eq!(sign_extend(0b00001, 5), 0x0001);
        assert_eq!(sign_extend(0b01111, 5), 0x000F);
        assert_eq!(sign_extend(0b10000, 5), 0xFFF0);
        assert_eq!(sign_extend(0b10001, 5), 0xFFF1);
        assert_eq!(sign_extend(0b11111, 5), 0xFFFF);
    }
}
