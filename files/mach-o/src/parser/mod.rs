pub mod shared;
pub mod variants;
pub use shared::ByteOrder;

use crate::shared::EOF;

#[derive(Debug, PartialEq, Eq)]
pub enum MachOFile<'b> {
    Singlearch(variants::singlearch::File<'b>),
    MultiArch(variants::multiarch::File<'b>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownMagicValue,
    EOF,
}

impl From<EOF> for ParseError {
    fn from(_: EOF) -> Self {
        Self::EOF
    }
}

impl<'b> MachOFile<'b> {
    pub fn get_byte_order(&self) -> ByteOrder {
        match self {
            Self::Singlearch(f) => f.get_byte_order(),
            Self::MultiArch(_) => ByteOrder::BigEndian,
        }
    }

    pub fn parse(buffer: &'b [u8]) -> Result<Self, ParseError> {
        let (magic_buffer, rest_of_the_buffer) = buffer.split_at(4);
        let magic_buffer =
            magic_buffer.try_into().map_err(|_| ParseError::EOF)?;
        let magic_big_endian = u32::from_be_bytes(magic_buffer);
        if magic_big_endian == variants::multiarch::File::MAGIC_VALUE_BE {
            return variants::multiarch::File::parse(rest_of_the_buffer)
                .map(MachOFile::MultiArch)
                .map_err(|e| e.into());
        };
        variants::singlearch::File::try_parse_both_orders_from_be(
            magic_big_endian,
            rest_of_the_buffer,
        )
        .map(Self::Singlearch)
    }
}
