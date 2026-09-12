use super::{DR_FIELD, SR1_FIELD};
use crate::{
    instruction::{DecodedInstruction, InstructionError},
    operation::Execute,
    register::Register,
};

/// Represents an LC-3 NOT operation.
///
/// - `dr`: Destination register.
/// - `sr`: Source register.
///
/// The operation applies a bitwise complement on the value in `SR`.
/// The result is stored in `DR` and the condition code is updated based on the result.
#[derive(Debug)]
pub struct NotOp {
    dr: Register,
    sr: Register,
}

impl TryFrom<DecodedInstruction> for NotOp {
    type Error = InstructionError;

    fn try_from(instruction: DecodedInstruction) -> Result<Self, Self::Error> {
        let raw = instruction.raw();
        let dr = Register::from(raw.bits_as::<u8>(DR_FIELD)?);
        let sr = Register::from(raw.bits_as::<u8>(SR1_FIELD)?);

        Ok(Self { dr, sr })
    }
}

impl NotOp {
    /// Decode a `NOT` operation from an LC-3 instruction.
    ///
    /// # Errors
    /// - If there's an invalid bit range in the underlying instruction bits.
    pub fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError> {
        Self::try_from(instruction)
    }
}

impl Execute for NotOp {
    fn execute(self, vm: &mut crate::vm::VirtualMachine) {
        let result = !vm.read_register(self.sr);

        vm.write_register(self.dr, result);

        vm.set_cond(result);
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::RawInstruction, register::ConditionCode, vm::VirtualMachine};

    use super::*;

    #[test]
    fn create_not_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x9FFF));

        let not_op = NotOp::decode(decoded);

        assert!(not_op.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn execute_not_op() {
        let decoded = DecodedInstruction::from(RawInstruction::from(0x98BF));
        let mut vm = VirtualMachine::default();
        let dr = Register::R4;
        let sr = Register::R2;

        vm.write_register(sr, 0x0001);

        let not_op = NotOp::decode(decoded).unwrap();

        not_op.execute(&mut vm);

        assert_eq!(vm.read_register(dr), !vm.read_register(sr));
        assert_eq!(vm.read_cond(), ConditionCode::Neg);
    }
}
