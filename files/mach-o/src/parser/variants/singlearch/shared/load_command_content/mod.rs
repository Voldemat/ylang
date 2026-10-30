pub mod build_version;
pub mod compressed_link_edit_table;
pub mod dylinker;
pub mod entry_point;
pub mod fileset_entry;
pub mod link_library;
pub mod linked_it;
pub mod linker_option;
pub mod minimum_os_version;
pub mod note;
pub mod prebind_cksum;
pub mod rpath;
pub mod source_version;
pub mod symbol_table;
pub mod symbol_table_information;
pub mod two_level_hints;
pub mod uuid;

use super::parse_error::ParseError;
use crate::{
    parser::ByteOrder,
    shared::{IDynamicStruct, parse_dynamic_struct, parse_static_struct},
};

#[derive(Debug, PartialEq, Eq)]
pub enum SharedLoadCommandContent<'b> {
    LinkLibrary(link_library::LinkLibrary<'b>),
    SymbolTable(symbol_table::SymbolTable),
    SymbolTableInformation(symbol_table_information::SymbolTableInformation),
    CompressedLinkEditTable(
        compressed_link_edit_table::CompressedLinkEditTable,
    ),
    MainEntryPoint(entry_point::EntryPoint),
    UUIDNumber(uuid::UUIDNumber<'b>),
    MinimumOSVersion(minimum_os_version::MinimumOSVersion),
    LinkedIt(linked_it::LinkedIt),
    Dylinker(dylinker::Dylinker<'b>),
    SourceVersion(source_version::SourceVersion),
    TwoLevelHints(two_level_hints::TwoLevelHints),
    Thread(&'b [u8]),
    UnixThread(&'b [u8]),
    SubClient(&'b [u8]),
    Note(note::Note<'b>),
    LinkerOption(linker_option::LinkerOption<'b>),
    BuildVersion(build_version::BuildVersion<'b>),
    FilesetEntry(fileset_entry::FilesetEntry<'b>),
    RPath(rpath::RPath<'b>),
    PrebindCksum(prebind_cksum::PrebindCksum),
}

impl<'b> SharedLoadCommandContent<'b> {
    pub fn get_size(&self) -> u32 {
        match self {
            Self::LinkLibrary(c) => c.get_size(),
            Self::SymbolTable(_) => {
                symbol_table::SymbolTable::BUFFER_SIZE as u32
            }
            Self::SymbolTableInformation(_) => {
                symbol_table_information::SymbolTableInformation::BUFFER_SIZE
                    as u32
            }
            Self::CompressedLinkEditTable(_) => {
                compressed_link_edit_table::CompressedLinkEditTable::BUFFER_SIZE
                    as u32
            }
            Self::MainEntryPoint(_) => {
                entry_point::EntryPoint::BUFFER_SIZE as u32
            }
            Self::UUIDNumber(_) => uuid::UUIDNumber::BUFFER_SIZE as u32,
            Self::MinimumOSVersion(_) => {
                minimum_os_version::MinimumOSVersion::BUFFER_SIZE as u32
            }
            Self::LinkedIt(_) => linked_it::LinkedIt::BUFFER_SIZE as u32,
            Self::Dylinker(c) => c.get_size(),
            Self::SourceVersion(_) => {
                source_version::SourceVersion::BUFFER_SIZE as u32
            }
            Self::TwoLevelHints(_) => {
                two_level_hints::TwoLevelHints::BUFFER_SIZE as u32
            }
            Self::Thread(c) => c.len() as u32,
            Self::UnixThread(c) => c.len() as u32,
            Self::SubClient(c) => c.len() as u32,
            Self::Note(_) => note::Note::BUFFER_SIZE as u32,
            Self::LinkerOption(c) => c.get_size(),
            Self::BuildVersion(c) => c.get_size(),
            Self::FilesetEntry(c) => c.get_size(),
            Self::RPath(c) => c.get_size(),
            Self::PrebindCksum(_) => {
                prebind_cksum::PrebindCksum::BUFFER_SIZE as u32
            }
        }
    }

    pub fn parse(
        byte_order: ByteOrder,
        command_type: u32,
        buffer: &'b [u8],
    ) -> Result<Self, ParseError> {
        match command_type {
            0x0000000C | 0x0000000D | 0x80000018 | 0x8000001F | 0x8000023 | 0x20 => {
                parse_dynamic_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::LinkLibrary)
            }
            prebind_cksum::PrebindCksum::COMMAND_TYPE => {
                parse_static_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::PrebindCksum)
            },
            build_version::BuildVersion::COMMAND_TYPE =>
                parse_dynamic_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::BuildVersion),
            fileset_entry::FilesetEntry::COMMAND_TYPE =>
                parse_dynamic_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::FilesetEntry),
            rpath::RPath::COMMAND_TYPE =>
                parse_dynamic_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::RPath),
            45 => parse_dynamic_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::LinkerOption),
            note::Note::COMMAND_TYPE =>
                parse_static_struct(byte_order, buffer)
                    .ok_or(ParseError::EOF)
                    .map(Self::Note),
            4 => Ok(Self::Thread(buffer)),
            5 => Ok(Self::UnixThread(buffer)),
            20 => Ok(Self::SubClient(buffer)),
            22 => parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::TwoLevelHints),
            42 => parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::SourceVersion),
            14 | 15 | 39 | 0x12 | 0x13 | 0x15 | 0x39 =>
                parse_dynamic_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::Dylinker),
            0x80000034 | 0x3 | 0x80000033 | 43 | 41 | 30 | 38 | 29 | 0x2E | 0x36 | 0x37 | 0x38 | 0x3A =>
                parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::LinkedIt),
            0x00000002 =>
                parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::SymbolTable),
            0x0000000B => parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::SymbolTableInformation),
            compressed_link_edit_table::CompressedLinkEditTable::COMMAND_TYPE |
            compressed_link_edit_table::CompressedLinkEditTable::COMMAND_TYPE_ONLY
                => parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::CompressedLinkEditTable),
            entry_point::EntryPoint::COMMAND_TYPE =>
                parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::MainEntryPoint),
            uuid::UUIDNumber::COMMAND_TYPE =>
                parse_static_struct(byte_order, buffer)
                .ok_or(ParseError::EOF)
                .map(Self::UUIDNumber),
            minimum_os_version::MinimumOSVersion::COMMAND_TYPE_MACOSX |
                minimum_os_version::MinimumOSVersion::COMMAND_TYPE_WATCHOS |
                minimum_os_version::MinimumOSVersion::COMMAND_TYPE_TVOS |
                minimum_os_version::MinimumOSVersion::COMMAND_TYPE_IPHONEOS =>
                parse_static_struct(byte_order, buffer).ok_or(ParseError::EOF)
                    .map(Self::MinimumOSVersion),
            unknown_command_type => {
                Err(ParseError::UnknownLoadCommandType(unknown_command_type))
            }
        }
    }
}
