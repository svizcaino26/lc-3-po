use std::ops::{Index, IndexMut};

use crate::memory::Address;

/// Starting memory address for user space.
pub(crate) const PC_START: u16 = 0x3000;
pub(crate) const GENERAL_REGISTERS_SIZE: usize = 8;
/// Represents the LC-3 architecture registers.
///
/// The spec defines 10 registers total:
/// - 8 general purpose registers R0 - R7 Represented as an array of 8 `u16` values.
/// - 1 PC (Program Counter) register.
/// - 1 COND (Condition Flags) register.
#[derive(Debug)]
pub struct Registers {
    general: [u16; GENERAL_REGISTERS_SIZE],
    pc: Address,
    cond: ConditionFlag,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            general: [0; GENERAL_REGISTERS_SIZE],
            pc: Address::from(PC_START),
            cond: ConditionFlag::default(),
        }
    }
}

impl Registers {
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.general.len()
    }

    #[must_use]
    pub const fn pc(&self) -> Address {
        self.pc
    }

    #[must_use]
    pub const fn cond(&self) -> ConditionFlag {
        self.cond
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ConditionFlag {
    Pos,
    #[default]
    Zro,
    Neg,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
}

impl From<Register> for usize {
    #[allow(clippy::as_conversions)]
    fn from(register: Register) -> Self {
        register as Self
    }
}

impl Index<Register> for Registers {
    type Output = u16;

    #[allow(clippy::as_conversions, clippy::indexing_slicing)]
    fn index(&self, register: Register) -> &Self::Output {
        &self.general[usize::from(register)]
    }
}

impl IndexMut<Register> for Registers {
    #[allow(clippy::as_conversions, clippy::indexing_slicing)]
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        &mut self.general[usize::from(register)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_indexing() {
        let mut registers = Registers::default();
        let index = Register::R1;
        registers[index] = 0x00FF;

        assert_eq!(registers[index], 0x00FF);
    }
}
