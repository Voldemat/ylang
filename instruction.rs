/// Common instructions for ARM64 architecture.
pub struct SharedInstruction {
    pub rd: u8,
    pub rn: u8,
    pub imm12: u16,
}
pub mod add;

pub trait Instruction: Sized {
    const MASK: u32; // The mask used to extract the opcode from a 32-bit instruction.

/// fixed opcode for the instruction.
    const OP_CODE: u32;
///converts an instruction into a 32-bit integer representation.
    fn encode(&self) -> u32;
/// Decodes a 32-bit integer into an instruction, if possible.
    fn decode(i: u32) -> Option<Self>;
/// checks whether word belongs to the instruction format by applying the mask
    fn matches_opcode(i: u32) -> bool {

        (i & Self::MASK) == Self::OP_CODE 
    } 
}

