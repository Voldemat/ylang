pub mod types;

#[derive(Debug, PartialEq, Eq)]
pub enum MachOFile<'b> {
    B32(types::singlearch::b32::File<'b>),
    B64(types::singlearch::b64::File<'b>),
    MultiArch(types::multiarch::File<'b>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownMagicValue,
}

impl From<types::singlearch::b32::ParseError> for ParseError {
    fn from(_: types::singlearch::b32::ParseError) -> Self {
        Self::UnknownMagicValue
    }
}

impl From<types::singlearch::b64::ParseError> for ParseError {
    fn from(_: types::singlearch::b64::ParseError) -> Self {
        Self::UnknownMagicValue
    }
}

impl From<types::multiarch::ParseError> for ParseError {
    fn from(_: types::multiarch::ParseError) -> Self {
        Self::UnknownMagicValue
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl ByteOrder {
    pub const fn host_order() -> Self {
        cfg_select! {
            target_endian = "little" => Self::LittleEndian,
            target_endian = "big" => Self::BigEndian,
            _ => compile_error!("Unknown host endiannes")
        }
    }
}

pub fn read_u32_with_byte_order(byte_order: ByteOrder, buffer: [u8; 4]) -> u32 {
    match byte_order {
        ByteOrder::BigEndian => u32::from_be_bytes(buffer),
        ByteOrder::LittleEndian => u32::from_le_bytes(buffer),
    }
}

impl<'b> MachOFile<'b> {
    pub fn get_byte_order(self: &Self) -> ByteOrder {
        match self {
            Self::B64(f) => f.byte_order,
            Self::B32(f) => f.byte_order,
            Self::MultiArch(_) => ByteOrder::BigEndian,
        }
    }

    pub fn parse_with_magic(
        magic: u32,
        byte_order: ByteOrder,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match magic {
            0xfeedface => {
                types::singlearch::b32::File::parse(byte_order, buffer)
                    .map(|f| MachOFile::B32(f))
                    .map_err(|e| e.into())
            }
            0xfeedfacf => {
                types::singlearch::b64::File::parse(byte_order, buffer)
                    .map(|f| MachOFile::B64(f))
                    .map_err(|e| e.into())
            }
            _ => Err(ParseError::UnknownMagicValue),
        }
    }

    pub fn parse(buffer: &'b [u8]) -> Result<Self, ParseError> {
        let (magic_buffer, rest_of_the_buffer) = buffer.split_at(4);
        let magic_buffer = magic_buffer.try_into().unwrap();
        let magic_big_endian = u32::from_be_bytes(magic_buffer);
        if magic_big_endian == 0xcafebabe {
            return types::multiarch::File::parse(rest_of_the_buffer)
                .map(|f| MachOFile::MultiArch(f))
                .map_err(|e| e.into());
        };
        Self::parse_with_magic(
            magic_big_endian,
            ByteOrder::BigEndian,
            rest_of_the_buffer,
        )
        .or_else(|e| {
            if ParseError::UnknownMagicValue == e {
                Self::parse_with_magic(
                    u32::from_le_bytes(magic_buffer),
                    ByteOrder::LittleEndian,
                    rest_of_the_buffer,
                )
            } else {
                Err(e)
            }
        })
    }
}
