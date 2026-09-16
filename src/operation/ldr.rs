use crate::{
    instruction::{DecodedInstruction, InstructionError},
    memory::Address,
    operation::{
        sign_extend, MemoryLoadOp, MemoryLoadOperands, Offset6, OFFSET_6_BIT_COUNT, OFFSET_6_FIELD,
        SR1_FIELD,
    },
    register::Register,
};

/// Implements the LC-3 Load Base + offset (LDR) operation.
///
/// Loads a value from memory into the destination register using an
/// address calculated from the instruction's base register + 6-bit offset.
pub struct LdrOp {
    dr: Register,
    offset: Offset6,
}

impl MemoryLoadOp for LdrOp {
    type Offset = Offset6;

    fn from_parts(dr: Register, offset: Self::Offset) -> Self {
        Self { dr, offset }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, InstructionError> {
        let offset = instruction.raw().bits(OFFSET_6_FIELD)?;
        let base_r = instruction.raw().decode_register(SR1_FIELD)?;

        Ok(Offset6 {
            base_r,
            value: offset,
        })
    }

    fn operands(self) -> MemoryLoadOperands<Self::Offset> {
        MemoryLoadOperands {
            dr: self.dr,
            offset: self.offset,
        }
    }

    /// Reads the value at the address obtained by adding the sign-extended
    /// offset to the value stored at the base register.
    fn operate(offset: Self::Offset, vm: &crate::vm::VirtualMachine) -> u16 {
        let sext_offset = sign_extend(offset.value(), OFFSET_6_BIT_COUNT);

        let address = Address::from(vm.read_register(offset.base_r()).wrapping_add(sext_offset));

        vm.read_memory(address)
    }
}
