use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct UUIDNumber {
    pub id: [u8; 16],
}

impl UUIDNumber {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE: u32 = 0x0000001B;
}

impl IStaticStruct<{ UUIDNumber::BUFFER_SIZE }> for UUIDNumber {
    fn new(_: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self { id: *buffer }
    }
}
