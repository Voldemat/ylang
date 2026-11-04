pub mod encryption_info;
pub mod routines;
pub mod segment;

use gshared::{
    byte_order::ByteOrder,
    structs::{IDynamicStruct, parse_dynamic_struct, parse_static_struct},
};

use crate::parser::variants::singlearch::shared::{
    iload_command_content::ILoadCommandContent,
    load_command_content::SharedLoadCommandContent, parse_error::ParseError,
};

#[derive(Debug, PartialEq, Eq)]
pub enum LoadCommandContent<'b> {
    Segment(segment::Segment<'b>),
    EncryptionInfo(encryption_info::EncryptionInfo),
    Routines(routines::Routines),
    Shared(SharedLoadCommandContent<'b>),
}

impl<'b> ILoadCommandContent<'b> for LoadCommandContent<'b> {
    fn get_size(&self) -> u32 {
        match self {
            Self::Segment(c) => c.get_size(),
            Self::EncryptionInfo(_) => {
                encryption_info::EncryptionInfo::BUFFER_SIZE as u32
            }
            Self::Shared(c) => c.get_size(),
            Self::Routines(_) => routines::Routines::BUFFER_SIZE as u32,
        }
    }

    fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match command_type {
            segment::Segment::COMMAND_TYPE => {
                parse_dynamic_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::Segment)
            }
            routines::Routines::COMMAND_TYPE => {
                parse_static_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::Routines)
            }
            encryption_info::EncryptionInfo::COMMAND_TYPE => {
                parse_static_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::EncryptionInfo)
            }
            _ => SharedLoadCommandContent::parse(
                byte_order,
                command_type,
                buffer,
            )
            .map(Self::Shared),
        }
    }
}
