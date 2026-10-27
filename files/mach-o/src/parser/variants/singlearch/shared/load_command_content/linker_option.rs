use crate::{
    parser::ByteOrder,
    shared::{CStringIterator, EOF, IDynamicStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct LinkerOption<'b> {
    pub count: u32,
    pub strings_buffer: &'b [u8],
}

impl<'b> LinkerOption<'b> {
    pub const PARAMS_BUFFER_SIZE: usize = std::mem::size_of::<u32>();

    pub fn get_strings(
        &self,
    ) -> impl Iterator<Item = Result<&'b std::ffi::CStr, EOF>> {
        CStringIterator::new(self.strings_buffer, Some(self.count as usize))
    }
}

impl<'b> IDynamicStruct<'b, { LinkerOption::PARAMS_BUFFER_SIZE }>
    for LinkerOption<'b>
{
    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.strings_buffer.len()) as u32
    }

    fn new(
        byte_order: ByteOrder,
        params_buffer: &'b [u8; LinkerOption::PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self {
        Self {
            count: byte_order.read_u32(*params_buffer),
            strings_buffer: rest_of_the_buffer,
        }
    }
}
