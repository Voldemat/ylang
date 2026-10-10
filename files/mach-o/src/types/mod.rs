pub mod cpu;
pub mod flags;
pub mod file_type;

pub enum AddressSize {
    B32,
    B64,
}

pub struct LoadCommand {

}

pub struct File {
    pub address_size: AddressSize,
    pub cpu_type: cpu::CPUType,
    pub file_type: file_type::FileType,
    pub load_commands: Vec<LoadCommand>,
    pub flags: flags::Flags,
}
