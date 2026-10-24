use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandEntryPoint {
    pub address: u64,
    pub stack_memory_size: u64,
}

impl LoadCommandEntryPoint {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<{ LoadCommandEntryPoint::BUFFER_SIZE }>
    for LoadCommandEntryPoint
{
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            address: byte_order
                .read_u64(buffer.get(0..8).unwrap().try_into().unwrap()),
            stack_memory_size: byte_order
                .read_u64(buffer.get(8..16).unwrap().try_into().unwrap()),
        }
    }
}
