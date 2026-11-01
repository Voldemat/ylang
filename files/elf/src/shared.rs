use zerocopy::transmute_ref;

use crate::parser::ByteOrder;
use crate::shared::IStaticStruct;
use crate::types::header::{ElfData, Ident, EI_NIDENT};
use crate::{ElfError, Result};


pub fn byte_order_of(data: ElfData) -> ByteOrder {
    match data {
        ElfData::Little => ByteOrder::LittleEndian,
        ElfData::Big => ByteOrder::BigEndian,
    }
}


impl<'b> IStaticStruct<'b, EI_NIDENT> for Ident {
    fn new(_byte_order: ByteOrder, buffer: &'b [u8; EI_NIDENT]) -> Self {
        let ident: &Ident = transmute_ref!(buffer);
        *ident
    }
}


pub fn slice_at<'a>(
    buf: &'a [u8],
    what: &'static str,
    offset: u64,
    size: u64,
) -> Result<&'a [u8]> {
    let out_of_bounds = || ElfError::OutOfBounds {
        what,
        offset,
        size,
        buffer_len: buf.len(),
    };
    let start = usize::try_from(offset).map_err(|_| out_of_bounds())?;
    let len = usize::try_from(size).map_err(|_| out_of_bounds())?;
    let end = start.checked_add(len).ok_or_else(out_of_bounds)?;
    buf.get(start..end).ok_or_else(out_of_bounds)
}
