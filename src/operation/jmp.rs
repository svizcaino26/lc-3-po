use std::ops::RangeInclusive;

use crate::error::Lc3Error;
use crate::{
    instruction::DecodedInstruction, memory::Address, operation::ControlFlowOp, register::Register,
    vm::VirtualMachine,
};

const BASE_REGISTER_FIELD: RangeInclusive<u8> = 8..=10;

/// Represents an LC-3 Jump (JMP) operation.
///
/// The operation unconditionally sets the [`VirtualMachine`] program counter
/// to the address stored in the base [`Register`].
pub struct JmpOp(Register);

impl ControlFlowOp for JmpOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, Lc3Error> {
        Ok(Self(
            instruction.raw().decode_register(BASE_REGISTER_FIELD)?,
        ))
    }

    /// Sets the `PC` to the address stored in the base register.
    fn execute(self, vm: &mut VirtualMachine) {
        vm.set_pc(Address::from(vm.read_register(self.0)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_is_set_from_base_register_address() {
        let mut vm = VirtualMachine::default();
        vm.write_register(Register::R4, 0x45FF);

        let op = JmpOp(Register::R4);

        op.execute(&mut vm);

        assert_eq!(vm.read_pc(), Address::from(0x45FF));
    }
}
