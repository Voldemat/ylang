use crate::parser::ByteOrder;

pub trait IFile<'b> {
    fn get_byte_order(&self) -> ByteOrder;
    fn get_rest_of_the_buffer(&self) -> &'b [u8];
    fn get_load_commands_count(&self) -> u32;
    fn get_load_commands_size(&self) -> u32;
}
