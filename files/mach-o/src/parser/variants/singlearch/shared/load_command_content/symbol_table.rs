use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable {
    pub symbols_offset: u32,
    pub symbols_count: u32,
    pub string_table_offset: u32,
    pub string_table_size: u32,
}

impl SymbolTable {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<'_, { SymbolTable::BUFFER_SIZE }> for SymbolTable {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            symbols_offset: byte_order
                .read_u32(buffer.get(0..4).unwrap().try_into().unwrap()),
            symbols_count: byte_order
                .read_u32(buffer.get(4..8).unwrap().try_into().unwrap()),
            string_table_offset: byte_order
                .read_u32(buffer.get(8..12).unwrap().try_into().unwrap()),
            string_table_size: byte_order
                .read_u32(buffer.get(12..16).unwrap().try_into().unwrap()),
        }
    }
}
