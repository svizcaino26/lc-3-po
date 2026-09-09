//! Memory and address types for the LC-3 virtual machine.
//!
//! The LC-3 has a 16-bit address space containing 65,536 addressable
//! memory locations. Each location stores a 16-bit word, for a total
//! memory capacity of 128 KiB.
//!
//! [`Memory`] provides access to LC-3 memory using [`Address`], which
//! represents a valid 16-bit memory address.

use std::ops::{Index, IndexMut};

/// Number of addressable memory locations defined by the LC-3 architecture.
pub(crate) const MEMORY_SIZE: usize = 1 << 16;

/// LC-3 memory containing 65,536 16-bit words.
///
/// Memory is indexed using [`Address`] values. Since [`Address`] is
/// represented by a `u16`, every possible address is guaranteed to fall
/// within the LC-3 address space.
#[derive(Debug)]
pub struct Memory {
    words: Box<[u16; MEMORY_SIZE]>,
}

impl Default for Memory {
    #[allow(clippy::large_stack_arrays)]
    fn default() -> Self {
        Self {
            words: Box::new([0; MEMORY_SIZE]),
        }
    }
}

impl Index<Address> for Memory {
    type Output = u16;

    #[allow(clippy::indexing_slicing)]
    fn index(&self, address: Address) -> &Self::Output {
        &self.words[usize::from(address.as_u16())]
    }
}

impl IndexMut<Address> for Memory {
    #[allow(clippy::indexing_slicing)]
    fn index_mut(&mut self, address: Address) -> &mut Self::Output {
        &mut self.words[usize::from(address.as_u16())]
    }
}

impl Memory {
    /// Reads the 16-bit word stored at `address`.
    ///
    /// Every [`Address`] represents a valid location in the LC-3 address space,
    /// so this operation cannot address memory outside the allocated buffer.
    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub fn read(&self, address: Address) -> u16 {
        self.words[usize::from(address.as_u16())]
    }

    /// Returns the number of addressable memory locations.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.words.len()
    }
}

/// Represents a 16-bit address in the LC-3 address space.
///
/// Since the LC-3 uses a 16-bit address bus, all `u16` values represent
/// valid memory addresses.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Address(u16);

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Address {
    /// Returns the address as its underlying 16-bit value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    /// Adds `rhs` to the address using 16-bit wrapping arithmetic.
    ///
    /// This mirrors the behavior of the LC-3's 16-bit address arithmetic:
    /// values that exceed `0xFFFF` wrap around to the beginning of the
    /// address space.
    #[must_use]
    pub const fn wrapping_add(self, rhs: u16) -> Self {
        Self(self.0.wrapping_add(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_wrap_around() {
        let address = Address::from(0xFFFF);
        assert_eq!(address.wrapping_add(1), Address::from(0x0000));
    }
}
