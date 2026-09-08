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
pub struct Register {
    general: [u16; GENERAL_REGISTERS_SIZE],
    pc: Address,
    cond: ConditionFlag,
}

impl Default for Register {
    fn default() -> Self {
        Self {
            general: [0; GENERAL_REGISTERS_SIZE],
            pc: Address::from(PC_START),
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
