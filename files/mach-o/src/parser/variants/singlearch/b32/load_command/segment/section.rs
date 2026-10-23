use crate::{
    parser::{ByteOrder, variants::shared::EOF},
    shared::AsciiString,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub section_name: AsciiString<16>,
    pub segment_name: AsciiString<16>,
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

impl Section {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Section>();

    #[allow(clippy::unwrap_used)]
    pub fn new(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
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

pub struct SegmentSectionIterator<'b> {
    byte_order: ByteOrder,
    buffer: &'b [u8],
    sections_count: u32,
    count: u32,
}

impl<'b> SegmentSectionIterator<'b> {
    pub fn new(
        byte_order: ByteOrder,
        buffer: &'b [u8],
        sections_count: u32,
    ) -> Self {
        Self {
            byte_order,
            buffer,
            sections_count,
            count: 0,
        }
    }
}

impl<'b> Iterator for SegmentSectionIterator<'b> {
    type Item = Result<Section, EOF>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.sections_count {
            return None;
        };
        let Some(section_buffer) = self
            .buffer
            .get(
                self.count as usize * Section::BUFFER_SIZE
                    ..(self.count as usize + 1) * Section::BUFFER_SIZE,
            )
            .and_then(|b| b.try_into().ok())
        else {
            return Some(Err(EOF));
        };
        self.count += 1;
        Some(Ok(Section::new(self.byte_order, section_buffer)))
    }
}
