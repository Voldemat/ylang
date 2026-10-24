pub mod header;
pub mod load_command;

use crate::{parser::shared::ByteOrder, shared::EOF};

use super::shared::{
    ifile::IFile, iload_command::LoadCommandIterator, parse_error::ParseError,
};

#[derive(Debug, PartialEq, Eq)]
pub struct File<'b> {
    pub byte_order: ByteOrder,
    pub header: header::Header,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> File<'b> {
    pub const MAGIC_VALUE: u32 = 0xfeedface;
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
pub struct Symbol {
    pub name_offset: u32,
    pub symbol_type: u8,
    pub section_number: u8,
    pub data_info: u16,
    pub symbol_address: u32,
}

impl<'b> File<'b> {
    pub fn parse(byte_order: ByteOrder, buffer: &'b [u8]) -> Result<Self, EOF> {
        let (header_buffer, rest_of_the_buffer) = buffer
            .split_at_checked(std::mem::size_of::<header::Header>())
            .ok_or(EOF)?;
        let header_buffer: [u8; std::mem::size_of::<header::Header>()] =
            header_buffer.try_into().map_err(|_| EOF)?;
        Ok(Self {
            byte_order,
            header: header::Header::new(byte_order, header_buffer),
            rest_of_the_buffer,
        })
    }

    pub fn get_load_commands(
        &self,
    ) -> impl Iterator<Item = Result<load_command::LoadCommand<'b>, ParseError>>
    {
        LoadCommandIterator::new(self)
    }
}
