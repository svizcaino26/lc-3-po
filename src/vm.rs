use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Stdout, Write};

use crate::error::Lc3Error;
use crate::instruction::{DecodedInstruction, Opcode, RawInstruction};
use crate::memory::Address;
use crate::operation::add::AddOp;
use crate::operation::and::AndOp;
use crate::operation::br::BrOp;
use crate::operation::jmp::JmpOp;
use crate::operation::jsr::JsrOp;
use crate::operation::ld::LdOp;
use crate::operation::ldi::LdiOp;
use crate::operation::ldr::LdrOp;
use crate::operation::lea::LeaOp;
use crate::operation::not::NotOp;
use crate::operation::st::StOp;
use crate::operation::sti::StiOp;
use crate::operation::str::StrOp;
use crate::operation::trap::TrapOp;
use crate::operation::Lc3Op;
use crate::register::{ConditionCode, Register};
use crate::{memory::Memory, register::Registers};

/// Represents an LC-3 virtual machine.
///
/// The architecture spec defines 65536 16 bit memory locations
/// for a total of 128 KiB of memory and 10 registers.
/// 8 general-purpose registers, the program counter, and the
/// condition code register.
#[derive(Debug)]
pub struct VirtualMachine<R = BufReader<Stdin>, W = BufWriter<Stdout>> {
    memory: Memory,
    registers: Registers,
    state: VmState,
    stdin: R,
    stdout: W,
}

#[derive(Default, Debug)]
enum VmState {
    #[default]
    Running,
    Stopped,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            stdin: BufReader::new(stdin()),
            stdout: BufWriter::new(stdout()),
            memory: Memory::default(),
            registers: Registers::default(),
            state: VmState::default(),
        }
    }
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

    /// Returns the current [`Address`] store in the pogram counter.
    #[must_use]
    pub const fn read_pc(&self) -> Address {
        self.registers.pc()
    }

    pub const fn set_pc(&mut self, address: Address) {
        self.registers.set_pc(address);
    }

    /// Returns the underlying `u16` at the specified [`Address`]
    #[must_use]
    pub fn read_memory(&self, address: Address) -> u16 {
        self.memory.read(address)
    }

    /// Writes a `u16` value at the specified [`Address`]
    pub fn write_memory(&mut self, address: Address, value: u16) {
        self.memory.write(address, value);
    }

    /// Runs the virtual machine, fetching and executing instructions until
    /// execution is terminated.
    ///
    /// # Errors
    ///
    /// Returns [`Lc3Error::Instruction`] if instruction decoding fails.
    /// Returns [`Lc3Error::Io`] if an I/O operation fails.
    /// Returns [`Lc3Error::UnsupportedInstruction`] if a reserved or unsupported
    /// operation is encountered.
    #[allow(clippy::todo)]
    pub fn run(&mut self) -> Result<(), Lc3Error> {
        while self.is_running() {
            let raw = self.fetch();
            let instruction = Self::decode(raw);
            match instruction.opcode() {
                Opcode::Br => BrOp::decode(instruction)?.execute(self)?,
                Opcode::Add => AddOp::decode(instruction)?.execute(self)?,
                Opcode::Ld => LdOp::decode(instruction)?.execute(self)?,
                Opcode::St => StOp::decode(instruction)?.execute(self)?,
                Opcode::Jsr => JsrOp::decode(instruction)?.execute(self)?,
                Opcode::And => AndOp::decode(instruction)?.execute(self)?,
                Opcode::Ldr => LdrOp::decode(instruction)?.execute(self)?,
                Opcode::Str => StrOp::decode(instruction)?.execute(self)?,
                Opcode::Rti | Opcode::Res => {
                    return Err(Lc3Error::UnsupportedInstruction(*instruction.opcode()))
                }
                Opcode::Not => NotOp::decode(instruction)?.execute(self)?,
                Opcode::Ldi => LdiOp::decode(instruction)?.execute(self)?,
                Opcode::Sti => StiOp::decode(instruction)?.execute(self)?,
                Opcode::Jmp => JmpOp::decode(instruction)?.execute(self)?,
                Opcode::Lea => LeaOp::decode(instruction)?.execute(self)?,
                Opcode::Trap => TrapOp::decode(instruction)?.execute(self)?,
            }
        }
        Ok(())
    }

    /// Halts execution.
    pub const fn halt(&mut self) {
        self.state = VmState::Stopped;
    }

    /// Checks if the [`VirtualMachine`] is in a running state.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        matches!(self.state, VmState::Running)
    }

    /// Reads a single byte from [`Stdin`]
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] if the underlying IO operation fails.
    pub fn read_byte(&mut self) -> Result<u8, std::io::Error> {
        let mut byte = [0u8];
        self.stdin.read_exact(&mut byte)?;
        Ok(byte[0])
    }

    /// Writes a slice of bytes to [`Stdout`]
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] if the underlying IO operation fails.
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.stdout.write_all(bytes)?;
        self.stdout.flush()?;
        Ok(())
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
