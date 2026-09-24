use std::ops::RangeInclusive;

use crate::error::Lc3Error;
use crate::operation::Lc3Op;
use crate::{
    instruction::DecodedInstruction,
    memory::Address,
    operation::{sign_extend, Offset11},
    register::Register,
    vm::VirtualMachine,
};

const BASE_REGISTER_FIELD: RangeInclusive<u8> = 8..=10;
const OFFSET_11_BIT_COUNT: u8 = 11;
const PC_OFFSET_11_FIELD: RangeInclusive<u8> = 6..=16;
const MODE_BIT_FIELD: RangeInclusive<u8> = 5..=5;

/// Represents an LC-3 Jump to Subroutine (JSR) operation.
///
/// The operation stores the incremented [`VirtualMachine`] program counter
/// in `R7`, then sets the program counter using either a sign-extended
/// 11-bit PC-relative offset or a base [`Register`].
pub struct JsrOp {
    mode: JsrMode,
}

/// Specifies the addressing mode used by a [`JsrOp`].
///
/// A [`JsrOp`] can either use an 11-bit PC-relative offset or a base
/// [`Register`] as its jump target.
pub enum JsrMode {
    Offset(Offset11),
    Register(Register),
}

impl Lc3Op for JsrOp {
    #[allow(clippy::unreachable)]
    fn decode(instruction: DecodedInstruction) -> Result<Self, Lc3Error> {
        let raw = instruction.raw();
        match raw.bits(MODE_BIT_FIELD)? {
            0 => Ok(Self {
                mode: JsrMode::Register(raw.decode_register(BASE_REGISTER_FIELD)?),
            }),
            1 => Ok(Self {
                mode: JsrMode::Offset(Offset11(raw.bits(PC_OFFSET_11_FIELD)?)),
            }),
            _ => unreachable!(),
        }
    }

    fn execute(self, vm: &mut VirtualMachine) -> Result<(), Lc3Error> {
        vm.write_register(Register::R7, vm.read_pc().into());
        match self.mode {
            JsrMode::Offset(offset) => {
                let sext_offset = sign_extend(offset.value(), OFFSET_11_BIT_COUNT);
                vm.set_pc(vm.read_pc().wrapping_add(sext_offset));
            }
            JsrMode::Register(register) => {
                vm.set_pc(Address::from(vm.read_register(register)));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::RawInstruction;

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn set_pc_with_offset() {
        let mut vm = VirtualMachine::default();
        let decoded = DecodedInstruction::from(RawInstruction::from(0x48FF));
        let op = JsrOp::decode(decoded).unwrap();

        op.execute(&mut vm).unwrap();

        assert_eq!(vm.read_register(Register::R7), 0x3000);

        assert_eq!(vm.read_pc(), Address::from(0x30FF));
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn set_pc_with_base_register() {
        let mut vm = VirtualMachine::default();
        let decoded = DecodedInstruction::from(RawInstruction::from(0x40C0));
        let op = JsrOp::decode(decoded).unwrap();

        vm.write_register(Register::R3, 0xF1F1);
        op.execute(&mut vm).unwrap();

        assert_eq!(vm.read_register(Register::R7), 0x3000);

        assert_eq!(vm.read_pc(), Address::from(0xF1F1));
    }
}
