use crate::parser::ByteOrder;

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandLinkedIt {
    pub command_type: u32,
    pub file_offset: u32,
    pub datasize: u32,
}

impl LoadCommandLinkedIt {
    pub const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();
    #[allow(clippy::unwrap_used)]
    pub fn new(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            command_type,
            file_offset: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            datasize: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
        }
    }
}
