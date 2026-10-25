use crate::{
    parser::ByteOrder,
    shared::{AsciiString, EOF, FixedSizeStructIterator, IDynamicStruct},
};

pub mod section;

#[derive(Debug, PartialEq, Eq)]
pub struct Segment<'b> {
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

impl<'b> Segment<'b> {
    pub const COMMAND_TYPE: u32 = 0x00000001;
    pub const PARAMS_BUFFER_SIZE: usize =
        std::mem::size_of::<AsciiString<16>>() + std::mem::size_of::<u32>() * 8;

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
}

impl<'b> IDynamicStruct<'b, { Segment::PARAMS_BUFFER_SIZE }> for Segment<'b> {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        params_buffer: &'b [u8; Segment::PARAMS_BUFFER_SIZE],
        sections_buffer: &'b [u8],
    ) -> Self {
        Self {
            segment_name: AsciiString(params_buffer[0..16].try_into().unwrap()),
            address: byte_order
                .read_u32(params_buffer[16..20].try_into().unwrap()),
            address_size: byte_order
                .read_u32(params_buffer[20..24].try_into().unwrap()),
            file_offset: byte_order
                .read_u32(params_buffer[24..28].try_into().unwrap()),
            size: byte_order
                .read_u32(params_buffer[28..32].try_into().unwrap()),
            maximum_virtual_memory_protections: byte_order
                .read_u32(params_buffer[32..36].try_into().unwrap()),
            initial_virtual_memory_protections: byte_order
                .read_u32(params_buffer[36..40].try_into().unwrap()),
            sections_count: byte_order
                .read_u32(params_buffer[40..44].try_into().unwrap()),
            flags: byte_order
                .read_u32(params_buffer[44..48].try_into().unwrap()),
            sections_buffer,
        }
    }

    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.sections_buffer.len()) as u32
    }
}
