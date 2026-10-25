use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandMinimumOSVersion {
    pub platform_type: u32,
    pub minimum_os_version: u32,
    pub sdk_version: u32,
    pub number_of_tools_used: u32,
}

impl LoadCommandMinimumOSVersion {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE_WATCHOS: u32 = 48;
    pub const COMMAND_TYPE_IPHONEOS: u32 = 37;
    pub const COMMAND_TYPE_MACOSX: u32 = 36;
    pub const COMMAND_TYPE_TVOS: u32 = 47;
}

impl IStaticStruct<{ LoadCommandMinimumOSVersion::BUFFER_SIZE }>
    for LoadCommandMinimumOSVersion
{
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            platform_type: byte_order
                .read_u32(buffer.get(0..4).unwrap().try_into().unwrap()),
            minimum_os_version: byte_order
                .read_u32(buffer.get(4..8).unwrap().try_into().unwrap()),
            sdk_version: byte_order
                .read_u32(buffer.get(8..12).unwrap().try_into().unwrap()),
            number_of_tools_used: byte_order
                .read_u32(buffer.get(12..16).unwrap().try_into().unwrap()),
        }
    }
}
