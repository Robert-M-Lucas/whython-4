pub mod assign_instruction;

pub trait Instruction {
    fn get_code(&self) -> u16;
    fn get_address(&self) -> usize;
    fn get_size(&self) -> usize;
}