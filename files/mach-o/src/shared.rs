#[derive(PartialEq, Eq)]
pub struct AsciiString<const N: usize>(pub [u8; N]);

impl<const N: usize> std::fmt::Debug for AsciiString<N> {
    #[allow(clippy::unwrap_used)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<const N: usize> AsRef<str> for AsciiString<N> {
    #[allow(clippy::unwrap_used)]
    fn as_ref(&self) -> &str {
        std::ffi::CStr::from_bytes_until_nul(&self.0)
            .unwrap()
            .to_str()
            .unwrap()
    }
}
