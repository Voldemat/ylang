use crate::{
    parser::ByteOrder,
    shared::{AsciiString, IStaticStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Section<'b> {
    pub section_name: AsciiString<'b, 16>,
    pub segment_name: AsciiString<'b, 16>,
    pub address: u32,
    pub size: u32,
    pub file_offset: u32,
    pub alignment: u32,
    pub relocation_entries_file_offset: u32,
    pub relocation_entries_count: u32,
    pub flags: u32,
    pub _reserved1: u32,
    pub _reserved2: u32,
}

impl Section<'_> {
    pub const BUFFER_SIZE: usize = 16 * 2 + std::mem::size_of::<u32>() * 9;
}

impl<'b> IStaticStruct<'b, { Section::BUFFER_SIZE }> for Section<'b> {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        buffer: &'b [u8; Section::BUFFER_SIZE],
    ) -> Self {
        Self {
            section_name: AsciiString(buffer[0..16].try_into().unwrap()),
            segment_name: AsciiString(buffer[16..32].try_into().unwrap()),
            address: byte_order.read_u32(buffer[32..36].try_into().unwrap()),
            size: byte_order.read_u32(buffer[36..40].try_into().unwrap()),
            file_offset: byte_order
                .read_u32(buffer[40..44].try_into().unwrap()),
            alignment: byte_order.read_u32(buffer[44..48].try_into().unwrap()),
            relocation_entries_file_offset: byte_order
                .read_u32(buffer[48..52].try_into().unwrap()),
            relocation_entries_count: byte_order
                .read_u32(buffer[52..56].try_into().unwrap()),
            flags: byte_order.read_u32(buffer[56..60].try_into().unwrap()),
            _reserved1: byte_order.read_u32(buffer[60..64].try_into().unwrap()),
            _reserved2: byte_order.read_u32(buffer[64..68].try_into().unwrap()),
        }
    }
}
