use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::LoadCommandContent,
        shared::load_command_content::SharedLoadCommandContent,
    },
};

#[test]
pub fn test_linker_option_object_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_linker_option_object");
    let buffer: Vec<u8> = std::fs::read(path).unwrap();
    let general_file = parser::MachOFile::parse(&buffer).unwrap();
    assert_eq!(general_file.get_byte_order(), ByteOrder::LittleEndian);
    let parser::MachOFile::Singlearch(parser::variants::singlearch::File::B64(
        file,
    )) = general_file
    else {
        panic!("Expected 64 bit file");
    };
    assert_eq!(file.header.cpu_type, 16777223);
    assert_eq!(file.header.cpu_subtype, 3);
    assert_eq!(file.header.file_type, 1);
    assert_eq!(file.header.load_commands_count, 6);
    assert_eq!(file.header.load_commands_size, 336);
    assert_eq!(file.header.flags, 0);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );

    // LC_BUILD_VERSION without tools
    let LoadCommandContent::Shared(SharedLoadCommandContent::BuildVersion(
        build_version,
    )) = &load_commands[1].content
    else {
        panic!("Expected build version load command");
    };
    assert_eq!(build_version.platform, 1);
    assert_eq!(build_version.minos, 0x001A0500); // 26.5
    assert_eq!(build_version.sdk, 0);
    assert_eq!(build_version.tools_count, 0);
    assert_eq!(
        build_version
            .get_tools(ByteOrder::LittleEndian)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        0
    );

    // LC_LINKER_OPTION with a single string
    assert_eq!(load_commands[4].header.command_type, 0x2D);
    assert_eq!(load_commands[4].header.command_size, 16);
    let LoadCommandContent::Shared(SharedLoadCommandContent::LinkerOption(
        linker_option,
    )) = &load_commands[4].content
    else {
        panic!("Expected linker option load command");
    };
    assert_eq!(linker_option.count, 1);
    assert_eq!(linker_option.strings_buffer, b"-lz\0");
    assert_eq!(
        linker_option
            .get_strings()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![c"-lz"]
    );

    // LC_LINKER_OPTION with two strings
    assert_eq!(load_commands[5].header.command_type, 0x2D);
    assert_eq!(load_commands[5].header.command_size, 40);
    let LoadCommandContent::Shared(SharedLoadCommandContent::LinkerOption(
        linker_option,
    )) = &load_commands[5].content
    else {
        panic!("Expected linker option load command");
    };
    assert_eq!(linker_option.count, 2);
    assert_eq!(
        linker_option.strings_buffer,
        b"-framework\0Foundation\0\0\0\0\0\0\0"
    );
    assert_eq!(
        linker_option
            .get_strings()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![c"-framework", c"Foundation"]
    );
}
