use crate::processing::blocks::BlockCoordinator;
use crate::processing::blocks::function_block::FunctionBlock;
use crate::processing::blocks::if_block::IfBlock;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::{Block, Symbol};

pub struct FunctionLine {}

impl LineHandler for FunctionLine {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers,
                    block_coordinator: &mut BlockCoordinator) -> ProcessingResult {
        if line.len() == 0 { return ProcessingResult::Unmatched; }

        match line[0] {
            Symbol::Block(block) => {
                match block {
                    Block::Function => {
                        match block_coordinator.add_block_handler(FunctionBlock::new(), memory_managers, line) {
                            Err(e) => ProcessingResult::Failure(e),
                            Ok(_) => ProcessingResult::Success
                        }
                    },
                    _ => ProcessingResult::Unmatched
                }
            },
            _ => ProcessingResult::Unmatched
        }
    }
}