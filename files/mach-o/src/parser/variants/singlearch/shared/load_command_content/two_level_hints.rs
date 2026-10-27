use crate::{
    parser::{ByteOrder, variants::singlearch::shared::iheader::IHeader},
    shared::{EOF, FixedSizeStructIterator, IStaticStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct TwoLevelHints {
    pub file_offset: u32,
    pub hints_count: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TwoLevelHint(pub u32);

impl TwoLevelHint {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<u32>();
}

impl IStaticStruct<'_, { TwoLevelHint::BUFFER_SIZE }> for TwoLevelHint {
    fn new(
        byte_order: ByteOrder,
        buffer: &'_ [u8; TwoLevelHint::BUFFER_SIZE],
    ) -> Self {
        Self(byte_order.read_u32(*buffer))
    }
}

impl TwoLevelHints {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    pub fn get_hints<'b, const S: usize, THeader: IHeader<S>>(
        &self,
        byte_order: ByteOrder,
        file_buffer: &'b [u8],
    ) -> Result<impl Iterator<Item = Result<TwoLevelHint, EOF>> + 'b, EOF> {
        (self.file_offset as usize)
            .checked_sub(THeader::BUFFER_SIZE + std::mem::size_of::<u32>())
            .and_then(|offset| file_buffer.get(offset..))
            .map(|buffer| {
                FixedSizeStructIterator::new(
                    byte_order,
                    buffer,
                    self.hints_count as usize,
                )
            })
            .ok_or(EOF)
    }
}

impl IStaticStruct<'_, { TwoLevelHints::BUFFER_SIZE }> for TwoLevelHints {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            file_offset: byte_order
                .read_u32(buffer.get(0..4).unwrap().try_into().unwrap()),
            hints_count: byte_order
                .read_u32(buffer.get(4..8).unwrap().try_into().unwrap()),
        }
    }
}
