pub const PF_X: u32 = 1;
pub const PF_W: u32 = 2;
pub const PF_R: u32 = 4;

pub const SHF_WRITE: u64 = 0x1;
pub const SHF_ALLOC: u64 = 0x2;
pub const SHF_EXECINSTR: u64 = 0x4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SegmentFlags(pub u32);

impl SegmentFlags {
    pub fn is_readable(&self) -> bool {
        self.0 & PF_R != 0
    }

    pub fn is_writable(&self) -> bool {
        self.0 & PF_W != 0
    }

    pub fn is_executable(&self) -> bool {
        self.0 & PF_X != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionFlags(pub u64);

impl SectionFlags {
    pub fn is_write(&self) -> bool {
        self.0 & SHF_WRITE != 0
    }

    pub fn is_alloc(&self) -> bool {
        self.0 & SHF_ALLOC != 0
    }

    pub fn is_exec(&self) -> bool {
        self.0 & SHF_EXECINSTR != 0
    }
}
