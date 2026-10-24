use crate::{
    parser::ByteOrder,
    shared::{EOF, FixedSizeStructIterator, IStaticStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandBuildVersion<'b> {
    pub platform: u32,
    pub minos: u32,
    pub sdk: u32,
    pub tools_count: u32,
    pub tools_buffer: &'b [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub struct BuildToolVersion {
    pub tool: u32,
    pub version: u32,
}

impl BuildToolVersion {
    pub const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
}

impl IStaticStruct<{ BuildToolVersion::BUFFER_SIZE }> for BuildToolVersion {
    #[allow(clippy::indexing_slicing, clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            tool: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            version: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
        }
    }
}

impl<'b> LoadCommandBuildVersion<'b> {
    #[allow(clippy::indexing_slicing)]
    pub fn new(byte_order: ByteOrder, buffer: &'b [u8]) -> Result<Self, EOF> {
        Ok(Self {
            platform: byte_order
                .read_u32(buffer[0..4].try_into().map_err(|_| EOF)?),
            minos: byte_order
                .read_u32(buffer[4..8].try_into().map_err(|_| EOF)?),
            sdk: byte_order
                .read_u32(buffer[8..12].try_into().map_err(|_| EOF)?),
            tools_count: byte_order
                .read_u32(buffer[12..16].try_into().map_err(|_| EOF)?),
            tools_buffer: &buffer[16..],
        })
    }

    pub fn get_size(&self) -> u32 {
        (std::mem::size_of::<u32>() * 4 + self.tools_buffer.len()) as u32
    }

    pub fn get_tools(
        &self,
        byte_order: ByteOrder,
    ) -> impl Iterator<Item = Result<BuildToolVersion, EOF>> {
        FixedSizeStructIterator::<
            'b,
            { BuildToolVersion::BUFFER_SIZE },
            BuildToolVersion,
        >::new(byte_order, self.tools_buffer, self.tools_count as usize)
    }
}
