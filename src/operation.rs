use std::ops::RangeInclusive;

use crate::{
    instruction::{DecodedInstruction, InstructionError},
    memory::Address,
    register::Register,
    vm::VirtualMachine,
};

pub mod add;
pub mod and;
pub mod not;

pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;

pub mod st;
pub mod sti;
pub mod str;

pub mod br;
const DR_FIELD: RangeInclusive<u8> = 5..=7;
const MEM_OP_REG_FIELD: RangeInclusive<u8> = 5..=7;
const SR1_FIELD: RangeInclusive<u8> = 8..=10;
const MODE_FIELD: RangeInclusive<u8> = 11..=11;
const SR2_FIELD: RangeInclusive<u8> = 14..=16;
const IMM5_FIELD: RangeInclusive<u8> = 12..=16;
const IMM5_BIT_COUNT: u8 = 5;
const OFFSET_9_BIT_COUNT: u8 = 9;
const OFFSET_6_BIT_COUNT: u8 = 6;
const PC_OFFSET_9_FIELD: RangeInclusive<u8> = 8..=16;
const OFFSET_6_FIELD: RangeInclusive<u8> = 11..=16;

pub trait UnaryOp: Sized {
    /// Constructs the operation from its decoded operands.
    fn from_parts(dr: Register, sr: Register) -> Self;

    /// Decodes the operands of a unary operation from a decoded instruction.
    ///
    /// The destination and source registers are extracted from their respective fields.
    /// The requested bit fields must be in the range `1..=16`
    ///
    /// # Errors
    /// - If an invalid bit range in requested.
    fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError> {
        let raw = instruction.raw();
        let dr = Register::from(raw.bits_as::<u8>(DR_FIELD)?);
        let sr = Register::from(raw.bits_as::<u8>(SR1_FIELD)?);

        Ok(Self::from_parts(dr, sr))
    }

    /// Resolves the operation's operands from the virtual machine state.
    fn operands(self, vm: &VirtualMachine) -> UnaryOperands;

    /// Performs the operation-specific computation.
    fn operate(value: u16) -> u16;

    /// Executes the unary operation and updates the condition code.
    fn execute(self, vm: &mut VirtualMachine) {
        let operands = Self::operands(self, vm);

        let result = Self::operate(operands.value);

        vm.write_register(operands.dr, result);

        vm.set_cond(result);
    }
}

/// Provides shared decoding and execution logic for LC-3 binary operations.
///
/// Binary operations have two source operands and one destination register.
/// The second operand can either be a register or a sign-extended immediate
/// value, depending on the operation's mode.
///
/// Implementors provide the operation-specific construction, operand
/// resolution, and computation while the common decoding and execution
/// logic is provided by this trait.
pub trait BinaryOp: Sized {
    /// Constructs the operation from its decoded operands.
    fn from_parts(dr: Register, sr1: Register, mode: BinaryOpMode) -> Self;

    /// Decodes the operands of a binary operation from a decoded instruction.
    ///
    /// The destination and first source registers are extracted from their
    /// respective instruction fields. The second operand is decoded as either
    /// a register or a 5-bit immediate value according to the mode bit.
    ///
    /// The requested bit fields must be in the range `1..=16`
    ///
    /// # Errors
    /// - If an invalid bit range in requested.
    #[allow(clippy::unreachable)]
    fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError> {
        let raw = instruction.raw();
        let dr = raw.decode_register(DR_FIELD)?;
        let sr1 = raw.decode_register(SR1_FIELD)?;
        let mode = match raw.bits(MODE_FIELD)? {
            0 => BinaryOpMode::Register(raw.decode_register(SR2_FIELD)?),
            1 => BinaryOpMode::Immediate(raw.bits(IMM5_FIELD)?),
            _ => unreachable!(),
        };

        Ok(Self::from_parts(dr, sr1, mode))
    }

    /// Performs the operation-specific computation on two operands.
    fn operate(lhs: u16, rhs: u16) -> u16;

    /// Resolves the operation's operands from the virtual machine state.
    fn operands(self, vm: &VirtualMachine) -> BinaryOperands;

    /// Executes the binary operation and updates the condition code.
    fn execute(self, vm: &mut VirtualMachine) {
        let operands = Self::operands(self, vm);
        let result = Self::operate(operands.lhs, operands.rhs);

        vm.write_register(operands.dr, result);

        vm.set_cond(result);
    }
}

/// Provides shared decoding logic for LC-3 memory operations.
///
/// Memory operations have one [`Register`] and an operation-specific
/// [`MemoryOffset`]. The offset representation is selected by each implementation
/// through the [`Self::Offset`] associated type.
///
/// Implementors provide the operation-specific offset decoding and execution,
/// while this trait provides the common instruction decoding logic.
pub trait MemoryOp: Sized {
    /// The offset representation used by this memory operation.
    type Offset: MemoryOffset;

    /// Construct the operation from its decoded parts.
    fn from_parts(register: Register, offset: Self::Offset) -> Self;

    /// Decodes the operation's offset from an LC-3 instruction.
    ///
    /// # Errors
    ///
    /// Returns [`InstructionError`] if an invalid bit range is requested while
    /// decoding the offset.
    fn offset(instruction: &DecodedInstruction) -> Result<Self::Offset, InstructionError>;

    /// Decodes a memory operation from a decoded instruction.
    ///
    /// The register is extracted from the [`MEM_OP_REG_FIELD`] instruction field,
    /// while the offset is decoded using the implementation's [`Self::Offset`] type.
    ///
    /// # Errors
    ///
    /// Returns [`InstructionError`] if an invalid bit range is requested while
    /// decoding the instruction.
    fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError> {
        let offset: Self::Offset = Self::offset(&instruction)?;
        let register = instruction.raw().decode_register(MEM_OP_REG_FIELD)?;
        Ok(Self::from_parts(register, offset))
    }

    /// Executes the memory operation.
    fn execute(self, vm: &mut VirtualMachine);
}

/// Provides shared execution logic for LC-3 memory load operations.
///
/// Memory load operations have one destination [`Register`] and an operation-specific
/// [`MemoryOffset`]. The offset representation is selected by each implementation
/// through the [`Self::Offset`] associated type defined by the [`MemoryOp`] supertrait.
///
/// Implementors provide operation-specific operand resolution and computation,
/// while this trait provides the common execution logic.
pub trait MemoryLoadOp: Sized + MemoryOp {
    /// Resolves the operation's operands from the constructed operation.
    fn operands(self) -> MemoryOperands<Self::Offset>;

    /// Performs the operation-specific computation.
    fn operate(offset: Self::Offset, vm: &VirtualMachine) -> u16;

    /// Executes the memory load operation and updates the condition code.
    fn execute_load(self, vm: &mut VirtualMachine) {
        let operands: MemoryOperands<Self::Offset> = Self::operands(self);
        let result = Self::operate(operands.offset, vm);

        vm.write_register(operands.register, result);

        vm.set_cond(result);
    }
}

/// Provides shared execution logic for LC-3 memory store operations.
///
/// Memory store operations have one source [`Register`] and an
/// operation-specific [`MemoryOffset`]. The offset representation is selected
/// by each implementation through the [`MemoryOp::Offset`] associated type.
///
/// Implementors provide operation-specific operand resolution and address
/// computation, while this trait provides the common execution logic for
/// writing the source register's value to memory.
pub trait MemoryStoreOp: MemoryOp {
    /// Resolves the operation's operands from the constructed operation.
    fn operands(self) -> MemoryOperands<Self::Offset>;

    /// Computes the memory address targeted by the store operation.
    fn compute_address(offset: Self::Offset, vm: &VirtualMachine) -> Address;

    /// Executes the memory store operation.
    ///
    /// The source register's value is written to the address computed by
    /// [`Self::compute_address`].
    fn execute_store(self, vm: &mut VirtualMachine) {
        let operands: MemoryOperands<Self::Offset> = Self::operands(self);

        let address = Self::compute_address(operands.offset, vm);

        vm.write_memory(address, vm.read_register(operands.register));
    }
}

pub trait ControlFlowOp {
    /// Decodes a control flow operation from a decoded instruction.
    /// Each implementor of the trait must provide its own decoding logic.
    ///
    /// # Errors
    ///
    /// Returns [`InstructionError`] if an invalid bit range is requested while
    /// decoding the instruction.
    fn decode(instruction: DecodedInstruction) -> Result<Self, InstructionError>
    where
        Self: Sized;

    /// Executes the control flow operation, potentially altering the
    /// [`VirtualMachine`] state.
    fn execute(self, vm: &mut VirtualMachine);
}

/// Contains the resolved operands required to execute a binary operation.
pub struct BinaryOperands {
    dr: Register,
    lhs: u16,
    rhs: u16,
}

/// Contains the resolved operands required to execute a unary operation.
pub struct UnaryOperands {
    dr: Register,
    value: u16,
}

/// Contains the resolved operands required for a memory load operation.
pub struct MemoryOperands<T: MemoryOffset> {
    register: Register,
    offset: T,
}

/// Represents the operation mode based on instruction bit 11.
///
/// - 0 = [`BinaryOpMod::Register`]
/// - 1 = [`BinaryOpMode::Immediate`]
#[derive(Debug)]
pub enum BinaryOpMode {
    Register(Register),
    Immediate(u16),
}

/// Marks types that represent an LC-3 memory operation offset.
pub trait MemoryOffset {}

/// Represents a 9-bit immediate offset encoded in an LC-3 instruction.
///
/// The value is stored in its encoded, unsigned representation. Operations
/// using this offset are responsible for sign-extending it when resolving
/// a memory address.
#[derive(Debug)]
pub struct Offset9(u16);

impl MemoryOffset for Offset9 {}

impl Offset9 {
    /// Returns the encoded offset value.
    #[must_use]
    pub const fn value(&self) -> u16 {
        self.0
    }
}

/// Represents a 6-bit base-register offset encoded in an LC-3 instruction.
///
/// The offset consists of a base [`Register`] and a 6-bit immediate value.
/// The immediate value is stored in its encoded, unsigned representation and
/// must be sign-extended when resolving a memory address.
#[derive(Debug)]
pub struct Offset6 {
    base_r: Register,
    value: u16,
}

impl MemoryOffset for Offset6 {}

impl Offset6 {
    /// Returns the encoded 6-bit offset value.
    #[must_use]
    pub const fn value(&self) -> u16 {
        self.value
    }

    /// Returns the base register used to resolve the memory address.
    #[must_use]
    pub const fn base_r(&self) -> Register {
        self.base_r
    }
}

/// Sign-extends an LC-3 value to 16 bits using two's complement representation.
///
/// The most significant bit of the value's bit field is used as the sign bit.
/// If set, the unused upper bits are filled with `1`s; otherwise, they remain
/// `0`.
///
/// # Examples
///
/// A 5-bit immediate value of `0b11111` represents `-1` and is sign-extended
/// to `0xFFFF`.
///
/// # Panics
///
/// Panics if `bit_count` is `0` or greater than `16`.
#[allow(clippy::arithmetic_side_effects)]
const fn sign_extend(value: u16, bit_count: u8) -> u16 {
    assert!(bit_count > 0 && bit_count <= 16);

    let shift = bit_count - 1;
    if (value >> shift) & 1 == 1 {
        value | (0xFFFF << bit_count)
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_extension() {
        assert_eq!(sign_extend(0b00000, 5), 0x0000);
        assert_eq!(sign_extend(0b00001, 5), 0x0001);
        assert_eq!(sign_extend(0b01111, 5), 0x000F);
        assert_eq!(sign_extend(0b10000, 5), 0xFFF0);
        assert_eq!(sign_extend(0b10001, 5), 0xFFF1);
        assert_eq!(sign_extend(0b11111, 5), 0xFFFF);
    }
}
