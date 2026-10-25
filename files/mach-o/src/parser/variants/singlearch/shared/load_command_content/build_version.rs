use crate::{
    parser::ByteOrder,
    shared::{EOF, FixedSizeStructIterator, IDynamicStruct, IStaticStruct},
};

#[derive(Debug, PartialEq, Eq)]
pub struct BuildVersion<'b> {
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

impl IStaticStruct<'_, { BuildToolVersion::BUFFER_SIZE }> for BuildToolVersion {
    #[allow(clippy::indexing_slicing, clippy::unwrap_used)]
    fn new(byte_order: ByteOrder, buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self {
            tool: byte_order.read_u32(buffer[0..4].try_into().unwrap()),
            version: byte_order.read_u32(buffer[4..8].try_into().unwrap()),
        }
    }
}

impl<'b> BuildVersion<'b> {
    pub const COMMAND_TYPE: u32 = 50;
    pub const PARAMS_BUFFER_SIZE: usize = std::mem::size_of::<u32>() * 4;

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

impl<'b> IDynamicStruct<'b, { BuildVersion::PARAMS_BUFFER_SIZE }>
    for BuildVersion<'b>
{
    #[allow(clippy::unwrap_used)]
    fn new(
        byte_order: ByteOrder,
        params_buffer: &[u8; BuildVersion::PARAMS_BUFFER_SIZE],
        tools_buffer: &'b [u8],
    ) -> Self {
        Self {
            platform: byte_order
                .read_u32(params_buffer[0..4].try_into().unwrap()),
            minos: byte_order.read_u32(params_buffer[4..8].try_into().unwrap()),
            sdk: byte_order.read_u32(params_buffer[8..12].try_into().unwrap()),
            tools_count: byte_order
                .read_u32(params_buffer[12..16].try_into().unwrap()),
            tools_buffer,
        }
    }

    fn get_size(&self) -> u32 {
        (Self::PARAMS_BUFFER_SIZE + self.tools_buffer.len()) as u32
    }
}
