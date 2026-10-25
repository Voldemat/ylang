use crate::{
    parser::ByteOrder,
    shared::{AsciiString, IStaticStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Note {
    pub data_owner: AsciiString<16>,
    pub offset: u64,
    pub size: u64,
}

impl Note {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE: u32 = 0x31;
}

impl IStaticStruct<{ Note::BUFFER_SIZE }> for Note {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            data_owner: AsciiString(buffer[0..16].try_into().unwrap()),
            offset: byte_order.read_u64(buffer[16..24].try_into().unwrap()),
            size: byte_order.read_u64(buffer[24..32].try_into().unwrap()),
        }
    }
}
