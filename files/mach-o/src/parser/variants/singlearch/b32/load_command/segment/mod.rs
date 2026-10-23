use crate::{
    parser::{ByteOrder, variants::shared::EOF},
    shared::AsciiString,
};

pub mod section;

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSegment<'b> {
    pub segment_name: AsciiString<16>,
    pub address: u32,
    pub address_size: u32,
    pub file_offset: u32,
    pub size: u32,
    pub maximum_virtual_memory_protections: u32,
    pub initial_virtual_memory_protections: u32,
    pub sections_count: u32,
    pub flags: u32,
    pub sections_buffer: &'b [u8],
}

impl<'b> LoadCommandSegment<'b> {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE: u32 = 0x00000001;

    pub fn parse_sections(
        &self,
        byte_order: ByteOrder,
    ) -> impl Iterator<Item = Result<section::Section, EOF>> {
        section::SegmentSectionIterator::new(
            byte_order,
            self.sections_buffer,
            self.sections_count,
        )
    }

    #[allow(clippy::unwrap_used, clippy::indexing_slicing)]
    pub fn new(byte_order: ByteOrder, buffer: &'b [u8]) -> Self {
        Self {
            segment_name: AsciiString(buffer[0..16].try_into().unwrap()),
            address: byte_order.read_u32(buffer[16..20].try_into().unwrap()),
            address_size: byte_order
                .read_u32(buffer[20..24].try_into().unwrap()),
            file_offset: byte_order
                .read_u32(buffer[24..28].try_into().unwrap()),
            size: byte_order.read_u32(buffer[28..32].try_into().unwrap()),
            maximum_virtual_memory_protections: byte_order
                .read_u32(buffer[32..36].try_into().unwrap()),
            initial_virtual_memory_protections: byte_order
                .read_u32(buffer[36..40].try_into().unwrap()),
            sections_count: byte_order
                .read_u32(buffer[40..44].try_into().unwrap()),
            flags: byte_order.read_u32(buffer[44..48].try_into().unwrap()),
            sections_buffer: &buffer[48..],
        }
    }
}
