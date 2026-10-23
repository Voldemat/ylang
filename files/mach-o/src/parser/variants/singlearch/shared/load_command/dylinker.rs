#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandDylinker<'b> {
    pub command_type: u32,
    pub file_path: &'b [u8],
}

impl<'b> LoadCommandDylinker<'b> {
    pub fn get_size(&self) -> u32 {
        self.file_path.len() as u32
    }

    pub fn new(command_type: u32, buffer: &'b [u8]) -> Self {
        Self {
            command_type,
            file_path: buffer,
        }
    }
}
