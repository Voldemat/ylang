use crate::{
    parser::{ByteOrder, variants::singlearch::shared::load_command},
    shared::{EOF, IDynamicStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Dylinker<'b> {
    pub name_offset: u32,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> Dylinker<'b> {
    pub const PARAMS_BUFFER_SIZE: usize = std::mem::size_of::<u32>();

    pub fn get_name(&self) -> Result<&'b std::ffi::CStr, EOF> {
        (self.name_offset as usize)
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

impl<'b> IDynamicStruct<'b, { Dylinker::PARAMS_BUFFER_SIZE }> for Dylinker<'b> {
    fn new(
        byte_order: ByteOrder,
        params_buffer: &[u8; Dylinker::PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self {
        let name_offset = byte_order.read_u32(*params_buffer);
        Self {
            name_offset,
            rest_of_the_buffer,
        }
    }

    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.rest_of_the_buffer.len()) as u32
    }
}
