use crate::error::Lc3Error;
use crate::operation::Lc3Op;
use crate::{
    instruction::DecodedInstruction,
    memory::Address,
    operation::{
        sign_extend, MemoryOp, MemoryOperands, MemoryStoreOp, Offset6, OFFSET_6_BIT_COUNT,
        OFFSET_6_FIELD, SR1_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

/// Stores the value of a register at an address calculated relative to a base
/// register.
///
/// The address is calculated by sign-extending the encoded 6-bit offset and
/// adding it to the value of the base register.
pub struct StrOp {
    sr: Register,
    offset: Offset6,
}

impl Lc3Op for StrOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, Lc3Error> {
        Self::decode_mem_op(instruction)
    }

    fn execute(self, vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        self.execute_store(vm);
        Ok(())
    }
}

impl MemoryOp for StrOp {
    type Offset = Offset6;

    fn from_parts(register: Register, offset: Self::Offset) -> Self {
        Self {
            sr: register,
            offset,
        }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, Lc3Error> {
        let raw = instruction.raw();
        let offset = raw.bits(OFFSET_6_FIELD)?;
        let register = raw.decode_register(SR1_FIELD)?;
        Ok(Offset6 {
            base_r: register,
            value: offset,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_stores_register_value_at_base_register_relative_address() {
        let mut vm = VirtualMachine::default();
        vm.write_register(Register::R1, 0x4002);
        vm.write_register(Register::R4, 0x4321);

        let offset = Offset6 {
            base_r: Register::R1,
            value: 0b11_1110, // -2
        };

        let op = StrOp {
            sr: Register::R4,
            offset,
        };

        op.execute(&mut vm).unwrap();

        assert_eq!(vm.read_memory(Address::from(0x4000)), 0x4321);
    }
}
