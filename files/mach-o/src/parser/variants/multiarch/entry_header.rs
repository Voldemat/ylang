use crate::parser::variants::shared::EOF;

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
    buffer: &'b [u8],
    binaries_count: u32,
    count: u32,
}

impl<'b> EntryHeaderIterator<'b> {
    pub fn new(buffer: &'b [u8], binaries_count: u32) -> Self {
        Self {
            buffer,
            binaries_count,
            count: 0,
        }
    }
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
