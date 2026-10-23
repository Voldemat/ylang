use crate::parser::{ByteOrder, variants::shared::EOF};

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandLinkLibrary<'b> {
    pub string_offset: u32,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatible_version: u32,
    pub file_path: &'b [u8],
}

impl<'b> LoadCommandLinkLibrary<'b> {
    pub fn get_size(&self) -> u32 {
        (std::mem::size_of::<u32>() * 4 + self.file_path.len()) as u32
    }

    pub fn parse(byte_order: ByteOrder, buffer: &'b [u8]) -> Result<Self, EOF> {
        Ok(Self {
            string_offset: byte_order.read_u32(
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            timestamp: byte_order.read_u32(
                buffer.get(4..8).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            current_version: byte_order.read_u32(
                buffer.get(8..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            compatible_version: byte_order.read_u32(
                buffer.get(12..16).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            file_path: buffer.get(16..).ok_or(EOF)?,
        })
    }
}
