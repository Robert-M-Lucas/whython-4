use crate::processing::blocks::BlockCoordinator;

use crate::processing::blocks::while_block::WhileBlock;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::{Block, Symbol};

pub struct WhileLine {}

impl LineHandler for WhileLine {
    fn process_line(
        line: &Vec<Symbol>,
        memory_managers: &mut MemoryManagers,
        block_coordinator: &mut BlockCoordinator,
    ) -> ProcessingResult {
        if line.is_empty() {
            return ProcessingResult::Unmatched;
        }

        match line[0] {
            Symbol::Block(block) => match block {
                Block::While => {
                    match block_coordinator.add_block_handler(
                        WhileBlock::new(),
                        memory_managers,
                        line,
                    ) {
                        Err(e) => ProcessingResult::Failure(e),
                        Ok(_) => ProcessingResult::Success,
                    }
                }
                _ => ProcessingResult::Unmatched,
            },
            _ => ProcessingResult::Unmatched,
        }
    }
}
