pub mod subtypes;

#[allow(non_camel_case_types)]
pub enum CPUType {
    VAX,
    ROMP,
    NS32032,
    NS32332,
    MC680x0,
    X86(subtypes::x86::Subtype),
    MIPS,
    NS32352,
    HP_PA,
    ARM(subtypes::arm::Subtype),
    MC88000,
    SPARC,
    I860BigEndian,
    I860LittleEndian,
    RS_6000,
    PowerPC,
    RISC_V,
}

pub enum Error {
    InvalidType,
    InvalidSubtype,
}

impl TryFrom<(u32, u32)> for CPUType {
    type Error = Error;

    fn try_from(value: (u32, u32)) -> Result<Self, Self::Error> {
        match value.0 {
            0x00000001 => Ok(Self::VAX),
            0x00000002 => Ok(Self::ROMP),
            0x00000004 => Ok(Self::NS32032),
            0x00000005 => Ok(Self::NS32332),
            0x00000006 => Ok(Self::MC680x0),
            0x00000007 => subtypes::x86::Subtype::try_from(value.0)
                .map_err(|_| Error::InvalidSubtype)
                .map(Self::X86),
            0x00000008 => Ok(Self::MIPS),
            0x00000009 => Ok(Self::NS32352),
            0x0000000B => Ok(Self::HP_PA),
            0x0000000C => subtypes::arm::Subtype::try_from(value.1)
                .map_err(|_| Error::InvalidSubtype)
                .map(Self::ARM),
            0x0000000D => Ok(Self::MC88000),
            0x0000000E => Ok(Self::SPARC),
            0x0000000F => Ok(Self::I860BigEndian),
            0x00000010 => Ok(Self::I860LittleEndian),
            0x00000011 => Ok(Self::RS_6000),
            0x00000012 => Ok(Self::PowerPC),
            0x00000018 => Ok(Self::RISC_V),
            _ => Err(Error::InvalidType),
        }
    }
}
