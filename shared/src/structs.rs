use crate::byte_order::ByteOrder;

pub trait IStaticStruct<'b, const BUFFER_SIZE: usize>: 'b {
    fn new(byte_order: ByteOrder, buffer: &'b [u8; BUFFER_SIZE]) -> Self;
}

pub fn parse_static_struct<
    'b,
    const BUFFER_SIZE: usize,
    T: IStaticStruct<'b, BUFFER_SIZE>,
>(
    byte_order: ByteOrder,
    buffer: &'b [u8],
) -> Option<T> {
    buffer.try_into().ok().map(|b| T::new(byte_order, b))
}

pub trait IDynamicStruct<'b, const PARAMS_BUFFER_SIZE: usize>: 'b {
    fn new(
        byte_order: ByteOrder,
        params_buffer: &'b [u8; PARAMS_BUFFER_SIZE],
        rest_of_the_buffer: &'b [u8],
    ) -> Self;

    fn get_size(&self) -> u32;
}

pub fn parse_dynamic_struct<
    'b,
    const PARAMS_BUFFER_SIZE: usize,
    T: IDynamicStruct<'b, PARAMS_BUFFER_SIZE>,
>(
    byte_order: ByteOrder,
    buffer: &'b [u8],
) -> Option<T> {
    buffer.split_first_chunk::<PARAMS_BUFFER_SIZE>().map(
        |(params_buffer, rest_of_the_buffer)| {
            T::new(byte_order, params_buffer, rest_of_the_buffer)
        },
    )
}
