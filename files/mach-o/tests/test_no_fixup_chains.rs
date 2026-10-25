use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::LoadCommandContent,
        shared::load_command_content::{
            SharedLoadCommandContent,
            compressed_link_edit_table::CompressedLinkEditTable,
        },
    },
};

#[test]
pub fn test_no_fixup_chains_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_no_fixup_chains");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::LittleEndian);
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B64(
        file,
    )) = general_file
    else {
        panic!("Expected 64 bit file");
    };
    assert_eq!(file.header.cpu_type, 16777228);
    assert_eq!(file.header.cpu_subtype, 0);
    assert_eq!(file.header.file_type, 2);
    assert_eq!(file.header.load_commands_count, 17);
    assert_eq!(file.header.load_commands_size, 1384);
    assert_eq!(file.header.flags, 0x00200085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );

    // LC_DYLD_INFO_ONLY
    assert_eq!(load_commands[5].header.command_type, 0x80000022);
    assert_eq!(load_commands[5].header.command_size, 48);
    assert_eq!(
        load_commands[5].content,
        LoadCommandContent::Shared(
            SharedLoadCommandContent::CompressedLinkEditTable(
                CompressedLinkEditTable {
                    rebase_file_offset: 49152,
                    rebase_size: 8,
                    bind_file_offset: 49160,
                    bind_size: 48,
                    weak_bind_file_offset: 0,
                    weak_bind_size: 0,
                    lazy_bind_file_offset: 49208,
                    lazy_bind_size: 40,
                    export_file_offset: 49248,
                    export_size: 72,
                }
            )
        )
    );
}
