use crate::{memory::Memory, register::Registers};

/// Represents the amount of general purpose registers defined by the LC-3 spec.
const OPCODE_SHIFT: u16 = 12;

/// Represents an LC-3 virtual machine.
///
/// The architecture spec defines 65536 16 bit memory locations
/// for a total of 128 KiB of memory and 10 registers.
#[derive(Debug, Default)]
pub struct VirtualMachine {
    memory: Memory,
    registers: Registers,
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

impl From<u16> for Opcode {
    /// Extracts the opcode field from a 16-bit LC-3 instruction.
    ///
    /// The bit shift ensures the matched value is a number between 0 and 15.
    #[allow(clippy::unreachable)]
    fn from(value: u16) -> Self {
        match value >> OPCODE_SHIFT {
            0 => Self::Br,
            1 => Self::Add,
            2 => Self::Ld,
            3 => Self::St,
            4 => Self::Jsr,
            5 => Self::And,
            6 => Self::Ldr,
            7 => Self::Str,
            8 => Self::Rti,
            9 => Self::Not,
            10 => Self::Ldi,
            11 => Self::Sti,
            12 => Self::Jmp,
            13 => Self::Res,
            14 => Self::Lea,
            15 => Self::Trap,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::{
        memory::{Address, MEMORY_SIZE},
        register::{ConditionFlag, GENERAL_REGISTERS_SIZE, PC_START},
    };

    use super::*;

    #[test]
    fn vm_default_state() {
        let vm = VirtualMachine::default();
        assert_eq!(vm.memory.len(), MEMORY_SIZE);
        assert_eq!(vm.registers.pc(), Address::from(PC_START));
        assert_matches!(vm.registers.cond(), ConditionFlag::Zro);
        assert_eq!(vm.registers.len(), GENERAL_REGISTERS_SIZE);
    }

    #[test]
    fn opcode_conversion() {
        assert!(matches!(Opcode::from(0x0000), Opcode::Br));
        assert!(matches!(Opcode::from(0x1000), Opcode::Add));
        assert!(matches!(Opcode::from(0xF000), Opcode::Trap));
    }
}
