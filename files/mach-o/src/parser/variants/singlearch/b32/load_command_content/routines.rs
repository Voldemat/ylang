use gshared::{byte_order::ByteOrder, structs::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct Routines {
    pub init_address: u32,
    pub init_module: u32,
    pub _reserved1: u32,
    pub _reserved2: u32,
    pub _reserved3: u32,
    pub _reserved4: u32,
    pub _reserved5: u32,
    pub _reserved6: u32,
}

impl Routines {
    pub const COMMAND_TYPE: u32 = 0x11;
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<'_, { Routines::BUFFER_SIZE }> for Routines {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        buffer: &[u8; Routines::BUFFER_SIZE],
    ) -> Self {
        Self {
            init_address: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            init_module: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
            _reserved1: byte_order.read_u32(buffer[8..12].try_into().unwrap()),
            _reserved2: byte_order.read_u32(buffer[12..16].try_into().unwrap()),
            _reserved3: byte_order.read_u32(buffer[16..20].try_into().unwrap()),
            _reserved4: byte_order.read_u32(buffer[20..24].try_into().unwrap()),
            _reserved5: byte_order.read_u32(buffer[24..28].try_into().unwrap()),
            _reserved6: byte_order.read_u32(buffer[28..32].try_into().unwrap()),
        }
    }
}
