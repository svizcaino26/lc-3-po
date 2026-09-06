/// bit shift for extacting opcode field from raw `u16` instruction
const OPCODE_SHIFT: u16 = 12;

#[derive(Debug, PartialEq, Eq)]
pub struct RawInstruction(u16);

impl From<u16> for RawInstruction {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

pub struct DecodedInstruction {
    opcode: Opcode,
    raw: RawInstruction,
}

impl DecodedInstruction {
    pub const fn opcode(&self) -> &Opcode {
        &self.opcode
    }
}

/// Represents the LC-3 instruction set.
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
    /// Extracts the opcode field from a 16-bit LC-3 instruction.
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
}
