use crate::parser::ByteOrder;

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSourceVersion {
    pub version: u64,
}

impl LoadCommandSourceVersion {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    pub fn new(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            version: byte_order.read_u64(*buffer),
        }
    }
}
