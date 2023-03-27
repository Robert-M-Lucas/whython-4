use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::Symbol;

pub mod block_line;
pub mod variable_initialisation_line;

pub trait LineHandler {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers) -> ProcessingResult;
}