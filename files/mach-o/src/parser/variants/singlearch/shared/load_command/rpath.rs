use crate::{parser::ByteOrder, shared::IDynamicStruct};

#[derive(Debug, PartialEq, Eq)]
pub struct RPath<'b> {
    pub path_offset: u32,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> RPath<'b> {
    pub const COMMAND_TYPE: u32 = 0x8000001c;
    pub const PARAMS_BUFFER_SIZE: usize = std::mem::size_of::<u32>();
}

impl<'b> IDynamicStruct<'b, { RPath::PARAMS_BUFFER_SIZE }> for RPath<'b> {
    fn new(
        byte_order: ByteOrder,
        params_buffer: &'b [u8; RPath::PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self {
        let path_offset = byte_order.read_u32(*params_buffer);
        Self {
            path_offset,
            rest_of_the_buffer,
        }
    }

    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.rest_of_the_buffer.len()) as u32
    }
}
