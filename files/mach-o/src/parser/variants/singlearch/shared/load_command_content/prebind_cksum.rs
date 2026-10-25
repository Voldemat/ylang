use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct PrebindCksum {
    pub checksum: u32,
}

impl PrebindCksum {
    pub const COMMAND_TYPE: u32 = 0x17;
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<{ PrebindCksum::BUFFER_SIZE }> for PrebindCksum {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        buffer: &[u8; PrebindCksum::BUFFER_SIZE],
    ) -> Self {
        Self {
            checksum: byte_order.read_u32(*buffer),
        }
    }
}
