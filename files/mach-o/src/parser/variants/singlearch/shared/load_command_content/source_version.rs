use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct SourceVersion {
    pub version: u64,
}

impl SourceVersion {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<'_, { SourceVersion::BUFFER_SIZE }> for SourceVersion {
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            version: byte_order.read_u64(*buffer),
        }
    }
}
