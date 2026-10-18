use macho::parser::{self, ByteOrder};

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
