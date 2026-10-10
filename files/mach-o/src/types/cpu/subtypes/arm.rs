#[allow(non_camel_case_types)]
pub enum Subtype {
	All,
	ARM_A500_ARCH,
	ARM_A500,
	ARM_A440,
	ARM_M4,
	ARM_V4T,
	ARM_V6,
	ARM_V5TEJ,
	ARM_XSCALE,
	ARM_V7,
	ARM_V7F,
	ARM_V7S,
	ARM_V7K,
	ARM_V8,
	ARM_V6M,
	ARM_V7M,
	ARM_V7EM,
}

impl TryFrom<u32> for Subtype {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
			0x00000000 => Ok(Self::All),
			0x00000001 => Ok(Self::ARM_A500_ARCH),
			0x00000002 => Ok(Self::ARM_A500),
			0x00000003 => Ok(Self::ARM_A440),
			0x00000004 => Ok(Self::ARM_M4),
			0x00000005 => Ok(Self::ARM_V4T),
			0x00000006 => Ok(Self::ARM_V6),
			0x00000007 => Ok(Self::ARM_V5TEJ),
			0x00000008 => Ok(Self::ARM_XSCALE),
			0x00000009 => Ok(Self::ARM_V7),
			0x0000000A => Ok(Self::ARM_V7F),
			0x0000000B => Ok(Self::ARM_V7S),
			0x0000000C => Ok(Self::ARM_V7K),
			0x0000000D => Ok(Self::ARM_V8),
			0x0000000E => Ok(Self::ARM_V6M),
			0x0000000F => Ok(Self::ARM_V7M),
			0x00000010 => Ok(Self::ARM_V7EM),
            _ => Err(())
        }
    }
}
