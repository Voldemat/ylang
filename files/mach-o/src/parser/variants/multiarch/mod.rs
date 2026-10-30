use crate::parser::{ByteOrder, ParseError};

pub mod b32;
pub mod b64;
pub mod shared;

#[derive(Debug, PartialEq, Eq)]
pub enum File<'b> {
    B32(b32::File<'b>),
    B64(b64::File<'b>),
}

impl<'b> File<'b> {
    pub fn parse_with_magic(
        magic: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match magic {
            b32::File::MAGIC_VALUE_BE => b32::File::parse(buffer)
                .map(File::B32)
                .map_err(|e| e.into()),
            b64::File::MAGIC_VALUE_BE => b64::File::parse(buffer)
                .map(File::B64)
                .map_err(|e| e.into()),
            _ => Err(ParseError::UnknownMagicValue(
                ByteOrder::BigEndian.encode_u32(magic),
            )),
        }
    }

    pub fn parse_with_magic_buffer(
        magic_buffer: [u8; 4],
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        let magic_big_endian = u32::from_be_bytes(magic_buffer);
        Self::parse_with_magic(magic_big_endian, buffer)
    }

    pub fn parse(buffer: &'b [u8]) -> Result<Self, ParseError> {
        let (magic_buffer, rest_of_the_buffer) =
            buffer.split_first_chunk::<4>().ok_or(ParseError::EOF)?;
        Self::parse_with_magic_buffer(*magic_buffer, rest_of_the_buffer)
    }
}
