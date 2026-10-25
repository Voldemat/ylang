use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct UUIDNumber<'b> {
    pub id: &'b [u8; 16],
}

impl<'b> UUIDNumber<'b> {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<[u8; 16]>();
    pub const COMMAND_TYPE: u32 = 0x0000001B;
}

impl<'b> IStaticStruct<'b, { UUIDNumber::BUFFER_SIZE }> for UUIDNumber<'b> {
    fn new(_: ByteOrder, buffer: &'b [u8; UUIDNumber::BUFFER_SIZE]) -> Self {
        Self { id: buffer }
    }
}
