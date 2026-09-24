use crate::error::Lc3Error;
use crate::operation::Lc3Op;
use crate::{
    instruction::DecodedInstruction,
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

impl Lc3Op for LeaOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, Lc3Error> {
        Self::decode_mem_op(instruction)
    }

    /// Computes a `PC` relative address and writes this computed address into
    /// the destination register.
    fn execute(self, vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        let address_value: u16 = vm
            .read_pc()
            .wrapping_add(sign_extend(self.offset.value(), OFFSET_9_BIT_COUNT))
            .into();

        vm.write_register(self.dr, address_value);

        Ok(())
    }
}

impl MemoryOp for LeaOp {
    type Offset = Offset9;

    fn from_parts(register: Register, offset: Self::Offset) -> Self {
        Self {
            dr: register,
            offset,
        }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, Lc3Error> {
        let offset = instruction.raw().bits(PC_OFFSET_9_FIELD)?;
        Ok(Offset9(offset))
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::RawInstruction, register::ConditionCode};

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn load_effectve_address() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0xE801));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;

        let op = LeaOp::decode_mem_op(decoded).unwrap();

        op.execute(&mut vm).unwrap();

        assert_eq!(vm.read_register(dr), 0x3001);
        assert_eq!(vm.read_cond(), ConditionCode::Zro);
    }
}
