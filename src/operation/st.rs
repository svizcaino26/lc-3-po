use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::{
        sign_extend, MemoryOp, MemoryOperands, MemoryStoreOp, Offset9, OFFSET_9_BIT_COUNT,
        PC_OFFSET_9_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

pub struct StOp {
    sr: Register,
    offset: Offset9,
}

impl MemoryOp for StOp {
    type Offset = Offset9;

    fn from_parts(register: Register, offset: Self::Offset) -> Self {
        Self {
            sr: register,
            offset,
        }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, InstructionError> {
        let offset = instruction.raw().bits(PC_OFFSET_9_FIELD)?;
        Ok(Offset9(offset))
    }

    fn execute(self, vm: &mut VirtualMachine) {
        self.execute_store(vm);
    }
}

impl MemoryStoreOp for StOp {
    fn operands(self) -> MemoryOperands<Self::Offset> {
        MemoryOperands {
            register: self.sr,
            offset: self.offset,
        }
    }

    fn compute_address(offset: Self::Offset, vm: &VirtualMachine) -> crate::memory::Address {
        let sext_offset = sign_extend(offset.value(), OFFSET_9_BIT_COUNT);
        vm.read_pc().wrapping_add(sext_offset)
    }
}
