use crate::parser::{ParseError, shared::ByteOrder};

pub mod b32;
pub mod b64;
pub mod shared;

#[derive(Debug, PartialEq, Eq)]
pub enum File<'b> {
    B32(b32::File<'b>),
    B64(b64::File<'b>),
}

impl<'b> File<'b> {
    pub fn get_byte_order(&self) -> ByteOrder {
        match self {
            Self::B32(f) => f.byte_order,
            Self::B64(f) => f.byte_order,
        }
    }

    pub fn parse_with_magic_and_byte_order(
        magic: u32,
        byte_order: ByteOrder,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match magic {
            b32::File::MAGIC_VALUE => b32::File::parse(byte_order, buffer)
                .map(File::B32)
                .map_err(|e| e.into()),
            b64::File::MAGIC_VALUE => b64::File::parse(byte_order, buffer)
                .map(File::B64)
                .map_err(|e| e.into()),
            _ => {
                Err(ParseError::UnknownMagicValue(byte_order.encode_u32(magic)))
            }
        }
    }

    pub fn parse_with_magic_buffer(
        magic_buffer: [u8; 4],
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        let magic_big_endian = u32::from_be_bytes(magic_buffer);
        Self::try_parse_both_orders_from_be(magic_big_endian, buffer)
    }

    pub fn try_parse_both_orders_from_be(
        magic_big_endian: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        Self::parse_with_magic_and_byte_order(
            magic_big_endian,
            ByteOrder::BigEndian,
            buffer,
        )
        .or_else(|e| {
            if let ParseError::UnknownMagicValue(_) = e {
                Self::parse_with_magic_and_byte_order(
                    magic_big_endian.swap_bytes(),
                    ByteOrder::LittleEndian,
                    buffer,
                )
            } else {
                Err(e)
            }
        })
    }

    pub fn parse(buffer: &'b [u8]) -> Result<Self, ParseError> {
        let (magic_buffer, rest_of_the_buffer) =
            buffer.split_first_chunk::<4>().ok_or(ParseError::EOF)?;
        Self::parse_with_magic_buffer(*magic_buffer, rest_of_the_buffer)
    }
}
