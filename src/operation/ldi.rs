use crate::{
    instruction::{DecodedInstruction, InstructionError},
    memory::Address,
    operation::{
        sign_extend, MemoryLoadOp, MemoryLoadOperands, Offset, OFFSET_9_BIT_COUNT,
        PC_OFFSET_9_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

/// Implements the LC-3 Load Indirect (LDI) operation.
///
/// Loads a value from memory into the destination register using a
/// PC-relative address. The value at the computed address is interpreted
/// as a second memory address, from which the value loaded into the
/// destination register is obtained.
pub struct LdiOp {
    dr: Register,
    offset: Offset,
}

impl MemoryLoadOp for LdiOp {
    fn from_parts(dr: Register, offset: Offset) -> Self {
        Self { dr, offset }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Offset, InstructionError> {
        let offset = instruction.raw().bits(PC_OFFSET_9_FIELD)?;
        Ok(Offset::Offset9(offset))
    }

    fn operands(self) -> MemoryLoadOperands {
        MemoryLoadOperands {
            dr: self.dr,
            offset: self.offset,
        }
    }

    /// Computes a PC-relative address and performs two memory reads:
    /// the first obtains the address of the value to load, and the second
    /// obtains the value itself.
    fn operate(offset: Offset, vm: &VirtualMachine) -> u16 {
        let sext_offset = sign_extend(offset.value(), OFFSET_9_BIT_COUNT);
        let address = vm.read_pc().wrapping_add(sext_offset);
        vm.read_memory(Address::from(vm.read_memory(address)))
    }
}
