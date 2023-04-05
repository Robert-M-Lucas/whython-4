use crate::processing::block_handler::BlockCoordinator;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::Symbol;

pub mod block_line;
pub mod variable_initialisation_line;
mod arithmetic;

pub trait LineHandler {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers, block_coordinator: &mut  BlockCoordinator) -> ProcessingResult;
}