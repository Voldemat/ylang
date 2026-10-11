use super::shared::EOF;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub binaries_count: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct File<'b> {
    pub header: Header,
    pub rest_of_the_buffer: &'b [u8],
}

impl<'b> File<'b> {
    pub fn parse(buffer: &'b [u8]) -> Result<Self, EOF> {
        let (header_buffer, rest_of_the_buffer) = buffer
            .split_at_checked(std::mem::size_of::<Header>())
            .ok_or(EOF)?;
        let header_buffer: [u8; std::mem::size_of::<Header>()] =
            header_buffer.try_into().map_err(|_| EOF)?;
        Ok(Self {
            header: Header {
                binaries_count: u32::from_be_bytes(header_buffer),
            },
            rest_of_the_buffer,
        })
    }
}
