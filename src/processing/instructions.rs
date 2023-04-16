pub mod copy_instruction_0;
pub mod invert_instruction_1;
pub mod jump_if_not_instruction_2;
pub mod jump_instruction_3;
pub mod jump_variable_instruction_4;
pub mod print_instruction_5;
pub mod and_instruction_6;
pub mod equal_instruction_7;
pub mod or_instruction_8;

pub const INSTRUCTION_CODE_LENGTH: usize = 2;

pub trait Instruction {
    fn get_address(&self) -> usize;
}