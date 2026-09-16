use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::{
        sign_extend, MemoryLoadOp, MemoryLoadOperands, Offset9, OFFSET_9_BIT_COUNT,
        PC_OFFSET_9_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

/// Implements the LC-3 Load (LD) operation.
///
/// Loads a value from memory into the destination register using a
/// PC-relative address calculated from the instruction's 9-bit offset.
pub struct LdOp {
    dr: Register,
    offset: Offset9,
}

impl MemoryLoadOp for LdOp {
    type Offset = Offset9;

    fn from_parts(dr: Register, offset: Self::Offset) -> Self {
        Self { dr, offset }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, InstructionError> {
        let raw = instruction.raw();
        let offset = raw.bits(PC_OFFSET_9_FIELD)?;
        Ok(Offset9(offset))
    }

    fn operands(self) -> MemoryLoadOperands<Self::Offset> {
        MemoryLoadOperands {
            dr: self.dr,
            offset: self.offset,
        }
    }

    /// Reads the value at the address obtained by adding the sign-extended
    /// offset to the incremented program counter.
    fn operate(offset: Self::Offset, vm: &VirtualMachine) -> u16 {
        let sext_offset = sign_extend(offset.value(), OFFSET_9_BIT_COUNT);
        let address = vm.read_pc().wrapping_add(sext_offset);
        vm.read_memory(address)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::{DecodedInstruction, RawInstruction},
        register::ConditionCode,
    };

    use super::*;

    #[test]
    fn create_ld_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2FFF));

        let ld_op = LdOp::decode(decoded);

        assert!(ld_op.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_ld_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2801));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;

        vm.write_memory(vm.read_pc().wrapping_add(0x0001), 0x0001);

        let ld_op = LdOp::decode(decoded).unwrap();

        ld_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0001);
        assert_eq!(vm.read_cond(), ConditionCode::Pos);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_ld_op_zero() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2801));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;

        vm.write_memory(vm.read_pc().wrapping_add(0x0001), 0x0000);

        let ld_op = LdOp::decode(decoded).unwrap();

        ld_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0000);
        assert_eq!(vm.read_cond(), ConditionCode::Zro);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_ld_op_negative() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2801));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;

        vm.write_memory(vm.read_pc().wrapping_add(0x0001), 0xFFFF);

        let ld_op = LdOp::decode(decoded).unwrap();

        ld_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0xFFFF);
        assert_eq!(vm.read_cond(), ConditionCode::Neg);
    }
}
