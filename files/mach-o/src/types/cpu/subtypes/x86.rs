#[allow(non_camel_case_types)]
pub enum Subtype {
	All,
	_486,
	_486SX,
	Pentium_M5,
	Celeron,
	Celeron_Mobile,
	Pentium_3,
	Pentium_3_M,
	Pentium_3_XEON,
	Pentium_4,
	Itanium,
	Itanium_2,
	XEON,
	XEON_MP,
}

impl TryFrom<u32> for Subtype {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
			0x00000003 => Ok(Self::All),
			0x00000004 => Ok(Self::_486),
			0x00000084 => Ok(Self::_486SX),
			0x00000056 => Ok(Self::Pentium_M5),
			0x00000067 => Ok(Self::Celeron),
			0x00000077 => Ok(Self::Celeron_Mobile),
			0x00000008 => Ok(Self::Pentium_3),
			0x00000018 => Ok(Self::Pentium_3_M),
			0x00000028 => Ok(Self::Pentium_3_XEON),
			0x0000000A => Ok(Self::Pentium_4),
			0x0000000B => Ok(Self::Itanium),
			0x0000001B => Ok(Self::Itanium_2),
			0x0000000C => Ok(Self::XEON),
			0x0000001C => Ok(Self::XEON_MP),
            _ => Err(())
        }
    }
}
