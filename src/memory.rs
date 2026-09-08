/// Represents the amount of memory locations defined by the LC-3 spec, 65536 memory locations.
const MEMORY_SIZE: usize = 1 << 16;

/// Memory is represented as an array 65536 `u16` values.
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
/// Represents a 16-bit address in the LC-3 address space.
///
/// Since the LC-3 uses a 16-bit address bus, all `u16` values represent
/// valid memory addresses.
#[derive(Debug, PartialEq, Eq)]
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
