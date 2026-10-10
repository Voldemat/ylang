#[derive(Debug, PartialEq, Eq)]
pub struct MachO32BitFile {}

impl MachO32BitFile {
    pub fn parse(buffer: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MachO64BitFile {}

impl MachO64BitFile {
    pub fn parse(buffer: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MachOMultiArchFile {}

impl MachOMultiArchFile {
    pub fn parse(buffer: &[u8]) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MachOFile {
    B32(MachO32BitFile),
    B64(MachO64BitFile),
    MultiArch(MachOMultiArchFile),
}

#[derive(Debug)]
pub enum ParseError {
    UnknownMagicValue,
}

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

impl MachOFile {
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &[u8],
    ) -> Result<Self, ParseError> {
        let (magic_buffer, rest_of_the_buffer) = buffer.split_at(4);
        let magic_buffer = magic_buffer.try_into().unwrap();
        if u32::from_be_bytes(magic_buffer) == 0xcafebabe {
            return MachOMultiArchFile::parse(rest_of_the_buffer)
                .map(|f| MachOFile::MultiArch(f));
        };
        let magic = read_u32_with_byte_order(byte_order, magic_buffer);
        match magic {
            0xfeedface => MachO32BitFile::parse(rest_of_the_buffer)
                .map(|f| MachOFile::B32(f)),
            0xfeedfacf => MachO64BitFile::parse(rest_of_the_buffer)
                .map(|f| MachOFile::B64(f)),
            _ => Err(ParseError::UnknownMagicValue),
        }
    }
}
