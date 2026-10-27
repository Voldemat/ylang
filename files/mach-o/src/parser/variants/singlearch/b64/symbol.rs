use crate::shared::IStaticStruct;

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name_offset: u32,
    pub symbol_type: u8,
    pub section_number: u8,
    pub data_info: u16,
    pub symbol_address: u64,
}

impl Symbol {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<u32>()
        + std::mem::size_of::<u8>() * 2
        + std::mem::size_of::<u16>()
        + std::mem::size_of::<u64>();
}

impl IStaticStruct<'_, { Symbol::BUFFER_SIZE }> for Symbol {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: crate::parser::ByteOrder,
        buffer: &'_ [u8; Symbol::BUFFER_SIZE],
    ) -> Self {
        Self {
            name_offset: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            symbol_type: buffer[4],
            section_number: buffer[5],
            data_info: byte_order.read_u16(buffer[6..8].try_into().unwrap()),
            symbol_address: byte_order
                .read_u64(buffer[8..16].try_into().unwrap()),
        }
    }
}
