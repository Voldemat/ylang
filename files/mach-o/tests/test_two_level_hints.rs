use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b32::{
            header::Header, load_command::LoadCommandContent, symbol::Symbol,
        },
        shared::load_command_content::{
            SharedLoadCommandContent, two_level_hints::TwoLevelHint,
        },
    },
};

#[test]
pub fn test_two_level_hints() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample_32_bit_big_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::BigEndian);
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B32(
        file,
    )) = general_file
    else {
        panic!("Expected 32 bit file");
    };
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // LC_TWOLEVEL_HINTS
    assert_eq!(load_commands[10].header.command_type, 0x16);
    assert_eq!(load_commands[10].header.command_size, 16);
    let LoadCommandContent::Shared(SharedLoadCommandContent::TwoLevelHints(
        two_level_hints,
    )) = &load_commands[10].content
    else {
        panic!("Expected two level hints load command");
    };
    assert_eq!(two_level_hints.file_offset, 52272);
    assert_eq!(two_level_hints.hints_count, 44);

    let hints = two_level_hints
        .get_hints::<{ Symbol::BUFFER_SIZE }, Header>(
            ByteOrder::BigEndian,
            file.rest_of_the_buffer,
        )
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let expected: [u32; 44] = [
        0x0000000D, 0x000012B1, 0x000012B5, 0x000012CD, 0x000012DF, 0x00001437,
        0x00001448, 0x0000159C, 0x00001686, 0x0000168E, 0x00001692, 0x00001698,
        0x0000169F, 0x000016B3, 0x0000014B, 0x000016CD, 0x00000185, 0x00000186,
        0x000001CA, 0x0000020C, 0x00000238, 0x0000023D, 0x00000430, 0x000004A0,
        0x0000050B, 0x0000050C, 0x00000539, 0x00000551, 0x000005B4, 0x000005C2,
        0x000005D9, 0x000005F6, 0x00000834, 0x00000883, 0x000008D1, 0x000008F0,
        0x000009AC, 0x000009DF, 0x00000A68, 0x00000B86, 0x00000BAE, 0x00000BB3,
        0x00000BBE, 0x00000D0B,
    ];
    assert_eq!(
        hints,
        expected.into_iter().map(TwoLevelHint).collect::<Vec<_>>()
    );
}
