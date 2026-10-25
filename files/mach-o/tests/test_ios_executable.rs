use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::{
            LoadCommandContent, encryption_info::EncryptionInfo,
        },
        shared::load_command_content::{
            SharedLoadCommandContent, build_version::BuildToolVersion,
        },
    },
};

#[test]
pub fn test_ios_executable_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_ios_executable");
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
    assert_eq!(file.header.load_commands_count, 16);
    assert_eq!(file.header.load_commands_size, 752);
    assert_eq!(file.header.flags, 0x00200085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );

    // LC_BUILD_VERSION for iOS
    assert_eq!(load_commands[9].header.command_type, 0x32);
    assert_eq!(load_commands[9].header.command_size, 32);
    let LoadCommandContent::Shared(SharedLoadCommandContent::BuildVersion(
        build_version,
    )) = &load_commands[9].content
    else {
        panic!("Expected build version load command");
    };
    assert_eq!(build_version.platform, 2); // PLATFORM_IOS
    assert_eq!(build_version.minos, 0x000F0000); // 15.0
    assert_eq!(build_version.sdk, 0x001B0100); // 27.1
    assert_eq!(build_version.tools_count, 1);
    assert_eq!(
        build_version
            .get_tools(ByteOrder::LittleEndian)
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![BuildToolVersion {
            tool: 3,             // TOOL_LD
            version: 0x699D0100, // 27037.1
        }]
    );

    // LC_ENCRYPTION_INFO_64
    assert_eq!(load_commands[12].header.command_type, 0x2C);
    assert_eq!(load_commands[12].header.command_size, 24);
    assert_eq!(
        load_commands[12].content,
        LoadCommandContent::EncryptionInfo(EncryptionInfo {
            file_offset: 16384,
            size: 16384,
            encryption_system: 0,
            _padding: 0,
        })
    );
}
