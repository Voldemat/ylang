use crate::shared::IStaticStruct;

pub trait IHeader<const SYMBOL_STRUCT_BUFFER_SIZE: usize> {
    const BUFFER_SIZE: usize;
    type SymbolStruct<'b>: IStaticStruct<'b, { SYMBOL_STRUCT_BUFFER_SIZE }>;
}
