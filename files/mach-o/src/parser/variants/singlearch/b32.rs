use crate::parser::{
    ByteOrder, read_u32_with_byte_order, variants::shared::EOF,
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
pub struct LoadCommandSegment {
    pub command_type: u32,
    pub command_size: u32,
    pub segment_name: [u8; 16],
    pub address: u32,
    pub address_size: u32,
    pub file_offset: u32,
    pub size: u32,
    pub maximum_virtual_memory_protections: u32,
    pub initial_virtual_memory_protections: u32,
    pub sections_count: u32,
    pub flags: u32,
}

impl LoadCommandSegment {
    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8],
    ) -> Result<Self, EOF> {
        Ok(Self {
            command_type,
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            segment_name: buffer
                .get(4..20)
                .ok_or(EOF)?
                .try_into()
                .map_err(|_| EOF)?,
            address: read_u32_with_byte_order(
                byte_order,
                buffer.get(20..24).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            address_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(24..28).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(28..32).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            size: read_u32_with_byte_order(
                byte_order,
                buffer.get(32..36).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            maximum_virtual_memory_protections: read_u32_with_byte_order(
                byte_order,
                buffer.get(36..40).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            initial_virtual_memory_protections: read_u32_with_byte_order(
                byte_order,
                buffer.get(40..44).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            sections_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(44..48).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            flags: read_u32_with_byte_order(
                byte_order,
                buffer.get(48..52).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
        })
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
    Segment(LoadCommandSegment),
    Shared(SharedLoadCommand<'b>),
}

impl<'b> ILoadCommand<'b> for LoadCommand<'b> {
    fn get_size(&self) -> u32 {
        match self {
            Self::Segment(c) => c.command_size,
            Self::Shared(c) => c.get_size(),
        }
    }

    fn parse(
        byte_order: ByteOrder,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        let (command_type_buffer, rest_of_the_buffer) =
            buffer.split_at_checked(4).ok_or(ParseError::EOF)?;
        let command_type = read_u32_with_byte_order(
            byte_order,
            command_type_buffer
                .try_into()
                .map_err(|_| ParseError::EOF)?,
        );
        match command_type {
            0x00000001 => LoadCommandSegment::parse(
                byte_order,
                command_type,
                rest_of_the_buffer,
            )
            .map(Self::Segment)
            .map_err(|e| e.into()),
            _ => SharedLoadCommand::parse(
                byte_order,
                command_type,
                rest_of_the_buffer,
            )
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
            header: super::shared::Header::parse(byte_order, header_buffer)
                .map_err(|_| EOF)?,
            rest_of_the_buffer,
        })
    }

    pub fn get_load_commands(
        &self,
    ) -> impl Iterator<Item = Result<LoadCommand<'b>, ParseError>> {
        LoadCommandIterator::new(self)
    }
}
