use crate::{
    parser::{
        ByteOrder, read_u32_with_byte_order, read_u64_with_byte_order,
        variants::shared::EOF,
    },
    shared::AsciiString,
};

use super::shared::{
    IFile, ILoadCommand, LoadCommandIterator, ParseError, SharedLoadCommand,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub load_commands_count: u32,
    pub load_commands_size: u32,
    pub flags: u32,
    pub _reserved: u32,
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
            cpu_type: read_u32_with_byte_order(byte_order, chunks[0]),
            cpu_subtype: read_u32_with_byte_order(byte_order, chunks[1]),
            file_type: read_u32_with_byte_order(byte_order, chunks[2]),
            load_commands_count: read_u32_with_byte_order(
                byte_order, chunks[3],
            ),
            load_commands_size: read_u32_with_byte_order(byte_order, chunks[4]),
            flags: read_u32_with_byte_order(byte_order, chunks[5]),
            _reserved: read_u32_with_byte_order(byte_order, chunks[6]),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct File<'b> {
    pub byte_order: ByteOrder,
    pub header: Header,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> IFile<'b> for File<'b> {
    fn get_byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    fn get_rest_of_the_buffer(&self) -> &'b [u8] {
        self.rest_of_the_buffer
    }

    fn get_load_commands_count(&self) -> u32 {
        self.header.load_commands_count
    }

    fn get_load_commands_size(&self) -> u32 {
        self.header.load_commands_size
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub section_name: AsciiString<16>,
    pub segment_name: AsciiString<16>,
    pub address: u64,
    pub size: u64,
    pub file_offset: u32,
    pub alignment: u32,
    pub relocation_entries_file_offset: u32,
    pub relocation_entries_count: u32,
    pub flags: u32,
    pub _reserved1: u32,
    pub _reserved2: u32,
    pub _reserved3: u32,
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
            address: read_u64_with_byte_order(
                byte_order,
                buffer[32..40].try_into().unwrap(),
            ),
            size: read_u64_with_byte_order(
                byte_order,
                buffer[40..48].try_into().unwrap(),
            ),
            file_offset: read_u32_with_byte_order(
                byte_order,
                buffer[48..52].try_into().unwrap(),
            ),
            alignment: read_u32_with_byte_order(
                byte_order,
                buffer[52..56].try_into().unwrap(),
            ),
            relocation_entries_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer[56..60].try_into().unwrap(),
            ),
            relocation_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer[60..64].try_into().unwrap(),
            ),
            flags: read_u32_with_byte_order(
                byte_order,
                buffer[64..68].try_into().unwrap(),
            ),
            _reserved1: read_u32_with_byte_order(
                byte_order,
                buffer[68..72].try_into().unwrap(),
            ),
            _reserved2: read_u32_with_byte_order(
                byte_order,
                buffer[72..76].try_into().unwrap(),
            ),
            _reserved3: read_u32_with_byte_order(
                byte_order,
                buffer[76..80].try_into().unwrap(),
            ),
        }
    }
}

pub struct SegmentSectionIterator<'b> {
    pub sections_count: u32,
    pub buffer: &'b [u8],
    pub count: u32,
    pub byte_order: ByteOrder,
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
    pub fn get_size(&self) -> u32 {
        (std::mem::size_of::<AsciiString<16>>()
            + std::mem::size_of::<u64>() * 6
            + self.sections_buffer.len()) as u32
    }

    pub fn parse_sections(
        &self,
        byte_order: ByteOrder,
    ) -> impl Iterator<Item = Result<Section, EOF>> {
        SegmentSectionIterator {
            sections_count: self.sections_count,
            buffer: self.sections_buffer,
            count: 0,
            byte_order,
        }
    }

    #[allow(clippy::unwrap_used)]
    pub fn new(byte_order: ByteOrder, buffer: &'b [u8]) -> Self {
        Self {
            segment_name: AsciiString(
                buffer.get(0..16).unwrap().try_into().unwrap(),
            ),
            address: read_u64_with_byte_order(
                byte_order,
                buffer.get(16..24).unwrap().try_into().unwrap(),
            ),
            address_size: read_u64_with_byte_order(
                byte_order,
                buffer.get(24..32).unwrap().try_into().unwrap(),
            ),
            file_offset: read_u64_with_byte_order(
                byte_order,
                buffer.get(32..40).unwrap().try_into().unwrap(),
            ),
            size: read_u64_with_byte_order(
                byte_order,
                buffer.get(40..48).unwrap().try_into().unwrap(),
            ),
            maximum_virtual_memory_protections: read_u32_with_byte_order(
                byte_order,
                buffer.get(48..52).unwrap().try_into().unwrap(),
            ),
            initial_virtual_memory_protections: read_u32_with_byte_order(
                byte_order,
                buffer.get(52..56).unwrap().try_into().unwrap(),
            ),
            sections_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(56..60).unwrap().try_into().unwrap(),
            ),
            flags: read_u32_with_byte_order(
                byte_order,
                buffer.get(60..64).unwrap().try_into().unwrap(),
            ),
            sections_buffer: buffer.get(64..).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name_offset: u32,
    pub symbol_type: u8,
    pub section_number: u8,
    pub data_info: u16,
    pub symbol_address: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoadCommand<'b> {
    Segment(LoadCommandSegment<'b>),
    Shared(SharedLoadCommand<'b>),
}

impl<'b> ILoadCommand<'b> for LoadCommand<'b> {
    fn get_size(&self) -> u32 {
        match self {
            Self::Segment(c) => c.get_size(),
            Self::Shared(c) => c.get_size(),
        }
    }

    fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match command_type {
            0x00000019 => {
                Ok(Self::Segment(LoadCommandSegment::new(byte_order, buffer)))
            }
            _ => SharedLoadCommand::parse(byte_order, command_type, buffer)
                .map(Self::Shared),
        }
    }
}

impl<'b> File<'b> {
    pub fn parse(byte_order: ByteOrder, buffer: &'b [u8]) -> Result<Self, EOF> {
        let (header_buffer, rest_of_the_buffer) = buffer
            .split_at_checked(std::mem::size_of::<Header>())
            .ok_or(EOF)?;
        let header_buffer: [u8; std::mem::size_of::<Header>()] =
            header_buffer.try_into().map_err(|_| EOF)?;
        Ok(Self {
            byte_order,
            header: Header::new(byte_order, header_buffer),
            rest_of_the_buffer,
        })
    }

    pub fn get_load_commands(
        &self,
    ) -> impl Iterator<Item = Result<LoadCommand<'b>, ParseError>> {
        LoadCommandIterator::new(self)
    }
}
