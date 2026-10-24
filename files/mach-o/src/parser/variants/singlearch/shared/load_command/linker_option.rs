use crate::shared::{CStringIterator, EOF};

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandLinkerOption<'b> {
    pub strings_buffer: &'b [u8],
}

impl<'b> LoadCommandLinkerOption<'b> {
    pub fn get_size(&self) -> u32 {
        self.strings_buffer.len() as u32
    }

    pub fn new(buffer: &'b [u8]) -> Self {
        Self {
            strings_buffer: buffer,
        }
    }

    pub fn get_strings(
        &self,
    ) -> impl Iterator<Item = Result<&'b std::ffi::CStr, EOF>> {
        CStringIterator::new(self.strings_buffer)
    }
}
