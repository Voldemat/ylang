use macho::{
    parser::{self, ByteOrder},
    shared::AsciiString,
};

#[test]
pub fn test_64_bit_little_endian_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_64_bit_little_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::LittleEndian);
    let parser::MachOFile::B64(file) = general_file else {
        panic!("Expected 64 bit file");
    };
    assert_eq!(file.byte_order, ByteOrder::LittleEndian);
    assert_eq!(file.header.cpu_type, 16777228);
    assert_eq!(file.header.cpu_subtype, 0);
    assert_eq!(file.header.file_type, 2);
    assert_eq!(file.header.load_commands_count, 16);
    assert_eq!(file.header.load_commands_size, 744);
    assert_eq!(file.header.flags, 0x00200085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );
    let parser::variants::singlearch::b64::LoadCommand::Segment(first_command) =
        &load_commands[0]
    else {
        panic!("Expected first command to be segment load command");
    };
    assert_eq!(
        first_command
            .parse_sections(ByteOrder::LittleEndian)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        0
    );

    let parser::variants::singlearch::b64::LoadCommand::Segment(second_command) =
        &load_commands[1]
    else {
        panic!("Expected second command to be segment load command");
    };
    let second_command_sections = second_command
        .parse_sections(ByteOrder::LittleEndian)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(second_command_sections.len(), 2);
    assert_eq!(second_command_sections[0].section_name.as_ref(), "__text");
    assert_eq!(second_command_sections[0].segment_name.as_ref(), "__TEXT");
    assert_eq!(second_command_sections[0].address, 4294968104);
    assert_eq!(second_command_sections[0].size, 20);
    assert_eq!(second_command_sections[0].file_offset, 808);
    assert_eq!(second_command_sections[0].alignment, 2);
    assert_eq!(second_command_sections[0].relocation_entries_file_offset, 0);
    assert_eq!(second_command_sections[0].relocation_entries_count, 0);
    assert_eq!(second_command_sections[0].flags, 2147484672);
    assert_eq!(second_command_sections[1].section_name.as_ref(), "__unwind_info");
    assert_eq!(second_command_sections[1].segment_name.as_ref(), "__TEXT");
    assert_eq!(second_command_sections[1].address, 4294968124);
    assert_eq!(second_command_sections[1].size, 88);
    assert_eq!(second_command_sections[1].file_offset, 828);
    assert_eq!(second_command_sections[1].alignment, 2);
    assert_eq!(second_command_sections[1].relocation_entries_file_offset, 0);
    assert_eq!(second_command_sections[1].relocation_entries_count, 0);
    assert_eq!(second_command_sections[1].flags, 0);
    let parser::variants::singlearch::b64::LoadCommand::Segment(third_command) =
        &load_commands[2]
    else {
        panic!("Expected third command to be segment load command");
    };
    assert_eq!(
        third_command
            .parse_sections(ByteOrder::LittleEndian)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        0
    );
}

#[test]
pub fn test_32_bit_little_endian_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_32_bit_little_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::LittleEndian);
    let parser::MachOFile::B32(file) = general_file else {
        panic!("Expected 32 bit file");
    };
    assert_eq!(file.byte_order, ByteOrder::LittleEndian);
    assert_eq!(file.header.cpu_type, 7);
    assert_eq!(file.header.cpu_subtype, 3);
    assert_eq!(file.header.file_type, 1);
    assert_eq!(file.header.load_commands_count, 4);
    assert_eq!(file.header.load_commands_size, 388);
    assert_eq!(file.header.flags, 0x00002000);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );
}

#[test]
pub fn test_32_bit_big_endian_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_32_bit_big_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::BigEndian);
    let parser::MachOFile::B32(file) = general_file else {
        panic!("Expected 32 bit file");
    };
    assert_eq!(file.byte_order, ByteOrder::BigEndian);
    assert_eq!(file.header.cpu_type, 18);
    assert_eq!(file.header.cpu_subtype, 0);
    assert_eq!(file.header.file_type, 2);
    assert_eq!(file.header.load_commands_count, 13);
    assert_eq!(file.header.load_commands_size, 2020);
    assert_eq!(file.header.flags, 0x0001a085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );
}

#[test]
pub fn test_multiarch_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_multiarch");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::BigEndian);
    let parser::MachOFile::MultiArch(file) = general_file else {
        panic!("Expected multiarch file");
    };
    assert_eq!(file.header.binaries_count, 2);
}
