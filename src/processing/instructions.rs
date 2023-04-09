pub mod copy_instruction_0;
pub mod invert_instruction_1;
pub mod jump_if_instruction_2;

pub const INSTRUCTION_CODE_LENGTH: usize = 2;

pub trait Instruction {
    fn get_address(&self) -> usize;
}