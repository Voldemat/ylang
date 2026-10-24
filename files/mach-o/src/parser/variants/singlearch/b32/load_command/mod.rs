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
    shared::IStaticStruct,
};

#[derive(Debug, PartialEq, Eq)]
pub enum LoadCommand<'b> {
    Segment(segment::LoadCommandSegment<'b>),
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
            segment::LoadCommandSegment::COMMAND_TYPE => Ok(Self::Segment(
                segment::LoadCommandSegment::new(byte_order, buffer),
            )),
            encryption_info::EncryptionInfo::COMMAND_TYPE => buffer
                .try_into()
                .map(|command_buffer| {
                    encryption_info::EncryptionInfo::new(
                        byte_order,
                        command_buffer,
                    )
                })
                .map(Self::EncryptionInfo)
                .map_err(|_| ParseError::EOF),
            _ => SharedLoadCommand::parse(byte_order, command_type, buffer)
                .map(Self::Shared),
        }
    }
}
