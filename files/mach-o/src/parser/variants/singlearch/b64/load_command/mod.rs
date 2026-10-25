pub mod encryption_info;
pub mod segment;

use crate::{
    parser::{
        ByteOrder,
        variants::singlearch::shared::{
            iload_command::ILoadCommand, load_command::SharedLoadCommand,
            parse_error::ParseError,
        },
    },
    shared::{IDynamicStruct, parse_dynamic_struct, parse_static_struct},
};

#[derive(Debug, PartialEq, Eq)]
pub enum LoadCommand<'b> {
    Segment(segment::Segment<'b>),
    EncryptionInfo(encryption_info::EncryptionInfo),
    Shared(SharedLoadCommand<'b>),
}

impl<'b> ILoadCommand<'b> for LoadCommand<'b> {
    fn get_size(&self) -> u32 {
        match self {
            Self::Segment(c) => c.get_size(),
            Self::EncryptionInfo(_) => {
                encryption_info::EncryptionInfo::BUFFER_SIZE as u32
            }
            Self::Shared(c) => c.get_size(),
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
            encryption_info::EncryptionInfo::COMMAND_TYPE => {
                parse_static_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::EncryptionInfo)
            }
            _ => SharedLoadCommand::parse(byte_order, command_type, buffer)
                .map(Self::Shared),
        }
    }
}
