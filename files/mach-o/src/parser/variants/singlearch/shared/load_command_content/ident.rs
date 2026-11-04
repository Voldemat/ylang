use gshared::{
    byte_order::ByteOrder, eof::EOF, iterators::CStringIterator,
    structs::IDynamicStruct,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Ident<'b> {
    pub strings_buffer: &'b [u8],
}

impl<'b> Ident<'b> {
    pub const COMMAND_TYPE: u32 = 0x8;
    pub const PARAMS_BUFFER_SIZE: usize = 0;

    pub fn get_strings(
        &self,
    ) -> impl Iterator<Item = Result<&'b std::ffi::CStr, EOF>> + 'b {
        CStringIterator::new(self.strings_buffer, None)
    }
}

impl<'b> IDynamicStruct<'b, { Ident::PARAMS_BUFFER_SIZE }> for Ident<'b> {
    #[allow(clippy::unwrap_used)]
    fn new(
        _: ByteOrder,
        _: &[u8; Ident::PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self {
        Self {
            strings_buffer: rest_of_the_buffer,
        }
    }

    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.strings_buffer.len()) as u32
    }
}
