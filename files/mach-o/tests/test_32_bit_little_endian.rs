use macho::parser::{self, ByteOrder};

#[test]
pub fn test_32_bit_little_endian_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_32_bit_little_endian");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::LittleEndian);
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B32(
        file,
    )) = general_file
    else {
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
    let parser::variants::singlearch::b32::load_command::LoadCommandContent::Segment(
        segment,
    ) = &load_commands[0].content
    else {
        panic!("Expected load segment command as first command");
    };
    assert_eq!(
        segment.segment_name.as_ref(),
        "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
    );
    assert_eq!(segment.sections_count, 3);
    let sections = segment
        .parse_sections(ByteOrder::LittleEndian)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(sections.len(), 3);
    assert_eq!(
        sections[0].section_name.as_ref(),
        "__text\0\0\0\0\0\0\0\0\0\0"
    );
    assert_eq!(
        sections[0].segment_name.as_ref(),
        "__TEXT\0\0\0\0\0\0\0\0\0\0"
    );
    assert_eq!(sections[0].address, 0);
    assert_eq!(sections[0].size, 18);
    assert_eq!(sections[0].file_offset, 416);
    assert_eq!(sections[0].alignment, 4);
    assert_eq!(sections[0].relocation_entries_file_offset, 0);
    assert_eq!(sections[0].relocation_entries_count, 0);
    assert_eq!(sections[0].flags, 2147484672);
    assert_eq!(sections[0]._reserved1, 0);
    assert_eq!(sections[0]._reserved2, 0);

    assert_eq!(sections[1].section_name.as_ref(), "__compact_unwind");
    assert_eq!(
        sections[1].segment_name.as_ref(),
        "__LD\0\0\0\0\0\0\0\0\0\0\0\0"
    );
    assert_eq!(sections[1].address, 20);
    assert_eq!(sections[1].size, 20);
    assert_eq!(sections[1].file_offset, 436);
    assert_eq!(sections[1].alignment, 2);
    assert_eq!(sections[1].relocation_entries_file_offset, 508);
    assert_eq!(sections[1].relocation_entries_count, 1);
    assert_eq!(sections[1].flags, 33554432);
    assert_eq!(sections[1]._reserved1, 0);
    assert_eq!(sections[1]._reserved2, 0);

    assert_eq!(sections[2].section_name.as_ref(), "__eh_frame\0\0\0\0\0\0");
    assert_eq!(
        sections[2].segment_name.as_ref(),
        "__TEXT\0\0\0\0\0\0\0\0\0\0"
    );
    assert_eq!(sections[2].address, 40);
    assert_eq!(sections[2].size, 52);
    assert_eq!(sections[2].file_offset, 456);
    assert_eq!(sections[2].alignment, 2);
    assert_eq!(sections[2].relocation_entries_file_offset, 0);
    assert_eq!(sections[2].relocation_entries_count, 0);
    assert_eq!(sections[2].flags, 1744830475);
    assert_eq!(sections[2]._reserved1, 0);
    assert_eq!(sections[2]._reserved2, 0);
}
