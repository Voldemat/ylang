use crate::parser::ByteOrder;

#[derive(Debug, PartialEq, Eq)]
pub struct File<'b> {
    pub byte_order: ByteOrder,
    pub header: super::shared::Header,
    pub rest_of_the_buffer: &'b [u8],
}

pub enum ParseError {}

impl<'b> File<'b> {
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        let (header_buffer, rest_of_the_buffer) =
            buffer.split_at(std::mem::size_of::<super::shared::Header>());
        let header_buffer: [u8; std::mem::size_of::<super::shared::Header>()] =
            header_buffer.try_into().unwrap();
        Ok(Self {
            byte_order,
            header: super::shared::Header::parse(byte_order, header_buffer),
            rest_of_the_buffer,
        })
    }
}
