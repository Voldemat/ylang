use crate::parser::{ByteOrder, read_u32_with_byte_order};

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Header {
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub load_commands_count: u32,
    pub load_commands_size: u32,
    pub flags: u32,
    pub _reserved: u32,
}

impl Header {
    pub fn parse(
        byte_order: ByteOrder,
        value: [u8; std::mem::size_of::<Header>()],
    ) -> Self {
        Self {
            cpu_type: read_u32_with_byte_order(
                byte_order,
                value[0..4].try_into().unwrap(),
            ),
            cpu_subtype: read_u32_with_byte_order(
                byte_order,
                value[4..8].try_into().unwrap(),
            ),
            file_type: read_u32_with_byte_order(
                byte_order,
                value[8..12].try_into().unwrap(),
            ),
            load_commands_count: read_u32_with_byte_order(
                byte_order,
                value[12..16].try_into().unwrap(),
            ),
            load_commands_size: read_u32_with_byte_order(
                byte_order,
                value[16..20].try_into().unwrap(),
            ),
            flags: read_u32_with_byte_order(
                byte_order,
                value[20..24].try_into().unwrap(),
            ),
            _reserved: read_u32_with_byte_order(
                byte_order,
                value[24..28].try_into().unwrap(),
            ),
        }
    }
}
