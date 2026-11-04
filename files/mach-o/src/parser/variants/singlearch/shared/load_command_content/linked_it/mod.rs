pub mod payload;

use gshared::{byte_order::ByteOrder, eof::EOF, structs::IStaticStruct};

use crate::parser::variants::singlearch::shared::iheader::IHeader;

#[derive(Debug, PartialEq, Eq)]
pub struct LinkedIt {
    pub file_offset: u32,
    pub datasize: u32,
}

impl LinkedIt {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    pub fn get_payload<'b, const S: usize, THeader: IHeader<S>>(
        &self,
        file_buffer: &'b [u8],
    ) -> Result<payload::Payload<'b>, EOF> {
        (self.file_offset as usize)
            .checked_sub(THeader::BUFFER_SIZE)
            .and_then(|offset| {
                file_buffer.get(offset..offset + self.datasize as usize)
            })
            .map(|buffer| payload::Payload { buffer })
            .ok_or(EOF)
    }
}

impl IStaticStruct<'_, { LinkedIt::BUFFER_SIZE }> for LinkedIt {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            file_offset: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            datasize: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
        }
    }
}
