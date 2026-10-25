#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandUUIDNumber {
    pub id: [u8; 16],
}

impl LoadCommandUUIDNumber {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub const COMMAND_TYPE: u32 = 0x0000001B;

    pub fn parse(buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self { id: *buffer }
    }
}
