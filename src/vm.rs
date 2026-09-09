use crate::{memory::Memory, register::Registers};

/// Represents the amount of general purpose registers defined by the LC-3 spec.
const OPCODE_SHIFT: u16 = 12;
use crate::instruction::{DecodedInstruction, Opcode, RawInstruction};

/// Represents an LC-3 virtual machine.
///
/// The architecture spec defines 65536 16 bit memory locations
/// for a total of 128 KiB of memory and 10 registers.
#[derive(Debug, Default)]
pub struct VirtualMachine {
    memory: Memory,
    registers: Registers,
}

impl VirtualMachine {
    /// Fetches the instruction at the address stored by `PC`
    /// and advances `PC` to the next memory location.
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn fetch(&mut self) -> RawInstruction {
        let raw = self.memory[self.registers.pc()];
        self.registers.pc = self.registers.pc.wrapping_add(1);
        RawInstruction::from(raw)
    }

    #[must_use]
    pub fn decode(instruction: RawInstruction) -> DecodedInstruction {
        DecodedInstruction::from(instruction)
    }

    #[allow(clippy::todo)]
    pub fn run(&mut self) {
        loop {
            let raw = self.fetch();
            let decoded = DecodedInstruction::from(raw);
            match decoded.opcode() {
                Opcode::Br => todo!(),
                Opcode::Add => todo!(),
                Opcode::Ld => todo!(),
                Opcode::St => todo!(),
                Opcode::Jsr => todo!(),
                Opcode::And => todo!(),
                Opcode::Ldr => todo!(),
                Opcode::Str => todo!(),
                Opcode::Rti => todo!(),
                Opcode::Not => todo!(),
                Opcode::Ldi => todo!(),
                Opcode::Sti => todo!(),
                Opcode::Jmp => todo!(),
                Opcode::Res => todo!(),
                Opcode::Lea => todo!(),
                Opcode::Trap => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::{
        memory::{Address, MEMORY_SIZE},
        register::{ConditionCode, GENERAL_REGISTERS_SIZE, PC_START},
    };

    use super::*;

    #[test]
    fn vm_default_state() {
        let vm = VirtualMachine::default();
        assert_eq!(vm.memory.len(), MEMORY_SIZE);
        assert_eq!(vm.registers.pc(), Address::from(PC_START));
        assert_matches!(vm.registers.cond(), ConditionCode::Zro);
        assert_eq!(vm.registers.len(), GENERAL_REGISTERS_SIZE);
    }

    #[test]
    #[allow(clippy::indexing_slicing)]
    fn fetch_and_advance_pc() {
        let mut vm = VirtualMachine::default();
        vm.memory[usize::from(vm.registers.pc)] = 0x1234;

        let raw = vm.fetch();

        assert_eq!(raw, RawInstruction::from(0x1234));
        assert_eq!(vm.registers.pc, PC_START + 1);
    }

    #[test]
    #[allow(clippy::indexing_slicing)]
    fn pc_wrap_around() {
        let mut vm = VirtualMachine::default();
        vm.registers.pc = 0xFFFF;
        vm.memory[0xFFFF] = 0x1234;
        let raw = vm.fetch();

        assert_eq!(raw, RawInstruction::from(0x1234));
        assert_eq!(vm.registers.pc, 0x0000);
    }
}
