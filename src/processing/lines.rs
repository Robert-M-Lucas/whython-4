use crate::processing::blocks::BlockCoordinator;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::Symbol;

pub mod variable_initialisation_line;
pub mod arithmetic;
pub mod variable_assignment_line;
pub mod if_line;
pub mod function_line;
pub mod print_line;
pub mod call_line;
pub mod variable_initialisation_with_argument_line;
pub mod indexed_variable_assignment_line;
pub mod while_line;
pub(crate) mod break_line;

pub trait LineHandler {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers,
                    block_coordinator: &mut  BlockCoordinator) -> ProcessingResult;
}