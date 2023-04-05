use crate::processing::block_handler::BlockCoordinator;
use crate::processing::lines::arithmetic::handle_arithmetic_section;
use crate::processing::processor::MemoryManagers;
use crate::processing::processor::ProcessingResult;
use crate::processing::symbols::Symbol;
use super::LineHandler;

pub struct VariableInitialisationLine {}

impl LineHandler for VariableInitialisationLine {
    fn process_line(line: &Vec<Symbol>, memory_managers: &mut MemoryManagers, block_coordinator: &mut BlockCoordinator) -> ProcessingResult {
        if line.len() == 0 || !matches!(line[0], Symbol::Type(_)) { return ProcessingResult::Unmatched; }

        if line.len() < 4 {
            return ProcessingResult::Failure(
                "Type must be followed by a Name, '=' and value to initialise a variable"
                    .to_string());
        }

        let name = match &line[1] {
            Symbol::Name(name) => name,
            _ => return ProcessingResult::Failure(
                    "Type must be followed by a Name to initialise a variable".to_string())
        };

        let mut object = match handle_arithmetic_section(memory_managers, block_coordinator, &line[3..]) {
            Err(e) => return ProcessingResult::Failure(e),
            Ok(value) => value
        };

        object.set_name(name.clone());
        match block_coordinator.register_variable(object, name.clone()) {
            Err(e) => return ProcessingResult::Failure(e),
            Ok(_) => { }
        };

        ProcessingResult::Success
    }
}