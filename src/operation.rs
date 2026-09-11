use std::ops::RangeInclusive;

use crate::{register::Register, vm::VirtualMachine};

pub mod add;

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
