use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct TwoLevelHints {
    pub file_offset: u32,
    pub hints_count: u32,
}

impl TwoLevelHints {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
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
