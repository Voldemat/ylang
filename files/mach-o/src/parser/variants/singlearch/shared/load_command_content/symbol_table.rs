use crate::{
    parser::{ByteOrder, variants::singlearch::shared::iheader::IHeader},
    shared::{CStringIterator, EOF, FixedSizeStructIterator, IStaticStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable {
    pub symbols_offset: u32,
    pub symbols_count: u32,
    pub string_table_offset: u32,
    pub string_table_size: u32,
}

impl SymbolTable {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    pub fn get_symbols<
        'b,
        const SYMBOL_STRUCT_BUFFER_SIZE: usize,
        THeader: IHeader<SYMBOL_STRUCT_BUFFER_SIZE>,
    >(
        &self,
        byte_order: ByteOrder,
        file_buffer: &'b [u8],
    ) -> Result<impl Iterator<Item = Result<THeader::SymbolStruct<'b>, EOF>>, EOF>
    {
        self.symbols_offset
            .checked_sub(
                THeader::BUFFER_SIZE as u32 + std::mem::size_of::<u32>() as u32,
            )
            .and_then(|offset| file_buffer.get(offset as usize..))
            .map(|buffer| {
                FixedSizeStructIterator::new(
                    byte_order,
                    buffer,
                    self.symbols_count as usize,
                )
            })
            .ok_or(EOF)
    }

    pub fn get_strings<
        'b,
        const SYMBOL_STRUCT_BUFFER_SIZE: usize,
        THeader: IHeader<SYMBOL_STRUCT_BUFFER_SIZE>,
    >(
        &self,
        file_buffer: &'b [u8],
    ) -> Result<impl Iterator<Item = Result<&'b std::ffi::CStr, EOF>>, EOF>
    {
        self.string_table_offset
            .checked_sub(
                THeader::BUFFER_SIZE as u32 + std::mem::size_of::<u32>() as u32,
            )
            .and_then(|offset| {
                file_buffer.get(
                    offset as usize
                        ..offset as usize + self.string_table_size as usize,
                )
            })
            .map(|buffer| CStringIterator::new(buffer, None))
            .ok_or(EOF)
    }
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
