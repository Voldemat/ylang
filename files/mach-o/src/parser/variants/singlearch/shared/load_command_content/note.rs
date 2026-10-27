use crate::{
    parser::{ByteOrder, variants::singlearch::shared::iheader::IHeader},
    shared::{AsciiString, EOF, IStaticStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Note<'b> {
    pub data_owner: AsciiString<'b, 16>,
    pub offset: u64,
    pub size: u64,
}

impl<'b> Note<'b> {
    pub const BUFFER_SIZE: usize = 16 + std::mem::size_of::<u64>() * 2;
    pub const COMMAND_TYPE: u32 = 0x31;

    pub fn get_payload<const S: usize, THeader: IHeader<S>>(
        &self,
        file_buffer: &'b [u8],
    ) -> Result<&'b [u8], EOF> {
        self.offset
            .checked_sub(
                THeader::BUFFER_SIZE as u64 + std::mem::size_of::<u32>() as u64,
            )
            .and_then(|offset| {
                file_buffer
                    .get(offset as usize..offset as usize + self.size as usize)
            })
            .ok_or(EOF)
    }
}

impl<'b> IStaticStruct<'b, { Note::BUFFER_SIZE }> for Note<'b> {
    #[allow(clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &'b [u8; Note::BUFFER_SIZE]) -> Self {
        Self {
            data_owner: AsciiString(buffer[0..16].try_into().unwrap()),
            offset: byte_order.read_u64(buffer[16..24].try_into().unwrap()),
            size: byte_order.read_u64(buffer[24..32].try_into().unwrap()),
        }
    }
}
