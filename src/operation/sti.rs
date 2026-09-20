use crate::error::Lc3Error;
use crate::{
    instruction::DecodedInstruction,
    memory::Address,
    operation::{
        sign_extend, MemoryOp, MemoryOperands, MemoryStoreOp, Offset9, OFFSET_9_BIT_COUNT,
        PC_OFFSET_9_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

/// Stores the value of a register at an address obtained indirectly through
/// a PC-relative address.
///
/// The address of the indirect pointer is calculated by sign-extending the
/// encoded 9-bit offset and adding it to the current program counter. The
/// value read from that address is then used as the target memory address.
pub struct StiOp {
    sr: Register,
    offset: Offset9,
}

impl MemoryOp for StiOp {
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

    fn execute(self, vm: &mut VirtualMachine) {
        self.execute_store(vm);
    }
}

impl MemoryStoreOp for StiOp {
    fn operands(self) -> MemoryOperands<Self::Offset> {
        MemoryOperands {
            register: self.sr,
            offset: self.offset,
        }
    }

    fn compute_address(offset: Self::Offset, vm: &VirtualMachine) -> crate::memory::Address {
        let sext_offset = sign_extend(offset.value(), OFFSET_9_BIT_COUNT);
        let address_indirect = vm.read_pc().wrapping_add(sext_offset);
        Address::from(vm.read_memory(address_indirect))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_stores_register_value_at_indirect_address() {
        let mut vm = VirtualMachine::default();
        vm.write_register(Register::R4, 0x4321);

        let offset = Offset9(0b0_0000_0010);
        let sext_offset = sign_extend(offset.value(), OFFSET_9_BIT_COUNT);
        let pointer_address = vm.read_pc().wrapping_add(sext_offset);
        let target_address = Address::from(0x4000);

        vm.write_memory(pointer_address, target_address.into());

        let op = StiOp {
            sr: Register::R4,
            offset,
        };

        op.execute(&mut vm);

        assert_eq!(vm.read_memory(target_address), 0x4321);
    }
}
