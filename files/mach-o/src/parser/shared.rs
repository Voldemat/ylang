#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl ByteOrder {
    pub const fn host_order() -> Self {
        cfg_select! {
            target_endian = "little" => Self::LittleEndian,
            target_endian = "big" => Self::BigEndian,
            _ => compile_error!("Unknown host endiannes")
        }
    }

    pub const fn encode_u32(&self, n: u32) -> [u8; 4] {
        match self {
            ByteOrder::BigEndian => n.to_be_bytes(),
            ByteOrder::LittleEndian => n.to_le_bytes(),
        }
    }

    pub const fn read_u16(&self, buffer: [u8; 2]) -> u16 {
        match self {
            ByteOrder::BigEndian => u16::from_be_bytes(buffer),
            ByteOrder::LittleEndian => u16::from_le_bytes(buffer),
        }
    }

    pub const fn read_u32(&self, buffer: [u8; 4]) -> u32 {
        match self {
            ByteOrder::BigEndian => u32::from_be_bytes(buffer),
            ByteOrder::LittleEndian => u32::from_le_bytes(buffer),
        }
    }

    pub const fn read_u64(&self, buffer: [u8; 8]) -> u64 {
        match self {
            ByteOrder::BigEndian => u64::from_be_bytes(buffer),
            ByteOrder::LittleEndian => u64::from_le_bytes(buffer),
        }
    }
}
