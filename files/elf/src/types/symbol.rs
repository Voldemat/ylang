use core::mem::size_of;

use zerocopy::byteorder::{ByteOrder, LittleEndian, U16, U32, U64};
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

// symbol binding
pub const STB_LOCAL: u8 = 0;
pub const STB_GLOBAL: u8 = 1;
pub const STB_WEAK: u8 = 2;

// symbol type 
pub const STT_NOTYPE: u8 = 0;
pub const STT_OBJECT: u8 = 1;
pub const STT_FUNC: u8 = 2;
pub const STT_SECTION: u8 = 3;
pub const STT_FILE: u8 = 4;
pub const STT_COMMON: u8 = 5;
pub const STT_TLS: u8 = 6;

// symbol visibility
pub const STV_DEFAULT: u8 = 0;
pub const STV_INTERNAL: u8 = 1;
pub const STV_HIDDEN: u8 = 2;
pub const STV_PROTECTED: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolBinding {
    Local,
    Global,
    Weak,
    Unknown(u8),
}

impl From<u8> for SymbolBinding {
    fn from(value: u8) -> Self {
        match value {
            STB_LOCAL => Self::Local,
            STB_GLOBAL => Self::Global,
            STB_WEAK => Self::Weak,
            other => Self::Unknown(other),
        }
    }
}

impl From<SymbolBinding> for u8 {
    fn from(value: SymbolBinding) -> Self {
        match value {
            SymbolBinding::Local => STB_LOCAL,
            SymbolBinding::Global => STB_GLOBAL,
            SymbolBinding::Weak => STB_WEAK,
            SymbolBinding::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolType {
    NoType,
    Object,
    Func,
    Section,
    File,
    Common,
    Tls,
    Unknown(u8),
}

impl From<u8> for SymbolType {
    fn from(value: u8) -> Self {
        match value {
            STT_NOTYPE => Self::NoType,
            STT_OBJECT => Self::Object,
            STT_FUNC => Self::Func,
            STT_SECTION => Self::Section,
            STT_FILE => Self::File,
            STT_COMMON => Self::Common,
            STT_TLS => Self::Tls,
            other => Self::Unknown(other),
        }
    }
}

impl From<SymbolType> for u8 {
    fn from(value: SymbolType) -> Self {
        match value {
            SymbolType::NoType => STT_NOTYPE,
            SymbolType::Object => STT_OBJECT,
            SymbolType::Func => STT_FUNC,
            SymbolType::Section => STT_SECTION,
            SymbolType::File => STT_FILE,
            SymbolType::Common => STT_COMMON,
            SymbolType::Tls => STT_TLS,
            SymbolType::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolVisibility {
    Default,
    Internal,
    Hidden,
    Protected,
    Unknown(u8),
}

impl From<u8> for SymbolVisibility {
    fn from(value: u8) -> Self {
        match value {
            STV_DEFAULT => Self::Default,
            STV_INTERNAL => Self::Internal,
            STV_HIDDEN => Self::Hidden,
            STV_PROTECTED => Self::Protected,
            other => Self::Unknown(other),
        }
    }
}

impl From<SymbolVisibility> for u8 {
    fn from(value: SymbolVisibility) -> Self {
        match value {
            SymbolVisibility::Default => STV_DEFAULT,
            SymbolVisibility::Internal => STV_INTERNAL,
            SymbolVisibility::Hidden => STV_HIDDEN,
            SymbolVisibility::Protected => STV_PROTECTED,
            SymbolVisibility::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Sym32<O: ByteOrder> {
    pub st_name: U32<O>,
    pub st_value: U32<O>,
    pub st_size: U32<O>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: U16<O>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct Sym64<O: ByteOrder> {
    pub st_name: U32<O>,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: U16<O>,
    pub st_value: U64<O>,
    pub st_size: U64<O>,
}

// sizes must match the spec, otherwise it won't compile
const _: [(); 16] = [(); size_of::<Sym32<LittleEndian>>()];
const _: [(); 24] = [(); size_of::<Sym64<LittleEndian>>()];

impl<O: ByteOrder> Sym32<O> {
    pub fn get_sym_type(&self) -> SymbolType {
        SymbolType::from(self.st_info & 0xf)
    }

    pub fn get_binding(&self) -> SymbolBinding {
        SymbolBinding::from(self.st_info >> 4)
    }

    pub fn visibility(&self) -> SymbolVisibility {
        SymbolVisibility::from(self.st_other & 0x3)
    }
}

impl<O: ByteOrder> Sym64<O> {
    pub fn get_sym_type(&self) -> SymbolType {
        SymbolType::from(self.st_info & 0xf)
    }

    pub fn get_binding(&self) -> SymbolBinding {
        SymbolBinding::from(self.st_info >> 4)
    }

    pub fn visibility(&self) -> SymbolVisibility {
        SymbolVisibility::from(self.st_other & 0x3)
    }
}
