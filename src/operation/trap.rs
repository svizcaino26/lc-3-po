use crate::{
    instruction::{DecodedInstruction, InstructionError},
    register::Register,
    vm::VirtualMachine,
};

const TRAP_CODE_FIELD: RangeInclusive<u8> = 9..=16;

pub struct TrapOp(TrapRoutine);
impl TrapOp {
    fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError> {
        let trap_code = instruction.raw().bits_as::<u8>(TRAP_CODE_FIELD)?;
        match trap_code {
            0x20 => Ok(Self(TrapRoutine::GetC)),
            0x21 => Ok(Self(TrapRoutine::Out)),
            0x22 => Ok(Self(TrapRoutine::PutS)),
            0x23 => Ok(Self(TrapRoutine::In)),
            0x24 => Ok(Self(TrapRoutine::PutSp)),
            0x25 => Ok(Self(TrapRoutine::Halt)),
            _ => Err(InstructionError::InvalidTrapCode(trap_code)),
        }
    }
pub enum TrapRoutine {
    GetC,
    Out,
    PutS,
    In,
    PutSp,
    Halt,
}

#[cfg(test)]
mod tests {
    use crate::instruction::RawInstruction;

    use super::*;

    #[test]
    fn valid_trap_code_is_decoded() {
        let instructions: [u16; 6] = [0xF020, 0xF021, 0xF022, 0xF023, 0xF024, 0xF025];

        for instr in instructions {
            assert!(TrapOp::decode(DecodedInstruction::from(RawInstruction::from(instr))).is_ok());
        }
    }

    #[test]
    fn invalid_trap_code_returns_error() {
        let instr = DecodedInstruction::from(RawInstruction::from(0xF0FF));
        assert!(TrapOp::decode(instr).is_err());
    }
}
