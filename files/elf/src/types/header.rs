use core::mem::size_of;
use zerocopy::byteorder::{ByteOrder, LittleEndian, U16, U32, U64};
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

use crate::{ElfError, Result};

// Константы спецификации
pub const ELFMAG: [u8; 4] = [0x7f, b'E', b'L', b'F'];
pub const EI_NIDENT: usize = 16;
pub const EV_CURRENT: u8 = 1;

pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;

pub const ELFDATA2LSB: u8 = 1; // little-endian
pub const ELFDATA2MSB: u8 = 2; // big-endian

pub const ET_NONE: u16 = 0;
pub const ET_REL: u16 = 1;
pub const ET_EXEC: u16 = 2;
pub const ET_DYN: u16 = 3;
pub const ET_CORE: u16 = 4;
pub const ET_LOOS: u16 = 0xfe00;
pub const ET_HIOS: u16 = 0xfeff;
pub const ET_LOPROC: u16 = 0xff00;
pub const ET_HIPROC: u16 = 0xffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElfClass {
    Elf32,
    Elf64,
}

impl TryFrom<u8> for ElfClass {
    type Error = ElfError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            ELFCLASS32 => Ok(Self::Elf32),
            ELFCLASS64 => Ok(Self::Elf64),
            other => Err(ElfError::UnsupportedClass(other)),
        }
    }
}

impl From<ElfClass> for u8 {
    fn from(value: ElfClass) -> Self {
        match value {
            ElfClass::Elf32 => ELFCLASS32,
            ElfClass::Elf64 => ELFCLASS64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElfData {
    Little,
    Big,
}

impl TryFrom<u8> for ElfData {
    type Error = ElfError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            ELFDATA2LSB => Ok(Self::Little),
            ELFDATA2MSB => Ok(Self::Big),
            other => Err(ElfError::UnsupportedData(other)),
        }
    }
}

impl From<ElfData> for u8 {
    fn from(value: ElfData) -> Self {
        match value {
            ElfData::Little => ELFDATA2LSB,
            ElfData::Big => ELFDATA2MSB,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElfType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    OsSpecific(u16),
    ProcSpecific(u16),
    Unknown(u16),
}

impl From<u16> for ElfType {
    fn from(value: u16) -> Self {
        match value {
            ET_NONE => Self::None,
            ET_REL => Self::Rel,
            ET_EXEC => Self::Exec,
            ET_DYN => Self::Dyn,
            ET_CORE => Self::Core,
            ET_LOOS..=ET_HIOS => Self::OsSpecific(value),
            ET_LOPROC..=ET_HIPROC => Self::ProcSpecific(value),
            other => Self::Unknown(other),
        }
    }
}

impl From<ElfType> for u16 {
    fn from(value: ElfType) -> Self {
        match value {
            ElfType::None => ET_NONE,
            ElfType::Rel => ET_REL,
            ElfType::Exec => ET_EXEC,
            ElfType::Dyn => ET_DYN,
            ElfType::Core => ET_CORE,
            ElfType::OsSpecific(v) | ElfType::ProcSpecific(v) | ElfType::Unknown(v) => v,
        }
    }
}

// Первые 16 байт файла
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Ident {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub pad: [u8; 7],
}

impl Ident {
    pub fn parse(bytes: &[u8]) -> Result<&Self> {
        let (ident, _) = Self::ref_from_prefix(bytes).map_err(|_| ElfError::UnexpectedEof {
            offset: 0,
            needed: EI_NIDENT,
            available: bytes.len(),
        })?;
        ident.check_magic()?;
        Ok(ident)
    }

    pub fn check_magic(&self) -> Result<()> {
        if self.magic == ELFMAG {
            Ok(())
        } else {
            Err(ElfError::BadMagic { found: self.magic })
        }
    }

    pub fn class(&self) -> Result<ElfClass> {
        ElfClass::try_from(self.class)
    }

    pub fn data(&self) -> Result<ElfData> {
        ElfData::try_from(self.data)
    }
}

const _: [(); EI_NIDENT] = [(); size_of::<Ident>()];

// Общий парсинг и проверка заголовков
macro_rules! impl_ehdr_common {
    ($ty:ident, $class:expr, $size:literal) => {
        const _: [(); $size] = [(); size_of::<$ty<LittleEndian>>()];

        impl<O: ByteOrder> $ty<O> {
            pub const CLASS: ElfClass = $class;
            pub const EHSIZE: u16 = $size;

            pub fn parse(bytes: &[u8]) -> Result<&Self> {
                let (hdr, _) = Self::ref_from_prefix(bytes).map_err(|_| ElfError::UnexpectedEof {
                    offset: 0,
                    needed: usize::from(Self::EHSIZE),
                    available: bytes.len(),
                })?;
                hdr.validate()?;
                Ok(hdr)
            }

            pub fn validate(&self) -> Result<()> {
                self.e_ident.check_magic()?;

                let class = self.e_ident.class()?;
                if class != Self::CLASS {
                    return Err(ElfError::ClassMismatch {
                        expected: Self::CLASS,
                        found: class,
                    });
                }

                self.e_ident.data()?;

                if self.e_ident.version != EV_CURRENT {
                    return Err(ElfError::UnsupportedVersion(u32::from(self.e_ident.version)));
                }

                let version = self.e_version.get();
                if version != u32::from(EV_CURRENT) {
                    return Err(ElfError::UnsupportedVersion(version));
                }

                let ehsize = self.e_ehsize.get();
                if ehsize != Self::EHSIZE {
                    return Err(ElfError::InvalidHeaderSize {
                        expected: Self::EHSIZE,
                        found: ehsize,
                    });
                }

                Ok(())
            }

            pub fn file_type(&self) -> ElfType {
                ElfType::from(self.e_type.get())
            }
        }
    };
}

// Заголовок 32-битного ELF (52 байта)
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Ehdr32<O: ByteOrder> {
    pub e_ident: Ident,
    pub e_type: U16<O>,
    pub e_machine: U16<O>,
    pub e_version: U32<O>,
    pub e_entry: U32<O>,
    pub e_phoff: U32<O>,
    pub e_shoff: U32<O>,
    pub e_flags: U32<O>,
    pub e_ehsize: U16<O>,
    pub e_phentsize: U16<O>,
    pub e_phnum: U16<O>,
    pub e_shentsize: U16<O>,
    pub e_shnum: U16<O>,
    pub e_shstrndx: U16<O>,
}

// Заголовок 64-битного ELF (64 байта)
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Ehdr64<O: ByteOrder> {
    pub e_ident: Ident,
    pub e_type: U16<O>,
    pub e_machine: U16<O>,
    pub e_version: U32<O>,
    pub e_entry: U64<O>,
    pub e_phoff: U64<O>,
    pub e_shoff: U64<O>,
    pub e_flags: U32<O>,
    pub e_ehsize: U16<O>,
    pub e_phentsize: U16<O>,
    pub e_phnum: U16<O>,
    pub e_shentsize: U16<O>,
    pub e_shnum: U16<O>,
    pub e_shstrndx: U16<O>,
}

impl_ehdr_common!(Ehdr32, ElfClass::Elf32, 52);
impl_ehdr_common!(Ehdr64, ElfClass::Elf64, 64);
