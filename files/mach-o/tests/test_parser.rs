use macho::parser::{self, ByteOrder};

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
