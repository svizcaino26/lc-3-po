use crate::{
    instruction::{DecodedInstruction, InstructionError},
    memory::Address,
    operation::{
        sign_extend, MemoryOp, MemoryOperands, MemoryStoreOp, Offset6, OFFSET_6_BIT_COUNT,
        OFFSET_6_FIELD, SR1_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

pub struct StrOp {
    sr: Register,
    offset: Offset6,
}

impl MemoryOp for StrOp {
    type Offset = Offset6;

    fn from_parts(register: Register, offset: Self::Offset) -> Self {
        Self {
            sr: register,
            offset,
        }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, InstructionError> {
        let raw = instruction.raw();
        let offset = raw.bits(OFFSET_6_FIELD)?;
        let register = raw.decode_register(SR1_FIELD)?;
        Ok(Offset6 {
            base_r: register,
            value: offset,
        })
    }

    fn execute(self, vm: &mut VirtualMachine) {
        self.execute_store(vm);
    }
}

impl MemoryStoreOp for StrOp {
    fn operands(self) -> MemoryOperands<Self::Offset> {
        MemoryOperands {
            register: self.sr,
            offset: self.offset,
        }
    }

    fn compute_address(offset: Self::Offset, vm: &VirtualMachine) -> crate::memory::Address {
        let sext_offset = sign_extend(offset.value(), OFFSET_6_BIT_COUNT);
        Address::from(vm.read_register(offset.base_r()).wrapping_add(sext_offset))
    }
}
