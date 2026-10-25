use crate::parser::ByteOrder;

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandCompressedLinkEditTable {
    pub rebase_file_offset: u32,
    pub rebase_size: u32,
    pub bind_file_offset: u32,
    pub bind_size: u32,
    pub weak_bind_file_offset: u32,
    pub weak_bind_size: u32,
    pub lazy_bind_file_offset: u32,
    pub lazy_bind_size: u32,
    pub export_file_offset: u32,
    pub export_size: u32,
}

impl LoadCommandCompressedLinkEditTable {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE: u32 = 0x00000022;

    #[allow(clippy::unwrap_used)]
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            rebase_file_offset: byte_order
                .read_u32(buffer.get(0..4).unwrap().try_into().unwrap()),
            rebase_size: byte_order
                .read_u32(buffer.get(4..8).unwrap().try_into().unwrap()),
            bind_file_offset: byte_order
                .read_u32(buffer.get(8..12).unwrap().try_into().unwrap()),
            bind_size: byte_order
                .read_u32(buffer.get(12..16).unwrap().try_into().unwrap()),
            weak_bind_file_offset: byte_order
                .read_u32(buffer.get(16..20).unwrap().try_into().unwrap()),
            weak_bind_size: byte_order
                .read_u32(buffer.get(20..24).unwrap().try_into().unwrap()),
            lazy_bind_file_offset: byte_order
                .read_u32(buffer.get(24..28).unwrap().try_into().unwrap()),
            lazy_bind_size: byte_order
                .read_u32(buffer.get(28..32).unwrap().try_into().unwrap()),
            export_file_offset: byte_order
                .read_u32(buffer.get(32..36).unwrap().try_into().unwrap()),
            export_size: byte_order
                .read_u32(buffer.get(36..40).unwrap().try_into().unwrap()),
        }
    }
}
