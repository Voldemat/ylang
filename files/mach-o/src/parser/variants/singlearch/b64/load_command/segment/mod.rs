use crate::{
    parser::ByteOrder,
    shared::{AsciiString, EOF, FixedSizeStructIterator},
};

pub mod section;

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSegment<'b> {
    pub segment_name: AsciiString<16>,
    pub address: u64,
    pub address_size: u64,
    pub file_offset: u64,
    pub size: u64,
    pub maximum_virtual_memory_protections: u32,
    pub initial_virtual_memory_protections: u32,
    pub sections_count: u32,
    pub flags: u32,
    pub sections_buffer: &'b [u8],
}

impl<'b> LoadCommandSegment<'b> {
    pub const COMMAND_TYPE: u32 = 0x00000019;
    pub fn get_size(&self) -> u32 {
        (std::mem::size_of::<AsciiString<16>>()
            + std::mem::size_of::<u64>() * 6
            + self.sections_buffer.len()) as u32
    }

    pub fn parse_sections(
        &self,
        byte_order: ByteOrder,
    ) -> impl Iterator<Item = Result<section::Section, EOF>> {
        FixedSizeStructIterator::new(
            byte_order,
            self.sections_buffer,
            self.sections_count as usize,
        )
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(byte_order: ByteOrder, buffer: &'b [u8]) -> Self {
        Self {
            segment_name: AsciiString(
                buffer.get(0..16).unwrap().try_into().unwrap(),
            ),
            address: byte_order
                .read_u64(buffer.get(16..24).unwrap().try_into().unwrap()),
            address_size: byte_order
                .read_u64(buffer.get(24..32).unwrap().try_into().unwrap()),
            file_offset: byte_order
                .read_u64(buffer.get(32..40).unwrap().try_into().unwrap()),
            size: byte_order
                .read_u64(buffer.get(40..48).unwrap().try_into().unwrap()),
            maximum_virtual_memory_protections: byte_order
                .read_u32(buffer.get(48..52).unwrap().try_into().unwrap()),
            initial_virtual_memory_protections: byte_order
                .read_u32(buffer.get(52..56).unwrap().try_into().unwrap()),
            sections_count: byte_order
                .read_u32(buffer.get(56..60).unwrap().try_into().unwrap()),
            flags: byte_order
                .read_u32(buffer.get(60..64).unwrap().try_into().unwrap()),
            sections_buffer: buffer.get(64..).unwrap(),
        }
    }
}
