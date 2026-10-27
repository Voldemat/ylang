use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b32, b64,
        shared::{
            load_command_content::{
                SharedLoadCommandContent,
                symbol_table::SymbolTable,
                symbol_table_information::{
                    IndirectSymbol, SymbolTableInformation,
                },
            },
            relocation_entry::RelocationEntry,
        },
    },
};

fn read_sample(name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples")
        .join(name);
    std::fs::read(path).unwrap()
}

#[test]
pub fn test_symbol_table_information_32_bit_big_endian() {
    let buffer = read_sample("sample_32_bit_big_endian");
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B32(
        file,
    )) = parser::MachOFile::parse(&buffer).unwrap()
    else {
        panic!("Expected 32 bit file");
    };
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // LC_DYSYMTAB
    assert_eq!(load_commands[9].header.command_type, 0xB);
    assert_eq!(load_commands[9].header.command_size, 80);
    let b32::load_command::LoadCommandContent::Shared(
        SharedLoadCommandContent::SymbolTableInformation(information),
    ) = &load_commands[9].content
    else {
        panic!("Expected symbol table information load command");
    };
    assert_eq!(
        *information,
        SymbolTableInformation {
            local_symbols_index: 0,
            local_symbols_count: 62,
            external_symbols_index: 62,
            external_symbols_count: 154,
            undefined_symbols_index: 216,
            undefined_symbols_count: 44,
            content_table_offset: 0,
            content_table_entries_count: 0,
            module_table_offset: 0,
            module_table_entries_count: 0,
            referenced_symbol_table_offset: 0,
            referenced_symbol_table_entries_count: 0,
            indirect_table_offset: 52496,
            indirect_table_entries_count: 154,
            external_relocation_table_offset: 52448,
            external_relocation_table_entries_count: 6,
            local_relocation_table_offset: 0,
            local_relocation_table_entries_count: 0,
        }
    );

    assert_eq!(
        information
            .get_indirections::<{ b32::symbol::Symbol::BUFFER_SIZE }, b32::header::Header>(
                ByteOrder::BigEndian,
                file.rest_of_the_buffer,
            )
            .unwrap()
            .map(|indirection| indirection.map(|i| i.index))
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![
            243, 232, 240, 249, 236, 245, 237, 241, 238, 256, 164, 229, 228,
            250, 218, 84, 66, 253, 86, 217, 257, 216, 246, 122, 90, 166, 134,
            178, 259, 168, 247, 227, 225, 224, 96, 244, 170, 144, 70, 172,
            106, 220, 78, 126, 128, 154, 116, 142, 88, 68, 176, 104, 258, 255,
            239, 124, 174, 108, 162, 254, 146, 252, 140, 251, 102, 160, 72,
            219, 226, 230, 248, 242, 235, 233, 196, 222, 191, 192, 194, 221,
            231, 234, 190, 193, 243, 232, 240, 249, 236, 245, 237, 241, 238,
            256, 164, 229, 228, 250, 218, 84, 66, 253, 86, 217, 257, 216, 246,
            122, 90, 166, 134, 178, 259, 168, 247, 227, 225, 224, 96, 244,
            170, 144, 70, 172, 106, 220, 78, 126, 128, 154, 116, 142, 88, 68,
            176, 104, 258, 255, 239, 124, 174, 108, 162, 254, 146, 252, 140,
            251, 102, 160, 72, 219, 226, 230,
        ]
    );

    assert_eq!(
        information
            .get_external_relocations::<{ b32::symbol::Symbol::BUFFER_SIZE }, b32::header::Header>(
                ByteOrder::BigEndian,
                file.rest_of_the_buffer,
            )
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![
            RelocationEntry {
                offset: 0xc2b8,
                options: 0xdd50,
            },
            RelocationEntry {
                offset: 0xc330,
                options: 0xdf50,
            },
            RelocationEntry {
                offset: 0xc318,
                options: 0xdf50,
            },
            RelocationEntry {
                offset: 0xc300,
                options: 0xdf50,
            },
            RelocationEntry {
                offset: 0xc2e8,
                options: 0xdf50,
            },
            RelocationEntry {
                offset: 0xc2d0,
                options: 0xdf50,
            },
        ]
    );

    assert_eq!(
        information
            .get_local_relocations::<{ b32::symbol::Symbol::BUFFER_SIZE }, b32::header::Header>(
                ByteOrder::BigEndian,
                file.rest_of_the_buffer,
            )
            .unwrap()
            .count(),
        0
    );
}

#[test]
pub fn test_symbol_table_information_no_fixup_chains() {
    let buffer = read_sample("sample_no_fixup_chains");
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B64(
        file,
    )) = parser::MachOFile::parse(&buffer).unwrap()
    else {
        panic!("Expected 64 bit file");
    };
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // LC_SYMTAB
    let b64::load_command::LoadCommandContent::Shared(
        SharedLoadCommandContent::SymbolTable(symbol_table),
    ) = &load_commands[6].content
    else {
        panic!("Expected symbol table load command");
    };
    assert_eq!(
        *symbol_table,
        SymbolTable {
            symbols_offset: 49328,
            symbols_count: 9,
            string_table_offset: 49504,
            string_table_size: 96,
        }
    );

    // LC_DYSYMTAB
    let b64::load_command::LoadCommandContent::Shared(
        SharedLoadCommandContent::SymbolTableInformation(information),
    ) = &load_commands[7].content
    else {
        panic!("Expected symbol table information load command");
    };
    assert_eq!(information.local_symbols_index, 0);
    assert_eq!(information.local_symbols_count, 1);
    assert_eq!(information.external_symbols_index, 1);
    assert_eq!(information.external_symbols_count, 4);
    assert_eq!(information.undefined_symbols_index, 5);
    assert_eq!(information.undefined_symbols_count, 4);

    assert_eq!(
        information
            .get_indirections::<{ b64::symbol::Symbol::BUFFER_SIZE }, b64::header::Header>(
                ByteOrder::LittleEndian,
                file.rest_of_the_buffer,
            )
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![
            // __TEXT,__stubs
            IndirectSymbol { index: 5 }, // _exit
            IndirectSymbol { index: 6 }, // _puts
            IndirectSymbol { index: 7 }, // _weak_fn
            // __DATA_CONST,__got
            IndirectSymbol { index: 8 }, // dyld_stub_binder
            IndirectSymbol { index: 7 }, // _weak_fn
            // __DATA,__la_symbol_ptr
            IndirectSymbol { index: 5 }, // _exit
            IndirectSymbol { index: 6 }, // _puts
            IndirectSymbol { index: 7 }, // _weak_fn
        ]
    );

    assert_eq!(
        information
            .get_external_relocations::<{ b64::symbol::Symbol::BUFFER_SIZE }, b64::header::Header>(
                ByteOrder::LittleEndian,
                file.rest_of_the_buffer,
            )
            .unwrap()
            .count(),
        0
    );
    assert_eq!(
        information
            .get_local_relocations::<{ b64::symbol::Symbol::BUFFER_SIZE }, b64::header::Header>(
                ByteOrder::LittleEndian,
                file.rest_of_the_buffer,
            )
            .unwrap()
            .count(),
        0
    );
}
