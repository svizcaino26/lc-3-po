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
