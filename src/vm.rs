/// Represents the amount of memory locations defined by the LC-3 spec, 65536 memory locations.
const MEMORY_SIZE: usize = 1 << 16;
/// Represents the amount of general purpose registers defined by the LC-3 spec.
const GENERAL_REGISTERS_SIZE: usize = 8;
const OPCODE_SHIFT: u16 = 12;
/// Starting memory address for user space.
const PC_START: u16 = 0x3000;

/// Represents an LC-3 virtual machine.
///
/// The architecture spec defines 65536 16 bit memory locations
/// for a total of 128 KiB of memory and 10 registers.
///
/// Memory is represented as an array 65536 `u16` values.
#[derive(Debug)]
pub struct VirtualMachine {
    memory: Box<[u16; MEMORY_SIZE]>,
    registers: Registers,
}

impl Default for VirtualMachine {
    #[allow(clippy::large_stack_arrays)]
    fn default() -> Self {
        Self {
            memory: Box::new([0; MEMORY_SIZE]),
            registers: Registers::default(),
        }
    }
}

/// Represents the LC-3 architecture registers.
///
/// The spec defines 10 registers total:
/// - 8 general purpose registers R0 - R7 Represented as an array of 8 `u16` values.
/// - 1 PC (Program Counter) register.
/// - 1 COND (Condition Flags) register.
#[derive(Debug)]
pub struct Registers {
    general: [u16; GENERAL_REGISTERS_SIZE],
    pc: u16,
    cond: ConditionFlag,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            general: [0; GENERAL_REGISTERS_SIZE],
            pc: PC_START,
            cond: ConditionFlag::default(),
        }
    }
}

#[derive(Debug, Default)]
pub enum ConditionFlag {
    Pos,
    #[default]
    Zro,
    Neg,
}


#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn vm_default_state() {
        let vm = VirtualMachine::default();
        assert_eq!(vm.memory.len(), MEMORY_SIZE);
        assert_eq!(vm.registers.pc, PC_START);
        assert_matches!(vm.registers.cond, ConditionFlag::Zro);
        assert_eq!(vm.registers.general.len(), GENERAL_REGISTERS_SIZE);
    }
}
