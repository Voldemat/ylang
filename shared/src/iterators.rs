use crate::{byte_order::ByteOrder, eof::EOF, structs::IStaticStruct};

pub struct CStringIterator<'b> {
    buffer: &'b [u8],
    offset: usize,
    max_count: Option<usize>,
    count: usize,
}

impl<'b> CStringIterator<'b> {
    pub fn new(buffer: &'b [u8], max_count: Option<usize>) -> Self {
        Self {
            buffer,
            offset: 0,
            max_count,
            count: 0,
        }
    }
}

impl<'b> Iterator for CStringIterator<'b> {
    type Item = Result<&'b std::ffi::CStr, EOF>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(max_count) = self.max_count {
            if self.count == max_count {
                return None;
            }
        } else {
            if self.offset == self.buffer.len() {
                return None;
            }
        }
        let Some(current_buffer) = self.buffer.get(self.offset..) else {
            self.offset = self.buffer.len();
            if let Some(max_count) = self.max_count {
                self.count = max_count
            }
            return Some(Err(EOF));
        };
        let Ok(cstring) = std::ffi::CStr::from_bytes_until_nul(current_buffer)
        else {
            self.offset = self.buffer.len();
            if let Some(max_count) = self.max_count {
                self.count = max_count
            }
            return Some(Err(EOF));
        };
        self.offset += cstring.count_bytes() + 1;
        self.count += 1;
        Some(Ok(cstring))
    }
}

pub struct FixedSizeStructIterator<'b, const BUFFER_SIZE: usize, T> {
    byte_order: ByteOrder,
    buffer: &'b [u8],
    total_count: usize,
    count: usize,
    _a: std::marker::PhantomData<T>,
}

impl<'b, const BUFFER_SIZE: usize, T: IStaticStruct<'b, BUFFER_SIZE>>
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

impl<'b, const BUFFER_SIZE: usize, T: IStaticStruct<'b, BUFFER_SIZE>> Iterator
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
