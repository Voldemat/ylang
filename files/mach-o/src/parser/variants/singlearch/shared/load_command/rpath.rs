#[derive(Debug, PartialEq, Eq)]
pub struct RPath<'b> {
    pub path_buffer: &'b [u8],
}

impl<'b> RPath<'b> {
    pub const COMMAND_TYPE: u32 = 0x8000001c;
    #[allow(clippy::unwrap_used)]
    pub fn parse(buffer: &'b [u8]) -> Self {
        Self {
            path_buffer: buffer,
        }
    }

    pub fn get_size(&self) -> u32 {
        self.path_buffer.len() as u32
    }
}
