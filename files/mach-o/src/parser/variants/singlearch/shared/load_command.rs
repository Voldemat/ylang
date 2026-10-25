use crate::parser::ByteOrder;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub command_type: u32,
    pub command_size: u32,
}

impl Header {
    #[allow(clippy::unwrap_used)]
    pub fn new(
        byte_order: ByteOrder,
        buffer: &[u8; std::mem::size_of::<Header>()],
    ) -> Self {
        Self {
            command_type: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            command_size: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
        }
    }
}

pub struct LoadCommand<T> {
    pub header: Header,
    pub content: T,
}
