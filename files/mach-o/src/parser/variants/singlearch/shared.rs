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
    pub fn parse(
        byte_order: ByteOrder,
        value: [u8; std::mem::size_of::<Header>()],
    ) -> Result<Self, EOF> {
        Ok(Self {
            cpu_type: read_u32_with_byte_order(
                byte_order,
                value[0..4].try_into().map_err(|_| EOF)?,
            ),
            cpu_subtype: read_u32_with_byte_order(
                byte_order,
                value[4..8].try_into().map_err(|_| EOF)?,
            ),
            file_type: read_u32_with_byte_order(
                byte_order,
                value[8..12].try_into().map_err(|_| EOF)?,
            ),
            load_commands_count: read_u32_with_byte_order(
                byte_order,
                value[12..16].try_into().map_err(|_| EOF)?,
            ),
            load_commands_size: read_u32_with_byte_order(
                byte_order,
                value[16..20].try_into().map_err(|_| EOF)?,
            ),
            flags: read_u32_with_byte_order(
                byte_order,
                value[20..24].try_into().map_err(|_| EOF)?,
            ),
            _reserved: read_u32_with_byte_order(
                byte_order,
                value[24..28].try_into().map_err(|_| EOF)?,
            ),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandLinkLibrary<'b> {
    pub command_type: u32,
    pub command_size: u32,
    pub string_offset: u32,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatible_version: u32,
    pub file_path: &'b [u8],
}

impl<'b> LoadCommandLinkLibrary<'b> {
    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, EOF> {
        let command_size = read_u32_with_byte_order(
            byte_order,
            buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
        );
        Ok(Self {
            command_type,
            command_size,
            string_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            timestamp: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            current_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            compatible_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(16..20).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            file_path: buffer.get(20..20 + command_size as usize).ok_or(EOF)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSymbolTable {
    pub command_type: u32,
    pub command_size: u32,
    pub symbols_offset: u32,
    pub symbols_count: u32,
    pub string_table_offset: u32,
    pub string_table_size: u32,
}

impl LoadCommandSymbolTable {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();
    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Result<Self, EOF> {
        Ok(Self {
            command_type,
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            symbols_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            string_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            string_table_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(16..20).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandSymbolTableInformation {
    pub command_type: u32,
    pub command_size: u32,
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
    const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();
    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Result<Self, EOF> {
        Ok(Self {
            command_type,
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            local_symbols_index: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            local_symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            external_symbols_index: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            external_symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(16..20).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            undefined_symbols_index: read_u32_with_byte_order(
                byte_order,
                buffer.get(20..24).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            undefined_symbols_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(24..28).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            content_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(28..32).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            content_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(32..36).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            module_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(36..40).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            module_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(40..44).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            referenced_symbol_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(44..48).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            referenced_symbol_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(48..52).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            indirect_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(52..56).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            indirect_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(56..60).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            external_relocation_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(60..64).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            external_relocation_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(64..68).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            local_relocation_table_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(68..72).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            local_relocation_table_entries_count: read_u32_with_byte_order(
                byte_order,
                buffer.get(72..76).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandCompressedLinkEditTable {
    pub command_type: u32,
    pub command_size: u32,
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
    const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Result<Self, EOF> {
        Ok(Self {
            command_type,
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            rebase_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            rebase_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            bind_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            bind_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(16..20).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            weak_bind_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(20..24).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            weak_bind_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(24..28).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            lazy_bind_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(28..32).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            lazy_bind_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(32..36).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            export_file_offset: read_u32_with_byte_order(
                byte_order,
                buffer.get(36..40).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            export_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(40..44).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandMainEntryPoint {
    pub command_type: u32,
    pub command_size: u32,
    pub address: u64,
    pub stack_memory_size: u64,
}

impl LoadCommandMainEntryPoint {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Result<Self, EOF> {
        Ok(Self {
            command_type,
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            address: read_u64_with_byte_order(
                byte_order,
                buffer.get(4..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            stack_memory_size: read_u64_with_byte_order(
                byte_order,
                buffer.get(12..20).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandUUIDNumber {
    pub command_type: u32,
    pub command_size: u32,
    pub id: [u8; 16],
}

impl LoadCommandUUIDNumber {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Result<Self, EOF> {
        Ok(Self {
            command_type,
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            id: buffer.get(4..20).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LoadCommandMinimumOSVersion {
    pub command_type: u32,
    pub command_size: u32,
    pub platform_type: u32,
    pub minimum_os_version: u32,
    pub sdk_version: u32,
    pub number_of_tools_used: u32,
}

impl LoadCommandMinimumOSVersion {
    const BUFFER_SIZE: usize =
        std::mem::size_of::<Self>() - std::mem::size_of::<u32>();

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &[u8; Self::BUFFER_SIZE],
    ) -> Result<Self, EOF> {
        Ok(Self {
            command_type,
            command_size: read_u32_with_byte_order(
                byte_order,
                buffer.get(0..4).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            platform_type: read_u32_with_byte_order(
                byte_order,
                buffer.get(4..8).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            minimum_os_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(8..12).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            sdk_version: read_u32_with_byte_order(
                byte_order,
                buffer.get(12..16).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
            number_of_tools_used: read_u32_with_byte_order(
                byte_order,
                buffer.get(16..20).ok_or(EOF)?.try_into().map_err(|_| EOF)?,
            ),
        })
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
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EOF,
    UnknownLoadCommandType,
}

impl From<EOF> for ParseError {
    fn from(_: EOF) -> Self {
        Self::EOF
    }
}

impl<'b> SharedLoadCommand<'b> {
    pub fn get_size(&self) -> u32 {
        match self {
            Self::LinkLibrary(c) => c.command_size,
            Self::SymbolTable(c) => c.command_size,
            Self::SymbolTableInformation(c) => c.command_size,
            Self::CompressedLinkEditTable(c) => c.command_size,
            Self::MainEntryPoint(c) => c.command_size,
            Self::UUIDNumber(c) => c.command_size,
            Self::MinimumOSVersion(c) => c.command_size,
        }
    }

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match command_type {
            0x0000000C | 0x0000000D | 0x00000018 => {
                LoadCommandLinkLibrary::parse(byte_order, command_type, buffer)
                    .map(Self::LinkLibrary)
                    .map_err(|e| e.into())
            }
            0x00000002 => LoadCommandSymbolTable::parse(
                byte_order,
                command_type,
                buffer
                    .get(0..LoadCommandSymbolTable::BUFFER_SIZE)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            )
            .map(Self::SymbolTable)
            .map_err(|e| e.into()),
            0x0000000B => LoadCommandSymbolTableInformation::parse(
                byte_order,
                command_type,
                buffer
                    .get(0..LoadCommandSymbolTableInformation::BUFFER_SIZE)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            )
            .map(Self::SymbolTableInformation)
            .map_err(|e| e.into()),
            0x00000022 => LoadCommandCompressedLinkEditTable::parse(
                byte_order,
                command_type,
                buffer
                    .get(0..LoadCommandCompressedLinkEditTable::BUFFER_SIZE)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            )
            .map(Self::CompressedLinkEditTable)
            .map_err(|e| e.into()),
            0x00000028 => LoadCommandMainEntryPoint::parse(
                byte_order,
                command_type,
                buffer
                    .get(0..LoadCommandMainEntryPoint::BUFFER_SIZE)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            )
            .map(Self::MainEntryPoint)
            .map_err(|e| e.into()),
            0x0000001B => LoadCommandUUIDNumber::parse(
                byte_order,
                command_type,
                buffer
                    .get(0..LoadCommandUUIDNumber::BUFFER_SIZE)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            )
            .map(Self::UUIDNumber)
            .map_err(|e| e.into()),
            0x00000032 => LoadCommandMinimumOSVersion::parse(
                byte_order,
                command_type,
                buffer
                    .get(0..LoadCommandMinimumOSVersion::BUFFER_SIZE)
                    .ok_or(ParseError::EOF)?
                    .try_into()
                    .map_err(|_| ParseError::EOF)?,
            )
            .map(Self::MinimumOSVersion)
            .map_err(|e| e.into()),
            _ => Err(ParseError::UnknownLoadCommandType),
        }
    }
}

pub trait IFile<'b> {
    fn get_byte_order(&self) -> ByteOrder;
    fn get_header(&self) -> &Header;
    fn get_rest_of_the_buffer(&self) -> &'b [u8];
}

pub trait ILoadCommand<'b>: Sized {
    fn parse(
        byte_order: ByteOrder,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError>;
    fn get_size(&self) -> u32;
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
    pub fn new(file: &'f TFile) -> Self {
        Self {
            file,
            count: 0,
            offset: 0,
            _a: Default::default(),
        }
    }
}

impl<'f, 'b, TFile: IFile<'b>, TLoadCommand: ILoadCommand<'b>> Iterator
    for LoadCommandIterator<'f, TFile, TLoadCommand>
{
    type Item = Result<TLoadCommand, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.file.get_header().load_commands_count {
            return None;
        };
        let Some(buffer) = &self.file.get_rest_of_the_buffer().get(
            self.offset..self.file.get_header().load_commands_size as usize,
        ) else {
            self.count = self.file.get_header().load_commands_count;
            return Some(Err(ParseError::EOF));
        };
        let command =
            match TLoadCommand::parse(self.file.get_byte_order(), buffer) {
                Ok(c) => c,
                Err(e) => {
                    self.count = self.file.get_header().load_commands_count;
                    return Some(Err(e));
                }
            };
        self.count += 1;
        self.offset += command.get_size() as usize;
        Some(Ok(command))
    }
}
