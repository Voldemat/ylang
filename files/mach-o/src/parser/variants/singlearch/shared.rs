use crate::parser::{
    ByteOrder, read_u32_with_byte_order, read_u64_with_byte_order,
    variants::shared::EOF,
};

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Header {
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub load_commands_count: u32,
    pub load_commands_size: u32,
    pub flags: u32,
    pub _reserved: u32,
}

impl Header {
    #[allow(clippy::indexing_slicing)]
    pub const fn new(
        byte_order: ByteOrder,
        value: [u8; std::mem::size_of::<Header>()],
    ) -> Self {
        let (chunks, []) = value.as_chunks::<4>() else {
            unreachable!()
        };
        Self {
            cpu_type: read_u32_with_byte_order(byte_order, chunks[0]),
            cpu_subtype: read_u32_with_byte_order(byte_order, chunks[1]),
            file_type: read_u32_with_byte_order(byte_order, chunks[2]),
            load_commands_count: read_u32_with_byte_order(
                byte_order, chunks[3],
            ),
            load_commands_size: read_u32_with_byte_order(byte_order, chunks[4]),
            flags: read_u32_with_byte_order(byte_order, chunks[5]),
            _reserved: read_u32_with_byte_order(byte_order, chunks[6]),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandHeader {
    pub command_type: u32,
    pub command_size: u32,
}

impl LoadCommandHeader {
    #[allow(clippy::unwrap_used)]
    pub fn new(
        byte_order: ByteOrder,
        buffer: &[u8; std::mem::size_of::<LoadCommandHeader>()],
    ) -> Self {
        Self {
            command_type: read_u32_with_byte_order(
                byte_order,
                buffer[0..4].try_into().unwrap(),
            ),
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer[4..8].try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandLinkedIt {
    pub command_type: u32,
    pub file_offset: u32,
    pub datasize: u32,
}

impl LoadCommandLinkedIt {
    pub const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();
    #[allow(clippy::unwrap_used)]
    pub fn new(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            command_type,
            file_offset: read_u32_with_byte_order(
                byte_order,
                buffer[0..4].try_into().unwrap(),
            ),
            datasize: read_u32_with_byte_order(
                byte_order,
                buffer[4..8].try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandDylinker<'b> {
    pub command_type: u32,
    pub file_path: &'b [u8],
}

impl<'b> LoadCommandDylinker<'b> {
    pub fn get_size(&self) -> u32 {
        self.file_path.len() as u32
    }

    pub fn new(command_type: u32, buffer: &'b [u8]) -> Self {
        Self {
            command_type,
            file_path: buffer,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSourceVersion {
    pub version: u64,
}

impl LoadCommandSourceVersion {
    const BUFFER_SIZE: usize = std::mem::size_of::<u64>();

    pub fn new(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            version: read_u64_with_byte_order(byte_order, *buffer),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandLinkLibrary<'b> {
    pub string_offset: u32,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatible_version: u32,
    pub file_path: &'b [u8],
}

impl<'b> LoadCommandLinkLibrary<'b> {
    pub fn get_size(&self) -> u32 {
        (std::mem::size_of::<u32>() * 4 + self.file_path.len()) as u32
    }

    pub fn parse(byte_order: ByteOrder, buffer: &'b [u8]) -> Result<Self, EOF> {
        Ok(Self {
            string_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            timestamp: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            current_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            compatible_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            file_path: buffer.get(16..).ok_or(EOF)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSymbolTable {
    pub symbols_offset: u32,
    pub symbols_count: u32,
    pub string_table_offset: u32,
    pub string_table_size: u32,
}

impl LoadCommandSymbolTable {
    const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    #[allow(clippy::unwrap_used)]
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            symbols_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).unwrap().try_into().unwrap(),
            ),
            symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).unwrap().try_into().unwrap(),
            ),
            string_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).unwrap().try_into().unwrap(),
            ),
            string_table_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).unwrap().try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSymbolTableInformation {
    pub local_symbols_index: u32,
    pub local_symbols_count: u32,
    pub external_symbols_index: u32,
    pub external_symbols_count: u32,
    pub undefined_symbols_index: u32,
    pub undefined_symbols_count: u32,
    pub content_table_offset: u32,
    pub content_table_entries_count: u32,
    pub module_table_offset: u32,
    pub module_table_entries_count: u32,
    pub referenced_symbol_table_offset: u32,
    pub referenced_symbol_table_entries_count: u32,
    pub indirect_table_offset: u32,
    pub indirect_table_entries_count: u32,
    pub external_relocation_table_offset: u32,
    pub external_relocation_table_entries_count: u32,
    pub local_relocation_table_offset: u32,
    pub local_relocation_table_entries_count: u32,
}

impl LoadCommandSymbolTableInformation {
    const BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    #[allow(clippy::unwrap_used)]
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            local_symbols_index: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).unwrap().try_into().unwrap(),
            ),
            local_symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).unwrap().try_into().unwrap(),
            ),
            external_symbols_index: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).unwrap().try_into().unwrap(),
            ),
            external_symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).unwrap().try_into().unwrap(),
            ),
            undefined_symbols_index: read_u32_with_byte_order(
                byte_order,
                buffer.get(16..20).unwrap().try_into().unwrap(),
            ),
            undefined_symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(20..24).unwrap().try_into().unwrap(),
            ),
            content_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(24..28).unwrap().try_into().unwrap(),
            ),
            content_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(28..32).unwrap().try_into().unwrap(),
            ),
            module_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(32..36).unwrap().try_into().unwrap(),
            ),
            module_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(36..40).unwrap().try_into().unwrap(),
            ),
            referenced_symbol_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(40..44).unwrap().try_into().unwrap(),
            ),
            referenced_symbol_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(44..48).unwrap().try_into().unwrap(),
            ),
            indirect_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(48..52).unwrap().try_into().unwrap(),
            ),
            indirect_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(52..56).unwrap().try_into().unwrap(),
            ),
            external_relocation_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(56..60).unwrap().try_into().unwrap(),
            ),
            external_relocation_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(60..64).unwrap().try_into().unwrap(),
            ),
            local_relocation_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(64..68).unwrap().try_into().unwrap(),
            ),
            local_relocation_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(68..72).unwrap().try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandCompressedLinkEditTable {
    pub rebase_file_offset: u32,
    pub rebase_size: u32,
    pub bind_file_offset: u32,
    pub bind_size: u32,
    pub weak_bind_file_offset: u32,
    pub weak_bind_size: u32,
    pub lazy_bind_file_offset: u32,
    pub lazy_bind_size: u32,
    pub export_file_offset: u32,
    pub export_size: u32,
}

impl LoadCommandCompressedLinkEditTable {
    const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    #[allow(clippy::unwrap_used)]
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            rebase_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).unwrap().try_into().unwrap(),
            ),
            rebase_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).unwrap().try_into().unwrap(),
            ),
            bind_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).unwrap().try_into().unwrap(),
            ),
            bind_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).unwrap().try_into().unwrap(),
            ),
            weak_bind_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(16..20).unwrap().try_into().unwrap(),
            ),
            weak_bind_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(20..24).unwrap().try_into().unwrap(),
            ),
            lazy_bind_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(24..28).unwrap().try_into().unwrap(),
            ),
            lazy_bind_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(28..32).unwrap().try_into().unwrap(),
            ),
            export_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(32..36).unwrap().try_into().unwrap(),
            ),
            export_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(36..40).unwrap().try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandMainEntryPoint {
    pub address: u64,
    pub stack_memory_size: u64,
}

impl LoadCommandMainEntryPoint {
    const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    #[allow(clippy::unwrap_used)]
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            address: read_u64_with_byte_order(
                byte_order,
                buffer.get(0..8).unwrap().try_into().unwrap(),
            ),
            stack_memory_size: read_u64_with_byte_order(
                byte_order,
                buffer.get(8..16).unwrap().try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandUUIDNumber {
    pub id: [u8; 16],
}

impl LoadCommandUUIDNumber {
    const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    pub fn parse(buffer: &[u8; Self::BUFFER_SIZE]) -> Self {
        Self { id: *buffer }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandMinimumOSVersion {
    pub platform_type: u32,
    pub minimum_os_version: u32,
    pub sdk_version: u32,
    pub number_of_tools_used: u32,
}

impl LoadCommandMinimumOSVersion {
    const BUFFER_SIZE: usize = std::mem::size_of::<Self>();

    #[allow(clippy::unwrap_used)]
    pub fn parse(
        byte_order: ByteOrder,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Self {
        Self {
            platform_type: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).unwrap().try_into().unwrap(),
            ),
            minimum_os_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).unwrap().try_into().unwrap(),
            ),
            sdk_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).unwrap().try_into().unwrap(),
            ),
            number_of_tools_used: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).unwrap().try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SharedLoadCommand<'b> {
    LinkLibrary(LoadCommandLinkLibrary<'b>),
    SymbolTable(LoadCommandSymbolTable),
    SymbolTableInformation(LoadCommandSymbolTableInformation),
    CompressedLinkEditTable(LoadCommandCompressedLinkEditTable),
    MainEntryPoint(LoadCommandMainEntryPoint),
    UUIDNumber(LoadCommandUUIDNumber),
    MinimumOSVersion(LoadCommandMinimumOSVersion),
    LinkedIt(LoadCommandLinkedIt),
    Dylinker(LoadCommandDylinker<'b>),
    SourceVersion(LoadCommandSourceVersion),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EOF,
    UnknownLoadCommandType(u32),
}

impl From<EOF> for ParseError {
    fn from(_: EOF) -> Self {
        Self::EOF
    }
}

impl<'b> SharedLoadCommand<'b> {
    pub fn get_size(&self) -> u32 {
        match self {
            Self::LinkLibrary(c) => c.get_size(),
            Self::SymbolTable(_) => LoadCommandSymbolTable::BUFFER_SIZE as u32,
            Self::SymbolTableInformation(_) => {
                LoadCommandSymbolTableInformation::BUFFER_SIZE as u32
            }
            Self::CompressedLinkEditTable(_) => {
                LoadCommandCompressedLinkEditTable::BUFFER_SIZE as u32
            }
            Self::MainEntryPoint(_) => {
                LoadCommandMainEntryPoint::BUFFER_SIZE as u32
            }
            Self::UUIDNumber(_) => LoadCommandUUIDNumber::BUFFER_SIZE as u32,
            Self::MinimumOSVersion(_) => {
                LoadCommandMinimumOSVersion::BUFFER_SIZE as u32
            }
            Self::LinkedIt(_) => LoadCommandLinkedIt::BUFFER_SIZE as u32,
            Self::Dylinker(c) => c.get_size(),
            Self::SourceVersion(_) => {
                LoadCommandSourceVersion::BUFFER_SIZE as u32
            }
        }
    }

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match command_type {
            0x0000000C | 0x0000000D | 0x00000018 => {
                LoadCommandLinkLibrary::parse(byte_order, buffer)
                    .map(Self::LinkLibrary)
                    .map_err(|e| e.into())
            }
            42 => buffer
                .try_into()
                .map_err(|_| ParseError::EOF)
                .map(|command_buffer| {
                    LoadCommandSourceVersion::new(byte_order, command_buffer)
                })
                .map(Self::SourceVersion),
            14 | 15 | 29 | 38 | 39 | 41 => Ok(Self::Dylinker(
                LoadCommandDylinker::new(command_type, buffer),
            )),
            0x80000034 | 0x80000033 => buffer
                .try_into()
                .map_err(|_| ParseError::EOF)
                .map(|command_buffer| {
                    LoadCommandLinkedIt::new(
                        byte_order,
                        command_type,
                        command_buffer,
                    )
                })
                .map(Self::LinkedIt),
            0x00000002 => buffer
                .get(0..LoadCommandSymbolTable::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    LoadCommandSymbolTable::parse(byte_order, command_buffer)
                })
                .map(Self::SymbolTable),
            0x0000000B => buffer
                .get(0..LoadCommandSymbolTableInformation::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    LoadCommandSymbolTableInformation::parse(
                        byte_order,
                        command_buffer,
                    )
                })
                .map(Self::SymbolTableInformation),
            0x00000022 => buffer
                .get(0..LoadCommandCompressedLinkEditTable::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    LoadCommandCompressedLinkEditTable::parse(
                        byte_order,
                        command_buffer,
                    )
                })
                .map(Self::CompressedLinkEditTable),
            0x00000028 | 0x80000028 => buffer
                .get(0..LoadCommandMainEntryPoint::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    LoadCommandMainEntryPoint::parse(byte_order, command_buffer)
                })
                .map(Self::MainEntryPoint),
            0x0000001B => buffer
                .get(0..LoadCommandUUIDNumber::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    LoadCommandUUIDNumber::parse(command_buffer)
                })
                .map(Self::UUIDNumber),
            0x00000032 => buffer
                .get(0..LoadCommandMinimumOSVersion::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    LoadCommandMinimumOSVersion::parse(
                        byte_order,
                        command_buffer,
                    )
                })
                .map(Self::MinimumOSVersion),
            unknown_command_type => {
                Err(ParseError::UnknownLoadCommandType(unknown_command_type))
            }
        }
    }
}

pub trait IFile<'b> {
    fn get_byte_order(&self) -> ByteOrder;
    fn get_header(&self) -> &Header;
    fn get_rest_of_the_buffer(&self) -> &'b [u8];
}

pub trait ILoadCommand<'b>: Sized + std::fmt::Debug {
    fn get_size(&self) -> u32;
    fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError>;
}

pub struct LoadCommandIterator<'f, TFile, TLoadCommand> {
    pub file: &'f TFile,
    pub count: u32,
    pub offset: usize,
    pub _a: std::marker::PhantomData<TLoadCommand>,
}

impl<'f, 'b, TFile: IFile<'b>, TLoadCommand: ILoadCommand<'b>>
    LoadCommandIterator<'f, TFile, TLoadCommand>
{
    pub const fn new(file: &'f TFile) -> Self {
        Self {
            file,
            count: 0,
            offset: 0,
            _a: std::marker::PhantomData::<TLoadCommand>,
        }
    }
}

impl<'f, 'b, TFile: IFile<'b>, TLoadCommand: ILoadCommand<'b>> Iterator
    for LoadCommandIterator<'f, TFile, TLoadCommand>
{
    type Item = Result<TLoadCommand, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.file.get_header().load_commands_count {
            assert!(
                self.offset as u32 == self.file.get_header().load_commands_size
            );
            return None;
        };
        let Some(buffer) = &self.file.get_rest_of_the_buffer().get(
            self.offset..self.file.get_header().load_commands_size as usize,
        ) else {
            self.count = self.file.get_header().load_commands_count;
            return Some(Err(ParseError::EOF));
        };
        let Some((header_buffer, rest_of_the_buffer)) = buffer
            .split_first_chunk::<{ std::mem::size_of::<LoadCommandHeader>() }>(
            )
        else {
            self.count = self.file.get_header().load_commands_count;
            return Some(Err(ParseError::EOF));
        };
        let command_header =
            LoadCommandHeader::new(self.file.get_byte_order(), header_buffer);
        let Some(command_buffer) = rest_of_the_buffer.get(
            0..command_header.command_size as usize
                - std::mem::size_of::<LoadCommandHeader>(),
        ) else {
            self.count = self.file.get_header().load_commands_count;
            return Some(Err(ParseError::EOF));
        };
        self.count += 1;
        self.offset += command_header.command_size as usize;
        Some(TLoadCommand::parse(
            self.file.get_byte_order(),
            command_header.command_type,
            command_buffer,
        ))
    }
}
