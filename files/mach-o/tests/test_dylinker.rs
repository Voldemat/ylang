use macho::parser::{
    self, ByteOrder,
    variants::singlearch::{
        b64::load_command::LoadCommandContent,
        shared::load_command_content::{
            SharedLoadCommandContent, dylinker::Dylinker, linked_it::LinkedIt,
        },
    },
};

#[test]
pub fn test_dylinker_file() {
    let path = std::path::PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("sample_dylinker");
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
    assert_eq!(file.header.file_type, 7);
    assert_eq!(file.header.load_commands_count, 15);
    assert_eq!(file.header.load_commands_size, 816);
    assert_eq!(file.header.flags, 0x00000085);
    let load_commands = file
        .get_load_commands()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        load_commands.len(),
        file.header.load_commands_count as usize
    );

    // LC_ID_DYLINKER
    assert_eq!(load_commands[6].header.command_type, 0xF);
    assert_eq!(load_commands[6].header.command_size, 32);
    assert_eq!(
        load_commands[6].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::Dylinker(
            Dylinker {
                name_offset: 12,
                rest_of_the_buffer: b"/usr/lib/dyld\0\0\0\0\0\0\0",
            }
        ))
    );
    let LoadCommandContent::Shared(SharedLoadCommandContent::Dylinker(c)) =
        &load_commands[6].content
    else {
        panic!("Expected dylinker command");
    };
    assert_eq!(c.get_name().unwrap(), c"/usr/lib/dyld");

    // LC_UNIXTHREAD with ARM_THREAD_STATE64
    assert_eq!(load_commands[10].header.command_type, 0x5);
    assert_eq!(load_commands[10].header.command_size, 288);
    let LoadCommandContent::Shared(SharedLoadCommandContent::UnixThread(
        thread,
    )) = &load_commands[10].content
    else {
        panic!("Expected unix thread load command");
    };
    assert_eq!(thread.len(), 280);
    // flavor = ARM_THREAD_STATE64
    assert_eq!(u32::from_le_bytes(thread[0..4].try_into().unwrap()), 6);
    // count = ARM_THREAD_STATE64_COUNT
    assert_eq!(u32::from_le_bytes(thread[4..8].try_into().unwrap()), 68);
    // pc
    assert_eq!(
        u64::from_le_bytes(thread[264..272].try_into().unwrap()),
        0x1000
    );

    // LC_SEGMENT_SPLIT_INFO
    assert_eq!(load_commands[11].header.command_type, 0x1E);
    assert_eq!(load_commands[11].header.command_size, 16);
    assert_eq!(
        load_commands[11].content,
        LoadCommandContent::Shared(SharedLoadCommandContent::LinkedIt(
            LinkedIt {
                file_offset: 16456,
                datasize: 8,
            }
        ))
    );
}
