use crate::parser::{
    ByteOrder, variants::singlearch::shared::iheader::IHeader,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub load_commands_count: u32,
    pub load_commands_size: u32,
    pub flags: u32,
}

impl IHeader for Header {
    const BUFFER_SIZE: usize = std::mem::size_of::<u32>() * 6;
}

impl Header {
    #[allow(clippy::indexing_slicing)]
    pub const fn new(
        byte_order: ByteOrder,
        value: [u8; std::mem::size_of::<Header>()],
    ) -> Self {
        let (chunks, []) = value.as_chunks::<4>() else {
            unreachable!()
        };
        Self {
            cpu_type: byte_order.read_u32(chunks[0]),
            cpu_subtype: byte_order.read_u32(chunks[1]),
            file_type: byte_order.read_u32(chunks[2]),
            load_commands_count: byte_order.read_u32(chunks[3]),
            load_commands_size: byte_order.read_u32(chunks[4]),
            flags: byte_order.read_u32(chunks[5]),
        }
    }
}
