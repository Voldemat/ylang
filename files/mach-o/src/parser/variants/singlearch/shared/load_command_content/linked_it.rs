use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct LinkedIt {
    pub file_offset: u32,
    pub datasize: u32,
}

impl LinkedIt {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<'_, { LinkedIt::BUFFER_SIZE }> for LinkedIt {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            file_offset: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            datasize: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
        }
    }
}
