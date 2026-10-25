use crate::{
    parser::{ByteOrder, variants::singlearch::shared::load_command},
    shared::{EOF, IDynamicStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct LinkLibrary<'b> {
    pub string_offset: u32,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatible_version: u32,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> LinkLibrary<'b> {
    pub const PARAMS_BUFFER_SIZE: usize = std::mem::size_of::<u32>() * 4;

    pub fn get_string(&self) -> Result<&'b std::ffi::CStr, EOF> {
        (self.string_offset as usize)
            .checked_sub(
                std::mem::size_of::<load_command::Header>()
                    + Self::PARAMS_BUFFER_SIZE,
            )
            .and_then(|offset| self.rest_of_the_buffer.get(offset..))
            .and_then(|buffer| {
                std::ffi::CStr::from_bytes_until_nul(buffer).ok()
            })
            .ok_or(EOF)
    }
}

impl<'b> IDynamicStruct<'b, { LinkLibrary::PARAMS_BUFFER_SIZE }>
    for LinkLibrary<'b>
{
    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.rest_of_the_buffer.len()) as u32
    }

    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        params_buffer: &'b [u8; LinkLibrary::PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self {
        Self {
            string_offset: byte_order
                .read_u32(params_buffer[0..4].try_into().unwrap()),
            timestamp: byte_order
                .read_u32(params_buffer[4..8].try_into().unwrap()),
            current_version: byte_order
                .read_u32(params_buffer[8..12].try_into().unwrap()),
            compatible_version: byte_order
                .read_u32(params_buffer[12..16].try_into().unwrap()),
            rest_of_the_buffer,
        }
    }
}
