use crate::parser::ByteOrder;

#[derive(Debug)]
pub struct EOF;

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
        std::str::from_utf8(&self.0).unwrap()
    }
}

pub struct CStringIterator<'b> {
    buffer: &'b [u8],
    offset: usize,
}

impl<'b> CStringIterator<'b> {
    pub fn new(buffer: &'b [u8]) -> Self {
        Self { buffer, offset: 0 }
    }
}

impl<'b> Iterator for CStringIterator<'b> {
    type Item = Result<&'b std::ffi::CStr, EOF>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.buffer.len() {
            return None;
        };
        let Some(current_buffer) = self.buffer.get(self.offset..) else {
            self.offset = self.buffer.len();
            return Some(Err(EOF));
        };
        let Ok(cstring) = std::ffi::CStr::from_bytes_until_nul(current_buffer)
        else {
            self.offset = self.buffer.len();
            return Some(Err(EOF));
        };
        self.offset = cstring.count_bytes() + 1;
        Some(Ok(cstring))
    }
}

pub trait IStaticStruct<const BUFFER_SIZE: usize> {
    fn new(byte_order: ByteOrder, buffer: &[u8; BUFFER_SIZE]) -> Self;
}

pub struct FixedSizeStructIterator<'b, const BUFFER_SIZE: usize, T> {
    byte_order: ByteOrder,
    buffer: &'b [u8],
    total_count: usize,
    count: usize,
    _a: std::marker::PhantomData<T>,
}

impl<'b, const BUFFER_SIZE: usize, T: IStaticStruct<BUFFER_SIZE>>
    FixedSizeStructIterator<'b, BUFFER_SIZE, T>
{
    pub fn new(
        byte_order: ByteOrder,
        buffer: &'b [u8],
        total_count: usize,
    ) -> Self {
        Self {
            byte_order,
            buffer,
            total_count,
            count: 0,
            _a: std::marker::PhantomData::<T>,
        }
    }
}

impl<'b, const BUFFER_SIZE: usize, T: IStaticStruct<BUFFER_SIZE>> Iterator
    for FixedSizeStructIterator<'b, BUFFER_SIZE, T>
{
    type Item = Result<T, EOF>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.total_count {
            return None;
        }
        let offset = BUFFER_SIZE * self.count;
        let Some(current_buffer) = self
            .buffer
            .get(offset..offset + BUFFER_SIZE)
            .and_then(|b| b.try_into().ok())
        else {
            self.count = self.total_count;
            return Some(Err(EOF));
        };
        self.count += 1;
        Some(Ok(T::new(self.byte_order, current_buffer)))
    }
}
