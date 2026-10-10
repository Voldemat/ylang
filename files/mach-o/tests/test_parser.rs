use macho::parser;

#[test]
pub fn test_parser() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("test_bin");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let file =
        parser::MachOFile::parse(parser::ByteOrder::host_order(), &buffer)
            .unwrap();
    assert_eq!(file, parser::MachOFile::B64(parser::MachO64BitFile {}));
}
