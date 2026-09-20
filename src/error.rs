use std::ops::RangeInclusive;

use thiserror::Error;

use crate::instruction::Opcode;

#[derive(Debug, Error)]
pub enum Lc3Error {
    #[error("IO operation error.")]
    Io(#[from] std::io::Error),

    #[error("Failed decoding instruction")]
    Instruction(#[from] InstructionError),

    #[error("Unsupported instructioin : {0:?}")]
    UnsupportedInstruction(Opcode),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum InstructionError {
    #[error("Invalid bit range: {0:?}")]
    InvalidBitRange(RangeInclusive<u8>),

    #[error("Failed to convert extracted bits to target type")]
    InvalidBitConversion,

    #[error("Invalid trap code: {0}")]
    InvalidTrapCode(u8),
}
