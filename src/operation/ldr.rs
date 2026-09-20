use crate::error::Lc3Error;
use crate::{
    instruction::DecodedInstruction,
    memory::Address,
    operation::{
        sign_extend, MemoryLoadOp, MemoryOp, MemoryOperands, Offset6, OFFSET_6_BIT_COUNT,
        OFFSET_6_FIELD, SR1_FIELD,
    },
    register::Register,
    vm::VirtualMachine,
};

/// Implements the LC-3 Load Base + offset (LDR) operation.
///
/// Loads a value from memory into the destination register using an
/// address calculated from the instruction's base register + 6-bit offset.
pub struct LdrOp {
    dr: Register,
    offset: Offset6,
}

impl MemoryOp for LdrOp {
    type Offset = Offset6;

    fn from_parts(register: Register, offset: Self::Offset) -> Self {
        Self {
            dr: register,
            offset,
        }
    }

    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, Lc3Error> {
        let offset = instruction.raw().bits(OFFSET_6_FIELD)?;
        let base_r = instruction.raw().decode_register(SR1_FIELD)?;

        Ok(Offset6 {
            base_r,
            value: offset,
        })
    }

    fn execute(self, vm: &mut VirtualMachine) {
        self.execute_load(vm);
    }
}

impl MemoryLoadOp for LdrOp {
    fn operands(self) -> MemoryOperands<Self::Offset> {
        MemoryOperands {
            register: self.dr,
            offset: self.offset,
        }
    }

    /// Reads the value at the address obtained by adding the sign-extended
    /// offset to the value stored at the base register.
    fn operate(offset: Self::Offset, vm: &VirtualMachine) -> u16 {
        let sext_offset = sign_extend(offset.value(), OFFSET_6_BIT_COUNT);

        let address = Address::from(vm.read_register(offset.base_r()).wrapping_add(sext_offset));

        vm.read_memory(address)
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::VirtualMachine;

    use super::*;

    #[test]
    fn ldr_reads_from_base_register_plus_positive_offset() {
        let mut vm = VirtualMachine::default();

        vm.write_register(Register::R1, 0x3000);
        vm.write_memory(Address::from(0x3002), 0x1234);

        let op = LdrOp {
            dr: Register::R0,
            offset: Offset6 {
                base_r: Register::R1,
                value: 0b00_0010,
            },
        };

        op.execute(&mut vm);

        assert_eq!(vm.read_register(Register::R0), 0x1234);
    }

    #[test]
    fn ldr_reads_from_base_register_plus_negative_offset() {
        let mut vm = VirtualMachine::default();

        vm.write_register(Register::R1, 0x3002);
        vm.write_memory(Address::from(0x3000), 0x5678);

        let op = LdrOp {
            dr: Register::R0,
            offset: Offset6 {
                base_r: Register::R1,
                value: 0b11_1110, // -2
            },
        };

        op.execute(&mut vm);

        assert_eq!(vm.read_register(Register::R0), 0x5678);
    }
}
