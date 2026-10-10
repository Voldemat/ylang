pub enum FileType {
	RelocatableObjectFile,
	DemandPagedExecutableFile,
	FixedVmSharedLibraryFile,
	CoreFile,
	PreloadedExecutableFile,
	DynamicallyBoundSharedLibraryFile,
	DynamicLinkEditor,
	DynamicallyBoundBundleFile,
	SharedLibraryStubForStaticLinkingOnly,
	CompanionFileWithOnlyDebugSections,
	X86_64Kexts,
	Composed,
}

impl TryFrom<u32> for FileType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
			0x00000001 => Ok(Self::RelocatableObjectFile),
			0x00000002 => Ok(Self::DemandPagedExecutableFile),
			0x00000003 => Ok(Self::FixedVmSharedLibraryFile),
			0x00000004 => Ok(Self::CoreFile),
			0x00000005 => Ok(Self::PreloadedExecutableFile),
			0x00000006 => Ok(Self::DynamicallyBoundSharedLibraryFile),
			0x00000007 => Ok(Self::DynamicLinkEditor),
			0x00000008 => Ok(Self::DynamicallyBoundBundleFile),
			0x00000009 => Ok(Self::SharedLibraryStubForStaticLinkingOnly),
			0x0000000A => Ok(Self::CompanionFileWithOnlyDebugSections),
			0x0000000B => Ok(Self::X86_64Kexts),
			0x0000000C => Ok(Self::Composed),
            _ => Err(())
        }
    }
}
