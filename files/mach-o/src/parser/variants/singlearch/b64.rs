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
pub struct File<'b> {
    pub byte_order: ByteOrder,
    pub header: super::shared::Header,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> IFile<'b> for File<'b> {
    fn get_byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    fn get_header(&self) -> &super::shared::Header {
        &self.header
    }

    fn get_rest_of_the_buffer(&self) -> &'b [u8] {
        self.rest_of_the_buffer
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {}

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
            .split_at_checked(std::mem::size_of::<super::shared::Header>())
            .ok_or(EOF)?;
        let header_buffer: [u8; std::mem::size_of::<super::shared::Header>()] =
            header_buffer.try_into().map_err(|_| EOF)?;
        Ok(Self {
            byte_order,
            header: super::shared::Header::new(byte_order, header_buffer),
            rest_of_the_buffer,
        })
    }

    pub fn get_load_commands(
        &self,
    ) -> impl Iterator<Item = Result<LoadCommand<'b>, ParseError>> {
        LoadCommandIterator::new(self)
    }
}
