use crate::{
    operation::{UnaryOp, UnaryOperands},
    register::Register,
    vm::VirtualMachine,
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

impl UnaryOp for NotOp {
    fn from_parts(dr: Register, sr: Register) -> Self {
        Self { dr, sr }
    }

    fn operands(self, vm: &VirtualMachine) -> UnaryOperands {
        UnaryOperands {
            dr: self.dr,
            value: vm.read_register(self.sr),
        }
    }

    fn operate(value: u16) -> u16 {
        !value
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::{DecodedInstruction, RawInstruction},
        register::ConditionCode,
        vm::VirtualMachine,
    };

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
