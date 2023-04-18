use crate::processing::blocks::BlockCoordinator;
use crate::processing::instructions::print_chars_instruction_9::PrintCharsInstruction;
use crate::processing::instructions::print_instruction_5::PrintInstruction;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::lines::LineHandler;
use crate::processing::processor::{MemoryManagers, ProcessingResult};
use crate::processing::symbols::{Builtin, Symbol};

pub struct PrintLine {}

impl LineHandler for PrintLine {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers,
                    block_coordinator: &mut BlockCoordinator) -> ProcessingResult {
        if line.len() == 0 { return ProcessingResult::Unmatched; }

        match line[0] {
            Symbol::Builtin(builtin) => {
                match builtin {
                    Builtin::Print => {
                        if line.len() == 1 { return ProcessingResult::Failure("'print' must be followed by something to print".to_string()) }
                        match handle_arithmetic_section(memory_managers, block_coordinator.get_reference_stack(), &line[1..], None, true) {
                            Err(e) => ProcessingResult::Failure(e),
                            Ok(value) => {
                                PrintInstruction::new_alloc(memory_managers, &value.unwrap());
                                ProcessingResult::Success
                            }
                        }
                    },
                    Builtin::PrintChars => {
                        if line.len() == 1 { return ProcessingResult::Failure("'printc' must be followed by something to print".to_string()) }
                        match handle_arithmetic_section(memory_managers, block_coordinator.get_reference_stack(), &line[1..], None, true) {
                            Err(e) => ProcessingResult::Failure(e),
                            Ok(value) => {
                                PrintCharsInstruction::new_alloc(memory_managers, &value.unwrap());
                                ProcessingResult::Success
                            }
                        }
                    },
                    _ => ProcessingResult::Unmatched
                }
            }
            _ => ProcessingResult::Unmatched
        }
    }
}