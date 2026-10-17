use crate::parser::{
    ByteOrder, read_u32_with_byte_order, variants::shared::EOF,
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
pub struct LoadCommandSegment<'b> {
    pub segment_name: [u8; 16],
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
    const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    const COMMAND_TYPE: u32 = 0x00000001;

    #[allow(clippy::unwrap_used, clippy::indexing_slicing)]
    pub fn new(byte_order: ByteOrder, buffer: &'b [u8]) -> Self {
        Self {
            segment_name: buffer[0..16].try_into().unwrap(),
            address: read_u32_with_byte_order(
                byte_order,
                buffer[16..20].try_into().unwrap(),
            ),
            address_size: read_u32_with_byte_order(
                byte_order,
                buffer[20..24].try_into().unwrap(),
            ),
            file_offset: read_u32_with_byte_order(
                byte_order,
                buffer[24..28].try_into().unwrap(),
            ),
            size: read_u32_with_byte_order(
                byte_order,
                buffer[28..32].try_into().unwrap(),
            ),
            maximum_virtual_memory_protections: read_u32_with_byte_order(
                byte_order,
                buffer[32..36].try_into().unwrap(),
            ),
            initial_virtual_memory_protections: read_u32_with_byte_order(
                byte_order,
                buffer[36..40].try_into().unwrap(),
            ),
            sections_count: read_u32_with_byte_order(
                byte_order,
                buffer[40..44].try_into().unwrap(),
            ),
            flags: read_u32_with_byte_order(
                byte_order,
                buffer[44..48].try_into().unwrap(),
            ),
            sections_buffer: &buffer[48..],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name_offset: u32,
    pub symbol_type: u8,
    pub section_number: u8,
    pub data_info: u16,
    pub symbol_address: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoadCommand<'b> {
    Segment(LoadCommandSegment<'b>),
    Shared(SharedLoadCommand<'b>),
}

impl<'b> ILoadCommand<'b> for LoadCommand<'b> {
    fn get_size(&self) -> u32 {
        match self {
            Self::Segment(_) => LoadCommandSegment::BUFFER_SIZE as u32,
            Self::Shared(c) => c.get_size(),
        }
    }

    fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match command_type {
            LoadCommandSegment::COMMAND_TYPE => {
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
