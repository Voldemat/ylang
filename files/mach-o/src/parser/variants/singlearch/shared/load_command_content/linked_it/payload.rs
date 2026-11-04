use gshared::{
    byte_order::ByteOrder, eof::EOF, iterators::FixedSizeStructIterator,
    structs::IStaticStruct,
};

#[derive(Debug, PartialEq, Eq)]
pub struct DataInCodeEntry {
    pub offset: u32,
    pub length: u16,
    pub kind: u16,
}

impl DataInCodeEntry {
    pub const BUFFER_SIZE: usize =
        std::mem::size_of::<u32>() + std::mem::size_of::<u16>() * 2;
}

impl IStaticStruct<'_, { DataInCodeEntry::BUFFER_SIZE }> for DataInCodeEntry {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        buffer: &'_ [u8; DataInCodeEntry::BUFFER_SIZE],
    ) -> Self {
        Self {
            offset: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            length: byte_order.read_u16(buffer[4..6].try_into().unwrap()),
            kind: byte_order.read_u16(buffer[6..8].try_into().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Payload<'b> {
    pub buffer: &'b [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub struct DyldChainedFixupsHeader {
    pub fixups_version: u32,
    pub starts_offset: u32,
    pub imports_offset: u32,
    pub symbols_offset: u32,
    pub imports_count: u32,
    pub imports_format: u32,
    pub symbols_format: u32,
}

impl DyldChainedFixupsHeader {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<'_, { DyldChainedFixupsHeader::BUFFER_SIZE }>
    for DyldChainedFixupsHeader
{
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        buffer: &'_ [u8; DyldChainedFixupsHeader::BUFFER_SIZE],
    ) -> Self {
        Self {
            fixups_version: byte_order
                .read_u32(buffer[0..4].try_into().unwrap()),
            starts_offset: byte_order
                .read_u32(buffer[4..8].try_into().unwrap()),
            imports_offset: byte_order
                .read_u32(buffer[8..12].try_into().unwrap()),
            symbols_offset: byte_order
                .read_u32(buffer[12..16].try_into().unwrap()),
            imports_count: byte_order
                .read_u32(buffer[16..20].try_into().unwrap()),
            imports_format: byte_order
                .read_u32(buffer[20..24].try_into().unwrap()),
            symbols_format: byte_order
                .read_u32(buffer[24..28].try_into().unwrap()),
        }
    }
}

pub struct ChainedFixups<'b> {
    pub header: DyldChainedFixupsHeader,
    pub payload: &'b [u8],
}

impl<'b> Payload<'b> {
    pub fn parse_as_data_in_code(
        &self,
        byte_order: ByteOrder,
    ) -> impl Iterator<Item = Result<DataInCodeEntry, EOF>> + 'b {
        FixedSizeStructIterator::new(
            byte_order,
            self.buffer,
            self.buffer.len().div_ceil(8),
        )
    }
}
