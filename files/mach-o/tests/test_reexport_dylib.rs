use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::LoadCommandContent,
        shared::load_command_content::{
            SharedLoadCommandContent, link_library::LinkLibrary,
        },
    },
};

#[test]
pub fn test_reexport_dylib_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_reexport_dylib");
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
    assert_eq!(file.header.file_type, 6);
    assert_eq!(file.header.load_commands_count, 15);
    assert_eq!(file.header.load_commands_size, 712);
    assert_eq!(file.header.flags, 0x00000085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );

    // LC_ID_DYLIB
    assert_eq!(load_commands[2].header.command_type, 0xD);
    assert_eq!(
        load_commands[2].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::LinkLibrary(
            LinkLibrary {
                string_offset: 24,
                timestamp: 1,
                current_version: 0,
                compatible_version: 0,
                rest_of_the_buffer: b"@rpath/liba.dylib\0\0\0\0\0\0\0",
            }
        ))
    );

    // LC_REEXPORT_DYLIB
    assert_eq!(load_commands[10].header.command_type, 0x8000001F);
    assert_eq!(load_commands[10].header.command_size, 48);

    // LC_LOAD_DYLIB
    assert_eq!(load_commands[11].header.command_type, 0xC);
    assert_eq!(
        load_commands[11].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::LinkLibrary(
            LinkLibrary {
                string_offset: 24,
                timestamp: 2,
                current_version: 0x054F0000, // 1359.0.0
                compatible_version: 0x00010000, // 1.0.0
                rest_of_the_buffer: b"/usr/lib/libSystem.B.dylib\0\0\0\0\0\0",
            }
        ))
    );
}
