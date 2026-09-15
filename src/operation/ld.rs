use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::{
        sign_extend, MemoryLoadOp, MemoryLoadOperands, Offset, OFFSET_9_BIT_COUNT,
        PC_OFFSET_9_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

pub struct LdOp {
    dr: Register,
    offset: Offset,
}

impl MemoryLoadOp for LdOp {
    fn from_parts(dr: Register, offset: Offset) -> Self {
        Self { dr, offset }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Offset, InstructionError> {
        let raw = instruction.raw();
        let offset = raw.bits(PC_OFFSET_9_FIELD)?;
        Ok(Offset::Offset9(offset))
    }

    fn operands(self) -> MemoryLoadOperands {
        MemoryLoadOperands {
            dr: self.dr,
            offset: self.offset,
        }
    }
