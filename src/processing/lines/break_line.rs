use crate::processing::blocks::BlockCoordinator;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::{Keyword, Symbol};

pub struct BreakLine {}

impl LineHandler for BreakLine {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers,
                    block_coordinator: &mut BlockCoordinator) -> ProcessingResult {
        if line.len() == 0 { return ProcessingResult::Unmatched; }

        #[allow(unreachable_patterns)]
        match line[0] {
            Symbol::Keyword(keyword) => match keyword {
                Keyword::Break => {},
                _ => return ProcessingResult::Unmatched
            },
            _ => return ProcessingResult::Unmatched
        };

        match block_coordinator.break_block_handler(memory_managers) {
            Ok(_) => ProcessingResult::Success,
            Err(e) => ProcessingResult::Failure(e)
        }
    }
}