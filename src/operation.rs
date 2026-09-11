use crate::vm::VirtualMachine;

pub mod add;

/// Executes an operation against a virtual machine.
///
/// Implementations mutate the virtual machine according to the semantics
/// of the operation.
pub trait Execute {
    fn execute(self, vm: &mut VirtualMachine);
}
