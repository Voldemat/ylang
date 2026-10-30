use macho::parser::{self, ByteOrder};

#[test]
pub fn test_64_bit_little_endian_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/dylib_64_bit_little_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::BigEndian);
    println!("{:?}", general_file);
    let parser::MachOFile::MultiArch(parser::variants::multiarch::File::B32(
        file,
    )) = general_file
    else {
        panic!("Expected multi arch file");
    };
    assert_eq!(file.header.binaries_count, 2);
    let entries = file
        .get_entry_headers()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        entries[0],
        parser::variants::multiarch::b32::EntryHeader {
            cpu_type: 16777223,
            cpu_subtype: 3,
            file_offset: 16384,
            size: 22544,
            section_alignment: 14
        }
    );
    assert_eq!(
        entries[1],
        parser::variants::multiarch::b32::EntryHeader {
            cpu_type: 16777228,
            cpu_subtype: 2147483650,
            file_offset: 49152,
            size: 51360,
            section_alignment: 14
        }
    );

    let first_entry_buffer = &buffer[entries[0].file_offset as usize
        ..entries[0].file_offset as usize + entries[0].size as usize];
    let parser::variants::singlearch::File::B64(singlearch_file) =
        parser::variants::singlearch::File::parse(first_entry_buffer).unwrap()
    else {
        panic!("Expected 64 bit file");
    };
    singlearch_file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
}
