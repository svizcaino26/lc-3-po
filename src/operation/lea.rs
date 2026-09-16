use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::{sign_extend, MemoryOp, Offset9, OFFSET_9_BIT_COUNT, PC_OFFSET_9_FIELD},
    register::Register,
    vm::VirtualMachine,
};

/// Loads an address calculated relative to the program counter into a register.
///
/// The address is calculated by sign-extending the encoded 9-bit offset and
/// adding it to the current program counter. The resulting address is written
/// to the destination register.
pub struct LeaOp {
    dr: Register,
    offset: Offset9,
}

impl MemoryOp for LeaOp {
    type Offset = Offset9;

    fn from_parts(register: Register, offset: Self::Offset) -> Self {
        Self {
            dr: register,
            offset,
        }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, InstructionError> {
        let offset = instruction.raw().bits(PC_OFFSET_9_FIELD)?;
        Ok(Offset9(offset))
    }

    fn execute(self, vm: &mut VirtualMachine) {
        let address_value: u16 = vm
            .read_pc()
            .wrapping_add(sign_extend(self.offset.value(), OFFSET_9_BIT_COUNT))
            .into();

        vm.write_register(self.dr, address_value);
    }
}
