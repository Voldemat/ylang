#[derive(PartialEq, Eq)]
pub struct AsciiString<const N: usize>(pub [u8; N]);

impl<const N: usize> std::fmt::Debug for AsciiString<N> {
    #[allow(clippy::unwrap_used)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str::from_utf8(&self.0).unwrap())
    }
}
