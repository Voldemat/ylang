use core::mem::size_of;

use zerocopy::byteorder::{ByteOrder, LittleEndian, U32, U64};
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

use crate::types::flags::SectionFlags;

//section types (sh_type)
pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;
pub const SHT_DYNAMIC: u32 = 6;
pub const SHT_NOTE: u32 = 7;
pub const SHT_NOBITS: u32 = 8;
pub const SHT_REL: u32 = 9;
pub const SHT_DYNSYM: u32 = 11;

//ranges reserved for the OS and processor
pub const SHT_LOOS: u32 = 0x6000_0000;
pub const SHT_HIOS: u32 = 0x6fff_ffff;
pub const SHT_LOPROC: u32 = 0x7000_0000;
pub const SHT_HIPROC: u32 = 0x7fff_ffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionType {
    Null,
    ProgBits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    DynSym,
    OsSpecific(u32),
    ProcSpecific(u32),
    //anything outside the groups above, raw value is kept
    Unknown(u32),
}

// u32 -> SectionType
impl From<u32> for SectionType {
    fn from(value: u32) -> Self {
        match value {
            SHT_NULL => Self::Null,
            SHT_PROGBITS => Self::ProgBits,
            SHT_SYMTAB => Self::SymTab,
            SHT_STRTAB => Self::StrTab,
            SHT_RELA => Self::Rela,
            SHT_HASH => Self::Hash,
            SHT_DYNAMIC => Self::Dynamic,
            SHT_NOTE => Self::Note,
            SHT_NOBITS => Self::NoBits,
            SHT_REL => Self::Rel,
            SHT_DYNSYM => Self::DynSym,
            SHT_LOOS..=SHT_HIOS => Self::OsSpecific(value),
            SHT_LOPROC..=SHT_HIPROC => Self::ProcSpecific(value),
            other => Self::Unknown(other),
        }
    }
}

//sectiontype -> u32, used when writing header
impl From<SectionType> for u32 {
    fn from(value: SectionType) -> Self {
        match value {
            SectionType::Null => SHT_NULL,
            SectionType::ProgBits => SHT_PROGBITS,
            SectionType::SymTab => SHT_SYMTAB,
            SectionType::StrTab => SHT_STRTAB,
            SectionType::Rela => SHT_RELA,
            SectionType::Hash => SHT_HASH,
            SectionType::Dynamic => SHT_DYNAMIC,
            SectionType::Note => SHT_NOTE,
            SectionType::NoBits => SHT_NOBITS,
            SectionType::Rel => SHT_REL,
            SectionType::DynSym => SHT_DYNSYM,
            SectionType::OsSpecific(v)
            | SectionType::ProcSpecific(v)
            | SectionType::Unknown(v) => v,
        }
    }
}

//elf32 section header, fields in spec order
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Shdr32<O: ByteOrder> {
    pub sh_name: U32<O>,
    pub sh_type: U32<O>,
    pub sh_flags: U32<O>,
    pub sh_addr: U32<O>,
    pub sh_offset: U32<O>,
    pub sh_size: U32<O>,
    pub sh_link: U32<O>,
    pub sh_info: U32<O>,
    pub sh_addralign: U32<O>,
    pub sh_entsize: U32<O>,
}

// ELF64 section header, flags, addr, sizes and alignment are 64-bit
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Shdr64<O: ByteOrder> {
    pub sh_name: U32<O>,
    pub sh_type: U32<O>,
    pub sh_flags: U64<O>,
    pub sh_addr: U64<O>,
    pub sh_offset: U64<O>,
    pub sh_size: U64<O>,
    pub sh_link: U32<O>,
    pub sh_info: U32<O>,
    pub sh_addralign: U64<O>,
    pub sh_entsize: U64<O>,
}

// sizes must match the spec, otherwise it won't compile
const _: [(); 40] = [(); size_of::<Shdr32<LittleEndian>>()];
const _: [(); 64] = [(); size_of::<Shdr64<LittleEndian>>()];

impl<O: ByteOrder> Shdr32<O> {
    pub fn section_type(&self) -> SectionType {
        SectionType::from(self.sh_type.get())
    }

    // widened to u64 so both classes share one flags type
    pub fn flags(&self) -> SectionFlags {
        SectionFlags(u64::from(self.sh_flags.get()))
    }
}

impl<O: ByteOrder> Shdr64<O> {
    pub fn section_type(&self) -> SectionType {
        SectionType::from(self.sh_type.get())
    }

    pub fn flags(&self) -> SectionFlags {
        SectionFlags(self.sh_flags.get())
    }
}
