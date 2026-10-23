use macho::parser::{self, ByteOrder};

#[test]
pub fn test_32_bit_big_endian_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_32_bit_big_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::BigEndian);
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B32(
        file,
    )) = general_file
    else {
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

    let parser::variants::singlearch::b32::LoadCommand::Segment(segment1) =
        &load_commands[0]
    else {
        panic!("Expected load segment command as first command");
    };
    assert_eq!(segment1.segment_name.as_ref(), "__PAGEZERO\0\0\0\0\0\0");
    assert_eq!(segment1.address, 0);
    assert_eq!(segment1.address_size, 4096);
    assert_eq!(segment1.file_offset, 0);
    assert_eq!(segment1.size, 0);
    assert_eq!(segment1.maximum_virtual_memory_protections, 0);
    assert_eq!(segment1.initial_virtual_memory_protections, 0);
    assert_eq!(segment1.sections_count, 0);
    assert_eq!(segment1.flags, 4);
    assert_eq!(segment1.sections_buffer.len(), 0);
}
