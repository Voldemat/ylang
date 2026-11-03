use super::{Instruction, SharedInstruction};

/// ADD immediate instruction for ARM64.
pub struct ADD(pub SharedInstruction);

impl Instruction for ADD {
    const MASK: u32 = 0xFFC00000;
    const OP_CODE: u32 = 0x91000000;

    fn encode(&self) -> u32 {
        Self::OP_CODE
            | ((self.0.imm12 as u32) << 10)
            | ((self.0.rn as u32) << 5)
            | (self.0.rd as u32)
    }

    fn decode(i: u32) -> Option<Self> {
        if !Self::matches_opcode(i) {
            return None;
        }

        Some(ADD(SharedInstruction {
            rd: (i & 0b11111) as u8,
            rn: ((i >> 5) & 0b11111) as u8,
            imm12: ((i >> 10) & 0xFFF) as u16,
        }))
    }
}

 #[test]
    fn test_add_encode(){
        let instruction= ADD(SharedInstruction{
            rd:0,
            rn:1,
            imm12: 5,
        });
        assert_eq!(instruction.encode(), 0x91001420);
        }

#[test]
    fn test_add_decode(){
        let instruction= ADD::decode(0x91001420).unwrap();
    
        assert_eq!(instruction.0.rd, 0);
        assert_eq!(instruction.0.rn, 1);
        assert_eq!(instruction.0.imm12, 5);
    }
#[test]
    fn test_add_round_trip(){
        let original= ADD(SharedInstruction{
            rd:0,
            rn:1,
            imm12: 5,
        });
        let encoded= original.encode();
        let decoded= ADD::decode(encoded).unwrap();
        assert_eq!(original.0.rd, decoded.0.rd);
        assert_eq!(original.0.rn, decoded.0.rn);
        assert_eq!(original.0.imm12, decoded.0.imm12);
    }