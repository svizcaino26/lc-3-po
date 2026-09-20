use std::ops::RangeInclusive;

use crate::error::{InstructionError, Lc3Error};
use crate::{
    instruction::DecodedInstruction, memory::Address, register::Register, vm::VirtualMachine,
};

const TRAP_CODE_FIELD: RangeInclusive<u8> = 9..=16;

pub struct TrapOp(TrapRoutine);

impl TrapOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, Lc3Error> {
        let trap_code = instruction.raw().bits_as::<u8>(TRAP_CODE_FIELD)?;
        match trap_code {
            0x20 => Ok(Self(TrapRoutine::GetC)),
            0x21 => Ok(Self(TrapRoutine::Out)),
            0x22 => Ok(Self(TrapRoutine::PutS)),
            0x23 => Ok(Self(TrapRoutine::In)),
            0x24 => Ok(Self(TrapRoutine::PutSp)),
            0x25 => Ok(Self(TrapRoutine::Halt)),
            _ => Err(InstructionError::InvalidTrapCode(trap_code).into()),
        }
    }

    fn execute(self, vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        match self.0 {
            TrapRoutine::GetC => TrapRoutine::getc(vm),
            TrapRoutine::Out => TrapRoutine::out(vm),
            TrapRoutine::PutS => TrapRoutine::puts(vm),
            TrapRoutine::In => TrapRoutine::_in(vm),
            TrapRoutine::PutSp => TrapRoutine::putsp(vm),
            TrapRoutine::Halt => TrapRoutine::halt(vm),
        }
    }
}

/// Represents the different trap routines defined by the LC-3 architecture.
pub enum TrapRoutine {
    GetC,
    Out,
    PutS,
    In,
    PutSp,
    Halt,
}

impl TrapRoutine {
    /// Reads a single character from the [`VirtualMachine`] stdin stream without
    /// echoing to the console.
    /// The character's ASCII code is stored in [`Register::R0`]
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] if the underlying IO operation fails.
    fn getc(vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        let byte = vm.read_byte()?;
        vm.write_register(Register::R0, byte.into());
        Ok(())
    }

    /// Writes an ascii character to the [`VirtualMachine`] stdout stream.
    /// The character ASCII code is stored in the lower 8 bits of the [`Register::R0`].
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] if the underlying IO operation fails.
    #[allow(clippy::expect_used)]
    fn out(vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        let byte =
            u8::try_from(vm.read_register(Register::R0) & 0x00FF).expect("mask value is 8 bits");
        vm.write_bytes(&[byte])?;
        Ok(())
    }

    /// Writes a string of ascii characters to the [`VirtualMachine`] stdout stream.
    ///
    /// The string characters are read sequentially from memory starting from the address stored in [`Register::R0`].
    /// Each memory location contains one character in its lower 8 bits and the string is terminated
    /// by a `0x0000` value.
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] if the underlying IO operation fails.
    #[allow(clippy::expect_used)]
    fn puts(vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        let mut address = Address::from(vm.read_register(Register::R0));
        let mut buff: Vec<u8> = Vec::new();
        loop {
            let word = vm.read_memory(address);
            if word != 0 {
                let byte = u8::try_from(word & 0x0FF).expect("mask value is 8 bits");
                buff.push(byte);
            } else {
                break;
            }
            address = address.wrapping_add(1);
        }

        vm.write_bytes(&buff)?;

        Ok(())
    }

    /// Reads a single character from the [`VirtualMachine`] stdin stream and
    /// echoes it to the console. The character's ASCII code is stored in
    /// [`Register::R0`].
    ///
    /// A prompt is displayed before reading the input.
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] if the underlying IO operation fails.
    fn _in(vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        vm.write_bytes(b"Enter a character: ")?;

        let byte = vm.read_byte()?;

        vm.write_bytes(&[byte])?;
        vm.write_register(Register::R0, byte.into());

        Ok(())
    }

    /// Writes a string of ascii characters to the [`VirtualMachine`] stdout stream.
    ///
    /// The string characters are read sequentially from memory starting from the address stored in [`Register::R0`].
    ///
    /// Each memory location contains two characters, each encoded in 8 bits.
    /// The character encoded in the lower 8 bits is displayed first, followed
    /// by the character encoded in the upper 8 bits when it is non-zero.
    /// The string is terminated when the value `0x0000` is found.
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] if the underlying IO operation fails.
    #[allow(clippy::expect_used)]
    fn putsp(vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        let mut buff: Vec<u8> = Vec::new();
        let mut address = Address::from(vm.read_register(Register::R0));

        loop {
            let word = vm.read_memory(address);

            if word != 0 {
                let first_byte = u8::try_from(word & 0x00FF).expect("mask value is 8 bits");
                let second_byte = u8::try_from((word >> 8) & 0x00FF).expect("mask value is 8 bits");

                buff.push(first_byte);

                if second_byte != 0 {
                    buff.push(second_byte);
                }
            } else {
                break;
            }

            address = address.wrapping_add(1);
        }

        vm.write_bytes(&buff)?;
        Ok(())
    }

    /// Halts execution and displays a message on the console.
    fn halt(vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        vm.write_bytes(b"Stopping execution\n")?;
        vm.halt();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::RawInstruction;

    use super::*;

    #[test]
    fn valid_trap_code_is_decoded() {
        let instructions: [u16; 6] = [0xF020, 0xF021, 0xF022, 0xF023, 0xF024, 0xF025];

        for instr in instructions {
            assert!(TrapOp::decode(DecodedInstruction::from(RawInstruction::from(instr))).is_ok());
        }
    }

    #[test]
    fn invalid_trap_code_returns_error() {
        let instr = DecodedInstruction::from(RawInstruction::from(0xF0FF));
        assert!(TrapOp::decode(instr).is_err());
    }
}
