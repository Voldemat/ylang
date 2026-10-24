pub mod build_version;
pub mod compressed_link_edit_table;
pub mod dylinker;
pub mod entry_point;
pub mod header;
pub mod link_library;
pub mod linked_it;
pub mod linker_option;
pub mod minimum_os_version;
pub mod note;
pub mod source_version;
pub mod symbol_table;
pub mod symbol_table_information;
pub mod two_level_hints;
pub mod uuid;

use super::parse_error::ParseError;
use crate::{parser::ByteOrder, shared::IStaticStruct};

#[derive(Debug, PartialEq, Eq)]
pub enum SharedLoadCommand<'b> {
    LinkLibrary(link_library::LoadCommandLinkLibrary<'b>),
    SymbolTable(symbol_table::LoadCommandSymbolTable),
    SymbolTableInformation(
        symbol_table_information::LoadCommandSymbolTableInformation,
    ),
    CompressedLinkEditTable(
        compressed_link_edit_table::LoadCommandCompressedLinkEditTable,
    ),
    MainEntryPoint(entry_point::LoadCommandEntryPoint),
    UUIDNumber(uuid::LoadCommandUUIDNumber),
    MinimumOSVersion(minimum_os_version::LoadCommandMinimumOSVersion),
    LinkedIt(linked_it::LoadCommandLinkedIt),
    Dylinker(dylinker::LoadCommandDylinker<'b>),
    SourceVersion(source_version::LoadCommandSourceVersion),
    TwoLevelHints(two_level_hints::LoadCommandTwoLevelHints),
    Thread(&'b [u8]),
    UnixThread(&'b [u8]),
    SubClient(&'b [u8]),
    Note(note::LoadCommandNote),
    LinkerOption(linker_option::LoadCommandLinkerOption<'b>),
    BuildVersion(build_version::LoadCommandBuildVersion<'b>),
}

impl<'b> SharedLoadCommand<'b> {
    pub fn get_size(&self) -> u32 {
        match self {
            Self::LinkLibrary(c) => c.get_size(),
            Self::SymbolTable(_) => {
                symbol_table::LoadCommandSymbolTable::BUFFER_SIZE as u32
            }
            Self::SymbolTableInformation(_) => {
                symbol_table_information::LoadCommandSymbolTableInformation::BUFFER_SIZE as u32
            }
            Self::CompressedLinkEditTable(_) => {
                compressed_link_edit_table::LoadCommandCompressedLinkEditTable::BUFFER_SIZE as u32
            }
            Self::MainEntryPoint(_) => {
                entry_point::LoadCommandEntryPoint::BUFFER_SIZE as u32
            }
            Self::UUIDNumber(_) => uuid::LoadCommandUUIDNumber::BUFFER_SIZE as u32,
            Self::MinimumOSVersion(_) => {
                minimum_os_version::LoadCommandMinimumOSVersion::BUFFER_SIZE as u32
            }
            Self::LinkedIt(_) => linked_it::LoadCommandLinkedIt::BUFFER_SIZE as u32,
            Self::Dylinker(c) => c.get_size(),
            Self::SourceVersion(_) => {
                source_version::LoadCommandSourceVersion::BUFFER_SIZE as u32
            }
            Self::TwoLevelHints(_) => {
                two_level_hints::LoadCommandTwoLevelHints::BUFFER_SIZE as u32
            }
            Self::Thread(c) => c.len() as u32,
            Self::UnixThread(c) => c.len() as u32,
            Self::SubClient(c) => c.len() as u32,
            Self::Note(_) => note::LoadCommandNote::BUFFER_SIZE as u32,
            Self::LinkerOption(c) => c.get_size(),
            Self::BuildVersion(c) => c.get_size(),
        }
    }

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match command_type {
            0x0000000C | 0x0000000D | 0x00000018 => {
                link_library::LoadCommandLinkLibrary::parse(byte_order, buffer)
                    .map(Self::LinkLibrary)
                    .map_err(|e| e.into())
            }
            45 => Ok(Self::LinkerOption(
                linker_option::LoadCommandLinkerOption::new(buffer)
            )),
            0x19 => buffer.try_into()
                .map(|command_buffer| note::LoadCommandNote::new(
                    byte_order,
                    command_buffer
                ))
                .map(Self::Note)
                .map_err(|_| ParseError::EOF),
            4 => Ok(Self::Thread(buffer)),
            5 => Ok(Self::UnixThread(buffer)),
            20 => Ok(Self::SubClient(buffer)),
            22 => buffer
                .try_into()
                .map_err(|_| ParseError::EOF)
                .map(|command_buffer| {
                    two_level_hints::LoadCommandTwoLevelHints::new(byte_order, command_buffer)
                })
                .map(Self::TwoLevelHints),
            42 => buffer
                .try_into()
                .map_err(|_| ParseError::EOF)
                .map(|command_buffer| {
                    source_version::LoadCommandSourceVersion::new(byte_order, command_buffer)
                })
                .map(Self::SourceVersion),
            14 | 15 | 39 => Ok(Self::Dylinker(dylinker::LoadCommandDylinker::new(
                command_type,
                buffer,
            ))),
            0x80000034 | 0x80000033 | 43 | 41 | 30 | 38 | 29 => buffer
                .try_into()
                .map_err(|_| ParseError::EOF)
                .map(|command_buffer| {
                    linked_it::LoadCommandLinkedIt::new(
                        byte_order,
                        command_type,
                        command_buffer,
                    )
                })
                .map(Self::LinkedIt),
            0x00000002 => buffer
                .get(0..symbol_table::LoadCommandSymbolTable::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    symbol_table::LoadCommandSymbolTable::parse(byte_order, command_buffer)
                })
                .map(Self::SymbolTable),
            0x0000000B => buffer
                .get(0..symbol_table_information::LoadCommandSymbolTableInformation::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    symbol_table_information::LoadCommandSymbolTableInformation::parse(
                        byte_order,
                        command_buffer,
                    )
                })
                .map(Self::SymbolTableInformation),
            0x00000022 => buffer
                .get(0..compressed_link_edit_table::LoadCommandCompressedLinkEditTable::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    compressed_link_edit_table::LoadCommandCompressedLinkEditTable::parse(
                        byte_order,
                        command_buffer,
                    )
                })
                .map(Self::CompressedLinkEditTable),
            0x00000028 | 0x80000028 => buffer
                .get(0..entry_point::LoadCommandEntryPoint::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    entry_point::LoadCommandEntryPoint::new(byte_order, command_buffer)
                })
                .map(Self::MainEntryPoint),
            0x0000001B => buffer
                .get(0..uuid::LoadCommandUUIDNumber::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    uuid::LoadCommandUUIDNumber::parse(command_buffer)
                })
                .map(Self::UUIDNumber),
            0x00000032 => buffer
                .get(0..minimum_os_version::LoadCommandMinimumOSVersion::BUFFER_SIZE)
                .ok_or(ParseError::EOF)
                .and_then(|b| b.try_into().map_err(|_| ParseError::EOF))
                .map(|command_buffer| {
                    minimum_os_version::LoadCommandMinimumOSVersion::new(
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
