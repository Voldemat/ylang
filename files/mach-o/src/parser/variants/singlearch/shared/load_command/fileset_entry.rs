use crate::parser::{
    ByteOrder, variants::singlearch::shared::parse_error::ParseError,
};

#[derive(Debug, PartialEq, Eq)]
pub struct FilesetEntry<'b> {
    pub address: u64,
    pub file_offset: u64,
    pub entry_id_offset: u32,
    pub _reserved: u32,
    pub entry_id_buffer: &'b [u8],
}

impl<'b> FilesetEntry<'b> {
    pub const COMMAND_TYPE: u32 = 0x80000035;

    pub fn parse(
        byte_order: ByteOrder,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        Ok(Self {
            address: byte_order.read_u64(
                buffer
                    .get(0..8)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            ),
            file_offset: byte_order.read_u64(
                buffer
                    .get(8..16)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            ),
            entry_id_offset: byte_order.read_u32(
                buffer
                    .get(16..20)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            ),
            _reserved: byte_order.read_u32(
                buffer
                    .get(20..24)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            ),
            entry_id_buffer: buffer.get(24..).ok_or(ParseError::EOF)?,
        })
    }

    pub fn get_size(&self) -> u32 {
        (std::mem::size_of::<u64>() * 3 + self.entry_id_buffer.len()) as u32
    }
}
