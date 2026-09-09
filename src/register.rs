//! Register types for the LC-3 virtual machine.
//!
//! The LC-3 defines ten registers:
//! - Eight general-purpose registers (`R0`-`R7`), each holding a 16-bit value.
//! - A 16-bit program counter (`PC`).
//! - A condition code register (`COND`) representing the result of the most
//!   recent operation.

use std::ops::{Index, IndexMut};

use crate::memory::Address;

/// Starting memory address for user space.
pub(crate) const PC_START: u16 = 0x3000;
///Number of general purpose registers defined by the LC-3 architecture.
pub(crate) const GENERAL_REGISTERS_SIZE: usize = 8;

/// Represents the registers defined by the LC-3 architecture.
///
/// The eight general-purpose registers are stored internally as an array of
/// 16-bit values. The program counter and condition code are represented
/// separately because they have different semantics from the general-purpose
/// registers.
#[derive(Debug)]
pub struct Registers {
    general: [u16; GENERAL_REGISTERS_SIZE],
    pc: Address,
    cond: ConditionCode,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            general: [0; GENERAL_REGISTERS_SIZE],
            pc: Address::from(PC_START),
            cond: ConditionCode::default(),
        }
    }
}

impl Registers {
    /// Returns the number of general purpose registers.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.general.len()
    }

    /// Returns the current `PC` [`Address`]
    #[must_use]
    pub const fn pc(&self) -> Address {
        self.pc
    }

    /// Returns the current [`ConditionCode`]
    #[must_use]
    pub const fn cond(&self) -> ConditionCode {
        self.cond
    }

    /// Advance the `PC` to the next memory [`Address`].
    /// If current `PC` points to the last address in the LC-3 space, it wraps
    /// around to the beginning.
    pub const fn advance_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    /// Sets `PC` to a specific [`Address`] ensuring a valid LC-3 memory location.
    pub const fn set_pc(&mut self, address: Address) {
        self.pc = address;
    }
}

/// Represents the LC-3 condition code.
///
/// The condition code records whether the result of the most recent
/// operation was positive, zero, or negative.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ConditionCode {
    Pos, // Positive
    #[default]
    Zro, // Zero
    Neg, // Negative
}

/// Identifies one of the LC-3 general-purpose registers.
///
/// The discriminant values correspond to the three-bit register identifiers
/// encoded in LC-3 instructions.
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
