pub mod variants;

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

impl From<variants::shared::EOF> for ParseError {
    fn from(_: variants::shared::EOF) -> Self {
        Self::EOF
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

pub const fn read_u32_with_byte_order(
    byte_order: ByteOrder,
    buffer: [u8; 4],
) -> u32 {
    match byte_order {
        ByteOrder::BigEndian => u32::from_be_bytes(buffer),
        ByteOrder::LittleEndian => u32::from_le_bytes(buffer),
    }
}

pub const fn read_u64_with_byte_order(
    byte_order: ByteOrder,
    buffer: [u8; 8],
) -> u64 {
    match byte_order {
        ByteOrder::BigEndian => u64::from_be_bytes(buffer),
        ByteOrder::LittleEndian => u64::from_le_bytes(buffer),
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
        if magic_big_endian == 0xcafebabe {
            return variants::multiarch::File::parse(rest_of_the_buffer)
                .map(MachOFile::MultiArch)
                .map_err(|e| e.into());
        };
        variants::singlearch::File::parse_with_big_endian_magic_first(
            magic_big_endian,
            rest_of_the_buffer,
        )
        .map(Self::Singlearch)
    }
}
