use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct EntryHeader {
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_offset: u32,
    pub size: u32,
    pub section_alignment: u32,
}

impl EntryHeader {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<{ EntryHeader::BUFFER_SIZE }> for EntryHeader {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            cpu_type: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            cpu_subtype: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
            file_offset: byte_order.read_u32(buffer[8..12].try_into().unwrap()),
            size: byte_order.read_u32(buffer[12..16].try_into().unwrap()),
            section_alignment: byte_order
                .read_u32(buffer[16..20].try_into().unwrap()),
        }
    }
}
