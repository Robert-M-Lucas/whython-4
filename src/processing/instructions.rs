pub mod copy_instruction_0;
pub mod invert_instruction_1;

pub trait Instruction {
    fn get_address(&self) -> usize;
}