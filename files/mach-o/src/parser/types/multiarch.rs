#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub binaries_count: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct File<'b> {
    pub header: Header,
    pub rest_of_the_buffer: &'b [u8],
}

pub enum ParseError {}

impl<'b> File<'b> {
    pub fn parse(buffer: &'b [u8]) -> Result<Self, ParseError> {
        let (header_buffer, rest_of_the_buffer) =
            buffer.split_at(std::mem::size_of::<Header>());
        let header_buffer: [u8; std::mem::size_of::<Header>()] =
            header_buffer.try_into().unwrap();
        Ok(Self {
            header: Header {
                binaries_count: u32::from_be_bytes(header_buffer),
            },
            rest_of_the_buffer,
        })
    }
}
