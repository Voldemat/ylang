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

#[derive(Debug, PartialEq, Eq)]
pub struct EntryHeader {
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_offset: u32,
    pub size: u32,
    pub section_alignment: u32,
}

impl EntryHeader {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    #[allow(clippy::unwrap_used)]
    pub fn new(buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            cpu_type: u32::from_be_bytes(buffer[0..4].try_into().unwrap()),
            cpu_subtype: u32::from_be_bytes(buffer[4..8].try_into().unwrap()),
            file_offset: u32::from_be_bytes(buffer[8..12].try_into().unwrap()),
            size: u32::from_be_bytes(buffer[12..16].try_into().unwrap()),
            section_alignment: u32::from_be_bytes(
                buffer[16..20].try_into().unwrap(),
            ),
        }
    }
}

pub struct EntryHeaderIterator<'b> {
    pub buffer: &'b [u8],
    pub binaries_count: u32,
    pub count: u32,
}

impl<'b> Iterator for EntryHeaderIterator<'b> {
    type Item = Result<EntryHeader, EOF>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.binaries_count {
            return None;
        };
        let offset = self.count as usize * EntryHeader::BUFFER_SIZE;
        let Some(entry_header_buffer) = self
            .buffer
            .get(offset..offset + EntryHeader::BUFFER_SIZE)
            .and_then(|b| b.try_into().ok())
        else {
            self.count = self.binaries_count;
            return Some(Err(EOF));
        };
        self.count += 1;
        Some(Ok(EntryHeader::new(entry_header_buffer)))
    }
}

impl<'b> File<'b> {
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
        EntryHeaderIterator {
            buffer: self.rest_of_the_buffer,
            binaries_count: self.header.binaries_count,
            count: 0,
        }
    }
}
