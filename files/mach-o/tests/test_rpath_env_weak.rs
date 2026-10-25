use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::LoadCommandContent,
        shared::load_command_content::{
            SharedLoadCommandContent, dylinker::Dylinker,
            link_library::LinkLibrary, rpath::RPath,
        },
    },
};

#[test]
pub fn test_rpath_env_weak_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_rpath_env_weak");
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
    assert_eq!(file.header.load_commands_count, 20);
    assert_eq!(file.header.load_commands_size, 936);
    assert_eq!(file.header.flags, 0x00200085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );

    // LC_LOAD_DYLINKER
    assert_eq!(load_commands[7].header.command_type, 0xE);
    assert_eq!(
        load_commands[7].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::Dylinker(
            Dylinker {
                name_offset: 12,
                rest_of_the_buffer: b"/usr/lib/dyld\0\0\0\0\0\0\0",
            }
        ))
    );

    // LC_LOAD_WEAK_DYLIB
    assert_eq!(load_commands[12].header.command_type, 0x80000018);
    assert_eq!(load_commands[12].header.command_size, 104);
    assert_eq!(
        load_commands[12].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::LinkLibrary(
            LinkLibrary {
                string_offset: 24,
                timestamp: 2,
                current_version: 0x13A30045,   // 5027.0.69
                compatible_version: 0x00960000, // 150.0.0
                rest_of_the_buffer: b"/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation\0\0\0",
            }
        ))
    );

    // LC_LOAD_DYLIB
    assert_eq!(load_commands[13].header.command_type, 0xC);
    assert_eq!(
        load_commands[13].content,
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

    // LC_RPATH
    assert_eq!(load_commands[14].header.command_type, 0x8000001C);
    assert_eq!(load_commands[14].header.command_size, 32);
    assert_eq!(
        load_commands[14].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::RPath(RPath {
            path_offset: 12,
            rest_of_the_buffer: b"@loader_path/lib\0\0\0\0",
        }))
    );

    // LC_RPATH
    assert_eq!(load_commands[15].header.command_type, 0x8000001C);
    assert_eq!(load_commands[15].header.command_size, 24);
    assert_eq!(
        load_commands[15].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::RPath(RPath {
            path_offset: 12,
            rest_of_the_buffer: b"/opt/x\0\0\0\0\0\0",
        }))
    );

    // LC_DYLD_ENVIRONMENT
    assert_eq!(load_commands[16].header.command_type, 0x27);
    assert_eq!(load_commands[16].header.command_size, 32);
    assert_eq!(
        load_commands[16].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::Dylinker(
            Dylinker {
                name_offset: 12,
                rest_of_the_buffer: b"DYLD_FOO=bar\0\0\0\0\0\0\0\0",
            }
        ))
    );
}
