//! Representation and decoding of LC-3 instructions.
//!
//! LC-3 instructions are encoded as 16-bit values. [`RawInstruction`]
//! represents an instruction in its encoded form, while [`DecodedInstruction`]
//! represents its decoded form.
//!
//! The four most significant bits of an instruction encode its [`Opcode`].

use std::ops::RangeInclusive;

use crate::register::Register;

/// bit shift for extacting opcode field from raw `u16` instruction
const OPCODE_SHIFT: u16 = 12;
const INSTRUCTION_BITS: u8 = 16;

/// Represents a raw `u16` instruction read from an LC-3 program.
#[derive(Debug, PartialEq, Eq)]
pub enum InstructionError {
    InvalidBitRange(RangeInclusive<u8>),
    InvalidBitConversion,
}

/// Represent the raw 16-bit instruction.
#[derive(Debug, PartialEq, Eq)]
pub struct RawInstruction(u16);

impl From<u16> for RawInstruction {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl RawInstruction {
    /// Extracts the field defined by the given inclusive bit range.
    /// Bits are numbered 1 - 16 from the most significant bit.
    ///
    /// # Errors
    /// - If lower bound of `range` is 0.
    /// - If lower bound of `range` > upper bound.
    /// - If upper bound of `range` > `INSTRUCTION_BITS`.
    #[allow(clippy::arithmetic_side_effects)]
    pub const fn bits(&self, range: RangeInclusive<u8>) -> Result<u16, InstructionError> {
        let start = *range.start();
        let end = *range.end();

        if start == 0 || start > end || end > INSTRUCTION_BITS {
            return Err(InstructionError::InvalidBitRange(range));
        }

        let len = end - start + 1;
        let shift = INSTRUCTION_BITS - end;
        let mask = (1u16 << len) - 1;
        Ok((self.0 >> shift) & mask)
    }

    /// This is a typed wrapper around [`RawInstruction::bits`]. The target type must
    /// implement `TryFrom<u16>`.
    ///
    /// # Errors
    ///
    /// Returns [`InstructionError::InvalidBitConversion`] if the extracted value cannot
    /// be represented by the target type.
    pub fn bits_as<T: TryFrom<u16>>(
        &self,
        range: RangeInclusive<u8>,
    ) -> Result<T, InstructionError> {
        self.bits(range)?
            .try_into()
            .map_err(|_| InstructionError::InvalidBitConversion)
    }

    /// Decodes a [`Register`] from an LC-3 16-bit instruction.
    ///
    /// The [`Register`] identifier is a 3-bit ecoded value.
    ///
    /// # Errors
    ///
    /// - If `range.len() != 3`.
    pub fn decode_register(&self, range: RangeInclusive<u8>) -> Result<Register, InstructionError> {
        if range.len() != 3 {
            return Err(InstructionError::InvalidBitRange(range));
        }
        let register_id = self.bits_as::<u8>(range)?;
        Ok(Register::from(register_id))
    }
}

/// Represents an LC-3 instruction after decoding its opcode and fields.
///
/// The original [`RawInstruction`] is retained so that additional instruction
/// fields can be extracted from the original encoding during instruction
/// processing.
pub struct DecodedInstruction {
    opcode: Opcode,
    raw: RawInstruction,
}

impl DecodedInstruction {
    #[must_use]
    pub const fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    #[must_use]
    pub const fn raw(self) -> RawInstruction {
        self.raw
    }
}

/// Represents one of the 16 opcodes defined by the LC-3 instruction set.
///
/// The opcode is encoded in the four most significant bits of every LC-3
/// instruction.
pub enum Opcode {
    Br = 0,    // branch
    Add = 1,   // add
    Ld = 2,    // load
    St = 3,    // store
    Jsr = 4,   // jump to subroutine
    And = 5,   // bitwise and
    Ldr = 6,   // load register
    Str = 7,   // store register
    Rti = 8,   // unused
    Not = 9,   // bitwise not
    Ldi = 10,  // load indirect
    Sti = 11,  // store indirect
    Jmp = 12,  // jump
    Res = 13,  // reserved (unused)
    Lea = 14,  // load effective address
    Trap = 15, // execute trap
}

impl From<RawInstruction> for DecodedInstruction {
    /// Decodes the opcode from a raw LC-3 instruction.
    ///
    /// The opcode is stored in the four most significant bits of the
    /// instruction.
    ///
    /// The bit shift ensures the matched value is a number between 0 and 15.
    #[allow(clippy::unreachable)]
    fn from(value: RawInstruction) -> Self {
        let opcode = match value.0 >> OPCODE_SHIFT {
            0 => Opcode::Br,
            1 => Opcode::Add,
            2 => Opcode::Ld,
            3 => Opcode::St,
            4 => Opcode::Jsr,
            5 => Opcode::And,
            6 => Opcode::Ldr,
            7 => Opcode::Str,
            8 => Opcode::Rti,
            9 => Opcode::Not,
            10 => Opcode::Ldi,
            11 => Opcode::Sti,
            12 => Opcode::Jmp,
            13 => Opcode::Res,
            14 => Opcode::Lea,
            15 => Opcode::Trap,
            _ => unreachable!(),
        };

        Self { opcode, raw: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_conversion() {
        let decoded_1 = DecodedInstruction::from(RawInstruction::from(0x0000));
        let decoded_2 = DecodedInstruction::from(RawInstruction::from(0x1000));
        let decoded_3 = DecodedInstruction::from(RawInstruction::from(0xF000));
        assert!(matches!(decoded_1.opcode(), Opcode::Br));
        assert!(matches!(decoded_2.opcode(), Opcode::Add));
        assert!(matches!(decoded_3.opcode(), Opcode::Trap));
    }

    #[test]
    fn extract_bits() {
        let raw_1 = RawInstruction::from(0xF000);
        let raw_2 = RawInstruction::from(0x10F0);
        let raw_3 = RawInstruction::from(0xF020);

        assert_eq!(raw_1.bits(1..=4), Ok(0x000F));
        assert_eq!(raw_2.bits(1..=4), Ok(0x0001));
        assert_eq!(raw_3.bits(11..=11), Ok(0x0001));
    }

    #[test]
    fn invalid_range() {
        let raw_1 = RawInstruction::from(0xF000);
        let raw_2 = RawInstruction::from(0x10F0);

        assert_eq!(
            raw_1.bits(0..=4),
            Err(InstructionError::InvalidBitRange(0..=4))
        );
        assert_eq!(
            raw_2.bits(4..=17),
            Err(InstructionError::InvalidBitRange(4..=17))
        );
    }
}
