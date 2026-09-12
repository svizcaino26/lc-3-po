use crate::instruction::{DecodedInstruction, Opcode, RawInstruction};
use crate::register::{ConditionCode, Register};
use crate::{memory::Memory, register::Registers};

/// Represents an LC-3 virtual machine.
///
/// The architecture spec defines 65536 16 bit memory locations
/// for a total of 128 KiB of memory and 10 registers.
/// 8 general-purpose registers, the program counter, and the
/// condition code register.
#[derive(Debug, Default)]
pub struct VirtualMachine {
    memory: Memory,
    registers: Registers,
}

impl VirtualMachine {
    /// Fetches the instruction at the address stored in the program counter
    /// and advances the program counter to the next memory location.
    ///
    /// The program counter is advanced before the fetched instruction is
    /// decoded or executed.
    #[must_use]
    pub fn fetch(&mut self) -> RawInstruction {
        let raw = self.memory[self.registers.pc()];
        self.registers.advance_pc();
        RawInstruction::from(raw)
    }

    /// Returns the value stored at the specified genearl purpose register.
    #[must_use]
    pub fn read_register(&self, register: Register) -> u16 {
        self.registers[register]
    }

    /// Writes a value to the specified general purpose register.
    pub fn write_register(&mut self, register: Register, value: u16) {
        self.registers[register] = value;
    }

    /// Sets the `COND` register to a [`ConditionCode`] based on the
    /// last operation result value.
    pub const fn set_cond(&mut self, op_result: u16) {
        self.registers.set_cond(op_result);
    }

    /// Returns the [`ConditionCode`] stored in the `COND` register.
    #[must_use]
    pub const fn read_cond(&self) -> ConditionCode {
        self.registers.cond()
    }

    /// Decodes a raw LC-3 instruction into its corresponding instruction
    /// representation.
    #[must_use]
    pub fn decode(instruction: RawInstruction) -> DecodedInstruction {
        DecodedInstruction::from(instruction)
    }

    /// Runs the virtual machine, fetching and executing instructions until
    /// execution is terminated.
    #[allow(clippy::todo)]
    pub fn run(&mut self) {
        loop {
            let raw = self.fetch();
            let decoded = Self::decode(raw);
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
    fn fetch_and_advance_pc() {
        let mut vm = VirtualMachine::default();
        vm.memory[vm.registers.pc()] = 0x1234;

        let raw = vm.fetch();

        assert_eq!(raw, RawInstruction::from(0x1234));
        assert_eq!(vm.registers.pc(), Address::from(PC_START + 1));
    }

    #[test]
    fn pc_wrap_around() {
        let mut vm = VirtualMachine::default();
        let address = Address::from(0xFFFF);
        vm.registers.set_pc(address);
        vm.memory[address] = 0x1234;
        let raw = vm.fetch();

        assert_eq!(raw, RawInstruction::from(0x1234));
        assert_eq!(vm.registers.pc(), Address::from(0x0000));
    }
}
