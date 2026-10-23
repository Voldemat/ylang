mod entry_header;
pub use entry_header::EntryHeader;

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
    pub const MAGIC_VALUE_BE: u32 = 0xcafebabe;

    pub fn parse(buffer: &'b [u8]) -> Result<Self, EOF> {
        let (header_buffer, rest_of_the_buffer) = buffer
            .split_at_checked(std::mem::size_of::<Header>())
            .ok_or(EOF)?;
        let header_buffer: [u8; std::mem::size_of::<Header>()] =
            header_buffer.try_into().map_err(|_| EOF)?;
        let header = Header {
            binaries_count: u32::from_be_bytes(header_buffer),
        };
        Ok(Self {
            header,
            rest_of_the_buffer,
        })
    }

    pub fn get_entry_headers(
        &self,
    ) -> impl Iterator<Item = Result<EntryHeader, EOF>> {
        entry_header::EntryHeaderIterator::new(
            self.rest_of_the_buffer,
            self.header.binaries_count,
        )
    }
}
