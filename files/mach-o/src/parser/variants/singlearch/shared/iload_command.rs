use crate::parser::ByteOrder;

use super::{ifile::IFile, parse_error::ParseError};

pub trait ILoadCommand<'b>: Sized + std::fmt::Debug {
    fn get_size(&self) -> u32;
    fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError>;
}

pub struct LoadCommandIterator<'f, TFile, TLoadCommand> {
    pub file: &'f TFile,
    pub count: u32,
    pub offset: usize,
    pub _a: std::marker::PhantomData<TLoadCommand>,
}

impl<'f, 'b, TFile: IFile<'b>, TLoadCommand: ILoadCommand<'b>>
    LoadCommandIterator<'f, TFile, TLoadCommand>
{
    pub const fn new(file: &'f TFile) -> Self {
        Self {
            file,
            count: 0,
            offset: 0,
            _a: std::marker::PhantomData::<TLoadCommand>,
        }
    }
}

impl<'f, 'b, TFile: IFile<'b>, TLoadCommand: ILoadCommand<'b>> Iterator
    for LoadCommandIterator<'f, TFile, TLoadCommand>
{
    type Item = Result<TLoadCommand, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.file.get_load_commands_count() {
            assert!(self.offset as u32 == self.file.get_load_commands_size());
            return None;
        };
        let Some(buffer) = &self
            .file
            .get_rest_of_the_buffer()
            .get(self.offset..self.file.get_load_commands_size() as usize)
        else {
            self.count = self.file.get_load_commands_count();
            return Some(Err(ParseError::EOF));
        };
        let Some((header_buffer, rest_of_the_buffer)) = buffer
            .split_first_chunk::<{ std::mem::size_of::<super::load_command::header::Header>() }>(
            )
        else {
            self.count = self.file.get_load_commands_count();
            return Some(Err(ParseError::EOF));
        };
        let command_header = super::load_command::header::Header::new(
            self.file.get_byte_order(),
            header_buffer,
        );
        if command_header.command_size
            <= std::mem::size_of::<super::load_command::header::Header>() as u32
        {
            self.count = self.file.get_load_commands_count();
            return Some(Err(ParseError::EOF));
        }
        let command_buffer_size = command_header.command_size as usize
            - std::mem::size_of::<super::load_command::header::Header>();
        let Some(command_buffer) =
            rest_of_the_buffer.get(0..command_buffer_size)
        else {
            self.count = self.file.get_load_commands_count();
            return Some(Err(ParseError::EOF));
        };
        self.count += 1;
        self.offset += command_header.command_size as usize;
        Some(TLoadCommand::parse(
            self.file.get_byte_order(),
            command_header.command_type,
            command_buffer,
        ))
    }
}
