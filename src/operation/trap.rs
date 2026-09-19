use std::{
    io::{Read, Write},
    ops::RangeInclusive,
};

use crate::{
    instruction::{DecodedInstruction, InstructionError},
    memory::Address,
    register::Register,
    vm::VirtualMachine,
};

const TRAP_CODE_FIELD: RangeInclusive<u8> = 9..=16;

pub struct TrapOp(TrapRoutine);

impl TrapOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError> {
        let trap_code = instruction.raw().bits_as::<u8>(TRAP_CODE_FIELD)?;
        match trap_code {
            0x20 => Ok(Self(TrapRoutine::GetC)),
            0x21 => Ok(Self(TrapRoutine::Out)),
            0x22 => Ok(Self(TrapRoutine::PutS)),
            0x23 => Ok(Self(TrapRoutine::In)),
            0x24 => Ok(Self(TrapRoutine::PutSp)),
            0x25 => Ok(Self(TrapRoutine::Halt)),
            _ => Err(InstructionError::InvalidTrapCode(trap_code)),
        }
    }

    #[allow(clippy::todo)]
    fn execute(self, vm: &mut VirtualMachine) -> Result<(), std::io::Error> {
        match self.0 {
            TrapRoutine::GetC => TrapRoutine::getc(vm),
            TrapRoutine::Out => TrapRoutine::out(vm),
            TrapRoutine::PutS => TrapRoutine::puts(vm),
            TrapRoutine::In => TrapRoutine::_in(vm),
            TrapRoutine::PutSp => TrapRoutine::putsp(vm),
            TrapRoutine::Halt => {
                TrapRoutine::halt(vm);
                Ok(())
            }
        }
    }
}

pub enum TrapRoutine {
    GetC,
    Out,
    PutS,
    In,
    PutSp,
    Halt,
}

impl TrapRoutine {
    /// # Errors
    fn getc(vm: &mut VirtualMachine) -> Result<(), std::io::Error> {
        let mut byte = [0u8];
        std::io::stdin().read_exact(&mut byte)?;
        vm.write_register(Register::R0, byte[0].into());
        Ok(())
    }

    #[allow(clippy::expect_used)]
    fn out(vm: &VirtualMachine) -> Result<(), std::io::Error> {
        let byte =
            u8::try_from(vm.read_register(Register::R0) & 0x00FF).expect("mask value is 8 bits");
        let _ = std::io::stdout().write(&[byte])?;
        Ok(())
    }

    #[allow(clippy::expect_used)]
    fn puts(vm: &VirtualMachine) -> Result<(), std::io::Error> {
        let mut address = Address::from(vm.read_register(Register::R0));
        let mut buff: Vec<u8> = Vec::new();
        loop {
            let word = vm.read_memory(address);
            if word != 0 {
                let byte =
                    u8::try_from(vm.read_memory(address) & 0x0FF).expect("mask value is 8 bits");
                buff.push(byte);
            } else {
                break;
            }
            address = address.wrapping_add(1);
        }

        let () = std::io::stdout().write_all(&buff)?;

        Ok(())
    }

    fn _in(vm: &mut VirtualMachine) -> Result<(), std::io::Error> {
        print!("Enter a character: ");
        std::io::stdout().flush()?;

        let mut byte = [0u8];
        std::io::stdin().read_exact(&mut byte)?;

        std::io::stdout().write_all(&byte)?;
        vm.write_register(Register::R0, byte[0].into());

        Ok(())
    }

    #[allow(clippy::expect_used)]
    fn putsp(vm: &VirtualMachine) -> Result<(), std::io::Error> {
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

        std::io::stdout().write_all(&buff)?;
        Ok(())
    }

    fn halt(vm: &mut VirtualMachine) {
        println!("Stopping execution");
        vm.halt();
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
