pub const EM_NONE: u16 = 0;
pub const EM_386: u16 = 3;
pub const EM_ARM: u16 = 40;
pub const EM_X86_64: u16 = 62;
pub const EM_AARCH64: u16 = 183;
pub const EM_RISCV: u16 = 243;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Machine {
    None,
    X86,
    Arm,
    X86_64,
    AArch64,
    RiscV,
    Unknown(u16),
}

impl From<u16> for Machine {
    fn from(value: u16) -> Self {
        match value {
            EM_NONE => Self::None,
            EM_386 => Self::X86,
            EM_ARM => Self::Arm,
            EM_X86_64 => Self::X86_64,
            EM_AARCH64 => Self::AArch64,
            EM_RISCV => Self::RiscV,
            other => Self::Unknown(other),
        }
    }
}

impl From<Machine> for u16 {
    fn from(value: Machine) -> Self {
        match value {
            Machine::None => EM_NONE,
            Machine::X86 => EM_386,
            Machine::Arm => EM_ARM,
            Machine::X86_64 => EM_X86_64,
            Machine::AArch64 => EM_AARCH64,
            Machine::RiscV => EM_RISCV,
            Machine::Unknown(v) => v,
        }
    }
}
