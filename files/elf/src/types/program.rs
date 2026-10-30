use core::mem::size_of;
use zerocopy::byteorder::{ByteOrder, LittleEndian, U32, U64};
use zerocopy::{FromBytes,Immutable,KNownLayout,Unaligned};
use crate::types::flags::SegmentFlags;
// типы сегментов 
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
// диапазон под ОС и процессор 
pub const PT_LOOS: u32 = 0x6000_0000;
pub const PT_HIOS: u32 = 0x6fff_ffff;
pub const PT_LOPROC: u32 = 0x7000_0000;
pub const PT_HIPROC: u32 = 0x7fff_ffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
    OsSpecific(u32),
    ProcSpecific(u32),
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(value: u32) -> Self {
        match value {
            PT_NULL => Self::Null,
            PT_LOAD => Self::Load,
            PT_DYNAMIC => Self::Dynamic,
            PT_INTERP => Self::Interp,
            PT_NOTE => Self::Note,
            PT_SHLIB => Self::Shlib,
            PT_PHDR => Self::Phdr,
            PT_TLS => Self::Tls,
            PT_LOOS..=PT_HIOS => Self::OsSpecific(value),
            PT_LOPROC..=PT_HIPROC => Self::ProcSpecific(value),
            other => Self::Unknown(other),
        }
    }
}


impl From<SegmentType> for u32 {
    fn from(value: SegmentType) -> Self {
        match value {
            SegmentType::Null => PT_NULL,
            SegmentType::Load => PT_LOAD,
            SegmentType::Dynamic => PT_DYNAMIC,
            SegmentType::Interp => PT_INTERP,
            SegmentType::Note => PT_NOTE,
            SegmentType::Shlib => PT_SHLIB,
            SegmentType::Phdr => PT_PHDR,
            SegmentType::Tls => PT_TLS,
            SegmentType::OsSpecific(v)
            | SegmentType::ProcSpecific(v)
            | SegmentType::Unknown(v) => v,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Phdr32<O: ByteOrder> {
    pub p_type: U32<O>,
    pub p_offset: U32<O>,
    pub p_vaddr: U32<O>,
    pub p_paddr: U32<O>,
    pub p_filesz: U32<O>,
    pub p_memsz: U32<O>,
    pub p_flags: U32<O>,
    pub p_align: U32<O>,
}

// program header для ELF64, p_flags здесь стоит сразу после p_type
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Phdr64<O: ByteOrder> {
    pub p_type: U32<O>,
    pub p_flags: U32<O>,
    pub p_offset: U64<O>,
    pub p_vaddr: U64<O>,
    pub p_paddr: U64<O>,
    pub p_filesz: U64<O>,
    pub p_memsz: U64<O>,
    pub p_align: U64<O>,
}

const _: [(); 32] = [(); size_of::<Phdr32<LittleEndian>>()];
const _: [(); 56] = [(); size_of::<Phdr64<LittleEndian>>()];

impl<O: ByteOrder> Phdr32<O> {
    pub fn segment_type(&self) -> SegmentType {
        SegmentType::from(self.p_type.get())
    }

    pub fn flags(&self) -> SegmentFlags {
        SegmentFlags(self.p_flags.get())
    }
}

impl<O: ByteOrder> Phdr64<O> {
    pub fn segment_type(&self) -> SegmentType {
        SegmentType::from(self.p_type.get())
    }

    pub fn flags(&self) -> SegmentFlags {
        SegmentFlags(self.p_flags.get())
    }
}
