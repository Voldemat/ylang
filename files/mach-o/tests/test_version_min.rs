use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::LoadCommandContent,
        shared::load_command_content::{
            SharedLoadCommandContent, minimum_os_version::MinimumOSVersion,
        },
    },
};

fn check_file(
    file_name: &str,
    command_index: usize,
    command_type: u32,
    expected: MinimumOSVersion,
) {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join(file_name);
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
    assert_eq!(file.header.file_type, 2);
    assert_eq!(file.header.load_commands_count, 14);
    assert_eq!(file.header.load_commands_size, 728);
    assert_eq!(file.header.flags, 0x00200085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );
    assert_eq!(
        load_commands[command_index].header.command_type,
        command_type
    );
    assert_eq!(load_commands[command_index].header.command_size, 16);
    assert_eq!(
        load_commands[command_index].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::MinimumOSVersion(
            expected
        ))
    );
}

#[test]
pub fn test_version_min_macos_file() {
    check_file(
        "samples/sample_version_min_macos",
        8,
        MinimumOSVersion::COMMAND_TYPE_MACOSX,
        MinimumOSVersion {
            minimum_os_version: 0x000A0D00, // 10.13
            sdk_version: 0x001B0000,        // 27.0
        },
    );
}

#[test]
pub fn test_version_min_ios_file() {
    check_file(
        "samples/sample_version_min_ios",
        13,
        MinimumOSVersion::COMMAND_TYPE_IPHONEOS,
        MinimumOSVersion {
            minimum_os_version: 0x000C0000, // 12.0
            sdk_version: 0x000F0000,        // 15.0
        },
    );
}

#[test]
pub fn test_version_min_tvos_file() {
    check_file(
        "samples/sample_version_min_tvos",
        13,
        MinimumOSVersion::COMMAND_TYPE_TVOS,
        MinimumOSVersion {
            minimum_os_version: 0x000C0000, // 12.0
            sdk_version: 0x000F0000,        // 15.0
        },
    );
}

#[test]
pub fn test_version_min_watchos_file() {
    check_file(
        "samples/sample_version_min_watchos",
        13,
        MinimumOSVersion::COMMAND_TYPE_WATCHOS,
        MinimumOSVersion {
            minimum_os_version: 0x00050000, // 5.0
            sdk_version: 0x00080000,        // 8.0
        },
    );
}
