use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct MinimumOSVersion {
    pub minimum_os_version: u32,
    pub sdk_version: u32,
}

impl MinimumOSVersion {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE_WATCHOS: u32 = 48;
    pub const COMMAND_TYPE_IPHONEOS: u32 = 37;
    pub const COMMAND_TYPE_MACOSX: u32 = 36;
    pub const COMMAND_TYPE_TVOS: u32 = 47;
}

impl IStaticStruct<{ MinimumOSVersion::BUFFER_SIZE }> for MinimumOSVersion {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            minimum_os_version: byte_order
                .read_u32(buffer.get(0..4).unwrap().try_into().unwrap()),
            sdk_version: byte_order
                .read_u32(buffer.get(4..8).unwrap().try_into().unwrap()),
        }
    }
}
