use crate::error::Lc3Error;
use crate::operation::Lc3Op;
use crate::{
    instruction::DecodedInstruction,
    operation::{
        sign_extend, MemoryOp, MemoryOperands, MemoryStoreOp, Offset9, OFFSET_9_BIT_COUNT,
        PC_OFFSET_9_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

/// Stores the value of a register at an address calculated relative to the
/// program counter.
///
/// The address is calculated by sign-extending the encoded 9-bit offset and
/// adding it to the current program counter.
pub struct StOp {
    sr: Register,
    offset: Offset9,
}

impl Lc3Op for StOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, Lc3Error> {
        Self::decode_mem_op(instruction)
    }

    fn execute(self, vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        self.execute_store(vm);
        Ok(())
    }
}

impl MemoryOp for StOp {
    type Offset = Offset9;

    fn from_parts(register: Register, offset: Self::Offset) -> Self {
        Self {
            sr: register,
            offset,
        }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, Lc3Error> {
        let offset = instruction.raw().bits(PC_OFFSET_9_FIELD)?;
        Ok(Offset9(offset))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_stores_register_value_at_pc_relative_address() {
        let mut vm = VirtualMachine::default();
        vm.write_register(Register::R4, 0x4321);

        let sext_offset = sign_extend(0b1_1111_1111, OFFSET_9_BIT_COUNT);
        let address = vm.read_pc().wrapping_add(sext_offset);

        let op = StOp {
            sr: Register::R4,
            offset: Offset9(0b1_1111_1111),
        };

        op.execute(&mut vm).unwrap();

        assert_eq!(vm.read_memory(address), 0x4321);
    }
}
