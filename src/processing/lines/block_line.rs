// use crate::processing::processor::{MemoryManagers, ProcessingResult};
// use crate::processing::symbols::Symbol;
// use super::LineHandler;
//
// pub struct BlockLine {}
//
// impl LineHandler for BlockLine {
//     fn process_line(line: &Vec<Symbol>, _memory_managers: &mut MemoryManagers) -> ProcessingResult {
//         if line.len() == 0 || !matches!(line[0], Symbol::Block(_)) { return ProcessingResult::Unmatched; }
//
//         return ProcessingResult::Failure("Not implemented".to_string());
//     }
// }