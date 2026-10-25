use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct RelocationEntry {
    pub offset: u32,
    pub options: u32,
}

impl RelocationEntry {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<RelocationEntry>();
}

impl IStaticStruct<'_, { RelocationEntry::BUFFER_SIZE }> for RelocationEntry {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        buffer: &'_ [u8; RelocationEntry::BUFFER_SIZE],
    ) -> Self {
        Self {
            offset: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            options: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
        }
    }
}
