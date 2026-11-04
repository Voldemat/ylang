#[derive(PartialEq, Eq)]
pub struct AsciiStringRef<'b, const N: usize>(&'b [u8; N]);

impl<'b, const N: usize> AsciiStringRef<'b, N> {
    pub fn new(buffer: &'b [u8; N]) -> Option<Self> {
        if buffer.is_ascii() {
            Some(Self(buffer))
        } else {
            None
        }
    }
}

impl<'b, const N: usize> std::fmt::Debug for AsciiStringRef<'b, N> {
    #[allow(clippy::unwrap_used)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<'b, const N: usize> AsRef<str> for AsciiStringRef<'b, N> {
    #[allow(clippy::unwrap_used)]
    fn as_ref(&self) -> &'b str {
        std::str::from_utf8(self.0).unwrap()
    }
}
