use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTableInformation {
    pub local_symbols_index: u32,
    pub local_symbols_count: u32,
    pub external_symbols_index: u32,
    pub external_symbols_count: u32,
    pub undefined_symbols_index: u32,
    pub undefined_symbols_count: u32,
    pub content_table_offset: u32,
    pub content_table_entries_count: u32,
    pub module_table_offset: u32,
    pub module_table_entries_count: u32,
    pub referenced_symbol_table_offset: u32,
    pub referenced_symbol_table_entries_count: u32,
    pub indirect_table_offset: u32,
    pub indirect_table_entries_count: u32,
    pub external_relocation_table_offset: u32,
    pub external_relocation_table_entries_count: u32,
    pub local_relocation_table_offset: u32,
    pub local_relocation_table_entries_count: u32,
}

impl SymbolTableInformation {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<{ SymbolTableInformation::BUFFER_SIZE }>
    for SymbolTableInformation
{
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            local_symbols_index: byte_order
                .read_u32(buffer.get(0..4).unwrap().try_into().unwrap()),
            local_symbols_count: byte_order
                .read_u32(buffer.get(4..8).unwrap().try_into().unwrap()),
            external_symbols_index: byte_order
                .read_u32(buffer.get(8..12).unwrap().try_into().unwrap()),
            external_symbols_count: byte_order
                .read_u32(buffer.get(12..16).unwrap().try_into().unwrap()),
            undefined_symbols_index: byte_order
                .read_u32(buffer.get(16..20).unwrap().try_into().unwrap()),
            undefined_symbols_count: byte_order
                .read_u32(buffer.get(20..24).unwrap().try_into().unwrap()),
            content_table_offset: byte_order
                .read_u32(buffer.get(24..28).unwrap().try_into().unwrap()),
            content_table_entries_count: byte_order
                .read_u32(buffer.get(28..32).unwrap().try_into().unwrap()),
            module_table_offset: byte_order
                .read_u32(buffer.get(32..36).unwrap().try_into().unwrap()),
            module_table_entries_count: byte_order
                .read_u32(buffer.get(36..40).unwrap().try_into().unwrap()),
            referenced_symbol_table_offset: byte_order
                .read_u32(buffer.get(40..44).unwrap().try_into().unwrap()),
            referenced_symbol_table_entries_count: byte_order
                .read_u32(buffer.get(44..48).unwrap().try_into().unwrap()),
            indirect_table_offset: byte_order
                .read_u32(buffer.get(48..52).unwrap().try_into().unwrap()),
            indirect_table_entries_count: byte_order
                .read_u32(buffer.get(52..56).unwrap().try_into().unwrap()),
            external_relocation_table_offset: byte_order
                .read_u32(buffer.get(56..60).unwrap().try_into().unwrap()),
            external_relocation_table_entries_count: byte_order
                .read_u32(buffer.get(60..64).unwrap().try_into().unwrap()),
            local_relocation_table_offset: byte_order
                .read_u32(buffer.get(64..68).unwrap().try_into().unwrap()),
            local_relocation_table_entries_count: byte_order
                .read_u32(buffer.get(68..72).unwrap().try_into().unwrap()),
        }
    }
}
