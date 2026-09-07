use crate::vm::VirtualMachine;

pub trait Execute {
    fn execute(&mut self, vm: &mut VirtualMachine);
}
