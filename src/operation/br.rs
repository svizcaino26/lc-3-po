const CONDITION_FLAGS_FIELD: RangeInclusive<u8> = 5..=7;
const N_FLAG: u8 = 4;
const Z_FLAG: u8 = 2;
const P_FLAG: u8 = 1;

use std::ops::RangeInclusive;

use crate::error::Lc3Error;
use crate::{
    instruction::DecodedInstruction,
    operation::{sign_extend, ControlFlowOp, Offset9, OFFSET_9_BIT_COUNT, PC_OFFSET_9_FIELD},
    register::ConditionCode,
    vm::VirtualMachine,
};

/// Represents an LC-3 conditional branch (`BR`) operation.
///
/// If any of the condition flags encoded in the instruction match the
/// [`VirtualMachine`] condition code, sets the program counter to an address
/// computed by adding the sign-extended 9-bit offset to the incremented
/// program counter.
pub struct BrOp {
    condition_flags: u8,
    offset: Offset9,
}

impl ControlFlowOp for BrOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, Lc3Error> {
        let raw = instruction.raw();
        let condition_flags = raw.bits_as::<u8>(CONDITION_FLAGS_FIELD)?;
        let offset = raw.bits(PC_OFFSET_9_FIELD)?;
        Ok(Self {
            condition_flags,
            offset: Offset9(offset),
        })
    }

    /// Checks whether any condition flag encoded in the instruction matches the
    /// [`VirtualMachine`] condition code. If a match is found, updates the
    /// program counter using the encoded offset.
    fn execute(self, vm: &mut VirtualMachine) {
        let cond = vm.read_cond();
        let should_branch = (self.condition_flags & N_FLAG != 0 && cond == ConditionCode::Neg)
            || (self.condition_flags & Z_FLAG != 0 && cond == ConditionCode::Zro)
            || (self.condition_flags & P_FLAG != 0 && cond == ConditionCode::Pos);

        if should_branch {
            let sext_offset = sign_extend(self.offset.value(), OFFSET_9_BIT_COUNT);
            let address = vm.read_pc().wrapping_add(sext_offset);
            vm.set_pc(address);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::RawInstruction, memory::Address};

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn branch_with_no_condition_flags() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x001F));
        let mut vm = VirtualMachine::default();

        let pc_before = vm.read_pc();

        let op = BrOp::decode(decoded).unwrap();

        op.execute(&mut vm);

        assert_eq!(pc_before, vm.read_pc());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn branch_on_zero_condition() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x07FF));
        let mut vm = VirtualMachine::default();

        let op = BrOp::decode(decoded).unwrap();

        op.execute(&mut vm);

        assert_eq!(vm.read_pc(), Address::from(0x2FFF));
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn branch_on_no_condition_match() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x03FF));
        let mut vm = VirtualMachine::default();

        let op = BrOp::decode(decoded).unwrap();

        op.execute(&mut vm);

        assert_eq!(vm.read_pc(), Address::from(0x3000));
    }
}
