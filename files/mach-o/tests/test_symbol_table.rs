use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::{
            header::Header, load_command::LoadCommandContent, symbol::Symbol,
        },
        shared::load_command_content::SharedLoadCommandContent,
    },
};

#[test]
pub fn test_symbol_table() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample_64_bit_little_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::LittleEndian);
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B64(
        file,
    )) = general_file
    else {
        panic!("Expected 64 bit file");
    };
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // LC_SYMTAB
    assert_eq!(load_commands[5].header.command_type, 0x2);
    assert_eq!(load_commands[5].header.command_size, 24);
    let LoadCommandContent::Shared(SharedLoadCommandContent::SymbolTable(
        symbol_table,
    )) = &load_commands[5].content
    else {
        panic!("Expected symbol table load command");
    };
    assert_eq!(symbol_table.symbols_offset, 16496);
    assert_eq!(symbol_table.symbols_count, 2);
    assert_eq!(symbol_table.string_table_offset, 16528);
    assert_eq!(symbol_table.string_table_size, 32);

    let symbols = symbol_table
        .get_symbols::<{ Symbol::BUFFER_SIZE }, Header>(
            ByteOrder::LittleEndian,
            file.rest_of_the_buffer,
        )
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        symbols,
        vec![
            // __mh_execute_header
            Symbol {
                name_offset: 2,
                symbol_type: 0x0F, // N_SECT | N_EXT
                section_number: 1,
                data_info: 0x0010, // REFERENCED_DYNAMICALLY
                symbol_address: 0x100000000,
            },
            // _main
            Symbol {
                name_offset: 22,
                symbol_type: 0x0F, // N_SECT | N_EXT
                section_number: 1,
                data_info: 0,
                symbol_address: 0x100000328,
            },
        ]
    );

    let strings = symbol_table
        .get_strings::<{ Symbol::BUFFER_SIZE }, Header>(file.rest_of_the_buffer)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        strings,
        vec![c" ", c"__mh_execute_header", c"_main", c"", c"", c"", c""]
    );
}
