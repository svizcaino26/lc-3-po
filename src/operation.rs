use crate::vm::VirtualMachine;

mod add;

pub trait Execute {
    fn execute(&mut self, vm: &mut VirtualMachine);
}
