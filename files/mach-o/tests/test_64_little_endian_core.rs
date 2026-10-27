use macho::parser::{self, ByteOrder};

#[test]
pub fn test_64_bit_little_endian_core() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/64_little_endian_core");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::LittleEndian);
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B64(
        file,
    )) = general_file
    else {
        panic!("Expected 64 bit file");
    };
    assert_eq!(file.byte_order, ByteOrder::LittleEndian);
    assert_eq!(file.header.cpu_type, 16777228);
    assert_eq!(file.header.cpu_subtype, 0);
    assert_eq!(file.header.file_type, 4);
    assert_eq!(file.header.load_commands_count, 26);
    assert_eq!(file.header.load_commands_size, 2016);
    assert_eq!(file.header.flags, 0);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );
    let parser::variants::singlearch::b64::load_command::LoadCommandContent::Shared(
        parser::variants::singlearch::shared::load_command_content::SharedLoadCommandContent::Note(first_command)) = &load_commands[0].content
    else {
        panic!("Expected first command to be note load command");
    };
    let note_payload = first_command
        .get_payload::<parser::variants::singlearch::b64::header::Header>(
            file.rest_of_the_buffer,
        )
        .unwrap();
    assert_eq!(
        note_payload,
        [4, 0, 0, 0, 47, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0]
    );
}
