use crate::parser::{
    ByteOrder,
    variants::singlearch::shared::{
        ILoadCommand, ParseError, SharedLoadCommand,
    },
};

pub mod segment;

#[derive(Debug, PartialEq, Eq)]
pub enum LoadCommand<'b> {
    Segment(segment::LoadCommandSegment<'b>),
    Shared(SharedLoadCommand<'b>),
}

impl<'b> ILoadCommand<'b> for LoadCommand<'b> {
    fn get_size(&self) -> u32 {
        match self {
            Self::Segment(_) => segment::LoadCommandSegment::BUFFER_SIZE as u32,
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
            _ => SharedLoadCommand::parse(byte_order, command_type, buffer)
                .map(Self::Shared),
        }
    }
}
