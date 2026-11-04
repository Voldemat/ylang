use gshared::{byte_order::ByteOrder, eof::EOF, structs::IDynamicStruct};

use crate::parser::variants::singlearch::shared::load_command;

#[derive(Debug, PartialEq, Eq)]
pub struct Fvmlib<'b> {
    pub name_offset: u32,
    pub minor_version: u32,
    pub header_addr: u32,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> Fvmlib<'b> {
    pub const COMMAND_TYPE_LOADFVMLIB: u32 = 0x6;
    pub const COMMAND_TYPE_LC_IDFVMLIB: u32 = 0x7;
    pub const PARAMS_BUFFER_SIZE: usize = std::mem::size_of::<u32>() * 3;

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

impl<'b> IDynamicStruct<'b, { Fvmlib::PARAMS_BUFFER_SIZE }> for Fvmlib<'b> {
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        params_buffer: &[u8; Fvmlib::PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self {
        Self {
            name_offset: byte_order
                .read_u32(params_buffer[0..4].try_into().unwrap()),
            minor_version: byte_order
                .read_u32(params_buffer[4..8].try_into().unwrap()),
            header_addr: byte_order
                .read_u32(params_buffer[8..12].try_into().unwrap()),
            rest_of_the_buffer,
        }
    }

    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.rest_of_the_buffer.len()) as u32
    }
}
