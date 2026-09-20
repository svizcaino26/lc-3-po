use crate::error::Lc3Error;
use crate::{
    instruction::DecodedInstruction,
    memory::Address,
    operation::{
        sign_extend, MemoryLoadOp, MemoryOp, MemoryOperands, Offset9, OFFSET_9_BIT_COUNT,
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
    offset: Offset9,
}

impl MemoryOp for LdiOp {
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

    fn execute(self, vm: &mut VirtualMachine) {
        self.execute_load(vm);
    }
}

impl MemoryLoadOp for LdiOp {
    fn operands(self) -> MemoryOperands<Self::Offset> {
        MemoryOperands {
            register: self.dr,
            offset: self.offset,
        }
    }

    /// Computes a PC-relative address and performs two memory reads:
    /// the first obtains the address of the value to load, and the second
    /// obtains the value itself.
    fn operate(offset: Self::Offset, vm: &VirtualMachine) -> u16 {
        let sext_offset = sign_extend(offset.value(), OFFSET_9_BIT_COUNT);
        let address = vm.read_pc().wrapping_add(sext_offset);
        vm.read_memory(Address::from(vm.read_memory(address)))
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
    fn create_ldi_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2801));

        let ldi_op = LdiOp::decode(decoded);

        assert!(ldi_op.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_ldi_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2801));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;
        let indirect_address = 0x4000;

        vm.write_memory(vm.read_pc().wrapping_add(0x0001), indirect_address);
        vm.write_memory(Address::from(indirect_address), 0x0001);

        let ldi_op = LdiOp::decode(decoded).unwrap();

        ldi_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0001);
        assert_eq!(vm.read_cond(), ConditionCode::Pos);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_ldi_op_zero() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2801));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;
        let indirect_address = 0x4000;

        vm.write_memory(vm.read_pc().wrapping_add(0x0001), indirect_address);
        vm.write_memory(Address::from(indirect_address), 0x0000);

        let ldi_op = LdiOp::decode(decoded).unwrap();

        ldi_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0x0000);
        assert_eq!(vm.read_cond(), ConditionCode::Zro);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_ldi_op_negative() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x2801));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;
        let indirect_address = 0x4000;

        vm.write_memory(vm.read_pc().wrapping_add(0x0001), indirect_address);
        vm.write_memory(Address::from(indirect_address), 0xFFFF);

        let ldi_op = LdiOp::decode(decoded).unwrap();

        ldi_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), 0xFFFF);
        assert_eq!(vm.read_cond(), ConditionCode::Neg);
    }
}
