use gshared::{byte_order::ByteOrder, structs::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct Routines {
    pub init_address: u64,
    pub init_module: u64,
    pub _reserved1: u64,
    pub _reserved2: u64,
    pub _reserved3: u64,
    pub _reserved4: u64,
    pub _reserved5: u64,
    pub _reserved6: u64,
}

impl Routines {
    pub const COMMAND_TYPE: u32 = 0x1A;
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<'_, { Routines::BUFFER_SIZE }> for Routines {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        buffer: &[u8; Routines::BUFFER_SIZE],
    ) -> Self {
        Self {
            init_address: byte_order.read_u64(buffer[0..8].try_into().unwrap()),
            init_module: byte_order.read_u64(buffer[8..16].try_into().unwrap()),
            _reserved1: byte_order.read_u64(buffer[16..24].try_into().unwrap()),
            _reserved2: byte_order.read_u64(buffer[24..32].try_into().unwrap()),
            _reserved3: byte_order.read_u64(buffer[32..40].try_into().unwrap()),
            _reserved4: byte_order.read_u64(buffer[40..48].try_into().unwrap()),
            _reserved5: byte_order.read_u64(buffer[48..56].try_into().unwrap()),
            _reserved6: byte_order.read_u64(buffer[56..64].try_into().unwrap()),
        }
    }
}
