use crate::shared::IStaticStruct;

#[derive(Debug, PartialEq, Eq)]
pub struct EncryptionInfo {
    pub file_offset: u32,
    pub size: u32,
    pub encryption_system: u32,
    pub _padding: u32,
}

impl EncryptionInfo {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE: u32 = 44;
}

impl IStaticStruct<{ EncryptionInfo::BUFFER_SIZE }> for EncryptionInfo {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: crate::parser::ByteOrder,
        buffer: &[u8; EncryptionInfo::BUFFER_SIZE],
    ) -> Self {
        Self {
            file_offset: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            size: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
            encryption_system: byte_order
                .read_u32(buffer[8..12].try_into().unwrap()),
            _padding: byte_order.read_u32(buffer[12..16].try_into().unwrap()),
        }
    }
}
