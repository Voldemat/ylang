use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::LoadCommandContent,
        shared::load_command_content::{
            SharedLoadCommandContent, build_version::BuildToolVersion,
        },
    },
};

#[test]
pub fn test_build_version_tools_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("samples/sample_build_version_tools");
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

    // LC_BUILD_VERSION
    assert_eq!(load_commands[15].header.command_type, 0x32);
    assert_eq!(load_commands[15].header.command_size, 40);
    let LoadCommandContent::Shared(SharedLoadCommandContent::BuildVersion(
        build_version,
    )) = &load_commands[15].content
    else {
        panic!("Expected build version load command");
    };
    assert_eq!(build_version.platform, 1);
    assert_eq!(build_version.minos, 0x000B0000); // 11.0
    assert_eq!(build_version.sdk, 0x000E0000); // 14.0
    assert_eq!(build_version.tools_count, 2);
    assert_eq!(build_version.tools_buffer.len(), 16);
    let tools = build_version
        .get_tools(ByteOrder::LittleEndian)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        tools,
        vec![
            BuildToolVersion {
                tool: 3,             // TOOL_LD
                version: 0x03E80000, // 1000.0
            },
            BuildToolVersion {
                tool: 1,             // TOOL_CLANG
                version: 0x05DC0100, // 1500.1
            },
        ]
    );
}
