use crate::{
    parser::{ByteOrder, variants::singlearch::shared::load_command},
    shared::{EOF, IDynamicStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct FilesetEntry<'b> {
    pub address: u64,
    pub file_offset: u64,
    pub entry_id_offset: u32,
    pub _reserved: u32,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> FilesetEntry<'b> {
    pub const COMMAND_TYPE: u32 = 0x80000035;
    pub const PARAMS_BUFFER_SIZE: usize =
        std::mem::size_of::<u64>() * 2 + std::mem::size_of::<u32>() * 2;

    pub fn get_entry_id(&self) -> Result<&'b std::ffi::CStr, EOF> {
        (self.entry_id_offset as usize)
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

impl<'b> IDynamicStruct<'b, { FilesetEntry::PARAMS_BUFFER_SIZE }>
    for FilesetEntry<'b>
{
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        params_buffer: &[u8; FilesetEntry::PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self {
        Self {
            address: byte_order
                .read_u64(params_buffer[0..8].try_into().unwrap()),
            file_offset: byte_order
                .read_u64(params_buffer[8..16].try_into().unwrap()),
            entry_id_offset: byte_order
                .read_u32(params_buffer[16..20].try_into().unwrap()),
            _reserved: byte_order
                .read_u32(params_buffer[20..24].try_into().unwrap()),
            rest_of_the_buffer,
        }
    }

    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.rest_of_the_buffer.len()) as u32
    }
}
